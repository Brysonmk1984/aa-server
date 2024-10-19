use std::env;

use aa_battles::{
    do_battle,
    entities::{
        battle_army::battle_army::BattleArmy, battle_result::battle_result::BattleResult,
        game_defaults::GameDefaults, nation::Nation, nation_army::nation_army::NationArmy,
    },
    enums::ArmyName,
    util::{create_hash_of_defaults, Stats},
    EndBattlePayload,
};
use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};

use crate::{utils::error::AppError, AOE_SPREAD_CELL, ARMY_DEFAULT_CELL, WEAPON_ARMOR_CELL};

#[derive(Deserialize, Debug)]
pub struct TestBattleArmy {
    id: i32,
    name: ArmyName,
    count: i32,
}

#[derive(Deserialize, Debug)]
pub struct TestBattlePayload {
    east: Vec<TestBattleArmy>,
    west: Vec<TestBattleArmy>,
}

#[debug_handler]
pub async fn run_test_battle(
    Json(body): Json<TestBattlePayload>,
) -> Result<Json<TestFrontEndPayload>, AppError> {
    // TestBattlePayload { east: [TestBattleArmy { id: 5, name: "Rōnin Immortals", count: 100 }], west: [TestBattleArmy { id: 1, name: "Peacekeeper Monks", count: 200 }] }
    let default_armies = ARMY_DEFAULT_CELL
        .get()
        .unwrap()
        .iter()
        .map(|army_default| army_default.army.clone())
        .collect();

    let army_default_hash = create_hash_of_defaults(default_armies);

    let game_defaults = GameDefaults {
        weapons_vs_armor: WEAPON_ARMOR_CELL.get().unwrap().clone(),
        aoe_vs_spread: AOE_SPREAD_CELL.get().unwrap().clone(),
        army_defaults: army_default_hash,
        environment: env::var("ENVIRONMENT").unwrap(),
    };

    let east_nation = Nation {
        id: -1,
        ..Default::default()
    };
    let east_nation_armies = body
        .east
        .iter()
        .map(|a| NationArmy {
            nation_id: -1,
            army_id: a.id,
            count: a.count,
            army_name: a.name,
            ..Default::default()
        })
        .collect::<Vec<NationArmy>>();

    let east_tuple = (east_nation, east_nation_armies);

    let west_nation = Nation {
        id: -2,
        ..Default::default()
    };
    let west_nation_armies = body
        .west
        .iter()
        .map(|a| NationArmy {
            nation_id: -2,
            army_id: a.id,
            count: a.count,
            army_name: a.name,
            ..Default::default()
        })
        .collect::<Vec<NationArmy>>();
    let west_tuple = (west_nation, west_nation_armies);
    let competitors = (east_tuple, west_tuple);

    let end_battle_payload = do_battle(game_defaults, competitors.clone())?;

    let front_end_payload = TestFrontEndPayload {
        ..end_battle_payload.into()
    };

    Ok(Json(front_end_payload))
}

#[derive(Serialize, Debug)]
pub struct TestFrontEndPayload {
    pub battle_result: BattleResult,
    pub army_compositions: (BattleArmy, BattleArmy),
    pub events: Vec<String>,
    pub stats: (Stats, Stats),
}
impl From<EndBattlePayload> for TestFrontEndPayload {
    fn from(end_battle_payload: EndBattlePayload) -> Self {
        Self {
            battle_result: end_battle_payload.battle_result,
            army_compositions: end_battle_payload.army_compositions,
            events: end_battle_payload.events,
            stats: end_battle_payload.stats,
        }
    }
}
