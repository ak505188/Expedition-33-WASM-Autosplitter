use asr::string::{ArrayCString, ArrayWString};
use asr::{timer};
use asr::{Address, Address64, Process, future::next_tick, print_message};
use asr::game_engine::unreal::{FNameKey, Module, Version};
use asr::watcher::{Pair, Watcher};

asr::async_main!(stable);
// asr::panic_handler!();

// static PROCESS_NAMES: [&str; 3] = [
//     "Sandfall-Win64-Shipping.exe",
//     "SandFallGOG-Win64-Shipping.exe",
//     "Sandfall-WinGDK-Shipping.exe"
// ];

struct State {
    module: Module,
    local_player: Address64,
    build_version: u32,
    game_state: GameState
}

struct GameState {
    battle_end_state: u8,
    battle_flow_state: u8,
    battle_manager_encounter_name: String,
    cs_cinematic_status: u32,
    cs_cinematic_name: String,
    cs_cinematic_serial_number: u32,
    cs_is_playing_cinematic: bool,
    cs_event_before_post_cinematic_transition_started: bool,
    is_changing_area: bool,
    is_pause_menu_visible: Watcher<bool>,
    is_save_point_menu_visible: bool,
    lsw_has_appeared: bool,
    time_played: Watcher<f64>,
    finished_game_count: i32,
    minimap_active: bool,
    pcm_in_game: f32,
    world: Watcher<String>,
}

impl State {
    pub async fn init<'a>(process: &'a Process, process_name: &'a str) -> Self {
        let base_addr = process.get_module_address(process_name).unwrap();
        print_message("Found base_addr");
        // let module_size = process.get_module_size(process_name).unwrap();
        // print_message("Found module size");
        let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
        print_message("Attached to module.");
        let build_version = State::get_build_version(process, &module);

        let local_player: Address64 = process.read_pointer_path(module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x10a8, 0x38]).expect("Local player error");

        State {
            module,
            local_player,
            build_version,
            game_state: GameState {
                battle_end_state: 0,
                battle_flow_state: 0,
                battle_manager_encounter_name: String::new(),
                cs_cinematic_status: 0,
                cs_cinematic_name: String::new(),
                cs_cinematic_serial_number: 0,
                cs_is_playing_cinematic: false,
                cs_event_before_post_cinematic_transition_started: false,
                finished_game_count: 0,
                is_changing_area: false,
                is_pause_menu_visible: Watcher::new(),
                is_save_point_menu_visible: false,
                lsw_has_appeared: false,
                minimap_active: false,
                pcm_in_game: 0.0,
                time_played: Watcher::new(),
                world: Watcher::new(),
            }
        }
    }

    fn get_build_version(process: &Process, module: &Module) -> u32 {
        loop {
            let build_version: ArrayWString<8> = match process.read_pointer_path(module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x10a8, 0x38, 0x0, 0x30, 0x878, 0x440, 0x1a0, 0x28, 0x0]) {
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
        let is_pause_menu_visible: bool = process.read_pointer_path(self.local_player, asr::PointerSize::Bit64, &[0x0, 0x30, 0xbc8]).unwrap_or(false);
        self.game_state.is_pause_menu_visible.update(Some(is_pause_menu_visible));

        let world: String = State::get_fname(process, &self.module, self.module.g_world(), &[0x0, 0x18], String::from(""));
        self.game_state.world.update(Some(world));

        let time_played: f64 = process.read_pointer_path(self.module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x10a8, 0x1f0]).unwrap_or(0.0);
        self.game_state.time_played.update(Some(time_played));

        self
    }

    fn get_fname(process: &Process, module: &Module, address: impl Into<Address>, path: &[u64], default: String) -> String {
        let key: FNameKey = match process.read_pointer_path(address, asr::PointerSize::Bit64, path) {
            Ok(v) => v,
            Err(_) => return default
        };

        let cstring: ArrayCString<64> = module.get_fname(process, key).unwrap();
        let str = String::from_utf8(cstring.as_bytes().to_vec()).unwrap_or(default);
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
    // TODO: Set up some general state and settings.

    asr::print_message("Hello, World!");

    loop {
        let process_name = "SandFall-Win64-Shipping.exe";
        let process = Process::wait_attach(process_name).await;
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let mut state = State::init(&process, process_name).await;
                // let module_size = process.get_module_size(process_name).unwrap();
                // let f_names_signature: Signature<7> = Signature::new("8B D9 74 ?? 48 8D 15 ?? ?? ?? ?? EB");
                // let f_names = f_names_signature.wait_scan_process_range(&process, (base_addr, module_size)).await;
                // print_message(&format!("fnames: {:?}", f_names.value()));

                // loop {
                //     let local_player: u64 = match process.read_pointer_path(module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x10a8, 0x38]) {
                //         Ok(v) => v,
                //         Err(err) => {
                //             print_message(&format!("local_player error: {:?}", err));
                //             continue
                //         }
                //     };
                //     print_message(&format!("{:x}", local_player));
                //     break;
                // }
                //
                // State::update(&process, module);

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

                        },
                        _ => {}
                    }
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
