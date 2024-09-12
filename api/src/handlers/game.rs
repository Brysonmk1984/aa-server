use crate::{
    types::game_defaults::ArmyDefaults, utils::error::AppError, AppState, AOE_SPREAD_CELL,
    ARMY_DEFAULT_CELL, WEAPON_ARMOR_CELL,
};

use axum::{debug_handler, extract::State, Json};
use serde::Serialize;
use std::{collections::HashMap, env};

#[derive(Serialize)]
struct IncomeDefaults {
    income_per_level: u16,
    income_base: u16,
    income_calc_minutes: u8,
    upkeep_calc_minutes: u8,
}

#[derive(Serialize)]
pub struct ClientGameDefaults {
    weapon_armor_values: HashMap<String, f64>,
    aoe_spread_values: HashMap<i32, Vec<(f64, i32)>>,
    income: IncomeDefaults,
    armies: Vec<ArmyDefaults>,
}

#[debug_handler]
pub async fn get_game_data(
    State(state): State<AppState>,
) -> Result<Json<ClientGameDefaults>, AppError> {
    let weapon_armor_values = WEAPON_ARMOR_CELL.get().unwrap();
    let aoe_spread_values = AOE_SPREAD_CELL.get().unwrap();
    let armies_values = ARMY_DEFAULT_CELL.get().unwrap();

    let income_defaults = IncomeDefaults {
        income_base: env::var("INCOME_BASE_PER_CALL").unwrap().parse().unwrap(),
        income_per_level: env::var("INCOME_PER_LEVEL").unwrap().parse().unwrap(),
        income_calc_minutes: state.income_calc_minutes.parse::<u8>()?,
        upkeep_calc_minutes: state.upkeep_calc_minutes.parse::<u8>()?,
    };

    let game_defaults = ClientGameDefaults {
        weapon_armor_values: weapon_armor_values.clone(),
        aoe_spread_values: aoe_spread_values.clone(),
        income: income_defaults,
        armies: armies_values.clone(),
    };

    Ok(Json(game_defaults))
}
