
use std::io;
use std::io::BufRead;
use std::str::Split;
use shakmaty::{Chess, Position, Move};
use shakmaty::fen;
use shakmaty::uci::Uci;

use super::engine;
use super::engine::Engine;

fn output_best_move(position: &Chess, best_move: &Move) {
    let uci = Uci::from_move(position, best_move);

    println!("bestmove {}", uci);
}

// UCI implementation based on http://wbec-ridderkerk.nl/html/UCIProtocol.html
pub struct Runner {
    engine: Engine,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            engine: Engine::default()
        }
    }

    pub fn main_loop(&mut self) {
        let mut _stdin = io::stdin();
        let mut input = _stdin.lock().lines();

        loop {
            match input.next() {
                Some(line) => if !(match line {
                    Ok(line) => self.process(&line),
                    Err(e) => {
                        eprintln!("{}", e);
                        false
                    }
                }) { break; },
                None => break,
            }

            if self.engine.active() {
                self.engine.run_search();

                if self.engine.get_search_type() !=
                        engine::SearchType::Infinite {
                    match self.engine.get_best_move() {
                        Some(best_move) => {
                            output_best_move(
                                self.engine.get_current_position(),
                                best_move);
                            self.engine.deactivate();
                        },
                        _ => (),
                    }
                }
            }
        }

        self.cleanup()
    }

    fn process(&mut self, line: &String) -> bool {
        let mut tokens = line.as_str().split(' ');
        match tokens.next() {
            Some(command) => match command {
                "uci" => self.uci_cmd(),
                "debug" => self.debug_cmd(&mut tokens),
                "isready" => self.isready_cmd(),
                "setoption" => self.setoption_cmd(&mut tokens),
                "register" => self.register_cmd(&mut tokens),
                "ucinewgame" => self.ucinewgame_cmd(&mut tokens),
                "position" => self.position_cmd(&mut tokens),
                "go" => self.go_cmd(&mut tokens),
                "stop" => self.stop_cmd(&mut tokens),
                "ponderhit" => self.ponderhit_cmd(&mut tokens),
                "quit" => return false,
                _ => println!("Unknown command {}", command),
            },
            None => return true,
        };
        true
    }

    fn cleanup(&mut self) {

    }

    fn uci_cmd(&mut self) {
        print!("id name xanadu\nid author Max Morehead\nuciok\n");
    }

    fn debug_cmd(&mut self, tokens: &mut Split<char>) {
        eprintln!("Debug unsupported, ignoring");
    }

    fn isready_cmd(&mut self) {
        println!("readyok");
    }

    fn setoption_cmd(&mut self, tokens: &mut Split<char>) {
        eprintln!("Setoption unsupported, ignoring");
    }

    fn register_cmd(&mut self, tokens: &mut Split<char>) {
        eprintln!("register unsupported, ignoring");
    }

    fn ucinewgame_cmd(&mut self, tokens: &mut Split<char>) {
        eprintln!("Ucinewgame unsupported, ignoring");
    }

    fn position_cmd(&mut self, tokens: &mut Split<char>) {
        let position = match tokens.next() {
            Some("startpos") => {
                tokens.next(); // consume "moves"
                Chess::default()
            },
            Some(fen_start) => {
                let mut fen_string = fen_start.to_string();
                loop {
                    match tokens.next() {
                        None => break,
                        Some("moves") => break,
                        Some(fen_part) => {
                            fen_string.push_str(fen_part);
                        },
                    }
                };

                match fen::Fen::from_ascii(fen_string.as_bytes()) {
                    Ok(pos_fen) => match pos_fen.position() {
                        Ok(pos) => pos,
                        Err(err) => {
                            eprintln!("Invalid position {}", err);
                            return;
                        }
                    },
                    Err(err) => {
                        eprintln!("Invalid fen string {}", err); 
                        return;
                    },
                }

            },
            None => {
                eprintln!("Invalid position command");
                return;
            }
        };

        self.engine.set_position(position);
    }

    fn go_cmd(&mut self, tokens: &mut Split<char>) {
        loop {
            match tokens.next() {
                Some(token) => match token {
                    "searchmoves" => (),
                    "ponder" => (),
                    "wtime" => (),
                    "btime" => (),
                    "winc" => (),
                    "binc" => (),
                    "movestogo" => (),
                    "depth" => (),
                    "nodes" => (),
                    "movetime" => (),
                    "infinite" => {
                        self.engine.set_search_type(engine::SearchType::Infinite);
                    },
                    _ => eprintln!("Unknown go argument"),
                },
                None => break,
            }
        }

        self.engine.activate();
    }

    fn stop_cmd(&mut self, tokens: &mut Split<char>) {
        //let best_move = self.engine.get_best_move();
        output_best_move(
                self.engine.get_current_position(),
                self.engine.get_best_move().expect("ERROR, no best move"));

        self.engine.deactivate();
    }

    

    fn ponderhit_cmd(&mut self, tokens: &mut Split<char>) {
        eprintln!("ponderhit unsupported, ignoring");
    }

}
