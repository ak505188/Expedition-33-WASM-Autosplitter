use core::num;
use core::ops::Add;

use asr::{Address, Process, future::next_tick, print_message};
use asr::signature::Signature;
use asr::game_engine::unreal::{Module, Version};

asr::async_main!(stable);
// asr::panic_handler!();

async fn main() {
    // TODO: Set up some general state and settings.

    asr::print_message("Hello, World!");

    loop {
        let process = Process::wait_attach("SandFall-Win64-Shipping.exe").await;
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                let base_addr = process.get_module_address("SandFall-Win64-Shipping.exe").unwrap();
                print_message("Found base_addr");
                let module = Module::wait_attach(&process, Version::V5_4, base_addr).await;
                print_message("Attached to module.");
                let debug_str = format!("g_engine: {}, g_world: {}", module.g_engine(), module.g_world());
                print_message(&debug_str);

                loop {
                    let local_player: u64 = match process.read_pointer_path(module.g_engine(), asr::PointerSize::Bit64, &[0x0, 0x10a8, 0x38]) {
                        Ok(v) => v,
                        Err(err) => {
                            print_message(&format!("local_player error: {:?}", err));
                            continue
                        }
                    };
                    print_message(&format!("{:x}", local_player));
                    break;
                }

                loop {
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
