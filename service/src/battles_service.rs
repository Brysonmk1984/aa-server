use ::entity::battles::Model as BattlesModel;

use entity::battles;

use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};
pub struct BattleMutation;
impl BattleMutation {
    pub async fn insert_battle_record(
        db: &DbConn,
        nation_id_east: i32,
        nation_id_west: i32,
        nation_campaign_level_id: Option<i32>,
        winner: i32,
    ) -> Result<BattlesModel, DbErr> {
        let battle = battles::ActiveModel {
            nation_id_east: Set(nation_id_east),
            nation_id_west: Set(nation_id_west),
            nation_campaign_level_id: Set(nation_campaign_level_id),
            winner: Set(winner),
            ..Default::default()
        };

        let result: BattlesModel = battle.clone().insert(db).await?;

        println!("{result:?}");

        return Ok(result);
    }
}
