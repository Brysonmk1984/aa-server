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
                (DEFAULT, 103, 19, 1000, 'Minute Men Militia'),
                --2
                (DEFAULT, 104, 19, 500, 'Minute Men Militia'),
                (DEFAULT, 104, 1, 1000, 'Peacekeeper Monks'),
                --3
                (DEFAULT, 105, 19, 500, 'Minute Men Militia'),
                (DEFAULT, 105, 3, 1000, 'North Watch Longbowmen'),
                (DEFAULT, 105, 22, 1500, 'Castlegate Crossbowmen'),
                --4
                (DEFAULT, 106, 19, 1000, 'Minute Men Militia'),
                (DEFAULT, 106, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 106, 17, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 106, 22, 500, 'Castlegate Crossbowmen'),
                --5
                (DEFAULT, 107, 5, 3000, 'Rōnin Immortals'),
                (DEFAULT, 107, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 107, 20, 1000, 'Death Dealer Assassins'),
                (DEFAULT, 107, 22, 500, 'Castlegate Crossbowmen'),
                --6
                (DEFAULT, 108, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 108, 3, 500, 'North Watch Longbowmen'),
                (DEFAULT, 108, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 108, 18, 500, 'Oath-Sworn Knights'),
                --7
                (DEFAULT, 109, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 109, 8, 4000, 'Avian Cliff Dwellers'),
                --8
                (DEFAULT, 110, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 110, 6, 2000, 'Shinobi Martial Artists'),
                (DEFAULT, 110, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 110, 8, 1000, 'Avian Cliff Dwellers'),
                (DEFAULT, 110, 17, 3000, 'Barbarians of the Outer Steppe'),
                --9
                (DEFAULT, 111, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 111, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 111, 17, 5000, 'Barbarians of the Outer Steppe'),
                --10
                (DEFAULT, 112, 1, 4000, 'Peacekeeper Monks'),
                (DEFAULT, 112, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 112, 4, 1000, 'Highborn Cavalry'),
                (DEFAULT, 112, 9, 3000, 'Magi Enforcers'),
                (DEFAULT, 112, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 112, 22, 3000, 'Castlegate Crossbowmen'),
                --11
                (DEFAULT, 113, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 113, 7, 3000, 'Amazonian Huntresses'),
                (DEFAULT, 113, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 113, 10, 3000, 'Skull Clan Death Cultists'),
                (DEFAULT, 113, 17, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 113, 20, 2000, 'Death Dealer Assassins'),
                --12
                (DEFAULT, 114, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 114, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 114, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 114, 21, 8500, 'Elven Archers'),
                --13
                (DEFAULT, 115, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 115, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 115, 17, 10000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 115, 18, 1500, 'Oath-Sworn Knights'),
                --14
                (DEFAULT, 116, 2, 1000, 'Imperial Legionnaires'),
                (DEFAULT, 116, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 116, 6, 3000, 'Shinobi Martial Artists'),
                (DEFAULT, 116, 10, 7000, 'Skull Clan Death Cultists'),
                (DEFAULT, 116, 18, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 116, 21, 4000, 'Elven Archers'),
                --15
                (DEFAULT, 117, 3, 10000, 'North Watch Longbowmen'),
                (DEFAULT, 117, 5, 20000, 'Rōnin Immortals'),
                --16
                (DEFAULT, 118, 1, 10000, 'Peacekeeper Monks'),
                (DEFAULT, 118, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 118, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 118, 22, 8000, 'Castlegate Crossbowmen'),
                --17
                (DEFAULT, 119, 1, 6000, 'Peacekeeper Monks'),
                (DEFAULT, 119, 2, 10000, 'Imperial Legionnaires'),
                (DEFAULT, 119, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 119, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 119, 9, 1000, 'Magi Enforcers'),
                (DEFAULT, 119, 17, 2000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 119, 21, 6000, 'Elven Archers'),
                --18
                (DEFAULT, 120, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 120, 4, 20000, 'Highborn Cavalry'),
                (DEFAULT, 120, 22, 9500, 'Castlegate Crossbowmen'),
                (DEFAULT, 120, 21, 5000, 'Elven Archers'),
                --19
                (DEFAULT, 121, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 121, 10, 17000, 'Skull Clan Death Cultists'),
                --20
                (DEFAULT, 122, 5, 15000, 'Rōnin Immortals'),
                (DEFAULT, 122, 6, 15000, 'Shinobi Martial Artists'),
                (DEFAULT, 122, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 122, 20, 2000, 'Death Dealer Assassins'),
                (DEFAULT, 122, 21, 8000, 'Elven Archers'),
                --21
                (DEFAULT, 123, 1, 17000, 'Peacekeeper Monks'),
                (DEFAULT, 123, 6, 30000, 'Shinobi Martial Artists'),
                --22
                (DEFAULT, 124, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 124, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 124, 7, 8000, 'Amazonian Huntresses'),
                (DEFAULT, 124, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 124, 17, 8000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 124, 18, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 124, 20, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 124, 21, 5000, 'Elven Archers'),
                --23
                (DEFAULT, 125, 2, 12000, 'Imperial Legionnaires'),
                (DEFAULT, 125, 3, 12000, 'North Watch Longbowmen'),
                (DEFAULT, 125, 9, 22000, 'Magi Enforcers'),
                (DEFAULT, 125, 22, 6000, 'Castlegate Crossbowmen'),
                --24
                (DEFAULT, 126, 7, 40000, 'Amazonian Huntresses'),
                (DEFAULT, 126, 21, 15000, 'Elven Archers'),
                --25
                (DEFAULT, 127, 5, 10000, 'Rōnin Immortals'),
                (DEFAULT, 127, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 127, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 127, 8, 11000, 'Avian Cliff Dwellers'),
                (DEFAULT, 127, 9, 10000, 'Magi Enforcers'),
                (DEFAULT, 127, 10, 10000, 'Skull Clan Death Cultists'),
                --26
                (DEFAULT, 128, 2, 15000, 'Imperial Legionnaires'),
                (DEFAULT, 128, 4, 15000, 'Highborn Cavalry'),
                (DEFAULT, 128, 8, 4000, 'Avian Cliff Dwellers'),
                (DEFAULT, 128, 18, 31000, 'Oath-Sworn Knights'),
                --27
                (DEFAULT, 129, 10, 40000, 'Skull Clan Death Cultists'),
                (DEFAULT, 129, 20, 30000, 'Death Dealer Assassins'),
                --28
                (DEFAULT, 130, 1, 50000, 'Peacekeeper Monks'),
                (DEFAULT, 130, 10, 16000, 'Skull Clan Death Cultists'),
                (DEFAULT, 130, 21, 10000, 'Elven Archers'),
                --29
                (DEFAULT, 131, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 131, 10, 20000, 'Skull Clan Death Cultists'),
                (DEFAULT, 131, 18, 40000, 'Oath-Sworn Knights'),
                --30
                (DEFAULT, 132, 19, 1000, 'Minute Men Militia'),
                (DEFAULT, 132, 1, 5000, 'Peacekeeper Monks'),
                (DEFAULT, 132, 2, 25000, 'Imperial Legionnaires'),
                (DEFAULT, 132, 3, 4000, 'North Watch Longbowmen'),
                (DEFAULT, 132, 4, 2000, 'Highborn Cavalry'),
                (DEFAULT, 132, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 132, 6, 5000, 'Shinobi Martial Artists'),
                (DEFAULT, 132, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 132, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 132, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 132, 10, 10000, 'Skull Clan Death Cultists'),
                (DEFAULT, 132, 17, 5000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 132, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 132, 20, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 132, 22, 4000, 'Castlegate Crossbowmen'),
                (DEFAULT, 132, 21, 4000, 'Elven Archers')
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
