use entity::{account, user};
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};

/// Options when creating a user with an account
struct CreateUserOptions {
    email: String,
    name: String,
    access_token: String,
    refresh_token: String,
    provider: String,
    provider_id: String,
}

/// Create a user with an account connected
pub async fn create_user_with_account(
    conn: &DatabaseConnection,
    opts: CreateUserOptions,
) -> (user::Model, account::Model) {
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
        .expect("Failed to create user");
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
        .expect("Failed to create account");

    (user_model, account_model)
}
