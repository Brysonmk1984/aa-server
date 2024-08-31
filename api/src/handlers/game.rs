use crate::{utils::error::AppError, AppState, AOE_SPREAD_CELL, WEAPON_ARMOR_CELL};
use axum::{debug_handler, extract::State, Json};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct GameDefaults {
    weapon_armor_values: HashMap<String, f64>,
    aoe_spread_values: HashMap<u8, [(f64, u8); 7]>,
    income_calc_minutes: u8,
    upkeep_calc_minutes: u8,
}

#[debug_handler]
pub async fn get_game_data(State(state): State<AppState>) -> Result<Json<GameDefaults>, AppError> {
    let weapon_armor_values = WEAPON_ARMOR_CELL.get().unwrap();
    let aoe_spread_values = AOE_SPREAD_CELL.get().unwrap();

    let game_defaults = GameDefaults {
        weapon_armor_values: weapon_armor_values.clone(),
        aoe_spread_values: aoe_spread_values.clone(),
        income_calc_minutes: state.income_calc_minutes.parse::<u8>()?,
        upkeep_calc_minutes: state.upkeep_calc_minutes.parse::<u8>()?,
    };

    Ok(Json(game_defaults))
}
