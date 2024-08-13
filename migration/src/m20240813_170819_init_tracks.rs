use crate::m20240813_170813_init_albums::Album;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, create the tracks table for storing individual tracks
        manager
            .create_table(
                Table::create()
                    .table(Track::Table)
                    .if_not_exists()
                    .col(pk_auto(Track::Id))
                    .col(string(Track::Title))
                    .col(
                        ColumnDef::new(Track::CreatedAt)
                            .not_null()
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Track::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;
        // Next, create the junctions table to store the relationship between tracks and albums
        manager
            .create_table(
                Table::create()
                    .table(AlbumTrack::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk_album_track")
                            .col(AlbumTrack::AlbumId)
                            .col(AlbumTrack::TrackId),
                    )
                    .col(ColumnDef::new(AlbumTrack::AlbumId).integer().not_null())
                    .col(ColumnDef::new(AlbumTrack::TrackId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_track_album_id")
                            .from(AlbumTrack::Table, AlbumTrack::AlbumId)
                            .to(Album::Table, Album::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_track_track_id")
                            .from(AlbumTrack::Table, AlbumTrack::TrackId)
                            .to(Track::Table, Track::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Track::Table)
                    .table(AlbumTrack::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Track {
    Table,
    Id,
    Title,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AlbumTrack {
    Table,
    AlbumId,
    TrackId,
}
