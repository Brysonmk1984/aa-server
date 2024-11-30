use crate::{
    types::game_defaults::ArmyDefaults, utils::error::AppError, AppState, AOE_SPREAD_CELL,
    ARMY_DEFAULT_CELL, WEAPON_ARMOR_CELL,
};

use aa_battles::{IS_MARCHING_AGILITY_MOD, MIN_RANGE_ATTACK_AIR};
use axum::{debug_handler, extract::State, Json};
use serde::Serialize;
use std::{collections::HashMap, env};

#[derive(Serialize)]
enum UpkeepLabels {
    None,
    Low,
    Medium,
    High,
}

#[derive(Serialize)]
struct UpkeepRates {
    None: u8,
    Low: u8,
    Medium: u8,
    High: u8,
}

const UPKEEP_RATES: UpkeepRates = UpkeepRates {
    None: 0,
    Low: 25,
    Medium: 75,
    High: 150,
};

#[derive(Serialize)]
struct UpkeepTiers(
    (UpkeepLabels, u32),
    (UpkeepLabels, u32),
    (UpkeepLabels, u32),
    (UpkeepLabels, u32),
);

const UPKEEP_TIERS: UpkeepTiers = UpkeepTiers(
    (UpkeepLabels::High, 110000),
    (UpkeepLabels::Medium, 50000),
    (UpkeepLabels::Low, 20000),
    (UpkeepLabels::None, 10000),
);

#[derive(Serialize)]
struct IncomeDefaults {
    per_level: u16,
    base: u16,
    calc_minutes: u8,
}

#[derive(Serialize)]
struct UpkeepDefaults {
    calc_minutes: u8,
    rates: UpkeepRates,
    tiers: UpkeepTiers,
}

#[derive(Serialize)]
pub struct GameConstants {
    MIN_RANGE_ATTACK_AIR: i32,
    IS_MARCHING_AGILITY_MOD: f64,
}
#[derive(Serialize)]
pub struct ClientGameDefaults {
    weapon_armor_values: HashMap<String, f64>,
    aoe_spread_values: HashMap<i32, Vec<(f64, i32)>>,
    income: IncomeDefaults,
    upkeep: UpkeepDefaults,
    armies: Vec<ArmyDefaults>,
    constants: GameConstants,
}

#[debug_handler]
pub async fn get_game_data(
    State(state): State<AppState>,
) -> Result<Json<ClientGameDefaults>, AppError> {
    let weapon_armor_values = WEAPON_ARMOR_CELL.get().unwrap();
    let aoe_spread_values = AOE_SPREAD_CELL.get().unwrap();
    let armies_values = ARMY_DEFAULT_CELL.get().unwrap();

    let income_defaults = IncomeDefaults {
        base: env::var("INCOME_BASE_PER_CALL").unwrap().parse().unwrap(),
        per_level: env::var("INCOME_PER_LEVEL").unwrap().parse().unwrap(),
        calc_minutes: state.income_calc_minutes.parse::<u8>()?,
    };

    let upkeep_defaults = UpkeepDefaults {
        calc_minutes: state.upkeep_calc_minutes.parse::<u8>()?,
        rates: UPKEEP_RATES,
        tiers: UPKEEP_TIERS,
    };

    let constants = GameConstants {
        MIN_RANGE_ATTACK_AIR,
        IS_MARCHING_AGILITY_MOD,
    };

    let game_defaults = ClientGameDefaults {
        weapon_armor_values: weapon_armor_values.clone(),
        aoe_spread_values: aoe_spread_values.clone(),
        income: income_defaults,
        upkeep: upkeep_defaults,
        armies: armies_values.clone(),
        constants,
    };

    Ok(Json(game_defaults))
}
