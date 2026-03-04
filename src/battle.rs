use asr::{Process, game_engine::unreal::Module, watcher::Watcher};
use asr::Address64;
use asr::PointerSize::Bit64;
// use asr::print_message;

use crate::helpers;

pub struct Battle {
    pub battle_end_state: Watcher<u8>,
    pub battle_flow_state: Watcher<u8>,
    pub battle_manager_encounter_name: Watcher<String>,
    pub battle_debug_last_flow_state: Watcher<String>,
}

impl Battle {
    pub fn new() -> Battle {
        Battle {
            battle_end_state: Watcher::new(),
            battle_flow_state: Watcher::new(),
            battle_manager_encounter_name: Watcher::new(),
            battle_debug_last_flow_state: Watcher::new(),
        }
    }

    pub fn update(&mut self, process: &Process, module: &Module, local_player: Address64) -> &Self {
        let battle_flow_state: u8 = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x9b0]).unwrap_or(u8::MAX);
        asr::timer::set_variable("battle_flow_state", &battle_flow_state.to_string());
        self.battle_flow_state.update_infallible(battle_flow_state);

        if battle_flow_state > 0 && battle_flow_state < 3 {
            let battle_end_state: u8 = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x920, 0x910]).unwrap_or(u8::MAX);
            self.battle_end_state.update_infallible(battle_end_state);
            asr::timer::set_variable("battle_end_state", &battle_end_state.to_string());

            let battle_manager_encounter_name = helpers::get_fname(process, module, local_player, &[0x0, 0x30, 0x920, 0x190], String::from(""));
            // timer::set_variable("battle_name", &battle_manager_encounter_name);
            self.battle_manager_encounter_name.update(Some(battle_manager_encounter_name));

            let battle_debug_last_flow_state: Option<Address64> = process.read_pointer_path(local_player, Bit64, &[0x0, 0x30, 0x920]).ok();

            if let Some(address) = battle_debug_last_flow_state {
                let address: u64 = address.value() + 0x9d8;
                let battle_debug_last_flow_state = helpers::read_fstring(&process, address);
                self.battle_debug_last_flow_state.update(Some(battle_debug_last_flow_state));
            } else {
                self.battle_debug_last_flow_state.update(Some(String::from("")));
            }
        }

        self
    }

    pub fn is_in_battle(&self) -> bool {
        self.battle_flow_state.pair.unwrap().current == 2
    }

    pub fn battle_lost(&self) -> bool {
        if let Some(battle_end_state) = self.battle_end_state.pair {
            return
                self.battle_flow_state.pair.unwrap().changed_to(&0) &&
                battle_end_state.old == 2
        }
        false
    }

    pub fn is_battle_finished(&self) -> bool {
        let battle_flow_state = self.battle_flow_state.pair.unwrap();
        battle_flow_state.changed_from_to(&2, &0) && !self.battle_lost()
        // (battle_end_state.old == 1 || battle_end_state.old == 3) && battle_flow_state.changed_to(&0)
    }

    pub fn is_battle_loading(&self) -> bool {
        if !self.is_in_battle() { return false }
        let battle_debug_last_flow_state = &self.battle_debug_last_flow_state.pair.as_ref().unwrap().current;

        battle_debug_last_flow_state == "InitBattle" ||
        battle_debug_last_flow_state == "LoadDependencies" ||
        battle_debug_last_flow_state == "Dependencies loaded"
    }
}
