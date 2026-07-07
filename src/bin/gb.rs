use coretilus::command::CommandV2;
use coretilus::commands::gb::cli_v2::Gb;

fn main() {
    coretilus::windows_timer::boost_timer_resolution();
    let mut gb = Gb {};
    gb.run();
}
