use asr::string::{ArrayCString, ArrayWString};
use asr::{timer};
use asr::{Address, Address64, Process, future::next_tick, print_message};
use asr::game_engine::unreal::{FNameKey, Module, Version};
use asr::watcher::Watcher;
use asr::PointerSize::Bit64;
use asr::settings::Gui;

use crate::settings::Settings;

mod settings;

asr::async_main!(stable);
// asr::panic_handler!();

// static PROCESS_NAMES: [&str; 3] = [
//     "Sandfall-Win64-Shipping.exe",
//     "SandFallGOG-Win64-Shipping.exe",
//     "Sandfall-WinGDK-Shipping.exe"
// ];

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

struct State {
    module: Module,
    local_player: Address64,
    build_version: u32,
    game_state: GameState
}

struct GameState {
    battle_end_state: Watcher<u8>,
    battle_flow_state: Watcher<u8>,
    battle_manager_encounter_name: Watcher<String>,
    battle_debug_last_flow_state: Watcher<String>,
    cs_cinematic_status: Watcher<u32>,
    cs_cinematic_name: Watcher<String>,
    cs_cinematic_serial_number: Watcher<u32>,
    cs_cinematic_paused: Watcher<bool>,
    cs_is_playing_cinematic: Watcher<bool>,
    cs_event_before_post_cinematic_transition_started: Watcher<bool>,
    is_changing_area: Watcher<bool>,
    is_changing_map: Watcher<bool>,
    is_pause_menu_visible: Watcher<bool>,
    // is_save_point_menu_visible: bool,
    lsw_has_appeared: Watcher<bool>,
    time_played: Watcher<f64>,
    // finished_game_count: i32,
    minimap_active: Watcher<bool>,
    pcm_in_game: Watcher<f32>,
    world: Watcher<String>,
}

impl State {
    pub async fn init<'a>(process: &'a Process, process_name: &'a str) -> Self {
        let base_addr = process.get_module_address(process_name).unwrap();
        print_message("Found base_addr");
        let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
        print_message("Attached to module.");
        // let build_version = State::get_build_version(process, &module);
        let build_version = 61711;

        let local_player: Address64 = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x38]).expect("Local player error");

        let mut cs_cinematic_name: Watcher<String> = Watcher::new();
        cs_cinematic_name.update_infallible(String::new());

        State {
            module,
            local_player,
            build_version,
            game_state: GameState {
                battle_end_state: Watcher::new(),
                battle_flow_state: Watcher::new(),
                battle_manager_encounter_name: Watcher::new(),
                battle_debug_last_flow_state: Watcher::new(),
                cs_cinematic_status: Watcher::new(),
                cs_cinematic_name,
                cs_cinematic_paused: Watcher::new(),
                cs_cinematic_serial_number: Watcher::new(),
                cs_is_playing_cinematic: Watcher::new(),
                cs_event_before_post_cinematic_transition_started: Watcher::new(),
                // finished_game_count: 0,
                is_changing_area: Watcher::new(),
                is_changing_map: Watcher::new(),
                is_pause_menu_visible: Watcher::new(),
                // is_save_point_menu_visible: false,
                lsw_has_appeared: Watcher::new(),
                minimap_active: Watcher::new(),
                pcm_in_game: Watcher::new(),
                time_played: Watcher::new(),
                world: Watcher::new(),
            }
        }
    }

    pub fn is_loading(&self) -> bool {
        self.game_state.is_game_loading() ||
        self.game_state.is_battle_loading() ||
        self.game_state.is_cutscene_loading() ||
        self.game_state.is_minimap_open()
    }

    fn get_build_version(process: &Process, module: &Module) -> u32 {
        loop {
            let build_version: ArrayWString<8> = match process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x38, 0x0, 0x30, 0x878, 0x440, 0x1a0, 0x28, 0x0]) {
                Ok(v) => v,
                Err(_) => continue
            };
            let build_version = String::from_utf16(build_version.as_slice()).expect("Failed to convert build version to string");
            let build_version: u32 = build_version.parse::<u32>().unwrap_or(999999);
            if build_version == 999999 { continue }
            return build_version
        }
    }

    pub fn update(&mut self, process: &Process) -> &Self {
        let is_pause_menu_visible: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0xbc8]).unwrap_or(false);
        self.game_state.is_pause_menu_visible.update(Some(is_pause_menu_visible));
        // timer::set_variable("state", is_pause_menu_visible.to_string().as_str());

        let world: String = State::get_fname(process, &self.module, self.module.g_world(), &[0x0, 0x18], String::from(""));
        self.game_state.world.update(Some(world));

        let time_played: f64 = process.read_pointer_path(self.module.g_engine(), Bit64, &[0x0, 0x10a8, 0x1f0]).unwrap_or(0.0);
        self.game_state.time_played.update_infallible(time_played);

        let is_changing_area: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0xde8]).unwrap_or(false);
        self.game_state.is_changing_area.update_infallible(is_changing_area);

        let is_changing_map: bool = process.read_pointer_path(self.module.g_engine(), Bit64, &[0x0, 0x10a8, 0x1d0]).unwrap_or(false);
        self.game_state.is_changing_map.update_infallible(is_changing_map);

        let lsw_has_appeared: bool = process.read_pointer_path(self.module.g_engine(), Bit64, &[0x0, 0x10a8, 0xb08, 0x300]).unwrap_or(false);
        self.game_state.lsw_has_appeared.update_infallible(lsw_has_appeared);

        let pcm_in_game: f32 = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x348, 0x1390]).unwrap_or(0.0);
        self.game_state.pcm_in_game.update_infallible(pcm_in_game);

        let battle_flow_state: Option<u8> = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x9b0]).ok();
        self.game_state.battle_flow_state.update(battle_flow_state);
        if let Some(bfs) = battle_flow_state {
            timer::set_variable("battle_flow_state", &bfs.to_string());
        }

        let battle_end_state: u8 = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x920, 0x910]).unwrap_or(u8::MAX);
        self.game_state.battle_end_state.update_infallible(battle_end_state);
        timer::set_variable("battle_end_state", &battle_end_state.to_string());

        let battle_manager_encounter_name = State::get_fname(process, &self.module, self.local_player, &[0x0, 0x30, 0x920, 0x190], String::from(""));
        self.game_state.battle_manager_encounter_name.update(Some(battle_manager_encounter_name));

        let battle_debug_last_flow_state: Option<Address64> = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x920]).ok();

        if let Some(address) = battle_debug_last_flow_state {
            let address: u64 = address.value() + 0x9d8;
            let battle_debug_last_flow_state = State::read_fstring(&process, address);
            self.game_state.battle_debug_last_flow_state.update(Some(battle_debug_last_flow_state));
        } else {
            self.game_state.battle_debug_last_flow_state.update(Some(String::from("")));
        }

        let cs_is_playing_cinematic: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x238]).unwrap_or(false);
        self.game_state.cs_is_playing_cinematic.update(Some(cs_is_playing_cinematic));

        if cs_is_playing_cinematic {
            let cs_cinematic_paused: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x239]).unwrap_or(false);
            self.game_state.cs_cinematic_paused.update(Some(cs_cinematic_paused));

            // TODO: Handle the unwrap_or here properly, u32::MAX is a filler value that shouldn't
            // break logic.
            let cs_cinematic_status: Option<u32> = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0xa8, 0x288]).ok();
            self.game_state.cs_cinematic_status.update(cs_cinematic_status);
            if let Some(cs_cinematic_status) = cs_cinematic_status {
                timer::set_variable("cs_cinematic_status", &cs_cinematic_status.to_string());
            }

            let cs_cinematic_name: String = State::get_fname(process, &self.module, self.local_player, &[0x0, 0x30, 0x8a8, 0xa8, 0x290, 0x18], String::from(""));
            self.game_state.cs_cinematic_name.update_infallible(cs_cinematic_name.clone());
            timer::set_variable("cs_cinematic_name", cs_cinematic_name.as_str());

            let cs_cinematic_serial_number: Option<u32> = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0xa8, 0x2a8]).ok();
            self.game_state.cs_cinematic_serial_number.update(cs_cinematic_serial_number);
        }

        let cs_event_before_post_cinematic_transition_started: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x298]).unwrap_or(false);
        self.game_state.cs_event_before_post_cinematic_transition_started.update(Some(cs_event_before_post_cinematic_transition_started));

        let minimap_active_path;
        if self.build_version >= 57661 {
            minimap_active_path = [0x0, 0x30, 0x980, 0x3d0, 0x368];
        } else {
            minimap_active_path = [0x0, 0x30, 0x980, 0x3c8, 0x368];
        }
        let minimap_active: bool = process.read_pointer_path(self.local_player, Bit64, &minimap_active_path).unwrap_or(false);
        self.game_state.minimap_active.update_infallible(minimap_active);

        self
    }

    fn get_fname(process: &Process, module: &Module, address: impl Into<Address>, path: &[u64], default: String) -> String {
        let key: FNameKey = match process.read_pointer_path(address, Bit64, path) {
            Ok(v) => v,
            Err(_) => return default
        };

        let cstring: ArrayCString<64> = match module.get_fname(process, key) {
            Ok(v) => v,
            Err(_) => return default
        };
        let str = String::from_utf8(cstring.as_bytes().to_vec()).unwrap_or(default);
        str
    }

    fn read_fstring(process: &Process, address: u64) -> String {
        let str_addr: u64 = match process.read(address).ok() {
            Some(addr) => addr,
            None => return String::from("")
        };

        let str: ArrayWString<64> = match process.read(str_addr).ok() {
            Some(v) => v,
            None => return String::from("")
        };

        let str = match String::from_utf16(str.as_slice()) {
            Ok(v) => v,
            Err(_) => String::from("")
        };
        str
    }
}

impl GameState {
    fn is_game_loading(&self) -> bool {
        let world = &self.world.pair.as_ref().unwrap().current;
        world == "Map_Game_Bootstrap" ||
        self.is_changing_area.pair.unwrap().current ||
        self.is_changing_map.pair.unwrap().current ||
        self.lsw_has_appeared.pair.unwrap().current ||
        (world != "Level_Main_Menu" && self.pcm_in_game.pair.unwrap().current < 0.5)
    }

    fn is_battle_loading(&self) -> bool {
        let battle_debug_last_flow_state = &self.battle_debug_last_flow_state.pair.as_ref().unwrap().current;
        self.battle_flow_state.pair.unwrap().current == 2 && (
        battle_debug_last_flow_state == "InitBattle" ||
        battle_debug_last_flow_state == "LoadDependencies" ||
        battle_debug_last_flow_state == "Dependencies loaded")
    }

    fn is_cutscene_loading(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().current && self.cs_cinematic_paused.pair.unwrap().current
    }

    fn is_minimap_open(&self) -> bool {
        self.world.pair.as_ref().unwrap().current == "Level_WorldMap_Main_V2" && self.minimap_active.pair.unwrap().current
    }

    fn is_in_battle(&self) -> bool {
        self.battle_flow_state.pair.unwrap().current == 2
    }

    fn is_in_cutscene(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().current
    }

    fn has_cutscene_started(&self) -> bool {
        self.cs_cinematic_name.pair.as_ref().unwrap().changed_from(&String::from(""))
    }

    fn is_cutscene_over(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().changed_to(&false)
    }

    fn is_battle_finished(&self) -> bool {
        let battle_flow_state = match self.battle_flow_state.pair {
            Some(v) => v,
            None => return false
        };
        // print_message(&format!("bfs: {}, change_to_0: {}", battle_flow_state.current, battle_flow_state.changed_to(&0)));
        let battle_end_state = self.battle_end_state.pair.unwrap();
        // print_message(&format!("battle_end_state cur: {}, old: {}", battle_end_state.current, battle_end_state.old));
        if battle_end_state.current > 2 && battle_end_state.current != u8::MAX {
            print_message(&format!("New battle end state: {}", battle_end_state.current));
        }
        (battle_end_state.old == 1 || battle_end_state.old == 3) && battle_flow_state.changed_to(&0)
        // self.battle_end_state.pair.unwrap().changed_from_to(&1, &0)
        // self.battle_end_state.pair.unwrap().check(|t| *t == 0)
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("g_world", &self.module.g_world())
            .field("g_engine", &self.module.g_engine())
            .field("local_player", &self.local_player)
            .field("build_version", &self.build_version)
            .finish()
    }
}

fn should_split(state: &GameState, settings: &Settings) -> bool {
    timer::set_variable("battle_name", &state.battle_manager_encounter_name.pair.as_ref().unwrap().current);
    if state.is_battle_finished() {
        let battle_name = &state.battle_manager_encounter_name.pair.as_ref().unwrap().old;
        print_message(&format!("Battle finished! Battle: {}", battle_name));
        return
            (battle_name == "LU_Act1_MaelleNoTutorialCivilian" && settings.maelle_tutorial) ||
            (battle_name == "SM_FirstLancelierNoTuto*1" && settings.sm_first_lancelier) ||
            (battle_name == "SM_FirstPortier_NoTuto*1" && settings.sm_first_portier) ||
            (battle_name == "SM_Volester_TutoFlying*1" && settings.sm_first_volesters) ||
            (battle_name == "SM_Eveque_ShieldTutorial*1" && settings.sm_eveque) ||
            (battle_name == "GO_Curator_JumpTutorial*1" && settings.fw_curator) ||
            (battle_name == "GO_Goblu" && settings.flw_goblu) ||
            (battle_name == "Petank_Blue" && settings.as_petank) ||
            (battle_name == "AS_PotatoBagTank*1_IntroFight" && settings.as_robust_sakapatate) ||
            (battle_name == "AS_PotatoBagBoss" && settings.as_ultimate_sakapatate) ||
            (battle_name == "QUEST_BertrandBigHands*1" && settings.gv_bertrand) ||
            (battle_name == "QUEST_DominiqueGiantFeet*1" && settings.gv_dominique) ||
            (battle_name == "QUEST_MatthieuTheColossus*1" && settings.gv_matthieu) ||
            (battle_name == "GV_Sciel*1" && settings.gv_sciel) ||
            (battle_name == "EN_Francois" && settings.en_francois) ||
            (battle_name == "SC_LampMaster" && settings.swc_lampmaster) ||
            (battle_name == "FB_Chalier_GradientCounterTutorial*1" && settings.fb_chalier) ||
            (battle_name == "FB_DuallisteLR" && settings.fb_dualliste ) ||
            (battle_name == "MS_Monoco" && settings.ms_monoco) ||
            (battle_name == "MM_Stalact_GradientAttackTutorial1" && settings.ms_stalact) ||
            (battle_name == "OL_VersoDisappears_Chevaliere2" && settings.ol_chevaliers) ||
            (battle_name == "OL_MirrorRenoir_FirstFight" && settings.ol_renoir) ||
            (battle_name == "MF_Axon_Visages" && settings.visages_mask_keeper) ||
            (battle_name == "SI_Glissando*1" && settings.sirene_glissando) ||
            (battle_name == "SI_Axon_Sirene" && settings.sirene_sirene) ||
            (battle_name == "SI_Axon_Sirene" && settings.sirene_sirene) ||
            (battle_name == "ML_PaintressIntro" && settings.monolith_feetress) ||
            (battle_name == "MM_MirrorRenoir" && settings.monolith_renoir) ||
            (battle_name == "L_Boss_Paintress_P1" && settings.monolith_paintress) ||
            (battle_name == "L_Boss_Curator_P1" && settings.lumiere_renoir) ||
            (battle_name == "FinalBossVerso" && settings.lumiere_verso) ||
            (battle_name == "FinalBossMaelle" && settings.lumiere_maelle)
    }

    // if state.is_cutscene_over() {
    //     let cutscene_name = &state.cs_cinematic_name.pair.as_ref().unwrap().current;
    //     print_message(&format!("cutscene over: {}", &cutscene_name));

    //     return
    //         (cutscene_name == "LS_Title_Act1" && settings.act_1_start) ||
    //         (cutscene_name == "LS_Title_Act2" && settings.act_2_start) ||
    //         (cutscene_name == "LS_Title_Act3" && settings.act_3_start)
    // }

    if state.has_cutscene_started() {
        let cutscene_name = &state.cs_cinematic_name.pair.as_ref().unwrap().current;
        // print_message(&format!("cutscene started: {}", &cutscene_name));

        return
            (cutscene_name == "LS_Title_Act1" && settings.act_1_start) ||
            (cutscene_name == "LS_Title_Act2" && settings.act_2_start) ||
            (cutscene_name == "LS_Title_Act3" && settings.act_3_start)
    }

    false
}

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = settings::Settings::register();

    loop {
        let process_name = "SandFall-Win64-Shipping.exe";
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let mut state = State::init(&process, process_name).await;
                print_message(&format!("{:?}", state));

                loop {
                    settings.update();
                    state.update(&process);
                    match timer::state() {
                        timer::TimerState::NotRunning => {
                            if (state.game_state.world.pair.as_ref().unwrap().current == "Level_MainMenu" ||
                                state.game_state.world.pair.as_ref().unwrap().current == "Level_Lumiere_Main_V2") &&
                                state.game_state.time_played.pair.unwrap().old == 0.0 &&
                                state.game_state.time_played.pair.unwrap().current > 0.0 &&
                                state.game_state.time_played.pair.unwrap().current < 10.0
                            {
                                timer::start()
                            }
                        },
                        timer::TimerState::Running => {
                            if should_split(&state.game_state, &settings) {
                                timer::split();
                            }
                            if state.is_loading() {
                                timer::pause_game_time();
                            } else {
                                timer::resume_game_time();
                            }
                        },
                        timer::TimerState::Paused => {},
                        _ => {}
                    }
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
