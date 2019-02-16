use std::env;

mod dir_deleter;

use crate::dir_deleter::DirDeleter;

fn main() {
    let dd = DirDeleter::new(env::args().collect());

}
