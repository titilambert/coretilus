use coretilus::command::CommandV2;
use coretilus::commands::pc::cli_v2::Pc;

fn main() {
    coretilus::windows_timer::boost_timer_resolution();
    let mut pc = Pc {};
    pc.run();
}
