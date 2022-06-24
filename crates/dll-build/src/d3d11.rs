#![allow(warnings)]
use crate::app::app::App;
use egui::{
    Align2, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Pos2, Rect,
    RichText, ScrollArea, Slider, Stroke, TextureId, Vec2, Widget, Modifiers, Key,
};
use egui_d3d11::DirectX11App;
use faithe::{internal::alloc_console, pattern::Pattern};
use std::{
    intrinsics::transmute,
    sync::{Arc, Once},
};
use std::thread::sleep;
use std::time::Duration;
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Dxgi::{Common::DXGI_FORMAT, IDXGISwapChain},
        UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
    },
};

static mut APP: DirectX11App<i32> = DirectX11App::new();
static mut OLD_WND_PROC: Option<WNDPROC> = None;
static mut PROMETEY_APP: Option<App> = None;


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
        PROMETEY_APP = Some(App::new());
        APP.init_default(&swap_chain, ui);
        eprintln!("6");

        let desc = swap_chain.GetDesc().unwrap();
        if desc.OutputWindow.0 == -1 {
            eprintln!("Invalid window handle");
        }

        // OLD_WND_PROC = Some(transmute(SetWindowLongPtrA(
        //     desc.OutputWindow,
        //     GWLP_WNDPROC,
        //     hk_wnd_proc as usize as _,
        // )));
    });

    eprintln!("4");
    sleep(Duration::from_millis(500));
    APP.present(&swap_chain);
    eprintln!("5");
    sleep(Duration::from_millis(500));

    match O_PRESENT.as_ref() {
        None => {
            eprintln!("None");
            unreachable!()
        }
        Some(present) => {
            eprintln!("some");
            sleep(Duration::from_millis(500));
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
    eprintln!("Resizing buffers");
    sleep(Duration::from_millis(500));
    APP.resize_buffers(&swap_chain, || {
        eprintln!("3");
        sleep(Duration::from_millis(500));

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
    eprintln!("1");
    sleep(Duration::from_millis(500));
    APP.wnd_proc(msg, wparam, lparam);
    sleep(Duration::from_millis(500));
    eprintln!("2");

    CallWindowProcW(OLD_WND_PROC.expect("OLD WND PROC"), hwnd, msg, wparam, lparam)

}

static mut FRAME: i32 = 0;
fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        App::new().render(ctx);
    }
}

pub unsafe fn main_thread(_hinst: usize) {
    alloc_console().unwrap();

    eprintln!("Hello World!");

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

    eprintln!("Present: {:X}", present);

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

    eprintln!("Buffers: {:X}", swap_buffers);

    eprintln!("Hook1");
    sunshine::create_hook(
        sunshine::HookType::Absolute,
        transmute::<_, FnPresent>(present),
        hk_present as FnPresent,
        &mut O_PRESENT,
    )
        .unwrap();
    eprintln!("Hook2");
    sunshine::create_hook(
        sunshine::HookType::Compact,
        transmute::<_, FnResizeBuffers>(swap_buffers),
        hk_resize_buffers as FnResizeBuffers,
        &mut O_RESIZE_BUFFERS,
    )
        .unwrap();
    eprintln!("Hook3");
    #[allow(clippy::empty_loop)]
    loop {}

}

// for<'r, 's> fn(&'r egui::Context, &'s mut i32) -> _
// for<'r, 's> fn(&'r egui::context::Context, &'s mut _) -> _