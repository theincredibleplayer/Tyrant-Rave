#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;

mod moveset;

static mut CONSTANT_OFFSET : usize = 0x3727390; //13.0.1

use skyline::libc::*;
use std::ffi::CStr;

#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_KAMUI_STATUS_KIND_NUM") {
        value = 0x1f7;
    }
    original!()(unk,constant,value)
}

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}

#[skyline::main(name = "hotpotwithudon")]
pub fn main() {
    skyline::install_hook!(const_allot_hook);
    moveset::install();
}