use asr::future::retry;
use asr::string::{ArrayCString, ArrayWString};
use asr::{set_tick_rate, timer};
use asr::{Address, Address64, Process, future::next_tick, print_message};
use asr::game_engine::unreal::{FNameKey, Module, Version};
use asr::watcher::Watcher;
use asr::PointerSize::Bit64;
use asr::settings::Gui;


use crate::gamestate::GameState;
use crate::splits::Splits;

mod gamestate;
mod settings;
mod splits;

asr::async_main!(stable);
// asr::panic_handler!();

static PROCESS_NAMES: [&str; 3] = [
    "SandFall-Win64-Shipping.exe",
    "SandFallGOG-Win64-Shipping.exe",
    "Sandfall-WinGDK-Shipping.exe",
];

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
    game_state: GameState,
}

impl State {
    pub async fn init<'a>(process: &'a Process, base_addr: Address) -> Self {
        let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
        print_message("Attached to module.");
        let build_version = State::get_build_version(process, &module).await;
        // let build_version = 61711;

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

    async fn get_build_version(process: &Process, module: &Module) -> u32 {
        print_message("Trying to get build version");

        let build_version: u32 = retry(|| {
            let build_version: ArrayWString<8> = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x38, 0x0, 0x30, 0x878, 0x440, 0x1a0, 0x28, 0x0]).ok()?;
            let build_version = String::from_utf16(build_version.as_slice()).ok()?;
            build_version.parse::<u32>().ok()
        }).await;

        print_message(&format!("Got build version {build_version}"));

        build_version
    }

    pub fn update(&mut self, process: &Process) -> &Self {
        let is_pause_menu_visible: bool = process.read_pointer_path(self.local_player, Bit64, &[0x0, 0x30, 0xbc8]).unwrap_or(false);
        self.game_state.is_pause_menu_visible.update(Some(is_pause_menu_visible));
        // timer::set_variable("state", is_pause_menu_visible.to_string().as_str());

        let world: String = State::get_fname(process, &self.module, self.module.g_world(), &[0x0, 0x18], String::from(""));
        timer::set_variable("world", &world);
        self.game_state.world.update(Some(world));

        let time_played: f64 = process.read_pointer_path(self.module.g_engine(), Bit64, &[0x0, 0x10a8, 0x1f0]).unwrap_or(0.0);
        timer::set_variable("time_played", &time_played.to_string());
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
        // timer::set_variable("battle_name", &battle_manager_encounter_name);
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
            // if let Some(cs_cinematic_status) = cs_cinematic_status {
            //     timer::set_variable("cs_cinematic_status", &cs_cinematic_status.to_string());
            // }

            let cs_cinematic_name: String = State::get_fname(process, &self.module, self.local_player, &[0x0, 0x30, 0x8a8, 0xa8, 0x290, 0x18], String::from(""));
            timer::set_variable("cs_cinematic_name", cs_cinematic_name.as_str());
            self.game_state.cs_cinematic_name.update_infallible(cs_cinematic_name.clone());

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
    let mut settings = settings::Settings::register();
    let mut splits = Splits::new();

    loop {
        let (process, process_name): (Process, &str) = retry(|| {
            PROCESS_NAMES.into_iter().find_map(|name| {
                Process::attach(name).and_then(|process| Some((process, name)))
            })
        }).await;

        let base_addr = retry(|| process.get_module_address(process_name)).await;
        print_message("Found base_addr");

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let mut state = State::init(&process, base_addr).await;
                print_message(&format!("{:?}", state));

                set_tick_rate(250.0);

                loop {
                    settings.update();
                    state.update(&process);
                    match timer::state() {
                        timer::TimerState::NotRunning => {
                            if settings.start && state.game_state.is_starting_run(settings.ng_plus) {
                                splits.reset();
                                timer::start();
                            }
                        },
                        timer::TimerState::Running => {
                            match state.is_loading() {
                                true => timer::pause_game_time(),
                                false => timer::resume_game_time(),
                            }
                            if settings.split && splits.should_split(&state.game_state) {
                                timer::split();
                            } else if
                                settings.reset &&
                                state.game_state.world.pair.as_ref().unwrap().changed_to(&String::from("Level_MainMenu")) {
                                splits.reset();
                                timer::reset();
                            }
                        },
                        timer::TimerState::Paused => {},
                        timer::TimerState::Ended => {}
                        _ => {}
                    }
                    next_tick().await;
                }
            })
            .await;
    }
}
