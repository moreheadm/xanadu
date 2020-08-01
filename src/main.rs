use shakmaty::{Chess, Position};
mod engine;
mod runner;

use crate::runner::Runner;

fn main() {
    let pos = Chess::default();
    let legals = pos.legals();

    let mut runner = Runner::new();
    runner.main_loop();
}
