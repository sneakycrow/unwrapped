use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecentTrack::Table)
                    .if_not_exists()
                    .col(pk_auto(RecentTrack::Id))
                    .col(string(RecentTrack::Title))
                    .col(string(RecentTrack::Artist))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecentTrack::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RecentTrack {
    Table,
    Id,
    Title,
    Artist,
}
