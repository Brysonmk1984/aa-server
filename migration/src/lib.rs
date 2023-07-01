pub use sea_orm_migration::prelude::*;
mod m20220101_000001_create_table;
mod m20230623_232009_update_shield_rating;
mod m20230624_132859_add_monk;
mod m20230624_140621_add_imperial_legionnaire;
mod m20230628_142201_north_glade_longbow_men;
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
            Box::new(m20230624_132859_add_monk::Migration),
            Box::new(m20230624_140621_add_imperial_legionnaire::Migration),
            Box::new(m20230628_142201_north_glade_longbow_men::Migration),
            Box::new(m20230701_171647_attribute_fixes::Migration),
            Box::new(m20230701_180327_insert_remaining_armies::Migration),
        ]
    }
}
