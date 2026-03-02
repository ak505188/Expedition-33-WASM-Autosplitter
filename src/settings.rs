use asr::settings::Gui;
use asr::settings::gui::Title;

#[derive(Gui)]
pub struct Settings {
    #[default = true]
    pub start: bool,
    #[default = true]
    pub split: bool,
    #[default = false]
    pub reset: bool,

    /// Start timer when starting NG+
    #[default = false]
    pub ng_plus: bool,

    /// Start of Act Splits
    _start_of_act_splits: Title,
    /// Act 1
    act_1_start: bool,
    /// Act 2
    act_2_start: bool,
    /// Act 3
    act_3_start: bool,

    /// Prologue
    _prologue: Title,
    /// Maelle Fight
    maelle_tutorial: bool,

    /// Act 1
    _act_1: Title,
    /// Spring Meadows
    _spring_meadows: Title,
    /// First Lancelier
    sm_first_lancelier: bool,
    /// First Portier
    sm_first_portier: bool,
    /// First Volesters
    sm_first_volesters: bool,
    /// Évêque
    sm_eveque: bool,
    /// Flying Waters
    _flying_waters: Title,
    /// Curator
    fw_curator: bool,
    /// Goblu
    fw_goblu: bool,
    /// Ancient Sanctuary
    _ancient_sanctuary: Title,
    /// Petank
    as_petank: bool,
    /// Robust Sakapatate
    as_robust_sakapatate: bool,
    /// Ultimate Sakapatate
    as_ultimate_sakapatate: bool,
    /// Gestral Village
    _gestral_village: Title,
    /// Bertrand Big Hands
    gv_bertrand: bool,
    /// Dominique Giant Feet
    gv_dominique: bool,
    /// Matthieu The Colossus
    gv_matthieu: bool,
    /// Sciel
    gv_sciel: bool,
    /// Esquie's Nest
    _esquies_nest: Title,
    /// François
    en_francois: bool,
    /// The Continent
    _wm_en_swc: Title,
    /// Cruler / Lancelier / Demineur
    wm_cruler_lancelier_demineur: bool,
    /// Stone Wave Cliffs
    _stone_wave_cliffs: Title,
    /// Lampmaster
    swc_lampmaster: bool,

    /// Act 2
    _act_2: Title,
    /// Forgotten Battlefield
    _forgotten_battlefield: Title,
    /// Chalier
    fb_chalier: bool,
    /// Dualliste
    fb_dualliste: bool,
    /// The Continent
    _wm_fb_ms: Title,
    /// Troubadour / Gault / Demineur
    wm_troubadour_gault_demineur: bool,
    /// Monoco Station
    _monoco_station: Title,
    /// Monoco
    ms_monoco: bool,
    /// Stalact
    ms_stalact: bool,
    /// Old Lumiere
    _old_lumiere: Title,
    /// Ceramic &amp; Steel Chevalière
    ol_chevaliers: bool,
    /// Renoir
    ol_renoir: bool,
    /// Visages
    _visages: Title,
    /// Mask Keeper
    visages_mask_keeper: bool,
    /// Sirène
    _sirene: Title,
    /// Glissando
    sirene_glissando: bool,
    /// Sirène
    sirene_sirene: bool,
    /// The Monolith
    _monolith: Title,
    /// Fake Paintress
    monolith_feetress: bool,
    /// Renoir
    monolith_renoir: bool,
    /// The Paintress
    monolith_paintress: bool,

    /// Act 3
    _act_3: Title,
    /// Return To Lumière
    _lumiere: Title,
    /// Renoir
    lumiere_renoir: bool,
    /// Verso
    lumiere_verso: bool,
    /// Maelle
    lumiere_maelle: bool,

    /// Optional Encounter Splits
    _optional_encounters: Title,
    /// Francois (Esquie Relationship LV6)
    quest_francoisduel: bool,
    /// Verso's Drafts
    _versos_drafts: Title,
    /// Balloon Gestral (Trigger Happy)
    vd_balloon_gestral: bool,
    /// Monsieur Frappe
    vd_frappe: bool,
    /// Osquio
    vd_osquio: bool,

    /// Yellow Harvest
    _yellow_harvest: Title,
    /// Glaise
    yh_glaise: bool,
    /// Merchant
    yh_merchant: bool,

    /// The Continent
    _the_continent: Title,
    /// Sprong
    wm_sprong: bool,
    /// Serpenphare
    wm_serpenphare: bool,

    /// Old Lumiere
    _ol_optionals: Title,
    /// Chromatic Danseuse
    ol_chromatic_danseuse: bool,

    /// Flying Manor
    _flying_manor: Title,
    /// Dualliste
    fm_dualliste: bool,
    /// Goblu
    fm_goblu: bool,
    /// Gargant
    fm_gargant: bool,
    /// Lampmaster
    fm_lampmaster: bool,
    /// Évêques
    fm_eveques: bool,
    /// Clea
    fm_clea: bool,

    /// Renoir's Drafts
    _renoirs_drafts: Title,
    /// Simon P1
    rd_simon_p1: bool,
    /// Simon P2
    rd_simon_p2: bool,

    /// Endless Tower
    _endless_tower: Title,
    /// Chromatic Lampmaster
    et_chromatic_lampmaster: bool,
    /// Clea Unleashed
    et_clea_unleashed: bool,
    /// Duolliste
    et_duolliste: bool,
    /// Simon The Divergent Star
    et_simon_tds: bool,
}
