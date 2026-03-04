use asr::{Process, game_engine::unreal::Module, watcher::Watcher};
use asr::Address64;
use asr::PointerSize::Bit64;
// use asr::print_message;

use crate::helpers;

pub struct Cutscene {
    pub cs_cinematic_status: Watcher<u32>,
    pub cs_cinematic_name: Watcher<String>,
    pub cs_cinematic_serial_number: Watcher<u32>,
    pub cs_cinematic_paused: Watcher<bool>,
    pub cs_is_playing_cinematic: Watcher<bool>,
    pub cs_event_before_post_cinematic_transition_started: Watcher<bool>,
}

impl Cutscene {
    pub fn new() -> Cutscene {
        let mut cs_cinematic_name: Watcher<String> = Watcher::new();
        cs_cinematic_name.update_infallible(String::new());

        Cutscene {
            cs_cinematic_status: Watcher::new(),
            cs_cinematic_name: cs_cinematic_name,
            cs_cinematic_serial_number: Watcher::new(),
            cs_cinematic_paused: Watcher::new(),
            cs_is_playing_cinematic: Watcher::new(),
            cs_event_before_post_cinematic_transition_started: Watcher::new(),
        }
    }

    pub fn update(&mut self, process: &Process, module: &Module, local_player: Address64) -> &Self {
        let cs_is_playing_cinematic: bool = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x238]).unwrap_or(false);
        asr::timer::set_variable("is_playing_cinematic", &cs_is_playing_cinematic.to_string());
        self.cs_is_playing_cinematic.update(Some(cs_is_playing_cinematic));

        if cs_is_playing_cinematic {
            let cs_cinematic_paused: bool = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x239]).unwrap_or(false);
            self.cs_cinematic_paused.update(Some(cs_cinematic_paused));

            // TODO: Handle the unwrap_or here properly, u32::MAX is a filler value that shouldn't
            // break logic.
            let cs_cinematic_status: Option<u32> = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x8a8, 0xa8, 0x288]).ok();
            self.cs_cinematic_status.update(cs_cinematic_status);
            // if let Some(cs_cinematic_status) = cs_cinematic_status {
            //     timer::set_variable("cs_cinematic_status", &cs_cinematic_status.to_string());
            // }

            let cs_cinematic_name: String = helpers::get_fname(process, &module, local_player, &[0x0, 0x30, 0x8a8, 0xa8, 0x290, 0x18], String::from(""));
            asr::timer::set_variable("cs_cinematic_name", cs_cinematic_name.as_str());
            self.cs_cinematic_name.update_infallible(cs_cinematic_name.clone());

            let cs_cinematic_serial_number: Option<u32> = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x8a8, 0xa8, 0x2a8]).ok();
            self.cs_cinematic_serial_number.update(cs_cinematic_serial_number);
        }

        let cs_event_before_post_cinematic_transition_started: bool = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x8a8, 0x298]).unwrap_or(false);
        self.cs_event_before_post_cinematic_transition_started.update(Some(cs_event_before_post_cinematic_transition_started));

        self
    }

    pub fn is_cutscene_loading(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().current && self.cs_cinematic_paused.pair.unwrap().current
    }

    pub fn has_started(&self) -> bool {
        self.cs_cinematic_name.pair.as_ref().unwrap().changed_from(&String::from(""))
    }

    pub fn is_active(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().current
    }

    pub fn get_name(&self) -> &String {
        &self.cs_cinematic_name.pair.as_ref().unwrap().current
    }

    pub fn is_over(&self) -> bool {
        self.cs_is_playing_cinematic.pair.unwrap().changed_to(&false)
    }
}
