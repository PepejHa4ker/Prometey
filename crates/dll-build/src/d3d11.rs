#![allow(warnings)]

use egui::{
    Align2, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Pos2, Rect,
    RichText, ScrollArea, Slider, Stroke, TextureId, Vec2, Widget, Modifiers, Key,
};
use egui_d3d11::DirectX11App;
use faithe::{c_str, internal::alloc_console, pattern::Pattern};
use std::{
    intrinsics::transmute,
    sync::{Arc, Once},
};
use ui::app::app::App;
use std::thread::sleep;
use std::time::Duration;
use backtrace::SymbolName;
use log::{debug, error};
use std::ffi::{CStr, c_void, CString, OsString};
use detour::RawDetour;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, HLOCAL, HMODULE, LPVOID, MAX_PATH, TRUE};

use winapi::um::errhandlingapi::{AddVectoredExceptionHandler, GetLastError};
use winapi::um::libloaderapi::{LoadLibraryA, GetModuleFileNameW};
use winapi::um::memoryapi::VirtualQuery;
use winapi::um::winbase::{FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_HMODULE, FORMAT_MESSAGE_FROM_SYSTEM, FormatMessageW, LocalFree};
use winapi::um::winnt::{EXCEPTION_POINTERS, EXCEPTION_RECORD, LANG_NEUTRAL, LONG, LPCSTR, LPWSTR, MAKELANGID, MEMORY_BASIC_INFORMATION, STATUS_ACCESS_VIOLATION, STATUS_IN_PAGE_ERROR, SUBLANG_DEFAULT};
use winapi::um::winuser::{GetAsyncKeyState, VK_HOME};
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Dxgi::{Common::DXGI_FORMAT, IDXGISwapChain},
        UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
    },
};
use wio::wide::FromWide;
use crate::LocalEntity;

static mut APP: DirectX11App<i32> = DirectX11App::new();
static mut OLD_WND_PROC: Option<WNDPROC> = None;


type FnPresent = unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32) -> HRESULT;

static mut O_PRESENT: Option<FnPresent> = None;

type FnResizeBuffers =
unsafe extern "stdcall" fn(IDXGISwapChain, u32, u32, u32, DXGI_FORMAT, u32) -> HRESULT;

static mut O_RESIZE_BUFFERS: Option<FnResizeBuffers> = None;

unsafe extern "stdcall" fn hk_present(
    swap_chain: IDXGISwapChain,
    sync_interval: u32,
    flags: u32,
) -> HRESULT {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        APP.init_default(&swap_chain, ui);

        let desc = swap_chain.GetDesc().unwrap();
        if desc.OutputWindow.0 == -1 {}

        OLD_WND_PROC = Some(transmute(SetWindowLongPtrA(
            desc.OutputWindow,
            GWLP_WNDPROC,
            hk_wnd_proc as usize as _,
        )));
    });

    APP.present(&swap_chain);

    match O_PRESENT.as_ref() {
        None => {
            unreachable!()
        }
        Some(present) => {
            present(swap_chain, sync_interval, flags)
        }
    }
}

unsafe extern "stdcall" fn hk_resize_buffers(
    swap_chain: IDXGISwapChain,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    swap_chain_flags: u32,
) -> HRESULT {
    APP.resize_buffers(&swap_chain, || {
        O_RESIZE_BUFFERS.as_ref().expect("O_RESIZE_BUFFERS")(
            swap_chain.clone(),
            buffer_count,
            width,
            height,
            new_format,
            swap_chain_flags,
        )
    })
}

unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    APP.wnd_proc(msg, wparam, lparam);

    if (GetAsyncKeyState(VK_HOME) & 1) == 1 {
        shared::toggle_menu()
    }

    CallWindowProcW(OLD_WND_PROC.expect("OLD WND PROC"), hwnd, msg, wparam, lparam)
}

fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        egui::Window::new("Prometey").show(ctx, |ui| {
            ui.label("a");
        });
    }
}

pub unsafe fn main_thread(_hinst: usize) {
    alloc_console().unwrap();
    ansi_term::enable_ansi_support().expect("enabling console ansi support failed");
    if AddVectoredExceptionHandler(0, Some(except)).is_null() {
        panic!("Unable to set exception handler");
    }

    let present = faithe::internal::find_pattern(
        "gameoverlayrenderer64.dll",
        Pattern::from_ida_style("48 89 6C 24 18 48 89 74 24 20 41 56 48 83 EC 20 41"),
    )
        .unwrap_or_else(|_| {
            faithe::internal::find_pattern(
                "dxgi.dll",
                Pattern::from_ida_style("48 89 5C 24 10 48 89 74 24 20 55 57 41 56"),
            )
                .unwrap()
        })
        .unwrap() as usize;

    debug!("Present: {:X}", present);

    let swap_buffers = faithe::internal::find_pattern(
        "gameoverlayrenderer64.dll",
        Pattern::from_ida_style(
            "48 89 5C 24 08 48 89 6C 24 10 48 89 74 24 18 57 41 56 41 57 48 83 EC 30 44",
        ),
    )
        .unwrap_or_else(|_| {
            faithe::internal::find_pattern(
                "dxgi.dll",
                Pattern::from_ida_style("48 8B C4 55 41 54 41 55 41 56 41 57 48 8D 68 B1 48 81 EC C0"),
            )
                .unwrap()
        })
        .unwrap() as usize;

    debug!("Buffers: {:X}", swap_buffers);

    let mut detour = RawDetour::new(present as _, hk_present as _).unwrap();
    detour.enable().unwrap();
    let trampoline = detour.trampoline() as *const ();
    std::mem::forget(detour);
    O_PRESENT = Some(transmute(trampoline));

    let mut detour = RawDetour::new(swap_buffers as _, hk_resize_buffers as _).unwrap();
    detour.enable().unwrap();
    let trampoline = detour.trampoline() as *const ();
    std::mem::forget(detour);
    O_RESIZE_BUFFERS = Some(transmute(trampoline));
    // sunshine::create_hook(
    //     sunshine::HookType::Absolute,
    //     transmute::<_, FnPresent>(present),
    //     hk_present as FnPresent,
    //     &mut O_PRESENT,
    // )
    // sunshine::create_hook(
    //     sunshine::HookType::Absolute,
    //     transmute::<_, FnResizeBuffers>(swap_buffers),
    //     hk_resize_buffers as FnResizeBuffers,
    //     &mut O_RESIZE_BUFFERS,
    // )
    #[allow(clippy::empty_loop)]
    loop {
        let entity = LocalEntity::new();
        if let Some (entity) = entity.as_ref() {
            debug!("IS_DEAD {:?}", entity.is_dead());
        }

    }
}

unsafe fn print_address_info(addr: *mut c_void, line: Option<u32>, symbol_name: SymbolName) {
    let mut mbi = MEMORY_BASIC_INFORMATION::default();
    let size = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();
    if VirtualQuery(addr, &mut mbi, size) == size {
        let mut name = [0; MAX_PATH];
        let len = GetModuleFileNameW(mbi.AllocationBase.cast(), name.as_mut_ptr(), MAX_PATH as u32);
        if len != 0 {
            let name = OsString::from_wide_ptr(name.as_ptr(), len as usize);
            let offset = addr as u64 - mbi.AllocationBase as u64;
            if let Some(line) = line {
                debug!(" at {} (line: {}) in '{}' + 0x{:X}", symbol_name, line, name.to_string_lossy(), offset)
            } else {
                debug!(" at {} in '{}' + 0x{:X}", symbol_name, name.to_string_lossy(), offset)
            }
        }
    }
}

fn get_op(code: usize) -> &'static str {
    match code {
        0 => "reading",
        8 => "DEP",
        _ => "writing"
    }
}

unsafe fn get_error_code_message(ntdll: HMODULE, rec: &EXCEPTION_RECORD) -> String {
    match rec.ExceptionCode {
        STATUS_ACCESS_VIOLATION => {
            let address = rec.ExceptionInformation[1];
            if rec.NumberParameters == 3 {
                let op = get_op(rec.ExceptionInformation[0]);
                format!("STATUS_ACCESS_VIOLATION {} 0x{:08X}", op, address)
            } else {
                String::from("STATUS_ACCESS_VIOLATION")
            }
        }
        STATUS_IN_PAGE_ERROR => {
            let address = rec.ExceptionInformation[1];
            if rec.NumberParameters == 3 {
                let op = get_op(rec.ExceptionInformation[0]);
                let code = rec.ExceptionInformation[3];
                format!("STATUS_IN_PAGE_ERROR {} 0x{:08X} with code 0x{:08X}", op, address, code)
            } else {
                String::from("STATUS_IN_PAGE_ERROR")
            }
        }
        code => {
            let mut buffer: LPWSTR = std::ptr::null_mut();
            let strlen = FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_HMODULE,
                                        ntdll as _,
                                        code,
                                        MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as _,
                                        (&mut buffer as *mut LPWSTR) as LPWSTR,
                                        0,
                                        std::ptr::null_mut());

            if buffer.is_null() {
                let err = GetLastError();
                format!("UNKNOWN (FormatMessageW() returned 0x{:08X})", err)
            } else {
                let message = OsString::from_wide_ptr(buffer, strlen as usize);
                LocalFree(buffer as HLOCAL);
                message.to_string_lossy().trim_matches(|c| c == '\r' || c == '\n').to_string()
            }
        }
    }
}

extern "system" fn except(info: *mut EXCEPTION_POINTERS) -> i32 {
    unsafe {
        let info = &mut *info;
        let rec = &mut *info.ExceptionRecord;
        let addr = rec.ExceptionAddress;
        let code = rec.ExceptionCode;
        if code == 0x40010006 {
            return 0;
        }
        let ntdll = LoadLibraryA(c_str!("ntdll.dll").as_ptr() as _);
        let message = get_error_code_message(ntdll, rec);
        // error!("Unhandled exception at 0x{:08X}: 0x{:08X} ({})", addr as u64, code, message);

        // let backtrace = backtrace::Backtrace::new_starting_from(addr as _, true);
        let backtrace = backtrace::Backtrace::new();

        for frame in backtrace.frames().iter() {
            for symbol in frame.symbols() {
                if let Some(addr) = symbol.addr().clone() {
                    let name = symbol.name().unwrap_or_else(|| SymbolName::new(b"<unknown>"));
                    print_address_info(addr, symbol.lineno(), name);
                }
            }
        }
        0 //EXCEPTION_CONTINUE_SEARCH
    }
}

// for<'r, 's> fn(&'r egui::Context, &'s mut i32) -> _
// for<'r, 's> fn(&'r egui::context::Context, &'s mut _) -> _