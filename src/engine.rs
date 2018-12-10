use shakmaty::{Chess, Move, Position};
//use rand::Rng;
use rand::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SearchType {
    Infinite,
    Timed,
}

pub struct Engine {
    position: Chess,
    active: bool,
    mode: SearchType,
    best_move: Option<Move>,
}

impl Engine {
    pub fn set_position(&mut self, position: Chess) {
        self.position = position;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.set_search_type(SearchType::Timed);
        self.best_move = None
    }

    pub fn run_search(&mut self) {
        let moves = self.position.legals();

        if self.best_move.is_none() {
            self.best_move = Some(moves[(thread_rng().gen::<f64>() *
                    moves.len() as f64) as usize].clone());
        }
    }

    pub fn set_search_type(&mut self, mode: SearchType) {
        self.mode = mode;
    }

    pub fn get_search_type(&self) -> SearchType {
        self.mode
    }

    pub fn get_best_move(&self) -> Option<&Move> {
        self.best_move.as_ref()
    }

    pub fn get_current_position(&self) -> &Chess {
        &self.position
    }
}

impl Default for Engine {
    fn default() -> Engine {
        Engine {
            position: Chess::default(),
            active: false,
            mode: SearchType::Timed,
            best_move: None,
        }
    }
}
