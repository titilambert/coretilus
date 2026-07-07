use coretilus::command::CommandV2;
use coretilus::commands::dog::cli_v2::Dog;

fn main() {
    coretilus::windows_timer::boost_timer_resolution();
    let mut dog = Dog {};
    dog.run();
}
