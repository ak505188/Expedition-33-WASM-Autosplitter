use core::num;
use core::ops::Add;

use asr::string::ArrayCString;
use asr::{signature, timer};
use asr::{Address, Process, future::next_tick, print_message};
use asr::signature::Signature;
use asr::game_engine::unreal::{self, Module, UObject, Version};

asr::async_main!(stable);
// asr::panic_handler!();

struct State {
    module: Module,
    local_player: Address,
    is_pause_menu_visible: bool,
}

impl State {
    pub async fn init<'a>(process: &'a Process, process_name: &'a str) -> Self {
        let base_addr = process.get_module_address(process_name).unwrap();
        print_message("Found base_addr");
        // let module_size = process.get_module_size(process_name).unwrap();
        // print_message("Found module size");
        let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
        print_message("Attached to module.");
        let debug_str = format!("g_engine: {}, g_world: {}", module.g_engine(), module.g_world());
        print_message(&debug_str);
        let local_player: u64 = process.read_pointer_path(module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x10a8, 0x38]).expect("Local player error");
        let is_pause_menu_visible: bool = match process.read_pointer_path(local_player, asr::PointerSize::Bit64, &[0x0, 0x30, 0xbc8]) {
            Ok(v) => v,
            _ => false
        };
        print_message(&format!("Pause menu visible: {}", is_pause_menu_visible));

        State {
            module,
            local_player: Address::new(local_player),
            is_pause_menu_visible
        }
    }

    // pub fn update(process: &Process, module: Module) -> Self {
    //     let gworld_name: u64 = match process.read_pointer_path(module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x18]) {
    //         Ok(v) => v,
    //         Err(_) => 0,
    //     };
    //     print_message(&format!("gworld_name: {:x}", gworld_name));
    //
    //     State {
    //         // gworld_name
    //     }
    // }
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
                let state = State::init(&process, process_name).await;
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

                loop {
                    match timer::state() {
                        timer::TimerState::NotRunning => {
                            timer::start()
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
