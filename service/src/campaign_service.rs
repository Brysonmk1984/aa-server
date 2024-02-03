use crate::Mutation;
use crate::Query;

use entity::campaign_levels::{self, Entity as CampaignLevel, Model as CampaignLevelModel};
use entity::nation_campaign_levels::{
    self, ActiveModel, Entity as NationCampaignLevel, Model as NationCampaignLevelModel,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter,
    Set, TryIntoModel,
};

impl Query {
    pub async fn get_campaign_level_by_level_number(
        db: &DbConn,
        level_number: i32,
    ) -> Result<CampaignLevelModel, DbErr> {
        let campaign_level_option = CampaignLevel::find()
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
}

impl Mutation {
    pub async fn upsert_nation_campaign_level(
        db: &DbConn,
        nation_id: i32,
        campaign_level_id: i32,
        nation_name: String,
        level_number: i32,
    ) -> Result<NationCampaignLevelModel, DbErr> {
        let existing_level_by_nation_id = NationCampaignLevel::find()
            .filter(nation_campaign_levels::Column::NationId.eq(nation_id))
            .filter(nation_campaign_levels::Column::Level.eq(level_number))
            .one(db)
            .await?;
        println!("{existing_level_by_nation_id:?}");

        let mut result;
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
                    ..Default::default()
                };

                result = Ok(nation_cl_active_model.clone().insert(db).await?);
            }
        }

        return result;
    }
}
