use ::entity::battles::{Entity as Battles, Model as BattlesModel};
use ::entity::campaign_levels::{self, Entity as CampaignLevels, Model as CampaignLevelsModel};
use entity::battles;
use sea_orm::sea_query::Query;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

use crate::Mutation;

impl Mutation {
    pub async fn insert_battle_record(
        db: &DbConn,
        nation_id_east: i32,
        nation_id_west: i32,
        nation_campaign_level_id: Option<i32>,
    ) -> Result<BattlesModel, DbErr> {
        let battle = battles::ActiveModel {
            nation_id_east: Set(nation_id_east),
            nation_id_west: Set(nation_id_west),
            nation_campaign_level_id: Set(nation_campaign_level_id),
            ..Default::default()
        };

        let result: BattlesModel = battle.clone().insert(db).await?;

        println!("{result:?}");

        return Ok(result);
    }
}
