#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(warnings)]

use smash::hash40;
use smash_script::macros::*;
use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};
use arcropolis_api::arc_callback;
use skyline::hooks::{Region,getRegionAddress};

mod tyrant;
pub static mut SOL_COLORS: [bool; 256] = [false; 256];
pub static mut CAPTAIN_COLORS: [bool; 256] = [false; 256];

pub unsafe extern "C" fn is_tyrant(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *module_accessor) == *FIGHTER_KIND_CAPTAIN{
        let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        crate::SOL_COLORS[color as usize] || crate::CAPTAIN_COLORS[color as usize]
    }else{
        false
    }
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "captain";
    const MARKER_FILE: &str = "tyrant.sol";
    const MARKER_FILE_2: &str = "tyrant.captain";
    let mut lowest_color: i32 = -1;
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/motion/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                SOL_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }else if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/motion/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE_2
        )) {
            unsafe {
                CAPTAIN_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }

    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }






    tyrant::install();
}


#[skyline::main(name = "tyrantrave")]
pub fn main() {
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
}