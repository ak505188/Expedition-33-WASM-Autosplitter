use asr::string::{ArrayCString, ArrayWString};
use asr::{Process};
use asr::{Address, print_message};
use asr::watcher::Watcher;
use asr::PointerSize::Bit64;
use asr::game_engine::unreal::{FNameKey, Module};
use core::fmt::Debug;

pub fn get_fname(process: &Process, module: &Module, address: impl Into<Address>, path: &[u64]) -> Option<String> {
    let key: FNameKey = process.read_pointer_path(address, Bit64, path).ok()?;
    let cstring: ArrayCString<64> = module.get_fname(process, key).ok()?;
    String::from_utf8(cstring.as_bytes().to_vec()).ok()
}

pub fn read_fstring(process: &Process, address: u64) -> Option<String> {
    let str_addr: u64 = process.read(address).ok()?;
    let str: ArrayWString<64> = process.read(str_addr).ok()?;
    String::from_utf16(str.as_slice()).ok()
}

pub fn print_debug<T: Debug + Eq>(name: &str, watcher: &Watcher<T>) {
    let Some(pair)= &watcher.pair else { return };
    if !pair.changed() { return }

    print_message(&format!("{} changed to {:?} from {:?}", name, pair.current, pair.old))
}
