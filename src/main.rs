extern crate ui;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use shared::toggle_menu;
use ui::app::app::App;
use dll_syringe::{Syringe, process::OwnedProcess};
use dll_syringe::process::Process;
use dll_injector::{inject_dll_load_library, inject_dll_manual_map};


fn main() {
    let target_process = OwnedProcess::find_first_by_name("DayZ_x64.exe").unwrap();
    let pid = target_process.pid().unwrap();
// create a new syringe for the target process
    match inject_dll_manual_map(pid.get(), "dll_build.dll") {
        Ok(_) => {
            debug!("injected");
        }
        Err(e) => {
            debug!("err {:?}", e);
        }
    };
    // let syringe = Syringe::for_process(target_process);

// inject the payload into the target process
//     let injected_payload = syringe.inject("./target/x86_64-pc-windows-msvc/debug/dll_build.dll").unwrap();
//     debug!("{:?}", injected_payload.handle());

}
