use ::entity::campaign_levels::{self, Entity as CampaignLevels, Model as CampaignLevelsModel};
use ::entity::nation_armies::{self, Entity as NationArmies, Model as NationArmiesModel};
use ::entity::nation_campaign_levels::{self, Model as NationCampaignLevelModel};
use ::entity::nations::{self, Entity as Nations, Model as NationsModel};
use entity::nation_campaign_levels::Entity as NationCampaignLevels;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbBackend, DbConn, DbErr, EntityTrait, QueryFilter, Set,
    Statement,
};

pub struct CampaignQuery;
impl CampaignQuery {
    pub async fn get_all_campaign_levels(db: &DbConn) -> Result<Vec<CampaignLevelsModel>, DbErr> {
        let campaign_levels: Vec<CampaignLevelsModel> = CampaignLevels::find().all(db).await?;
        Ok(campaign_levels)
    }

    pub async fn get_campaign_level_by_level_number(
        db: &DbConn,
        level_number: i32,
    ) -> Result<CampaignLevelsModel, DbErr> {
        let campaign_level_option = CampaignLevels::find()
            .filter(campaign_levels::Column::Level.eq(level_number))
            .one(db)
            .await?;
        if let Some(campaign_level) = campaign_level_option {
            return Ok(campaign_level);
        } else {
            return Err(DbErr::RecordNotFound(format!(
                "Unable to find level '{level_number}' record in the DB."
            )));
        }
    }

    pub async fn get_campaign_level_id_by_level_number(
        db: &DbConn,
        level: i32,
    ) -> Result<i32, DbErr> {
        let campaign_levels: Result<std::option::Option<CampaignLevelsModel>, DbErr> =
            CampaignLevels::find()
                .filter(campaign_levels::Column::Level.eq(level))
                .one(db)
                .await;

        let campaign_level: CampaignLevelsModel = campaign_levels.unwrap().unwrap();
        Ok(campaign_level.id)
    }

    pub async fn get_campaign_nation_with_nation_armies_by_nation_id(
        db: &DbConn,
        level: i32,
    ) -> Result<(NationsModel, Vec<NationArmiesModel>), DbErr> {
        println!("{level}");
        let campaign_level = CampaignLevels::find()
            .filter(campaign_levels::Column::Level.eq(level))
            .one(db)
            .await?;
        if campaign_level.is_none() {
            panic!("No campaign level: {}", level)
        }
        let level_nation_id = campaign_level.unwrap().nation_id.clone();
        let nation = Nations::find()
            .filter(nations::Column::Id.eq(level_nation_id))
            .one(db)
            .await?;
        println!("{nation:?}");
        if nation.is_none() {
            panic!("No nation with id: {}", level_nation_id)
        }

        let nation_armies = NationArmies::find()
            .filter(nation_armies::Column::NationId.eq(level_nation_id))
            .all(db)
            .await;
        println!("{nation_armies:?}");
        match nation_armies {
            Ok(n_armies) => Ok((nation.unwrap(), n_armies)),
            Err(_) => Ok((nation.unwrap(), vec![])),
        }
    }

    pub async fn get_highest_campaign_level_completed(
        db: &DbConn,
        nation_id: i32,
    ) -> Result<i32, DbErr> {
        let sql = format!(
            "   SELECT *
            FROM nation_campaign_levels
            WHERE nation_id = {nation_id}
               ORDER BY level  DESC
               limit 1"
        );

        let highest_completed_record_option = nation_campaign_levels::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                sql,
                [1.into()],
            ))
            .one(db)
            .await?;

        match highest_completed_record_option {
            Some(record) => Ok(record.level),
            None => Ok(0),
        }
    }
}

pub struct CampaignMutation;
impl CampaignMutation {
    pub async fn upsert_nation_campaign_level(
        db: &DbConn,
        nation_id: i32,
        campaign_level_id: i32,
        nation_name: String,
        level_number: i32,
        completed: bool,
    ) -> Result<NationCampaignLevelModel, DbErr> {
        println!("{nation_id} {level_number}");
        let the_result = NationCampaignLevels::find()
            .filter(nation_campaign_levels::Column::NationId.eq(nation_id))
            .filter(nation_campaign_levels::Column::Level.eq(level_number))
            .one(db)
            .await;

        let existing_level_by_nation_id = the_result.unwrap();
        println!("existing_level_by_nation_id::: {existing_level_by_nation_id:?}");

        let result;
        match existing_level_by_nation_id {
            // Update existing record
            Some(nation_cl) => {
                let nation_cl_active_model = nation_campaign_levels::ActiveModel {
                    id: Set(nation_cl.id),
                    nation_id: Set(nation_id),
                    campaign_level_id: Set(campaign_level_id),
                    nation_name: Set(nation_name),
                    level: Set(nation_cl.level),
                    attempts: Set(nation_cl.attempts + 1),
                    completed: Set(completed),
                    ..Default::default()
                };

                result = nation_cl_active_model.update(db).await;
            }
            // Create new record
            None => {
                let nation_cl_active_model = nation_campaign_levels::ActiveModel {
                    nation_id: Set(nation_id),
                    campaign_level_id: Set(campaign_level_id),
                    nation_name: Set(nation_name),
                    level: Set(level_number),
                    attempts: Set(1),
                    completed: Set(completed),
                    ..Default::default()
                };
                println!("{nation_cl_active_model:?}");
                result = nation_cl_active_model.clone().insert(db).await;
                println!("THE RESULRT {result:?}");
                match result {
                    Ok(result) => return Ok(result),
                    Err(error) => {
                        println!("{error:?}");
                        return Err(error);
                    }
                }
            }
        }

        return result;
    }
}
