pub use sea_orm_migration::prelude::*;
mod m20220101_000001_create_table;
mod m20230623_232009_update_shield_rating;
mod m20230701_171647_attribute_fixes;
mod m20230701_180327_insert_remaining_armies;
mod utils;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230623_232009_update_shield_rating::Migration),
            Box::new(m20230701_171647_attribute_fixes::Migration),
            Box::new(m20230701_180327_insert_remaining_armies::Migration),
        ]
    }
}
