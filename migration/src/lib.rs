pub use sea_orm_migration::prelude::*;
mod m20220101_000001_create_table;
mod m20230623_232009_update_shield_rating;
mod m20230701_171647_attribute_fixes;
mod m20230701_180327_insert_remaining_armies;
mod m20230709_184100_col_change_pad_change;
mod m20230709_185756_size_col_name_change;
mod m20230813_144846_aoe_support;
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
            Box::new(m20230709_184100_col_change_pad_change::Migration),
            Box::new(m20230709_185756_size_col_name_change::Migration),
            Box::new(m20230813_144846_aoe_support::Migration),
        ]
    }
}
