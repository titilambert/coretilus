use coretilus::command::CommandV2;
use coretilus::commands::sl::cli_v2::Sl;

fn main() {
    coretilus::windows_timer::boost_timer_resolution();
    let mut sl = Sl {};
    sl.run();
}
