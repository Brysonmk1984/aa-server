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
            -- name -- shield_rating -- flying -- range -- accuracy -- weapon_type -- armor_type -- agility -- speed -- aoe -- spread -- attack_speed --
            ('Peacekeeper Monks', 0.00, FALSE, 5, 1.00, 'blunt', 'unarmored', 0.25, 10, 0.00, 2.00, 2),
            ('Imperial Legionnaires', 0.65, FALSE, 10, 0.80, 'piercing', 'plate', 0.10, 5, 0.00, 1.00, 1),
            ('North Watch Longbowmen', 0.00, FALSE, 125, 0.60, 'piercing', 'unarmored', 0.10, 5, 0.00, 1.00, 1),
            ('Highborn Cavalry', 0.50, FALSE, 10, 0.80, 'edged', 'plate', 0.25, 15, 0.50, 3.00, 1),
            ('Rōnin Immortals', 0.00, FALSE, 5, 1.00, 'edged', 'chain', 0.35, 10, 0.00, 2.00, 2),
            -- name -- shield_rating -- flying -- range -- accuracy -- weapon_type -- armor_type -- agility -- speed -- aoe -- spread -- attack_speed --
            ('Shinobi Martial Artists', 0.00, FALSE, 5, 1.00, 'blunt', 'leather', 0.40, 10, 0.00, 2.00, 3),
            ('Amazonian Huntresses', 0.15, FALSE, 15, 1.00, 'piercing', 'leather', 0.35, 10, 0.00, 1.00, 2),
            ('Avian Cliff Dwellers', 0.00, TRUE, 15, 0.90, 'piercing', 'leather', 0.25, 15, 0.00, 3.00, 1),
            ('Magi Enforcers', 0.00, FALSE, 15, 1.00, 'magic', 'chain', 0.20, 10, 1.00, 2.00, 1),
            ('Skull Clan Death Cultists', 0.00, FALSE, 100, 0.75, 'magic', 'unarmored', 0.10, 10, 2.00, 1.00, 1),
            -- name -- shield_rating -- flying -- range -- accuracy -- weapon_type -- armor_type -- agility -- speed -- aoe -- spread -- attack_speed --
            ('Barbarians of the Outer Steppe', 0.00, FALSE, 5, 0.90, 'crushing', 'leather', 0.33, 10, 0.00, 2.00, 1),
            ('Oath-Sworn Knights', 0.40, FALSE, 5, 0.90, 'edged', 'plate', 0.15, 5, 0.00, 2.00, 1),
            ('Minute Men Militia', 0.00, FALSE, 5, 0.75, 'edged', 'unarmored', 0.25, 10, 0.00, 1.00, 1),
            ('Death Dealer Assassins', 0.00, FALSE, 10, 1.00, 'edged', 'unarmored', 0.35, 12, 0.00, 3.00, 2),
            ('Elven Archers', 0.00, FALSE, 75, 0.95, 'piercing', 'leather', 0.25, 12, 0.00, 2.00, 1),
            ('Castlegate Crossbowmen', 0.00, FALSE, 50, 0.70, 'piercing', 'chain', 0.15, 10, 0.00, 1.00, 2)
        ) as c(name, shield_rating, flying, range, accuracy, weapon_type, armor_type, agility, speed, aoe, spread, attack_speed) 
        where c.name = a.name;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql: &str = "
            UPDATE armies as a set
            name = c.name,
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
            -- name -- shield_rating -- flying -- range -- accuracy -- weapon_type -- armor_type -- agility -- speed -- aoe -- spread -- attack_speed --
            ('Peacekeeper Monks', 0.00, FALSE, 5, 1.00, 'blunt', 'unarmored', 0.25, 10, 0.00, 2.00, 2),
            ('Imperial Legionnaires', 0.65, FALSE, 10, 0.80, 'piercing', 'plate', 0.10, 5, 0.00, 1.00, 1),
            ('North Watch Longbowmen', 0.00, FALSE, 125, 0.60, 'piercing', 'unarmored', 0.10, 5, 0.00, 1.00, 1),
            ('Highborn Cavalry', 0.50, FALSE, 10, 0.80, 'edged', 'plate', 0.25, 15, 0.50, 3.00, 1),
            ('Rōnin Immortals', 0.00, FALSE, 5, 1.00, 'edged', 'chain', 0.35, 10, 0.00, 2.00, 2),
            -- name -- shield_rating -- flying -- range -- accuracy -- weapon_type -- armor_type -- agility -- speed -- aoe -- spread -- attack_speed --
            ('Shinobi Martial Artists', 0.00, FALSE, 5, 1.00, 'blunt', 'leather', 0.40, 10, 0.00, 2.00, 3),
            ('Amazonian Huntresses', 0.15, FALSE, 15, 1.00, 'piercing', 'leather', 0.35, 10, 0.00, 1.00, 2),
            ('Avian Cliff Dwellers', 0.00, TRUE, 15, 0.90, 'piercing', 'leather', 0.25, 15, 0.00, 3.00, 1),
            ('Magi Enforcers', 0.00, FALSE, 15, 1.00, 'magic', 'chain', 0.20, 10, 1.00, 2.00, 1),
            ('Skull Clan Death Cultists', 0.00, FALSE, 100, 0.75, 'magic', 'unarmored', 0.10, 10, 2.00, 1.00, 1),
            -- name -- shield_rating -- flying -- range -- accuracy -- weapon_type -- armor_type -- agility -- speed -- aoe -- spread -- attack_speed --
            ('Barbarians of the Outer Steppe', 0.00, FALSE, 5, 0.90, 'crushing', 'leather', 0.33, 10, 0.00, 2.00, 1),
            ('Oath-Sworn Knights', 0.40, FALSE, 5, 0.90, 'edged', 'plate', 0.15, 5, 0.00, 2.00, 1),
            ('Minute Men Militia', 0.00, FALSE, 5, 0.75, 'edged', 'unarmored', 0.25, 10, 0.00, 1.00, 1),
            ('Death Dealer Assassins', 0.00, FALSE, 10, 1.00, 'edged', 'unarmored', 0.35, 12, 0.00, 3.00, 2),
            ('Elven Archers', 0.00, FALSE, 75, 0.95, 'piercing', 'leather', 0.25, 12, 0.00, 2.00, 1),
            ('Castlegate Crossbowmen', 0.00, FALSE, 50, 0.70, 'piercing', 'chain', 0.15, 10, 0.00, 1.00, 2)
        ) as c(name, shield_rating, flying, range, accuracy, weapon_type, armor_type, agility, speed, aoe, spread, attack_speed) 
        where c.name = a.name;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
