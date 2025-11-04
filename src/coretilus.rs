#![doc(
    html_favicon_url = "https://gitlab.com/ttblt-oss/coretilus/coretilus/-/raw/main/coretilus.png?ref_type=heads&inline=false"
)]
#![doc(
    html_logo_url = "https://gitlab.com/ttblt-oss/coretilus/coretilus/-/raw/main/coretilus.png?ref_type=heads&inline=false"
)]

pub mod animation;
pub mod collision;
pub mod command;
pub mod commands;
pub mod coord;
pub mod engine;
pub mod frame;
pub mod sprite;
pub mod tools;
pub mod trajectory;

#[cfg(unix)]
mod signal {
    const SIGINT: i32 = 2;
    const SIG_IGN: usize = 1;
    unsafe extern "C" {
        unsafe fn signal(sig: i32, handler: usize) -> usize;
    }

    #[allow(dead_code)]
    pub fn ignore_sigint() {
        unsafe {
            signal(SIGINT, SIG_IGN);
        }
    }
}
