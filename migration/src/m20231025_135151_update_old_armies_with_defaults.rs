use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql: &str = "
            UPDATE armies as a set
            name = c.name,
            lore = c.lore,
            count = c.count,
            shield_rating = c.shield_rating,
            flying = c.flying,
            range = c.range,
            accuracy = c.accuracy,
            weapon_type = c.weapon_type,
            armor_type = c.armor_type,
            agility = c.agility,
            speed = c.speed,
            aoe = c.aoe,
            spread = c.spread,
            attack_speed = c.attack_speed
        from (values
            ('Peacekeeper Monks', 'TBD', 100, 0.00, FALSE, 5, 1.00, 'blunt', 'unarmored', 0.25, 5, 0.00, 2.00, 2),
            ('Imperial Legionnaires', 'TBD', 100, 0.60, FALSE, 5, 0.80, 'piercing', 'plate', 0.10, 5, 0.00, 1.00, 1),
            ('North Watch Longbowmen', 'TBD', 100, 0.00, FALSE, 150, 0.60, 'piercing', 'unarmored', 0.10, 5, 0.00, 1.00, 1),
            ('Highborn Cavalry', 'TBD', 100, 0.50, FALSE, 10, 0.90, 'edged', 'plate', 0.25, 10, 0.50, 3.00, 1),
            ('Rōnin Immortals', 'TBD', 100, 0.00, FALSE, 5, 1.00, 'edged', 'chain', 0.35, 5, 0.00, 1.00, 2),
            ('Shinobi Martial Artists', 'TBD', 100, 0.00, FALSE, 5, 1.00, 'blunt', 'leather', 0.40, 5, 0.00, 1.00, 3),
            ('Amazonian Huntresses', 'TBD', 100, 0.15, FALSE, 15, 1.00, 'piercing', 'leather', 0.35, 10, 0.00, 1.00, 2),
            ('Avian Cliff Dwellers', 'TBD', 100, 0.00, TRUE, 15, 0.90, 'piercing', 'leather', 0.25, 10, 0.00, 3.00, 1),
            ('Magi Enforcers', 'TBD', 100, 0.00, FALSE, 15, 1.00, 'magic', 'chain', 0.20, 5, 1.00, 1.00, 1),
            ('Skull Clan Death Cultists', 'TBD', 100, 0.00, FALSE, 100, 0.75, 'magic', 'unarmored', 0.10, 5, 2.00, 1.00, 1)
        ) as c(name, lore, count, shield_rating, flying, range, accuracy, weapon_type, armor_type, agility, speed, aoe, spread, attack_speed) 
        where c.name = a.name;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql: &str = "
            UPDATE armies as a set
            name = c.name,
            lore = c.lore
            count = c.count
            shield_rating = c.shield_rating
            flying = c.flying
            range = c.range
            accuracy = c.accuracy
            weapon_type = c.weapon_type
            armor_type = c.armor_type
            agility = c.agility
            speed = c.speed
            aoe = c.aoe
            spread = c.spread
            attack_speed = c.attack_speed
        from (values
            ('Peacekeeper Monks', 'TBD', 1000, 0.00, FALSE, 5, 1.00, blunt, unarmored, 0.25, 5, 0.00, 2.00, 2),
            ('Imperial Legionnaire', 'TBD', 500, 0.75, FALSE, 10, 0.80, edged, plate, 0.10, 5, 0.00, 1.00, 1),
            ('North Watch Longbowmen', 'TBD', 750, 0.00, FALSE, 150, 0.50, piercing, unarmored, 0.10, 5, 0.00, 1.00, 1),
            ('Highborn Cavalry', 'TBD', 500, 0.50, FALSE, 10, 0.90, edged, plate, 0.25, 10, 0.50, 3.00, 1),
            ('Rōnin Immortals', 'TBD', 750, 0.00, FALSE, 5, 0.80, edged, plate, 0.35, 5, 0.00, 1.00, 2),
            ('Shinobi Martial Artists', 'TBD', 750, 0.00, FALSE, 5, 0.80, blunt, leather, 0.50, 5, 0.00, 1.00, 3),
            ('Amazonian Huntresses', 'TBD', 1000, 0.25, FALSE, 15, 0.90, piercing, leather, 0.35, 10, 0.00, 1.00, 2),
            ('Avian Cliff Dwellers', 'TBD', 750, 0.00, TRUE, 10, 0.80, edged, leather, 0.25, 10, 0.00, 3.00, 1),
            ('Magi Enforcers', 'TBD', 500, 0.00, FALSE, 15, 1.00, magic, chain, 0.20, 5, 1.00, 1.00, 1),
            ('Skull Clan Death Cultists', 'TBD', 500, 0.00, FALSE, 100, 0.75, magic, unarmored, 0.10, 5, 2.00, 1.00, 1)
        ) as c(name, lore, count, shield_rating, flying, range, accuracy, weapon_type, armor_type, agility, speed, aoe, spread, attack_speed) 
        where c.id = a.id;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
