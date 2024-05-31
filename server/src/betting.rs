use std::cmp::Ordering;
use rocket::serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct GameResult {
    pub home: u16,
    pub away: u16,
}

pub fn goal_difference(result : &GameResult) -> i32 {
    i32::from(result.home) - i32::from(result.away)
}

#[repr(i8)] // copying std::cmp::ordering
#[derive(Eq, PartialEq)]
pub(crate) enum Winner {
    Home = -1,
    Draw = 0,
    Away = 1,
}

pub fn winner(result : &GameResult) -> Winner {
    match result.home.cmp(&result.away) {
        Ordering::Greater => { Winner::Home }
        Ordering::Equal => { Winner::Draw }
        Ordering::Less => { Winner::Away }
    }
}

pub fn points(guess : GameResult, actual : GameResult) -> u16 {
    if guess == actual { 8 }
    else if goal_difference(&guess) == goal_difference(&actual) { 6 }
    else if winner(&guess) == winner(&actual) { 4 }
    else { 0 }
}
