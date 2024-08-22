use entity::{album, album_artist, album_track, artist, play_log, track};
use migration::{Expr, IntoCondition, OnConflict};
use sea_orm::{
    ActiveValue, Condition, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use tokio::task::JoinSet;
use tracing::{debug, error};

use crate::db::DBError;

/// A function for upserting artists into the database
/// Returns all the IDs of the artists that were upserted
pub async fn upsert_artists(
    artists: Vec<artist::ActiveModel>,
    conn: &DatabaseConnection,
) -> Result<Vec<artist::Model>, DBError> {
    // Insert_many the artists into the database
    let res = artist::Entity::insert_many(artists.clone())
        .on_conflict(
            OnConflict::column(artist::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(conn)
        .await;
    // If we successfully inserted the artists, return the models we inserted
    match res {
        Ok(_) => {
            debug!("Artists were inserted, looking up their IDs");
            // The names is what we key off of to get the ID's
            let artist_names: Vec<String> = artists
                .into_iter()
                .map(|artist| artist.name.unwrap())
                .collect();
            // NOTE: Currently, sea orm can't return all the rows that were inserted from an insert_many
            // so we do a lookup on the name
            artist::Entity::find()
                .filter(
                    Condition::all()
                        .add(
                            Expr::col(artist::Column::Name)
                                .is_in(artist_names)
                                .into_condition(),
                        )
                        .into_condition(),
                )
                .all(conn)
                .await
                .map_err(|sea_err| {
                    error!("Error looking up artists: {:?}", sea_err);
                    DBError
                })
        }
        Err(sea_err) => {
            error!("Error upserting artists: {:?}", sea_err);
            Err(DBError)
        }
    }
}

/// A function for upserting albums and their artists into the database
/// Returns all the IDs of the albums
/// The album_artists are the artists that are associated with the album
pub async fn upsert_albums_with_artists(
    albums_with_artists: Vec<(album::ActiveModel, Vec<artist::Model>)>,
    conn: &DatabaseConnection,
) -> Result<Vec<album::Model>, DBError> {
    // First, create individual queries for each album with its artists
    type AlbumSaveResult = JoinSet<Result<album::Model, DBError>>;
    let mut album_queries: AlbumSaveResult = JoinSet::new();
    albums_with_artists
        .into_iter()
        .for_each(|(album, artists)| {
            let album_conn = conn.clone();
            let album_query = insert_album_with_artists(album, artists, album_conn);
            album_queries.spawn(album_query);
        });
    // Execute each query, saving albums
    let mut albums: Vec<album::Model> = vec![];
    while let Some(res) = album_queries.join_next().await {
        match res {
            Ok(Ok(album_model)) => {
                debug!("Album was inserted: {:?}", album_model);
                albums.push(album_model);
            }
            Ok(Err(db_err)) => {
                error!("Error inserting album: {:?}", db_err);
                return Err(DBError);
            }
            Err(join_err) => {
                error!("Error joining album insert: {:?}", join_err);
                return Err(DBError);
            }
        }
    }
    // Return the albums that were inserted
    Ok(albums)
}

/// An internal function for composing a transaction to insert an album with its artists
async fn insert_album_with_artists(
    album: album::ActiveModel,
    artists: Vec<artist::Model>,
    conn: DatabaseConnection,
) -> Result<album::Model, DBError> {
    // Start a transaction
    let txn = conn.begin().await.map_err(|sea_err| {
        error!("Error starting transaction for album: {:?}", sea_err);
        DBError
    })?;
    // Insert the album
    let album_model = album::Entity::insert(album)
        .exec_with_returning(&txn)
        .await
        .map_err(|sea_err| {
            error!("Error inserting album: {:?}", sea_err);
            DBError
        })?;
    // Convert artists to album_artists
    let album_artists: Vec<album_artist::ActiveModel> = artists
        .into_iter()
        .map(|artist| album_artist::ActiveModel {
            album_id: ActiveValue::set(album_model.id),
            artist_id: ActiveValue::set(artist.id),
        })
        .collect();
    // Insert the album artists
    album_artist::Entity::insert_many(album_artists)
        .exec(&txn)
        .await
        .map_err(|sea_err| {
            error!("Error inserting album artists: {:?}", sea_err);
            DBError
        })?;
    // Commit the transaction
    txn.commit().await.map_err(|sea_err| {
        error!("Error committing transaction for album: {:?}", sea_err);
        DBError
    })?;
    Ok(album_model)
}

/// A function for upserting tracks with their albums
/// Returns all the IDs of the tracks that were upserted
/// The album_id is the ID of the album that the track is associated with
pub async fn upsert_tracks_with_albums(
    tracks_with_ablums: Vec<(track::ActiveModel, album::Model)>,
    conn: &DatabaseConnection,
) -> Result<Vec<track::Model>, DBError> {
    // First, create individual queries for each track with its album
    type TrackSaveResult = JoinSet<Result<track::Model, DBError>>;
    let mut track_queries: TrackSaveResult = JoinSet::new();
    tracks_with_ablums.into_iter().for_each(|(track, album)| {
        let track_conn = conn.clone();
        let track_query = insert_track_with_album(track, album, track_conn);
        track_queries.spawn(track_query);
    });
    // Execute each query, saving tracks
    let mut tracks: Vec<track::Model> = vec![];
    while let Some(res) = track_queries.join_next().await {
        match res {
            Ok(Ok(track_model)) => {
                debug!("Track was inserted: {:?}", track_model);
                tracks.push(track_model);
            }
            Ok(Err(db_err)) => {
                error!("Error inserting track: {:?}", db_err);
                return Err(DBError);
            }
            Err(join_err) => {
                error!("Error joining track insert: {:?}", join_err);
                return Err(DBError);
            }
        }
    }
    // Return the tracks that were inserted
    Ok(tracks)
}

/// An internal function for composing a transaction to insert a track with its album
/// The album_id is the ID of the album that the track is associated with
/// The track is the track to insert
async fn insert_track_with_album(
    track: track::ActiveModel,
    album: album::Model,
    conn: DatabaseConnection,
) -> Result<track::Model, DBError> {
    // Start a transaction
    let txn = conn.begin().await.map_err(|sea_err| {
        error!("Error starting transaction for track: {:?}", sea_err);
        DBError
    })?;
    // Insert the track
    let track_model = track::Entity::insert(track)
        .exec_with_returning(&txn)
        .await
        .map_err(|sea_err| {
            error!("Error inserting track: {:?}", sea_err);
            DBError
        })?;
    // Insert the track album
    album_track::Entity::insert(album_track::ActiveModel {
        track_id: ActiveValue::set(track_model.id),
        album_id: ActiveValue::set(album.id),
    })
    .exec(&txn)
    .await
    .map_err(|sea_err| {
        error!("Error inserting track album: {:?}", sea_err);
        DBError
    })?;
    // Commit the transaction
    txn.commit().await.map_err(|sea_err| {
        error!("Error committing transaction for track: {:?}", sea_err);
        DBError
    })?;
    Ok(track_model)
}

/// A function for upserting play logs
pub async fn upsert_playlogs(
    playlogs: Vec<play_log::ActiveModel>,
    conn: &DatabaseConnection,
) -> Result<(), DBError> {
    play_log::Entity::insert_many(playlogs)
        .on_conflict(
            OnConflict::column(play_log::Column::PlayedAt)
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(conn)
        .await
        .map_err(|db_err| {
            error!("Error inserting play logs: {:?}", db_err);
            DBError
        })
        .map(|_| {
            debug!("Inserted play logs");
        })
}
