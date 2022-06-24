pub mod d3d11;
pub mod pattern;

use std::io::stdout;
use std::mem::{ManuallyDrop, transmute};
use std::panic::PanicInfo;
use std::ptr::null;
use backtrace::Backtrace;
use colored::{Colorize};
use faithe::internal::virtual_protect;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::{debug, error};
use winapi::ctypes::wchar_t;
use winapi::um::libloaderapi::{DisableThreadLibraryCalls, GetModuleHandleA};
use d3d11::main_thread;
use crate::pattern::{MemoryRegion, Pattern};

pub const LOG_ROOT: &'static str = "root";
pub const LOG_PANIC: &'static str = "panic";

#[repr(C)]
struct LocalEntity;

impl LocalEntity {


    pub unsafe fn new() -> ManuallyDrop<Option<Box<LocalEntity>>> {
        type FnGetLocalPlayer = extern "stdcall" fn() -> ManuallyDrop<Option<Box<LocalEntity>>>;
        match Pattern::compile("48 83 EC 28 48 8B 0D ? ? ? ? 48 85 C9 74 11 E8 ? ? ? ?")
            .find() {
            None => {
                unreachable!()
            }
            Some(region) => {
                match transmute::<_, Option<FnGetLocalPlayer>>(region.get::<u8>()) {
                    None => {
                        error!("huypizda");
                        unreachable!()
                    }
                    Some(fun) => {
                        fun()
                    }
                }
            }
        }

    }

    unsafe fn is_dead(&self) -> bool {
        let addr = (self as *const LocalEntity as usize) + 0x15D as usize;
        let addr = addr as *const bool;
        return *addr;
    }


    unsafe get_health(&self) -> f32 {


}


}

#[no_mangle]
unsafe extern "stdcall" fn DllMain(a: usize, reason: u32) -> i32 {
    if reason == 1 {
        setup_logger("Prometey", true);
        DisableThreadLibraryCalls(a as _);
        std::thread::spawn(move || unsafe { main_thread(a) });
    }

    1


}
pub fn downcast_str(string: &(dyn std::any::Any + Send)) -> &str {
    match string.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => {
            match string.downcast_ref::<String>() {
                Some(s) => &**s,
                None => {
                    "Box<Any>"
                }
            }
        }
    }
}

pub fn setup_logger(prefix: &str, debug: bool) {
    if prefix == "Prometey" {
        colored::control::set_override(false);
    }

    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::BrightBlue);

    Dispatch::new()
        .format(move |out, message, record| {
            let time = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]");
            match record.target() {
                LOG_ROOT => {
                    let level = format!("{}", colors.color(record.level()));
                    out.finish(format_args!(
                        "{}[{}] {}",
                        time,
                        (&*level).bold(),
                        message
                    ))
                }
                LOG_PANIC => {
                    let message = format!("{}", message);
                    out.finish(format_args!(
                        "{} {}",
                        time,
                        (&*message).red()
                    ))
                }
                _ => {
                    let level = format!("{}", colors.color(record.level()));
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        time,
                        record.target(),
                        (&*level).bold(),
                        message
                    ))
                }
            }
        })
        .level(if debug { log::LevelFilter::Debug } else { log::LevelFilter::Info })
        .chain(fern::log_file(&format!("{}.log", prefix)).unwrap())
        .chain(stdout())
        .apply().expect("Logger setup failed");

    std::panic::set_hook(Box::new(|info: &PanicInfo| {
        let backtrace = Backtrace::new();

        let thread = std::thread::current();
        let thread = thread.name().unwrap_or("unnamed");

        let reason = downcast_str(info.payload());

        let location = match info.location() {
            Some(location) => format!(": {}:{}:{}", location.file(), location.line(), location.column()),
            None => String::from("")
        };

        error!(target: LOG_PANIC, "thread '{}' panicked at '{}'{}", thread, reason, location);

        let s = format!("{:?}", backtrace);

        for line in s.lines() {
            debug!(target: LOG_PANIC, "{}", line);
        }
    }));
}

