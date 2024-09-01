use entity::weapon_armor::{Entity as WeaponArmor, Model as WeaponArmorModel};

use entity::aoe_spread::{Entity as AoeSpread, Model as AoeSpreadModel};

use sea_orm::{DbConn, DbErr, EntityTrait};

pub struct WeaponArmorQuery;
impl WeaponArmorQuery {
    pub async fn get_weapon_armor_reduction_values(
        db: &DbConn,
    ) -> Result<Vec<WeaponArmorModel>, DbErr> {
        let weapon_armor_values: Vec<WeaponArmorModel> = WeaponArmor::find().all(db).await?;

        Ok(weapon_armor_values)
    }
}

pub struct AoeSpreadQuery;
impl AoeSpreadQuery {
    pub async fn get_aoe_spread_values(db: &DbConn) -> Result<Vec<AoeSpreadModel>, DbErr> {
        let aoe_spread_values: Vec<AoeSpreadModel> = AoeSpread::find().all(db).await?;

        Ok(aoe_spread_values)
    }
}
