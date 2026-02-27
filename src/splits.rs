use std::collections::{HashMap, HashSet};

use asr::settings::{Map};
use asr::{timer};
use asr::print_message;

use crate::gamestate::GameState;
use crate::settings::Settings;

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
            ("SC_LampMaster", "swc_lampmaster"),
            ("FB_Chalier_GradientCounterTutorial*1", "fb_chalier"),
            ("FB_DuallisteLR", "fb_dualliste" ),
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
            ("FinalBossMaelle", "lumiere_maelle")
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

// Control Flow
// should_split -> bool
//   if battle_finished
//     check map of battle_splits for battle_name
//     if match, get value setting key
//     check settings for key, if true add to split_done set and split
//   if cutscene_finished
//     ...
