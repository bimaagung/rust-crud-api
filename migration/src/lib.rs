pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260922_124400_add_users;
mod m20260927_124748_add_role_to_user_and_author_id_to_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260922_124400_add_users::Migration),
            Box::new(m20260927_124748_add_role_to_user_and_author_id_to_posts::Migration),
        ]
    }
}
