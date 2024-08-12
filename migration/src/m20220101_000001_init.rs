use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the albums table first
        manager
            .create_table(
                Table::create()
                    .table(Album::Table)
                    .if_not_exists()
                    .col(pk_auto(Album::Id))
                    .col(string(Album::Title))
                    .col(string(Album::Type))
                    .col(date_time(Album::ReleaseDate))
                    .col(string(Album::Image))
                    .to_owned(),
            )
            .await?;
        // Create the Recent Tracks table
        manager
            .create_table(
                Table::create()
                    .table(RecentTrack::Table)
                    .if_not_exists()
                    .col(pk_auto(RecentTrack::Id))
                    .col(string(RecentTrack::Title))
                    .col(string(RecentTrack::Artist))
                    .col(date_time(RecentTrack::PlayedAt))
                    .col(string(RecentTrack::ProviderId))
                    .col(string(RecentTrack::Provider))
                    .col(integer(RecentTrack::AlbumId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_album_id")
                            .from(RecentTrack::Table, RecentTrack::AlbumId)
                            .to(Album::Table, Album::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the albums table first
        manager
            .drop_table(Table::drop().table(Album::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RecentTrack::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RecentTrack {
    Table,
    Id,
    Title,
    Artist,
    PlayedAt,
    ProviderId,
    Provider,
    AlbumId,
}

#[derive(DeriveIden)]
pub enum Album {
    Table,
    Id,
    Title,
    Type,
    ReleaseDate,
    Image,
}
