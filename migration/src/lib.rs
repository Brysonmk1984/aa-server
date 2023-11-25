pub use sea_orm_migration::prelude::*;
mod m20220101_000001_create_table;
mod m20230623_232009_update_shield_rating;
mod m20230701_171647_attribute_fixes;
mod m20230701_180327_insert_remaining_armies;
mod m20230709_184100_col_change_pad_change;
mod m20230709_185756_size_col_name_change;
mod m20230813_144846_aoe_support;
mod m20230814_134648_switch_attack_speed;
mod m20230830_133912_add_nation_table;
mod m20230830_140228_add_unique_name_constraint;
mod m20230830_140229_add_unique_name_table_constraint;
mod m20230830_140230_add_nation_armies_table;
mod m20230923_140246_add_user_table;
mod m20230923_143157_update_nation_table_user_fk;
mod m20230924_233249_add_unique_sub_constriant;
mod m20230924_233539_add_unique_sub_table_constraint;
mod m20231024_131717_change_shinobi_name;
mod m20231024_132726_insert_remaining_armies;
mod m20231025_135151_update_old_armies_with_defaults;
mod m20231107_133838_insert_initial_nations;
mod m20231107_134927_insert_initial_nation_armies;
mod m20231114_145034_add_gold_column;
mod m20231118_182739_add_gold_to_nation;
mod m20231124_144641_add_battles_table;
mod m20231125_175020_add_is_npc;
mod m20231125_180001_add_campaign_level;
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
            Box::new(m20230814_134648_switch_attack_speed::Migration),
            Box::new(m20230830_133912_add_nation_table::Migration),
            Box::new(m20230830_140228_add_unique_name_constraint::Migration),
            Box::new(m20230830_140229_add_unique_name_table_constraint::Migration),
            Box::new(m20230830_140230_add_nation_armies_table::Migration),
            Box::new(m20230923_140246_add_user_table::Migration),
            Box::new(m20230923_143157_update_nation_table_user_fk::Migration),
            Box::new(m20230924_233249_add_unique_sub_constriant::Migration),
            Box::new(m20230924_233539_add_unique_sub_table_constraint::Migration),
            Box::new(m20231024_131717_change_shinobi_name::Migration),
            Box::new(m20231024_132726_insert_remaining_armies::Migration),
            Box::new(m20231025_135151_update_old_armies_with_defaults::Migration),
            Box::new(m20231107_133838_insert_initial_nations::Migration),
            Box::new(m20231107_134927_insert_initial_nation_armies::Migration),
            Box::new(m20231114_145034_add_gold_column::Migration),
            Box::new(m20231118_182739_add_gold_to_nation::Migration),
            Box::new(m20231124_144641_add_battles_table::Migration),
            Box::new(m20231125_175020_add_is_npc::Migration),
            Box::new(m20231125_180001_add_campaign_level::Migration),
        ]
    }
}
