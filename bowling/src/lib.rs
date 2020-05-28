#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const MAX_FRAMES: usize = 10;
type Score = Option<u16>;

#[derive(Debug)]
enum FrameScore {
    Strike,
    Spare(u16),
    Score(u16, u16),
}

#[derive(Copy, Clone, Debug)]
struct Frame(Score, Score);

impl Frame {
    fn is_strike(&self) -> bool {
        self.0 == Some(10)
    }

    fn is_spare(&self) -> bool {
        self.0.unwrap_or(0) + self.1.unwrap_or(0) == 10
    }

    fn is_closed(&self) -> bool {
        self.1.is_some() || self.is_strike()
    }

    fn roll(&mut self, pins: u16) {
        assert!(!self.is_closed());

        match self {
            Self(None, None) => self.0 = Some(pins),
            Self(Some(_), None) => self.1 = Some(pins),
            _ => panic!("Douglas viajou!"),
        }
    }

    fn new(pins: u16) -> Self {
        Self(Some(pins), None)
    }

    fn score(&self) -> Option<FrameScore> {
        match self {
            _ if self.is_strike() => Some(FrameScore::Strike),
            Self(Some(x), _) if self.is_spare() => Some(FrameScore::Spare(*x)),
            Self(Some(x), None) => Some(FrameScore::Score(*x, 0)),
            Self(Some(x), Some(y)) => Some(FrameScore::Score(*x, *y)),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if (self.frames.len() == MAX_FRAMES
            && self.frames.last().unwrap().is_closed()
            && !self.frames.last().unwrap().is_strike()
            && !self.frames.last().unwrap().is_spare())
            || self.frames.len() > MAX_FRAMES
        {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.frames.last_mut() {
            None => self.frames.push(Frame::new(pins)),
            Some(f) if f.is_closed() => self.frames.push(Frame::new(pins)),
            Some(f) if f.0.unwrap() + pins > 10 => {
                return Err(Error::NotEnoughPinsLeft);
            }
            Some(f) => f.roll(pins),
        };

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < 10 {
            None
        } else {
            let two_empty_frames = [Frame(Some(0), Some(0)); 2];
            Some(
                self.frames
                    .iter()
                    .chain(two_empty_frames.iter())
                    .map(|f| f.score().expect("otavio ?"))
                    .collect::<Vec<FrameScore>>()
                    .windows(3)
                    .map(|w| match w {
                        &[FrameScore::Score(a, b), _, _] => a + b,
                        &[FrameScore::Spare(_), FrameScore::Score(a, _), _]
                        | &[FrameScore::Spare(_), FrameScore::Spare(a), _] => 10 + a,
                        &[FrameScore::Spare(_), FrameScore::Strike, _] => 20,
                        &[FrameScore::Strike, FrameScore::Score(a, b), _] => 10 + a + b,
                        &[FrameScore::Strike, FrameScore::Spare(_), _] => 20,
                        &[FrameScore::Strike, FrameScore::Strike, FrameScore::Score(a, _)]
                        | &[FrameScore::Strike, FrameScore::Strike, FrameScore::Spare(a)] => 20 + a,
                        &[FrameScore::Strike, FrameScore::Strike, FrameScore::Strike] => 30,
                        _ => 0,
                    })
                    .sum(),
            )
        }
    }
}
