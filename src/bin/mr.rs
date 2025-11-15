use std::cell::RefCell;
use std::rc::Rc;

use coretilus::command::Command;
use coretilus::commands::mr::cli::Mr;

fn main() {
    let mut mr = Mr {
        landed: Rc::new(RefCell::new(false)),
    };
    mr.run();
}
