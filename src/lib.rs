use asr::string::{ArrayCString, ArrayWString};
use asr::{timer};
use asr::{Address, Address64, Process, future::next_tick, print_message};
use asr::game_engine::unreal::{FNameKey, Module, Version};
use asr::watcher::Watcher;
use asr::PointerSize::Bit64;

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
        // let module_size = process.get_module_size(process_name).unwrap();
        // print_message("Found module size");
        print_message("Found base_addr");
        let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
        print_message("Attached to module.");
        // let build_version = State::get_build_version(process, &module);
        let build_version = 61711;

        let local_player: Address64 = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x38]).expect("Local player error");

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
                cs_cinematic_name: Watcher::new(),
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

        let battle_end_state: Option<u8> = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x920, 0x910]).ok();
        self.game_state.battle_end_state.update(battle_end_state);

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
            let cs_cinematic_status: u32 = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0xa8, 0x288]).unwrap_or(u32::MAX);
            self.game_state.cs_cinematic_status.update(Some(cs_cinematic_status));

            let cs_cinematic_name: String = State::get_fname(process, &self.module, self.local_player, &[0x0, 0x30, 0x8a8, 0xa8, 0x290, 0x18], String::from(""));
            self.game_state.cs_cinematic_name.update(Some(cs_cinematic_name));

            let cs_cinematic_serial_number: Option<u32> = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0xa8, 0x2a8]).ok();
            self.game_state.cs_cinematic_serial_number.update(cs_cinematic_serial_number);
        }

        let cs_event_before_post_cinematic_transition_started: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x298]).unwrap_or(false);
        self.game_state.cs_event_before_post_cinematic_transition_started.update(Some(cs_event_before_post_cinematic_transition_started));

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

async fn main() {
    // TODO: Set up some general state and settings.

    asr::print_message("Hello, World!");

    loop {
        let process_name = "SandFall-Win64-Shipping.exe";
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let mut state = State::init(&process, process_name).await;
                print_message(&format!("{:?}", state));

                loop {
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
