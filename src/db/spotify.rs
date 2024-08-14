use entity::{album, album_artist, artist};
use migration::{Expr, IntoCondition, OnConflict};
use sea_orm::{ActiveValue, Condition, EntityTrait, QueryFilter, TransactionTrait};
use tokio::task::JoinSet;
use tracing::{debug, error};

use crate::db::{get_connection, DBError};

/// A function for upserting artists into the database
/// Returns all the IDs of the artists that were upserted
pub async fn upsert_artists(
    artists: Vec<artist::ActiveModel>,
) -> Result<Vec<artist::Model>, DBError> {
    let conn = get_connection().await?;
    // Insert_many the artists into the database
    let res = artist::Entity::insert_many(artists.clone())
        .on_conflict(
            OnConflict::column(artist::Column::Name)
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(&conn)
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
                .all(&conn)
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
/// Returns all the IDs of the albums and album_artists that were upserted
/// The album_artists are the artists that are associated with the album
pub async fn upsert_albums_with_artists(
    albums_with_artists: Vec<(album::ActiveModel, Vec<artist::Model>)>,
) -> Result<Vec<album::Model>, DBError> {
    // First, create individual queries for each album with its artists
    type AlbumSaveResult = JoinSet<Result<album::Model, DBError>>;
    let mut album_queries: AlbumSaveResult = JoinSet::new();
    for (album, artists) in albums_with_artists {
        let album_query = insert_album_with_artists(album, artists);
        album_queries.spawn(album_query);
    }
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
) -> Result<album::Model, DBError> {
    let conn = get_connection().await?;
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
