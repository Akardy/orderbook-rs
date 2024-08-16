mod matching_engine;
mod cli;

use cli::run_cli;
use matching_engine::engine::MatchingEngine;



fn main() {
    let mut engine = MatchingEngine::new();
    run_cli(&mut engine)
}

