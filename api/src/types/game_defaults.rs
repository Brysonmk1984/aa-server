use aa_battles::entities::army::Army;
use entity::armies::Model as ArmyModel;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ArmyMeta {
    cost: i32,
    unlock_level: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct ArmyDefaults {
    pub army: Army,
    meta: ArmyMeta,
}

impl Into<ArmyDefaults> for ArmyModel {
    fn into(self) -> ArmyDefaults {
        let meta = ArmyMeta {
            cost: self.cost,
            unlock_level: self.unlock_level,
        };
        let army = Army { ..self.into() };

        ArmyDefaults { army, meta }
    }
}
