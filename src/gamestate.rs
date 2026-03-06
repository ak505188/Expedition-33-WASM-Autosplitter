use asr::{Process, game_engine::unreal::Module, watcher::Watcher};
use asr::Address64;
use asr::PointerSize::Bit64;
// use asr::print_message;

use crate::helpers;
use crate::battle::{Battle};
use crate::cutscene::{Cutscene};

pub struct GameState {
    pub battle: Battle,
    pub cutscene: Cutscene,
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
    pub fn new() -> GameState {
        GameState {
            battle: Battle::new(),
            cutscene: Cutscene::new(),
            is_changing_area: Watcher::new(),
            is_changing_map: Watcher::new(),
            is_pause_menu_visible: Watcher::new(),
            lsw_has_appeared: Watcher::new(),
            minimap_active: Watcher::new(),
            pcm_in_game: Watcher::new(),
            time_played: Watcher::new(),
            world: Watcher::new(),
        }
    }


    pub fn update(&mut self, process: &Process, module: &Module, local_player: Address64, build_version: u32) -> &Self {
        let is_pause_menu_visible: bool = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0xbc8]).unwrap_or(false);
        self.is_pause_menu_visible.update(Some(is_pause_menu_visible));
        // asr::timer::set_variable("state", is_pause_menu_visible.to_string().as_str());

        let world: String = helpers::get_fname(process, module, module.g_world(), &[0x0, 0x18], String::from(""));
        asr::timer::set_variable("world", &world);
        self.world.update(Some(world));

        let time_played: f64 = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x1f0]).unwrap_or(0.0);
        // asr::timer::set_variable("time_played", &time_played.to_string());
        self.time_played.update_infallible(time_played);

        let is_changing_area: bool = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0xde8]).unwrap_or(false);
        self.is_changing_area.update_infallible(is_changing_area);

        let is_changing_map: bool = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x1d0]).unwrap_or(false);
        self.is_changing_map.update_infallible(is_changing_map);

        let lsw_has_appeared: bool = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0xb08, 0x300]).unwrap_or(false);
        self.lsw_has_appeared.update_infallible(lsw_has_appeared);

        let pcm_in_game: f32 = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x348, 0x1390]).unwrap_or(0.0);
        self.pcm_in_game.update_infallible(pcm_in_game);

        self.battle.update(process, &module, local_player);
        self.cutscene.update(process, &module, local_player);

        let minimap_active_path;
        if build_version >= 57661 {
            minimap_active_path = [0x0, 0x30, 0x980, 0x3d0, 0x368];
        } else {
            minimap_active_path = [0x0, 0x30, 0x980, 0x3c8, 0x368];
        }
        let minimap_active: bool = process.read_pointer_path(local_player, Bit64, &minimap_active_path).unwrap_or(false);
        self.minimap_active.update_infallible(minimap_active);

        self
    }

    pub fn is_game_loading(&self) -> bool {
        let world = &self.world.pair.as_ref().unwrap().current;
        world == "Map_Game_Bootstrap" ||
        self.is_changing_area.pair.unwrap().current ||
        self.is_changing_map.pair.unwrap().current ||
        self.lsw_has_appeared.pair.unwrap().current ||
        (world != "Level_Main_Menu" && self.pcm_in_game.pair.unwrap().current < 0.5)
    }

    pub fn is_minimap_open(&self) -> bool {
        self.world.pair.as_ref().unwrap().current == "Level_WorldMap_Main_V2" && self.minimap_active.pair.unwrap().current
    }

    pub fn is_starting_run(&self, is_ng_plus: bool) -> bool {
        let time_played = self.time_played.pair.unwrap();
        if is_ng_plus && time_played.current > 10.0 && self.cutscene.is_active() {
            return self.cutscene.has_started() && self.cutscene.get_name().contains("MCS_MyFlower")
        }

        time_played.old == 0.0 &&
        time_played.current > 0.0 &&
        time_played.current < 5.0
    }
}

// const HEAVY_CINEMATICS: [&str;9] = [
//     "MCS_GobluOutro",
//     "MCS_PostDuallist",
//     "MCS_DiscoveringTheTruth_P2",
//     "CS_GPE_MonolithInterior_Locomotive_Monoco_To_Lumiere",
//     "CS_CleasFlyingHouse_DuallisteDeath",
//     "CS_CleasFlyingHouse_EvequeDeath",
//     "CS_CleasFlyingHouse_GobluDeath",
//     "CS_CleasFlyingHouse_LampmasterDeath",
//     "MCS_MirrorCleaOutro"
// ];

