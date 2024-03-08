use entity::weapon_armor::{Entity as WeaponArmor, Model as WeaponArmorModel};

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
