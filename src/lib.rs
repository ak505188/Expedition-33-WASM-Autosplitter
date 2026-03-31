use std::path::{Path};
use asr::future::retry;
use asr::string::{ArrayWString};
use asr::{set_tick_rate, timer};
use asr::{Address64, Process, future::next_tick, print_message};
use asr::game_engine::unreal::{Module, Version};
use asr::PointerSize::Bit64;
use asr::settings::Gui;

use crate::gamestate::GameState;
use crate::splits::Splits;

mod gamestate;
mod settings;
mod splits;
mod helpers;
mod battle;
mod cutscene;

asr::async_main!(stable);
// asr::panic_handler!();

static PROCESS_NAMES: [&str; 3] = [
    "SandFall-Win64-Shipping.exe",
    "SandFallGOG-Win64-Shipping.exe",
    "Sandfall-WinGDK-Shipping.exe",
];

static MODS_EXIST_MESSAGE: &str = "
Clair Obscur: Expedition 33 speedruns requires no mods to be in use.\n
If you are seeing this message, it means that the '~mods' folder has been detected.\n
Make sure to remove this folder to stop seeing this message and ensure the validity of a legitimate speedrun.
";

struct State {
    module: Module,
    local_player: Address64,
    build_version: u32,
    path: String,
    game_state: GameState,
}

impl State {
    pub async fn init<'a>(process: &'a Process, process_name: &str) -> Self {
        let base_addr = retry(|| process.get_module_address(process_name)).await;
        print_message("Found base_addr");

        let path = retry(|| process.get_module_path(process_name)).await;
        print_message("Found path");

        if State::mods_exist(&path) {
			      panic!("{MODS_EXIST_MESSAGE}");
        }
        print_message("No mods detected");

        let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
        print_message("Attached to module.");

        let build_version = State::get_build_version(process, &module).await;
        // let build_version = 56323;

        let local_player: Address64 = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x38]).expect("Local player error");

        State {
            module,
            local_player,
            build_version,
            path,
            game_state: GameState::new()
        }
    }

    pub fn is_loading(&self) -> bool {
        self.game_state.is_game_loading() ||
        self.game_state.battle.is_battle_loading() ||
        self.game_state.cutscene.is_cutscene_loading() ||
        self.game_state.is_minimap_open()
    }

    pub fn mods_exist(path: &String) -> bool {
        let path = Path::new(path).ancestors()
            .nth(3)
            .unwrap()
            .join("Content")
            .join("Paks")
            .join("~mods");

        path.exists()
    }

    async fn get_build_version(process: &Process, module: &Module) -> u32 {
        print_message("Trying to get build version");

        let build_version: u32 = retry(|| {
            // TODO: If running on game startup can get build version 999999.
            // Add check, run again if 999999
            let build_version: ArrayWString<8> = process.read_pointer_path(module.g_engine(), Bit64, &[0x0, 0x10a8, 0x38, 0x0, 0x30, 0x878, 0x440, 0x1a0, 0x28, 0x0]).ok()?;
            let build_version = String::from_utf16(build_version.as_slice()).ok()?;
            let build_version = build_version.parse::<u32>().ok()?;
            (build_version != 999999).then_some(build_version)
        }).await;

        print_message(&format!("Got build version {build_version}"));

        build_version
    }

    pub fn update(&mut self, process: &Process) -> &Self {
        self.game_state.update(process, &self.module, self.local_player, self.build_version);

        self
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

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let mut state = State::init(&process, process_name).await;
                print_message(&format!("{:?}", state));

                set_tick_rate(250.0);

                loop {
                    settings.update();
                    state.update(&process);
                    match timer::state() {
                        timer::TimerState::NotRunning => {
                            if settings.start && state.game_state.is_starting_run(settings.ng_plus) {
                                if State::mods_exist(&state.path) {
                                    panic!("{MODS_EXIST_MESSAGE}");
                                }
                                timer::start();
                            } else {
                                splits.reset();
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
                                timer::reset();
                                splits.reset();
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
