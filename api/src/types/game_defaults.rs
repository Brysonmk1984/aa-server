use aa_battles::types::Army;
use entity::armies::Model as ArmyModel;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ArmyMeta {
    cost: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct ArmyDefaults {
    pub army: Army,
    meta: ArmyMeta,
}

impl Into<ArmyDefaults> for ArmyModel {
    fn into(self) -> ArmyDefaults {
        let meta = ArmyMeta { cost: self.cost };
        let army = Army { ..self.into() };

        ArmyDefaults { army, meta }
    }
}
