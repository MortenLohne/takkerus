//
// This file is part of Takkerus.
//
// Takkerus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Takkerus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Takkerus. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright 2016 Chris Foster
//

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Piece {
    Flatstone(Color),
    StandingStone(Color),
    Capstone(Color),
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub enum TurnResult {
    Normal,
    Win(Color),
}

#[derive(Debug)]
pub enum GameError {
    OutOfBounds,
    InvalidMove,
    IllegalMove,
}

pub use self::board::Board;
pub use self::player::Player;
pub use self::ply::Ply;

mod board;
mod player;
mod ply;
