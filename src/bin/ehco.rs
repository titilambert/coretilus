use coretilus::command::CommandV2;
use coretilus::commands::ehco::cli_v2::Ehco;

fn main() {
    coretilus::windows_timer::boost_timer_resolution();
    let mut ehco = Ehco {};
    ehco.run();
}
