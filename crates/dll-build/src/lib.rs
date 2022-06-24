use hudhook::inject;
use std::process::Command;

fn main() {
    inject("DayZ_x64.exe", "dll_build.dll").ok();

}