#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
struct Frame {
    rolls: Vec<Roll>,
}

impl Frame {
    fn new(r1: Roll, r2: Roll) -> Result<Self, Error> {
        if r1 + r2 > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else {
            Ok(Self {
                rolls: vec![r1, r2],
            })
        }
    }

    fn is_spare(&self) -> bool {
        !self.is_strike() && self.rolls.iter().take(2).map(|r| r.pins).sum::<u16>() == 10u16
    }

    fn is_strike(&self) -> bool {
        self.rolls.first().map_or(false, |r| r.pins == 10)
    }

    fn strike() -> Self {
        Self {
            rolls: vec![Roll { pins: 10u16 }],
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct Roll {
    pins: u16,
}

impl Roll {
    fn new(pins: u16) -> Result<Self, Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else {
            Ok(Self { pins })
        }
    }
}

impl std::cmp::PartialEq<u16> for Roll {
    fn eq(&self, c: &u16) -> bool {
        self.pins == *c
    }
}

impl std::ops::Add for Roll {
    type Output = u16;

    fn add(self, rhs: Self) -> Self::Output {
        self.pins + rhs.pins
    }
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    current_roll: Option<Roll>,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let roll = Roll::new(pins)?;
        if self.is_game_finished() {
            return Err(Error::GameComplete);
        }

        if let Some(r) = self.current_roll.take() {
            self.frames
                .last_mut()
                .filter(|last| last.is_strike())
                .map(|last| {
                    last.rolls.push(r);
                    last.rolls.push(roll);
                });
            if self.frames.len() != 10 {
                self.frames.push(Frame::new(r, roll)?);
            }
        } else {
            if roll == 10 && self.frames.len() < 10 {
                self.frames.push(Frame::strike());
            } else {
                self.current_roll = Some(roll);
                self.frames
                    .last_mut()
                    .filter(|last| last.is_spare())
                    .map(|last| last.rolls.push(roll));
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_finished() {
            Some(self.frames.iter().fold(0u16, |acc, f| {
                acc + f.rolls.iter().fold(0u16, |sum, r| sum + r.pins)
            }))
        } else {
            None
        }
    }

    fn is_game_finished(&self) -> bool {
        self.frames.len() >= 10
            && ((!self.frames.last().unwrap().is_spare()
                && !self.frames.last().unwrap().is_strike())
                || self.frames.last().unwrap().rolls.len() == 3)
    }
}
