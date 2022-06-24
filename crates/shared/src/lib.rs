#![feature(const_option)]
#![feature(const_option_ext)]
#![feature(const_ptr_write)]
#![feature(const_mut_refs)]
#![feature(ptr_const_cast)]

mod shared;
pub mod settings;
mod shared_option;

use std::mem::ManuallyDrop;
use crate::shared::Shared;
use crate::settings::Settings;
use crate::shared_option::SharedOption;

struct State {
    settings: Shared<Settings>,
    menu_open: Shared<bool>,
}

static STATE: ManuallyDrop<State> = ManuallyDrop::new(State {
    settings: Shared::new(Settings::new()),
    menu_open: Shared::new(true),
});

#[inline]
pub fn settings() -> &'static mut Settings {
    unsafe {
        STATE.settings.as_mut()
    }
}

#[inline]
pub fn set_settings(settings: Settings) {
    unsafe {
        STATE.settings.write(settings);
    }
}

#[inline]
pub fn is_menu_open() -> bool {
    unsafe { *STATE.menu_open.as_mut() }
}

#[inline]
pub fn toggle_menu() {
    unsafe {
        *STATE.menu_open.as_mut() ^= true;
    }
}

