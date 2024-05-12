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
mod m20230924_233249_add_unique_sub_constraint;
mod m20230924_233539_add_unique_sub_table_constraint;
mod m20231024_132726_insert_remaining_armies;
mod m20231025_135151_update_old_armies_with_defaults;
mod m20231114_145034_add_gold_column;
mod m20231124_144641_add_battles_table;
mod m20231125_213302_add_unique_nation_name_constraint;
mod m20231125_214149_add_campaign_levels_table;
mod m20231125_223357_add_nation_campaign_levels_table;
mod m20231125_225246_add_nation_campaign_level_to_battles_table;
mod m20231126_144923_add_campaign_nations;
mod m20231126_195724_add_campaign_armies;
mod m20231203_211014_add_campaign_levels;
mod m20240204_203833_add_update_procedure;
mod m20240204_203834_add_level_completed_to_campaign_levels;
mod m20240205_142814_add_date_to_nation_armies;
mod m20240205_143646_add_date_to_battles;
mod m20240205_143748_add_date_to_nations;
mod m20240205_143859_add_date_to_users;
mod m20240205_150052_add_update_function_call_nation_cl;
mod m20240206_141829_add_update_2;
mod m20240206_141911_add_update_3;
mod m20240206_142013_add_update_4;
mod m20240206_142039_add_update_5;
mod m20240302_174457_add_col_unlock_level_to_armies;
mod m20240302_175430_add_add_level_value_to_unlock_level_col;
mod m20240302_182213_insert_army_lore_values;
mod m20240304_135636_create_weapon_armor_table;
mod m20240304_140403_insert_weapon_armor_values;
mod m20240502_131418_remove_nation_name_requirement;
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
            Box::new(m20230924_233249_add_unique_sub_constraint::Migration),
            Box::new(m20230924_233539_add_unique_sub_table_constraint::Migration),
            Box::new(m20231024_132726_insert_remaining_armies::Migration),
            Box::new(m20231025_135151_update_old_armies_with_defaults::Migration),
            Box::new(m20231114_145034_add_gold_column::Migration),
            Box::new(m20231124_144641_add_battles_table::Migration),
            Box::new(m20231125_213302_add_unique_nation_name_constraint::Migration),
            Box::new(m20231125_214149_add_campaign_levels_table::Migration),
            Box::new(m20231125_223357_add_nation_campaign_levels_table::Migration),
            Box::new(m20231125_225246_add_nation_campaign_level_to_battles_table::Migration),
            Box::new(m20231126_144923_add_campaign_nations::Migration),
            Box::new(m20231126_195724_add_campaign_armies::Migration),
            Box::new(m20231203_211014_add_campaign_levels::Migration),
            Box::new(m20240204_203833_add_update_procedure::Migration),
            Box::new(m20240204_203834_add_level_completed_to_campaign_levels::Migration),
            Box::new(m20240205_142814_add_date_to_nation_armies::Migration),
            Box::new(m20240205_143646_add_date_to_battles::Migration),
            Box::new(m20240205_143748_add_date_to_nations::Migration),
            Box::new(m20240205_143859_add_date_to_users::Migration),
            Box::new(m20240205_150052_add_update_function_call_nation_cl::Migration),
            Box::new(m20240206_141829_add_update_2::Migration),
            Box::new(m20240206_141911_add_update_3::Migration),
            Box::new(m20240206_142013_add_update_4::Migration),
            Box::new(m20240206_142039_add_update_5::Migration),
            Box::new(m20240302_174457_add_col_unlock_level_to_armies::Migration),
            Box::new(m20240302_175430_add_add_level_value_to_unlock_level_col::Migration),
            Box::new(m20240302_182213_insert_army_lore_values::Migration),
            Box::new(m20240304_135636_create_weapon_armor_table::Migration),
            Box::new(m20240304_140403_insert_weapon_armor_values::Migration),
            Box::new(m20240502_131418_remove_nation_name_requirement::Migration),
        ]
    }
}
