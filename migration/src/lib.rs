pub use sea_orm_migration::prelude::*;

mod m20240813_164238_init_artists;
mod m20240813_170813_init_albums;
mod m20240813_170819_init_tracks;
mod m20240813_170827_init_playlog;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240813_164238_init_artists::Migration),
            Box::new(m20240813_170813_init_albums::Migration),
            Box::new(m20240813_170819_init_tracks::Migration),
            Box::new(m20240813_170827_init_playlog::Migration),
        ]
    }
}
