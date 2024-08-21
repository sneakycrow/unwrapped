use crate::m20240820_031732_init_users::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(pk_auto(Account::Id))
                    .col(string(Account::Provider))
                    .col(string(Account::ProviderId).unique_key())
                    .col(string(Account::AccessToken))
                    .col(string(Account::RefreshToken))
                    .col(ColumnDef::new(Account::UserId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("account_user_id")
                            .from(Account::Table, Account::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Provider,
    ProviderId,
    AccessToken,
    RefreshToken,
    UserId,
}
