use crate::m20240813_170819_init_tracks::Track;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayLog::Table)
                    .if_not_exists()
                    .col(pk_auto(PlayLog::Id))
                    .col(ColumnDef::new(PlayLog::TrackId).integer().not_null())
                    .col(
                        ColumnDef::new(PlayLog::PlayedAt)
                            .not_null()
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_play_log_track_id")
                            .from(PlayLog::Table, PlayLog::TrackId)
                            .to(Track::Table, Track::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlayLog::Table).to_owned())
            .await
    }
}

// The Playlog represents a played track
#[derive(DeriveIden)]
enum PlayLog {
    Table,
    Id,
    TrackId,
    PlayedAt,
}
