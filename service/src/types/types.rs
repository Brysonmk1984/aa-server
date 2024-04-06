use strum_macros::{Display, EnumString};

#[derive(Display, Debug, Default, EnumString)]
pub enum ArmyNameForService {
    #[strum(serialize = "Amazonian Huntresses")]
    AmazonianHuntresses,
    #[strum(serialize = "Avian Cliff Dwellers")]
    AvianCliffDwellers,
    #[strum(serialize = "Highborn Cavalry")]
    HighbornCavalry,
    #[strum(serialize = "Imperial Legionnaires")]
    ImperialLegionnaires,
    #[strum(serialize = "Magi Enforcers")]
    MagiEnforcers,
    #[strum(serialize = "North Watch Longbowmen")]
    NorthWatchLongbowmen,
    #[strum(serialize = "Peacekeeper Monks")]
    PeacekeeperMonks,
    #[strum(serialize = "Rōnin Immortals")]
    RoninImmortals,
    #[strum(serialize = "Shinobi Martial Artists")]
    ShinobiMartialArtists,
    #[strum(serialize = "Skull Clan Death Cultists")]
    SkullClanDeathCultists,
    #[strum(serialize = "Barbarians of the Outer Steppe")]
    BarbariansOfTheOuterSteppe,
    #[strum(serialize = "Oath-Sworn Knights")]
    OathSwornKnights,
    #[default]
    #[strum(serialize = "Minute Men Militia")]
    MinuteMenMilitia,
    #[strum(serialize = "Death Dealer Assassins")]
    DeathDealerAssassins,
    #[strum(serialize = "Elven Archers")]
    ElvenArchers,
    #[strum(serialize = "Castlegate Crossbowmen")]
    CastlegateCrossbowmen,
}
