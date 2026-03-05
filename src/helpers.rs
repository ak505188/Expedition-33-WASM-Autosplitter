use std::path::{Component, Path, PathBuf};
use asr::string::{ArrayCString, ArrayWString};
use asr::{Process};
use asr::{Address};
use asr::PointerSize::Bit64;
use asr::game_engine::unreal::{FNameKey, Module};

pub fn get_fname(process: &Process, module: &Module, address: impl Into<Address>, path: &[u64], default: String) -> String {
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

pub fn read_fstring(process: &Process, address: u64) -> String {
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

pub fn normalize_mnt(path: &Path) -> PathBuf {
    let mut components = path.components();

    let mut new_path = PathBuf::new();

    // Preserve root if present
    if let Some(Component::RootDir) = components.next() {
        new_path.push("/");
    }

    // Skip "mnt" if it's the first normal component
    if let Some(Component::Normal(first)) = components.next() {
        if first != "mnt" {
            new_path.push(first);
        }
    }

    // Add remaining components
    for c in components {
        new_path.push(c.as_os_str());
    }

    new_path
}
