use std::collections::{HashMap, HashSet};

use asr::settings::{Map};
use asr::{timer};

use crate::gamestate::GameState;

pub struct Splits {
    done_splits: HashSet<String>,
    battle_splits: HashMap<String, String>,
    cutscene_start_splits: HashMap<String, String>,
}

impl Splits {
    pub fn new() -> Splits {
        let done_splits: HashSet<String> = HashSet::new();
        let battle_splits = [
            ("LU_Act1_MaelleNoTutorialCivilian", "maelle_tutorial"),
            ("SM_FirstLancelierNoTuto*1", "sm_first_lancelier"),
            ("SM_FirstPortier_NoTuto*1", "sm_first_portier"),
            ("SM_Volester_TutoFlying*1", "sm_first_volesters"),
            ("SM_Eveque_ShieldTutorial*1", "sm_eveque"),
            ("GO_Curator_JumpTutorial*1", "fw_curator"),
            ("GO_Goblu", "flw_goblu"),
            ("Petank_Blue", "as_petank"),
            ("AS_PotatoBagTank*1_IntroFight", "as_robust_sakapatate"),
            ("AS_PotatoBagBoss", "as_ultimate_sakapatate"),
            ("QUEST_BertrandBigHands*1", "gv_bertrand"),
            ("QUEST_DominiqueGiantFeet*1", "gv_dominique"),
            ("QUEST_MatthieuTheColossus*1", "gv_matthieu"),
            ("GV_Sciel*1", "gv_sciel"),
            ("EN_Francois", "en_francois"),
            ("WM_Cruler_Lancelier_Demineur", "wm_cruler_lancelier_demineur"),
            ("SC_LampMaster", "swc_lampmaster"),
            ("FB_Chalier_GradientCounterTutorial*1", "fb_chalier"),
            ("FB_DuallisteLR", "fb_dualliste" ),
            ("WM_Troubadour_Gault_Demineur", "wm_troubadour_gault_demineur"),
            ("MS_Monoco", "ms_monoco"),
            ("MM_Stalact_GradientAttackTutorial1", "ms_stalact"),
            ("OL_VersoDisappears_Chevaliere2", "ol_chevaliers"),
            ("OL_MirrorRenoir_FirstFight", "ol_renoir"),
            ("MF_Axon_Visages", "visages_mask_keeper"),
            ("SI_Glissando*1", "sirene_glissando"),
            ("SI_Axon_Sirene", "sirene_sirene"),
            ("SI_Axon_Sirene", "sirene_sirene"),
            ("ML_PaintressIntro", "monolith_feetress"),
            ("MM_MirrorRenoir", "monolith_renoir"),
            ("L_Boss_Paintress_P1", "monolith_paintress"),
            ("L_Boss_Curator_P1", "lumiere_renoir"),
            ("FinalBossVerso", "lumiere_verso"),
            ("FinalBossMaelle", "lumiere_maelle"),
            ("QUEST_FrancoisDuel", "quest_francoisduel"),
            ("VD_GestralBalloon*1", "vd_balloon_gestral"),
            ("Quest_FeintFight", "vd_frappe"),
            ("VD_Osquio*1", "vd_osquio"),
            ("YF_GlaiseBoss*1", "yh_glaise"),
            ("Merchant_YellowForest", "yh_merchant"),
            ("WM_Sprong", "wm_sprong"),
            ("WM_Serpenphare", "wm_serpenphare"),
            ("MM_DanseuseAlpha*1", "ol_chromatic_danseuse"),
            ("Boss_Simon*1", "rd_simon_p1"),
            ("Boss_SimonPhase2*1", "rd_simon_p2"),
            ("CFH_Boss_Goblu", "fm_goblu"),
            ("CFH_Boss_Eveque", "fm_eveques"),
            ("CFH_Gargant*1_Danseuse*2", "fm_gargant"),
            ("CFH_Boss_Lampmaster", "fm_lampmaster"),
            ("CFH_Boss_Dualliste", "fm_dualliste"),
            ("CFH_Boss_Clea", "fm_clea"),
            ("Boss_Clea_ALPHA", "et_clea_unleashed"),
            ("Boss_LampmasterALPHA", "et_chromatic_lampmaster"),
            ("Boss_Duolliste_P1", "et_duolliste"),
            ("Boss_SimonALPHA*1", "et_simon_tds"),
        ];
        let battle_splits = battle_splits.map(|(a, b)| (a.to_string(), b.to_string()));
        let battle_splits = HashMap::from(battle_splits);

        let cutscene_start_splits = [
            ("LS_Title_Act1", "act_1_start"),
            ("LS_Title_Act2", "act_2_start"),
            ("LS_Title_Act3", "act_3_start"),
        ];
        let cutscene_start_splits = cutscene_start_splits.map(|(a, b)| (a.to_string(), b.to_string()));
        let cutscene_start_splits = HashMap::from(cutscene_start_splits);

        Splits {
            done_splits,
            battle_splits,
            cutscene_start_splits,
        }
    }

    pub fn should_split(&mut self, state: &GameState) -> bool {
        timer::set_variable("battle_name", &state.battle_manager_encounter_name.pair.as_ref().unwrap().current);
        let settings_map = Map::load();

        if state.is_battle_finished() {
            let battle_name = &state.battle_manager_encounter_name.pair.as_ref().unwrap().old;
            // print_message(&format!("Battle finished! Battle: {}", battle_name));
            let split_key = match self.battle_splits.get(battle_name) {
                Some(v) => v,
                None => return false
            };
            let split_enabled: bool = match settings_map.get(split_key) {
                Some(v) => v.get_bool().unwrap_or(false),
                None => false
            };

            return split_enabled && self.done_splits.insert(split_key.clone());
        }

        if state.has_cutscene_started() {
            let cutscene_name = &state.cs_cinematic_name.pair.as_ref().unwrap().current;
            // print_message(&format!("cutscene started: {}", &cutscene_name));
            let split_key = match self.cutscene_start_splits.get(cutscene_name) {
                Some(v) => v,
                None => return false
            };
            let split_enabled: bool = match settings_map.get(split_key) {
                Some(v) => v.get_bool().unwrap_or(false),
                None => false
            };
            return split_enabled && self.done_splits.insert(split_key.clone());
        }

        false
    }

    pub fn reset(&mut self) -> &mut Self {
        self.done_splits.clear();
        self
    }
}
