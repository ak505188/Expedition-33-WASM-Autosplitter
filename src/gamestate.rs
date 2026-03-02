use asr::{timer, watcher::Watcher};
// use asr::print_message;

pub struct GameState {
    pub battle_end_state: Watcher<u8>,
    pub battle_flow_state: Watcher<u8>,
    pub battle_manager_encounter_name: Watcher<String>,
    pub battle_debug_last_flow_state: Watcher<String>,
    pub cs_cinematic_status: Watcher<u32>,
    pub cs_cinematic_name: Watcher<String>,
    pub cs_cinematic_serial_number: Watcher<u32>,
    pub cs_cinematic_paused: Watcher<bool>,
    pub cs_is_playing_cinematic: Watcher<bool>,
    pub cs_event_before_post_cinematic_transition_started: Watcher<bool>,
    pub is_changing_area: Watcher<bool>,
    pub is_changing_map: Watcher<bool>,
    pub is_pause_menu_visible: Watcher<bool>,
    // is_save_point_menu_visible: bool,
    pub lsw_has_appeared: Watcher<bool>,
    pub time_played: Watcher<f64>,
    pub minimap_active: Watcher<bool>,
    pub pcm_in_game: Watcher<f32>,
    pub world: Watcher<String>,
}

impl GameState {
    pub fn is_game_loading(&self) -> bool {
        let world = &self.world.pair.as_ref().unwrap().current;
        world == "Map_Game_Bootstrap" ||
        self.is_changing_area.pair.unwrap().current ||
        self.is_changing_map.pair.unwrap().current ||
        self.lsw_has_appeared.pair.unwrap().current ||
        (world != "Level_Main_Menu" && self.pcm_in_game.pair.unwrap().current < 0.5)
    }

    pub fn is_battle_loading(&self) -> bool {
        let battle_debug_last_flow_state = &self.battle_debug_last_flow_state.pair.as_ref().unwrap().current;
        self.battle_flow_state.pair.unwrap().current == 2 && (
        battle_debug_last_flow_state == "InitBattle" ||
        battle_debug_last_flow_state == "LoadDependencies" ||
        battle_debug_last_flow_state == "Dependencies loaded")
    }

    pub fn is_cutscene_loading(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().current && self.cs_cinematic_paused.pair.unwrap().current
    }

    pub fn is_minimap_open(&self) -> bool {
        self.world.pair.as_ref().unwrap().current == "Level_WorldMap_Main_V2" && self.minimap_active.pair.unwrap().current
    }

    pub fn is_starting_run(&self, is_ng_plus: bool) -> bool {
        let time_played = self.time_played.pair.unwrap();
        if is_ng_plus && time_played.current > 10.0 && self.cs_is_playing_cinematic.pair.unwrap().current {
            let current_cinematic = self.cs_cinematic_name.pair.as_ref().unwrap();
            return current_cinematic.changed() && current_cinematic.current.contains("MCS_MyFlower")
        }

        time_played.old == 0.0 &&
        time_played.current > 0.0 &&
        time_played.current < 5.0
    }

    /*
    pub fn is_in_battle(&self) -> bool {
        self.battle_flow_state.pair.unwrap().current == 2
    }

    pub fn is_in_cutscene(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().current
    }

    pub fn is_cutscene_over(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().changed_to(&false)
    }
    */

    pub fn has_cutscene_started(&self) -> bool {
        self.cs_cinematic_name.pair.as_ref().unwrap().changed_from(&String::from(""))
    }

    pub fn is_battle_finished(&self) -> bool {
        let battle_flow_state = match self.battle_flow_state.pair {
            Some(v) => v,
            None => return false
        };
        // print_message(&format!("bfs: {}, change_to_0: {}", battle_flow_state.current, battle_flow_state.changed_to(&0)));
        let battle_end_state = self.battle_end_state.pair.unwrap();
        // print_message(&format!("battle_end_state cur: {}, old: {}", battle_end_state.current, battle_end_state.old));
        (battle_end_state.old == 1 || battle_end_state.old == 3) && battle_flow_state.changed_to(&0)
    }
}
