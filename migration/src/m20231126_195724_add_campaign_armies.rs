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
                (DEFAULT, 67, 19, 1000, 'Minute Men Militia'),
                --2
                (DEFAULT, 68, 19, 500, 'Minute Men Militia'),
                (DEFAULT, 68, 1, 1000, 'Peacekeeper Monks'),
                --3
                (DEFAULT, 69, 19, 500, 'Minute Men Militia'),
                (DEFAULT, 69, 3, 1000, 'North Watch Longbowmen'),
                (DEFAULT, 69, 22, 1500, 'Castlegate Crossbowmen'),
                --4
                (DEFAULT, 70, 19, 1000, 'Minute Men Militia'),
                (DEFAULT, 70, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 70, 17, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 70, 22, 500, 'Castlegate Crossbowmen'),
                --5
                (DEFAULT, 71, 5, 3000, 'Rōnin Immortals'),
                (DEFAULT, 71, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 71, 20, 1000, 'Death Dealer Assassins'),
                (DEFAULT, 71, 22, 500, 'Castlegate Crossbowmen'),
                --6
                (DEFAULT, 72, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 72, 3, 500, 'North Watch Longbowmen'),
                (DEFAULT, 72, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 72, 18, 500, 'Oath-Sworn Knights'),
                --7
                (DEFAULT, 73, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 73, 8, 4000, 'Avian Cliff Dwellers'),
                --8
                (DEFAULT, 74, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 74, 6, 2000, 'Shinobi Martial Artists'),
                (DEFAULT, 74, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 74, 8, 1000, 'Avian Cliff Dwellers'),
                (DEFAULT, 74, 17, 3000, 'Barbarians of the Outer Steppe'),
                --9
                (DEFAULT, 75, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 75, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 75, 17, 5000, 'Barbarians of the Outer Steppe'),
                --10
                (DEFAULT, 76, 1, 4000, 'Peacekeeper Monks'),
                (DEFAULT, 76, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 76, 4, 1000, 'Highborn Cavalry'),
                (DEFAULT, 76, 9, 3000, 'Magi Enforcers'),
                (DEFAULT, 76, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 76, 22, 3000, 'Castlegate Crossbowmen'),
                --11
                (DEFAULT, 77, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 77, 7, 3000, 'Amazonian Huntresses'),
                (DEFAULT, 77, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 77, 10, 3000, 'Skull Clan Death Cultists'),
                (DEFAULT, 77, 17, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 77, 20, 2000, 'Death Dealer Assassins'),
                --12
                (DEFAULT, 78, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 78, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 78, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 78, 21, 8500, 'Elven Archers'),
                --13
                (DEFAULT, 79, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 79, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 79, 17, 10000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 79, 18, 1500, 'Oath-Sworn Knights'),
                --14
                (DEFAULT, 80, 2, 1000, 'Imperial Legionnaires'),
                (DEFAULT, 80, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 80, 6, 3000, 'Shinobi Martial Artists'),
                (DEFAULT, 80, 10, 7000, 'Skull Clan Death Cultists'),
                (DEFAULT, 80, 18, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 80, 21, 4000, 'Elven Archers'),
                --15
                (DEFAULT, 81, 3, 10000, 'North Watch Longbowmen'),
                (DEFAULT, 81, 5, 20000, 'Rōnin Immortals'),
                --16
                (DEFAULT, 82, 1, 10000, 'Peacekeeper Monks'),
                (DEFAULT, 82, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 82, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 82, 22, 8000, 'Castlegate Crossbowmen'),
                --17
                (DEFAULT, 83, 1, 6000, 'Peacekeeper Monks'),
                (DEFAULT, 83, 2, 10000, 'Imperial Legionnaires'),
                (DEFAULT, 83, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 83, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 83, 9, 1000, 'Magi Enforcers'),
                (DEFAULT, 83, 17, 2000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 83, 21, 6000, 'Elven Archers'),
                --18
                (DEFAULT, 84, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 84, 4, 20000, 'Highborn Cavalry'),
                (DEFAULT, 84, 22, 9500, 'Castlegate Crossbowmen'),
                (DEFAULT, 84, 21, 5000, 'Elven Archers'),
                --19
                (DEFAULT, 85, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 85, 10, 17000, 'Skull Clan Death Cultists'),
                --20
                (DEFAULT, 86, 5, 15000, 'Rōnin Immortals'),
                (DEFAULT, 86, 6, 15000, 'Shinobi Martial Artists'),
                (DEFAULT, 86, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 86, 20, 2000, 'Death Dealer Assassins'),
                (DEFAULT, 86, 21, 8000, 'Elven Archers'),
                --21
                (DEFAULT, 87, 1, 17000, 'Peacekeeper Monks'),
                (DEFAULT, 87, 6, 30000, 'Shinobi Martial Artists'),
                --22
                (DEFAULT, 88, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 88, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 88, 7, 8000, 'Amazonian Huntresses'),
                (DEFAULT, 88, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 88, 17, 8000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 88, 18, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 88, 20, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 88, 21, 5000, 'Elven Archers'),
                --23
                (DEFAULT, 89, 2, 12000, 'Imperial Legionnaires'),
                (DEFAULT, 89, 3, 12000, 'North Watch Longbowmen'),
                (DEFAULT, 89, 9, 22000, 'Magi Enforcers'),
                (DEFAULT, 89, 22, 6000, 'Castlegate Crossbowmen'),
                --24
                (DEFAULT, 90, 7, 40000, 'Amazonian Huntresses'),
                (DEFAULT, 90, 21, 15000, 'Elven Archers'),
                --25
                (DEFAULT, 91, 5, 10000, 'Rōnin Immortals'),
                (DEFAULT, 91, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 91, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 91, 8, 11000, 'Avian Cliff Dwellers'),
                (DEFAULT, 91, 9, 10000, 'Magi Enforcers'),
                (DEFAULT, 91, 10, 10000, 'Skull Clan Death Cultists'),
                --26
                (DEFAULT, 92, 2, 15000, 'Imperial Legionnaires'),
                (DEFAULT, 92, 4, 15000, 'Highborn Cavalry'),
                (DEFAULT, 92, 8, 4000, 'Avian Cliff Dwellers'),
                (DEFAULT, 92, 18, 31000, 'Oath-Sworn Knights'),
                --27
                (DEFAULT, 93, 10, 40000, 'Skull Clan Death Cultists'),
                (DEFAULT, 93, 20, 30000, 'Death Dealer Assassins'),
                --28
                (DEFAULT, 94, 1, 50000, 'Peacekeeper Monks'),
                (DEFAULT, 94, 10, 16000, 'Skull Clan Death Cultists'),
                (DEFAULT, 94, 21, 10000, 'Elven Archers'),
                --29
                (DEFAULT, 95, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 95, 10, 20000, 'Skull Clan Death Cultists'),
                (DEFAULT, 95, 18, 40000, 'Oath-Sworn Knights'),
                --30
                (DEFAULT, 96, 19, 1000, 'Minute Men Militia'),
                (DEFAULT, 96, 1, 5000, 'Peacekeeper Monks'),
                (DEFAULT, 96, 2, 25000, 'Imperial Legionnaires'),
                (DEFAULT, 96, 3, 4000, 'North Watch Longbowmen'),
                (DEFAULT, 96, 4, 2000, 'Highborn Cavalry'),
                (DEFAULT, 96, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 96, 6, 5000, 'Shinobi Martial Artists'),
                (DEFAULT, 96, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 96, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 96, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 96, 10, 10000, 'Skull Clan Death Cultists'),
                (DEFAULT, 96, 17, 5000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 96, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 96, 20, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 96, 22, 4000, 'Castlegate Crossbowmen'),
                (DEFAULT, 96, 21, 4000, 'Elven Archers')
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
