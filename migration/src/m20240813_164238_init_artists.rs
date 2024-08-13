use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Artist::Table)
                    .if_not_exists()
                    .col(pk_auto(Artist::Id))
                    .col(string(Artist::Name))
                    .col(
                        ColumnDef::new(Artist::CreatedAt)
                            .not_null()
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Artist::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Artist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Artist {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
