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
                (DEFAULT, 166, 19, 1000, 'Minute Men Militia'),
                --2
                (DEFAULT, 167, 19, 500, 'Minute Men Militia'),
                (DEFAULT, 167, 1, 1000, 'Peacekeeper Monks'),
                --3
                (DEFAULT, 168, 19, 500, 'Minute Men Militia'),
                (DEFAULT, 168, 3, 1000, 'North Watch Longbowmen'),
                (DEFAULT, 168, 22, 1500, 'Castlegate Crossbowmen'),
                --4
                (DEFAULT, 169, 19, 1000, 'Minute Men Militia'),
                (DEFAULT, 169, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 169, 17, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 169, 22, 500, 'Castlegate Crossbowmen'),
                --5
                (DEFAULT, 170, 5, 3000, 'Rōnin Immortals'),
                (DEFAULT, 170, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 170, 20, 1000, 'Death Dealer Assassins'),
                (DEFAULT, 170, 22, 500, 'Castlegate Crossbowmen'),
                --6
                (DEFAULT, 171, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 171, 3, 500, 'North Watch Longbowmen'),
                (DEFAULT, 171, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 171, 18, 500, 'Oath-Sworn Knights'),
                --7
                (DEFAULT, 172, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 172, 8, 4000, 'Avian Cliff Dwellers'),
                --8
                (DEFAULT, 173, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 173, 6, 2000, 'Shinobi Martial Artists'),
                (DEFAULT, 173, 7, 2000, 'Amazonian Huntresses'),
                (DEFAULT, 173, 8, 1000, 'Avian Cliff Dwellers'),
                (DEFAULT, 173, 17, 3000, 'Barbarians of the Outer Steppe'),
                --9
                (DEFAULT, 174, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 174, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 174, 17, 5000, 'Barbarians of the Outer Steppe'),
                --10
                (DEFAULT, 175, 1, 4000, 'Peacekeeper Monks'),
                (DEFAULT, 175, 2, 2000, 'Imperial Legionnaires'),
                (DEFAULT, 175, 4, 1000, 'Highborn Cavalry'),
                (DEFAULT, 175, 9, 3000, 'Magi Enforcers'),
                (DEFAULT, 175, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 175, 22, 3000, 'Castlegate Crossbowmen'),
                --11
                (DEFAULT, 176, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 176, 7, 3000, 'Amazonian Huntresses'),
                (DEFAULT, 176, 8, 2000, 'Avian Cliff Dwellers'),
                (DEFAULT, 176, 10, 3000, 'Skull Clan Death Cultists'),
                (DEFAULT, 176, 17, 1000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 176, 20, 2000, 'Death Dealer Assassins'),
                --12
                (DEFAULT, 177, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 177, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 177, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 177, 21, 8500, 'Elven Archers'),
                --13
                (DEFAULT, 178, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 178, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 178, 17, 10000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 178, 18, 1500, 'Oath-Sworn Knights'),
                --14
                (DEFAULT, 179, 2, 1000, 'Imperial Legionnaires'),
                (DEFAULT, 179, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 179, 6, 3000, 'Shinobi Martial Artists'),
                (DEFAULT, 179, 10, 7000, 'Skull Clan Death Cultists'),
                (DEFAULT, 179, 18, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 179, 21, 4000, 'Elven Archers'),
                --15
                (DEFAULT, 180, 3, 10000, 'North Watch Longbowmen'),
                (DEFAULT, 180, 5, 20000, 'Rōnin Immortals'),
                --16
                (DEFAULT, 181, 1, 10000, 'Peacekeeper Monks'),
                (DEFAULT, 181, 2, 5000, 'Imperial Legionnaires'),
                (DEFAULT, 181, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 181, 22, 8000, 'Castlegate Crossbowmen'),
                --17
                (DEFAULT, 182, 1, 6000, 'Peacekeeper Monks'),
                (DEFAULT, 182, 2, 10000, 'Imperial Legionnaires'),
                (DEFAULT, 182, 4, 6000, 'Highborn Cavalry'),
                (DEFAULT, 182, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 182, 9, 1000, 'Magi Enforcers'),
                (DEFAULT, 182, 17, 2000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 182, 21, 6000, 'Elven Archers'),
                --18
                (DEFAULT, 183, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 183, 4, 20000, 'Highborn Cavalry'),
                (DEFAULT, 183, 22, 9500, 'Castlegate Crossbowmen'),
                (DEFAULT, 183, 21, 5000, 'Elven Archers'),
                --19
                (DEFAULT, 184, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 184, 10, 17000, 'Skull Clan Death Cultists'),
                --20
                (DEFAULT, 185, 5, 15000, 'Rōnin Immortals'),
                (DEFAULT, 185, 6, 15000, 'Shinobi Martial Artists'),
                (DEFAULT, 185, 10, 5000, 'Skull Clan Death Cultists'),
                (DEFAULT, 185, 20, 2000, 'Death Dealer Assassins'),
                (DEFAULT, 185, 21, 8000, 'Elven Archers'),
                --21
                (DEFAULT, 186, 1, 17000, 'Peacekeeper Monks'),
                (DEFAULT, 186, 6, 30000, 'Shinobi Martial Artists'),
                --22
                (DEFAULT, 187, 3, 5000, 'North Watch Longbowmen'),
                (DEFAULT, 187, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 187, 7, 8000, 'Amazonian Huntresses'),
                (DEFAULT, 187, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 187, 17, 8000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 187, 18, 8000, 'Oath-Sworn Knights'),
                (DEFAULT, 187, 20, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 187, 21, 5000, 'Elven Archers'),
                --23
                (DEFAULT, 188, 2, 12000, 'Imperial Legionnaires'),
                (DEFAULT, 188, 3, 12000, 'North Watch Longbowmen'),
                (DEFAULT, 188, 9, 22000, 'Magi Enforcers'),
                (DEFAULT, 188, 22, 6000, 'Castlegate Crossbowmen'),
                --24
                (DEFAULT, 189, 7, 40000, 'Amazonian Huntresses'),
                (DEFAULT, 189, 21, 15000, 'Elven Archers'),
                --25
                (DEFAULT, 190, 5, 10000, 'Rōnin Immortals'),
                (DEFAULT, 190, 6, 10000, 'Shinobi Martial Artists'),
                (DEFAULT, 190, 7, 10000, 'Amazonian Huntresses'),
                (DEFAULT, 190, 8, 11000, 'Avian Cliff Dwellers'),
                (DEFAULT, 190, 9, 10000, 'Magi Enforcers'),
                (DEFAULT, 190, 10, 10000, 'Skull Clan Death Cultists'),
                --26
                (DEFAULT, 191, 2, 15000, 'Imperial Legionnaires'),
                (DEFAULT, 191, 4, 15000, 'Highborn Cavalry'),
                (DEFAULT, 191, 8, 4000, 'Avian Cliff Dwellers'),
                (DEFAULT, 191, 18, 31000, 'Oath-Sworn Knights'),
                --27
                (DEFAULT, 192, 10, 40000, 'Skull Clan Death Cultists'),
                (DEFAULT, 192, 20, 30000, 'Death Dealer Assassins'),
                --28
                (DEFAULT, 193, 1, 50000, 'Peacekeeper Monks'),
                (DEFAULT, 193, 10, 16000, 'Skull Clan Death Cultists'),
                (DEFAULT, 193, 21, 10000, 'Elven Archers'),
                --29
                (DEFAULT, 194, 9, 25000, 'Magi Enforcers'),
                (DEFAULT, 194, 10, 20000, 'Skull Clan Death Cultists'),
                (DEFAULT, 194, 18, 40000, 'Oath-Sworn Knights'),
                --30
                (DEFAULT, 195, 19, 1000, 'Minute Men Militia'),
                (DEFAULT, 195, 1, 5000, 'Peacekeeper Monks'),
                (DEFAULT, 195, 2, 25000, 'Imperial Legionnaires'),
                (DEFAULT, 195, 3, 4000, 'North Watch Longbowmen'),
                (DEFAULT, 195, 4, 2000, 'Highborn Cavalry'),
                (DEFAULT, 195, 5, 5000, 'Rōnin Immortals'),
                (DEFAULT, 195, 6, 5000, 'Shinobi Martial Artists'),
                (DEFAULT, 195, 7, 5000, 'Amazonian Huntresses'),
                (DEFAULT, 195, 8, 10000, 'Avian Cliff Dwellers'),
                (DEFAULT, 195, 9, 5000, 'Magi Enforcers'),
                (DEFAULT, 195, 10, 10000, 'Skull Clan Death Cultists'),
                (DEFAULT, 195, 17, 5000, 'Barbarians of the Outer Steppe'),
                (DEFAULT, 195, 18, 5000, 'Oath-Sworn Knights'),
                (DEFAULT, 195, 20, 5000, 'Death Dealer Assassins'),
                (DEFAULT, 195, 22, 4000, 'Castlegate Crossbowmen'),
                (DEFAULT, 195, 21, 4000, 'Elven Archers')
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
