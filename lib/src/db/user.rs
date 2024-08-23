use super::DBError;
use entity::{account, user};
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use tracing::error;

/// Options when creating a user with an account
pub struct CreateUserOptions {
    pub email: String,
    pub name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub provider: String,
    pub provider_id: String,
}

/// Create a user with an account connected
pub async fn create_user_with_account(
    conn: &DatabaseConnection,
    opts: CreateUserOptions,
) -> Result<(user::Model, account::Model), DBError> {
    // Create the user first
    let user = user::ActiveModel {
        id: NotSet,
        email: Set(opts.email),
        name: Set(opts.name),
        created_at: NotSet,
        updated_at: NotSet,
    };
    let user_model = user::Entity::insert(user)
        .exec_with_returning(conn)
        .await
        .map_err(|e| {
            error!("Failed to create user: {}", e);
            DBError
        })?;
    // Next, create the account connected by user_id
    let user_id = user_model.id.clone();
    let account = account::ActiveModel {
        id: NotSet,
        user_id: Set(user_id),
        access_token: Set(opts.access_token),
        refresh_token: Set(opts.refresh_token),
        provider: Set(opts.provider),
        provider_id: Set(opts.provider_id),
    };
    let account_model = account::Entity::insert(account)
        .exec_with_returning(conn)
        .await
        .map_err(|e| {
            error!("Failed to create account: {}", e);
            DBError
        })?;
    // Return both models for upstream use
    Ok((user_model, account_model))
}
