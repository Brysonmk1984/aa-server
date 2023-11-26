use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            INSERT INTO nations VALUES 
                (DEFAULT, NULL, 'The Fretful Ones', 0, TRUE),
                (DEFAULT, NULL, 'Garden Pacifists', 0, TRUE),
                (DEFAULT, NULL, 'Badlands Guild', 0, TRUE),
                (DEFAULT, NULL, 'Wayward Marauders', 0, TRUE),
                (DEFAULT, NULL, 'Dire Canyon', 0, TRUE),
                (DEFAULT, NULL, 'Merchants of the Narrows', 0, TRUE),
                (DEFAULT, NULL, 'Alpha Union', 0, TRUE),
                (DEFAULT, NULL, 'Beta Vigilantes', 0, TRUE),
                (DEFAULT, NULL, 'Commune of Gamma', 0, TRUE),
                (DEFAULT, NULL, 'Delta City', 0, TRUE),
                
                (DEFAULT, NULL, 'Epsilon Rough Necks', 0, TRUE),
                (DEFAULT, NULL, 'Zeta Realm', 0, TRUE),
                (DEFAULT, NULL, 'Warriors of Eta', 0, TRUE),
                (DEFAULT, NULL, 'Theta Troopers', 0, TRUE),
                (DEFAULT, NULL, 'Coastal Clan Iota', 0, TRUE),
                (DEFAULT, NULL, 'Kappa Society', 0, TRUE),
                (DEFAULT, NULL, 'Lambda Vanguard', 0, TRUE),
                (DEFAULT, NULL, 'Dominion of Mu', 0, TRUE),
                (DEFAULT, NULL, 'Nu Republic', 0, TRUE),
                (DEFAULT, NULL, 'Xi Tribe', 0, TRUE),
                
                (DEFAULT, NULL, 'Omicron Quadrant', 0, TRUE),
                (DEFAULT, NULL, 'Principality of Pi', 0, TRUE),
                (DEFAULT, NULL, 'Gates of Rho', 0, TRUE),
                (DEFAULT, NULL, 'Matriarchy of Sigma', 0, TRUE),
                (DEFAULT, NULL, 'Land of Tau', 0, TRUE),
                (DEFAULT, NULL, 'Upsilon Realm', 0, TRUE),
                (DEFAULT, NULL, 'Cult of Phi', 0, TRUE),
                (DEFAULT, NULL, 'Sovereignty of Chi', 0, TRUE),
                (DEFAULT, NULL, 'Psi Empire', 0, TRUE),
                (DEFAULT, NULL, 'Omega Dominion Elite', 0, TRUE)
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DELETE FROM nations 
                WHERE name='The Fretful Ones'
                OR name='Garden Pacifists'
                OR name='Garden Pacifists'
                OR name='People of the Plains'
                OR name='Badlands Marauders'
                OR name='Dire Canyon'
                OR name='Merchants of the Narrows'
                OR name='Alpha Union'
                OR name='Beta Vigilantes'
                OR name='Commune of Gamma'
                OR name='Delta City'

                OR name='Epsilon Rough Necks'
                OR name='Zeta Realm'
                OR name='Warriors of Eta'
                OR name='Theta Troopers'
                OR name='Coastal Clan Iota'
                OR name='Kappa Society'
                OR name='Lambda Vanguard'
                OR name='Dominion of Mu'
                OR name='Nu Republic'
                OR name='Xi Tribe'

                OR name='Omicron Quadrant'
                OR name='Principality of Pi'
                OR name='Gates of Rho'
                OR name='Matriarchy of Sigma'
                OR name='Land of Tau'
                OR name='Upsilon Realm'
                OR name='Cult of Phi'
                OR name='Sovereignty of Chi'
                OR name='Psi Empire'
                OR name='Omega Dominion Elite'
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
