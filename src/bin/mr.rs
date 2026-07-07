use std::cell::RefCell;
use std::rc::Rc;

use coretilus::command::CommandV2;
use coretilus::commands::mr::cli_v2::Mr;

fn main() {
    let mut mr = Mr {
        landed: Rc::new(RefCell::new(false)),
        retry: Rc::new(RefCell::new(false)),
    };
    mr.run();
}
