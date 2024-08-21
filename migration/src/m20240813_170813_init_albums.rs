use crate::m20240813_164238_init_artists::Artist;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, create the `albums` table
        manager
            .create_table(
                Table::create()
                    .table(Album::Table)
                    .if_not_exists()
                    .col(pk_auto(Album::Id))
                    .col(string(Album::Title))
                    .col(date(Album::ReleaseDate))
                    .col(
                        ColumnDef::new(Album::CreatedAt)
                            .not_null()
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Album::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;
        // Next, create the junctions table between `albums` and `artists`
        // This stores the artists on a respective album
        manager
            .create_table(
                Table::create()
                    .table(AlbumArtist::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .name("pk_album_artist")
                            .col(AlbumArtist::AlbumId)
                            .col(AlbumArtist::ArtistId),
                    )
                    .col(ColumnDef::new(AlbumArtist::AlbumId).integer().not_null())
                    .col(ColumnDef::new(AlbumArtist::ArtistId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_artist_album_id")
                            .from(AlbumArtist::Table, AlbumArtist::AlbumId)
                            .to(Album::Table, Album::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_artist_artist_id")
                            .from(AlbumArtist::Table, AlbumArtist::ArtistId)
                            .to(Artist::Table, Artist::Id)
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
                    .table(Album::Table)
                    .table(AlbumArtist::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Album {
    Table,
    Id,
    Title,
    ReleaseDate,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum AlbumArtist {
    Table,
    AlbumId,
    ArtistId,
}
