use asr::settings::Gui;
use asr::settings::gui::Title;

#[derive(Gui)]
pub struct Settings {
    /// Auto Reset when returning to Main Menu
    #[default = false]
    pub reset: bool,

    /// Turn On NG+ Run
    #[default = false]
    pub ng_plus: bool,

    /// Start of Act Splits
    _start_of_act_splits: Title,
    /// Act 1
    pub act_1_start: bool,
    /// Act 2
    pub act_2_start: bool,
    /// Act 3
    pub act_3_start: bool,

    /// Prologue
    _prologue: Title,
    /// Maelle Fight
    pub maelle_tutorial: bool,

    /// Act 1
    _act_1: Title,
    /// Spring Meadows
    _spring_meadows: Title,
    /// First Lancelier
    pub sm_first_lancelier: bool,
    /// First Portier
    pub sm_first_portier: bool,
    /// First Volesters
    pub sm_first_volesters: bool,
    /// Évêque
    pub sm_eveque: bool,
    /// Flying Waters
    pub _flying_waters: Title,
    /// Curator
    pub fw_curator: bool,
    /// Goblu
    pub flw_goblu: bool,
    /// Ancient Sanctuary
    _ancient_sanctuary: Title,
    /// Petank
    pub as_petank: bool,
    /// Robust Sakapatate
    pub as_robust_sakapatate: bool,
    /// Ultimate Sakapatate
    pub as_ultimate_sakapatate: bool,
    /// Gestral Village
    _gestral_village: Title,
    /// Bertrand Big Hands
    pub gv_bertrand: bool,
    /// Dominique Giant Feet
    pub gv_dominique: bool,
    /// Matthieu The Colossus
    pub gv_matthieu: bool,
    /// Sciel
    pub gv_sciel: bool,
    /// Esquie's Nest
    _esquies_nest: Title,
    /// François
    pub en_francois: bool,
    /// Stone Wave Cliffs
    _stone_wave_cliffs: Title,
    /// Lampmaster
    pub swc_lampmaster: bool,

    /// Act 2
    _act_2: Title,
    /// Forgotten Battlefield
    _forgotten_battlefield: Title,
    /// Chalier
    pub fb_chalier: bool,
    /// Dualliste
    pub fb_dualliste: bool,
    /// Monoco Station
    _monoco_station: Title,
    /// Monoco
    pub ms_monoco: bool,
    /// Stalact
    pub ms_stalact: bool,
    /// Old Lumiere
    _old_lumiere: Title,
    /// Ceramic & Steel Chevalière
    pub ol_chevaliers: bool,
    /// Renoir
    pub ol_renoir: bool,
    /// Visages
    _visages: Title,
    /// Mask Keeper
    pub visages_mask_keeper: bool,
    /// Sirène
    _sirene: Title,
    /// Glissando
    pub sirene_glissando: bool,
    /// Sirène
    pub sirene_sirene: bool,
    /// The Monolith
    _monolith: Title,
    /// Fake Paintress
    pub monolith_feetress: bool,
    /// Renoir
    pub monolith_renoir: bool,
    /// The Paintress
    pub monolith_paintress: bool,

    /// Act 3
    _act_3: Title,
    /// Return To Lumière
    _lumiere: Title,
    /// Renoir
    pub lumiere_renoir: bool,
    /// Verso
    pub lumiere_verso: bool,
    /// Maelle
    pub lumiere_maelle: bool,

    /// Optional Encounter Splits
    _optional_encounters: Title,
}
