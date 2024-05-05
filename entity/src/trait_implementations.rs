use std::str::FromStr;

use aa_battles::types::{ArmorType, Army, ArmyName, Nation, NationArmy, WeaponType};
use num_traits::ToPrimitive;

use crate::{
    armies::Model as ArmyModel, battles::Model as BattleModel,
    nation_armies::Model as NationArmyModel, nations::Model as NationModel,
};

impl Into<NationArmy> for NationArmyModel {
    fn into(self) -> NationArmy {
        NationArmy {
            id: self.id,
            nation_id: self.nation_id,
            army_id: self.army_id,
            count: self.count,
            army_name: ArmyName::from_str(self.army_name.as_str()).unwrap(),
        }
    }
}

impl Into<Nation> for NationModel {
    fn into(self) -> Nation {
        let name = match self.name {
            Some(name) => name,
            None => "".to_string(),
        };

        Nation {
            id: self.id,
            user_id: self.user_id.unwrap_or_default(),
            name,
            gold: self.gold,
            is_npc: self.is_npc,
        }
    }
}

impl Into<Army> for ArmyModel {
    fn into(self) -> Army {
        Army {
            id: self.id,
            name: ArmyName::from_str(self.name.as_str()).unwrap(),
            count: self.count,
            shield_rating: self.shield_rating.to_f64().unwrap(),
            range: self.range,
            attack_speed: self.attack_speed,
            accuracy: self.accuracy.to_f64().unwrap(),
            aoe: self.aoe.unwrap().to_f64().unwrap(),
            spread: self.spread.unwrap().to_f64().unwrap(),
            weapon_type: WeaponType::from_str(self.weapon_type.as_str()).unwrap(),
            armor_type: ArmorType::from_str(self.armor_type.as_str()).unwrap(),
            agility: self.agility.to_f64().unwrap(),
            speed: self.speed,
            flying: self.flying,
        }
    }
}

impl Default for BattleModel {
    fn default() -> Self {
        Self {
            id: Default::default(),
            nation_id_east: Default::default(),
            nation_id_west: Default::default(),
            nation_campaign_level_id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            winner: Default::default(),
        }
    }
}
