#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

use crate::RoundOutcome::{BothCheated, BothCooperated, LeftCheated, RightCheated};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left: Box<dyn Agent>,
    right: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Game {
            left,
            right,
            left_score: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let res = (self.left.play(), self.right.play());
        self.left.apply(res.1);
        self.right.apply(res.0);
        match res {
            (false, false) => BothCheated,
            (false, true) => {
                self.left_score += 3;
                self.right_score -= 1;
                LeftCheated
            }
            (true, false) => {
                self.left_score -= 1;
                self.right_score += 3;
                RightCheated
            }
            (true, true) => {
                self.left_score += 2;
                self.right_score += 2;
                BothCooperated
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Agent {
    fn play(&self) -> bool; // true - играл честно, false - жульничал
    fn apply(&mut self, _: bool); // сделать вывод на основе хода противника
}
#[derive(Default)]
pub struct CheatingAgent {}

impl CheatingAgent {
    pub fn new() -> Self {
        CheatingAgent {}
    }
}

impl Agent for CheatingAgent {
    fn play(&self) -> bool {
        false
    }

    fn apply(&mut self, _: bool) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}
impl CooperatingAgent {
    pub fn new() -> Self {
        CooperatingAgent {}
    }
}

impl Agent for CooperatingAgent {
    fn play(&self) -> bool {
        true
    }

    fn apply(&mut self, _: bool) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    opponent_was_fair: bool,
}

impl GrudgerAgent {
    pub fn new() -> Self {
        GrudgerAgent {
            opponent_was_fair: true,
        }
    }
}

impl Agent for GrudgerAgent {
    fn play(&self) -> bool {
        self.opponent_was_fair
    }

    fn apply(&mut self, opponent: bool) {
        self.opponent_was_fair &= opponent
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {
    previous_opponent_action: bool, // true - сыграл честно, false - обманул
}

impl CopycatAgent {
    pub fn new() -> Self {
        CopycatAgent {
            previous_opponent_action: true,
        }
    }
}

impl Agent for CopycatAgent {
    fn play(&self) -> bool {
        self.previous_opponent_action
    }

    fn apply(&mut self, opponent: bool) {
        self.previous_opponent_action = opponent
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    round: usize,
    opponent_was_fair: bool, // true - copycat, false - cheating
    prev: bool,
}

impl DetectiveAgent {
    pub fn new() -> Self {
        DetectiveAgent {
            round: 1,
            opponent_was_fair: true,
            prev: true,
        }
    }
}

impl Agent for DetectiveAgent {
    fn play(&self) -> bool {
        match self.round {
            1 | 3 | 4 => true,
            2 => false,
            _ => match self.opponent_was_fair {
                true => false,
                false => self.prev,
            },
        }
    }

    fn apply(&mut self, opponent: bool) {
        if self.round <= 4 {
            self.opponent_was_fair &= opponent;
        }

        self.prev = opponent;
        self.round += 1;
    }
}
