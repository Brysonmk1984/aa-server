use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            INSERT INTO nation_armies VALUES 
                -- 1
                (DEFAULT, 1, 13, 1000, 'Minute Men Militia'),
                --2
                (DEFAULT, 2, 13, 500, 'Minute Men Militia'),
                (DEFAULT, 2, 1, 1000, 'Peacekeeper Monks'),
                --3
                (DEFAULT, 3, 13, 500, 'Minute Men Militia'),
                (DEFAULT, 3, 3, 1000, 'North Watch Longbowmen'),
                (DEFAULT, 3, 16, 1500, 'Castlegate Crossbowmen'),
                --4
                (DEFAULT, 4, 13, 1000, 'Minute Men Militia'),
                (DEFAULT, 4, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 4, 11, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 4, 16, 500, 'Castlegate Crossbowmen'),
                --5
                (DEFAULT, 5, 5, 3000, 'Rōnin Immortals'),
                (DEFAULT, 5, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 5, 14, 1000, 'Death Dealer Assassins'),
                (DEFAULT, 5, 16, 500, 'Castlegate Crossbowmen'),
                --6
                (DEFAULT, 6, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 6, 3, 500, 'North Watch Longbowmen'),
                (DEFAULT, 6, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 6, 7, 500, 'Amazonian Huntresses'),
                --7
                (DEFAULT, 7, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 7, 8, 4000, 'Avian Cliff Dwellers'),
                --8
                (DEFAULT, 8, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 8, 6, 2000, 'Shinobi Martial Artists'),
                (DEFAULT, 8, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 8, 8, 1000, 'Avian Cliff Dwellers'),
                (DEFAULT, 8, 11, 3000, 'Barbarians of the Outer Steppe'),
                --9
                (DEFAULT, 9, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 9, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 9, 11, 5000, 'Barbarians of the Outer Steppe'),
                --10
                (DEFAULT, 10, 1, 4000, 'Peacekeeper Monks'),
                (DEFAULT, 10, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 10, 4, 1000, 'Highborn Cavalry'),
                (DEFAULT, 10, 9, 3000, 'Magi Enforcers'),
                (DEFAULT, 10, 12, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 10, 16, 3000, 'Castlegate Crossbowmen'),
                --11
                (DEFAULT, 11, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 11, 7, 3000, 'Amazonian Huntresses'),
                (DEFAULT, 11, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 11, 10, 3000, 'Skull Clan Death Cultists'),
                (DEFAULT, 11, 11, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 11, 14, 2000, 'Death Dealer Assassins'),
                --12
                (DEFAULT, 12, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 12, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 12, 12, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 12, 15, 8500, 'Elven Archers'),
                --13
                (DEFAULT, 13, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 13, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 13, 11, 10000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 13, 12, 1500, 'Oath-Sworn Knights'),
                --14
                (DEFAULT, 14, 2, 1000, 'Imperial Legionnaires'),
                (DEFAULT, 14, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 14, 6, 3000, 'Shinobi Martial Artists'),
                (DEFAULT, 14, 10, 7000, 'Skull Clan Death Cultists'),
                (DEFAULT, 14, 12, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 14, 15, 4000, 'Elven Archers'),
                --15
                (DEFAULT, 15, 3, 10000, 'North Watch Longbowmen'),
                (DEFAULT, 15, 5, 20000, 'Rōnin Immortals'),
                --16
                (DEFAULT, 16, 1, 10000, 'Peacekeeper Monks'),
                (DEFAULT, 16, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 16, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 16, 16, 8000, 'Castlegate Crossbowmen'),
                --17
                (DEFAULT, 17, 1, 6000, 'Peacekeeper Monks'),
                (DEFAULT, 17, 2, 10000, 'Imperial Legionnaires'),
                (DEFAULT, 17, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 17, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 17, 9, 1000, 'Magi Enforcers'),
                (DEFAULT, 17, 11, 2000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 17, 15, 6000, 'Elven Archers'),
                --18
                (DEFAULT, 18, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 18, 4, 20000, 'Highborn Cavalry'),
                (DEFAULT, 18, 16, 9500, 'Castlegate Crossbowmen'),
                (DEFAULT, 18, 15, 5000, 'Elven Archers'),
                --19
                (DEFAULT, 19, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 19, 10, 17000, 'Skull Clan Death Cultists'),
                --20
                (DEFAULT, 20, 5, 15000, 'Rōnin Immortals'),
                (DEFAULT, 20, 6, 15000, 'Shinobi Martial Artists'),
                (DEFAULT, 20, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 20, 14, 2000, 'Death Dealer Assassins'),
                (DEFAULT, 20, 15, 8000, 'Elven Archers'),
                --21
                (DEFAULT, 21, 1, 17000, 'Peacekeeper Monks'),
                (DEFAULT, 21, 6, 30000, 'Shinobi Martial Artists'),
                --22
                (DEFAULT, 22, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 22, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 22, 7, 8000, 'Amazonian Huntresses'),
                (DEFAULT, 22, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 22, 11, 8000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 22, 12, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 22, 14, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 22, 15, 5000, 'Elven Archers'),
                --23
                (DEFAULT, 23, 2, 12000, 'Imperial Legionnaires'),
                (DEFAULT, 23, 3, 12000, 'North Watch Longbowmen'),
                (DEFAULT, 23, 9, 22000, 'Magi Enforcers'),
                (DEFAULT, 23, 16, 6000, 'Castlegate Crossbowmen'),
                --24
                (DEFAULT, 24, 7, 40000, 'Amazonian Huntresses'),
                (DEFAULT, 24, 15, 15000, 'Elven Archers'),
                --25
                (DEFAULT, 25, 5, 10000, 'Rōnin Immortals'),
                (DEFAULT, 25, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 25, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 25, 8, 11000, 'Avian Cliff Dwellers'),
                (DEFAULT, 25, 9, 10000, 'Magi Enforcers'),
                (DEFAULT, 25, 10, 10000, 'Skull Clan Death Cultists'),
                --26
                (DEFAULT, 26, 2, 15000, 'Imperial Legionnaires'),
                (DEFAULT, 26, 4, 15000, 'Highborn Cavalry'),
                (DEFAULT, 26, 8, 4000, 'Avian Cliff Dwellers'),
                (DEFAULT, 26, 12, 31000, 'Oath-Sworn Knights'),
                --27
                (DEFAULT, 27, 10, 40000, 'Skull Clan Death Cultists'),
                (DEFAULT, 27, 14, 30000, 'Death Dealer Assassins'),
                --28
                (DEFAULT, 28, 1, 50000, 'Peacekeeper Monks'),
                (DEFAULT, 28, 10, 16000, 'Skull Clan Death Cultists'),
                (DEFAULT, 28, 15, 10000, 'Elven Archers'),
                --29
                (DEFAULT, 29, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 29, 10, 20000, 'Skull Clan Death Cultists'),
                (DEFAULT, 29, 12, 40000, 'Oath-Sworn Knights'),
                --30
                (DEFAULT, 30, 1, 5000, 'Peacekeeper Monks'),
                (DEFAULT, 30, 2, 25000, 'Imperial Legionnaires'),
                (DEFAULT, 30, 3, 4000, 'North Watch Longbowmen'),
                (DEFAULT, 30, 4, 2000, 'Highborn Cavalry'),
                (DEFAULT, 30, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 30, 6, 5000, 'Shinobi Martial Artists'),
                (DEFAULT, 30, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 30, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 30, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 30, 10, 10000, 'Skull Clan Death Cultists'),
                (DEFAULT, 30, 11, 5000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 30, 12, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 30, 13, 1000, 'Minute Men Militia'),
                (DEFAULT, 30, 14, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 30, 15, 4000, 'Elven Archers'),
                (DEFAULT, 30, 16, 4000, 'Castlegate Crossbowmen')
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DELETE FROM nation_armies 
                --1
                    WHERE nation_id=67
                --2
                    OR nation_id=68
                --3
                    OR nation_id=69
                --4
                    OR nation_id=70
                --5
                    OR nation_id=71
                --6
                    OR nation_id=72
                --7
                    OR nation_id=73
                --8
                    OR nation_id=74
                --9
                    OR nation_id=75
                --10
                    OR nation_id=76
                --11
                    OR nation_id=77
                --12
                    OR nation_id=78
                --13
                    OR nation_id=79
                --14
                    OR nation_id=80
                --15
                    OR nation_id=81
                --16
                    OR nation_id=82
                --17
                    OR nation_id=83
                --18
                    OR nation_id=84
                --19
                    OR nation_id=85
                --20
                    OR nation_id=86
                --21
                    OR nation_id=87
                --22
                    OR nation_id=88
                --23
                    OR nation_id=89
                --24
                    OR nation_id=90
                --25
                    OR nation_id=91
                --26
                    OR nation_id=92
                --27
                    OR nation_id=93
                --28
                    OR nation_id=94
                --29
                    OR nation_id=95
                --30
                    OR nation_id=96
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
