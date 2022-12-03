#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq, Eq, Clone)]
pub enum FrameType {
    Open,
    Spare,
    Strike,
    Last,
}

#[derive(Clone)]
pub struct Frame {
    first_throw: Option<u16>,
    second_throw: Option<u16>,
    fill_throw: Option<u16>,
    frame_type: FrameType,
    score: u16,
    completed: bool,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            first_throw: None,
            second_throw: None,
            fill_throw: None,
            frame_type: FrameType::Open,
            score: 0,
            completed: false,
        }
    }
}

fn is_strike(frame: Option<&Frame>) -> bool {
    frame
        .map(|f| f.frame_type == FrameType::Strike)
        .unwrap_or(false)
}

fn get_first_throw(frame: Option<&Frame>) -> u16 {
    frame.and_then(|f| f.first_throw).unwrap_or(0)
}

fn get_second_throw(frame: Option<&Frame>) -> u16 {
    frame.and_then(|f| f.second_throw).unwrap_or(0)
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    score: u16,
    completed: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            score: 0,
            completed: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.completed {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let current_frame = match self.frames.last() {
            Some(frame) if !frame.completed => self.frames.pop().unwrap(),
            _ => Frame::new(),
        };
        let is_last_frame = self.frames.len() == 9;
        let current_frame = self.update_frame(current_frame, is_last_frame, pins)?;
        self.frames.push(current_frame);
        if self.completed {
            self.compute_frame_scores();
        }
        Ok(())
    }

    fn update_frame(
        &mut self,
        mut current_frame: Frame,
        is_last_frame: bool,
        pins: u16,
    ) -> Result<Frame, Error> {
        let mut remaining_pins = 10;
        if is_last_frame {
            current_frame.frame_type = FrameType::Last;
        }
        match current_frame.first_throw {
            // The first throw at this frame is already taken
            Some(first_throw_score) => {
                let x_in_last_frame = is_last_frame && first_throw_score == 10;
                if !x_in_last_frame {
                    remaining_pins -= first_throw_score;
                }
                match current_frame.second_throw {
                    // Second throw at this frame is already taken but still the frame is not
                    // marked as completed. This can only happen in case of the last frame being
                    // a strike or a spare.
                    Some(second_throw_score) => {
                        // If the pins in the final frame are not cleared out after the first
                        // two throws (i.e. at the end of two throws, there are still pins left)
                        // then the input pins should be less than the fill throw in the final frame.
                        let is_spare = first_throw_score + second_throw_score == 10;
                        if !is_spare && second_throw_score < 10 {
                            remaining_pins = 10 - second_throw_score;
                            if pins > remaining_pins {
                                return Err(Error::NotEnoughPinsLeft);                            
                            }
                        }
                        current_frame.fill_throw = Some(pins);
                        // All three throws at the final frame are completed. Mark the frame and
                        // the game as complete.
                        current_frame.completed = true;
                        self.completed = true;
                    }
                    // The second throw is not yet taken.
                    None => {
                        if pins > remaining_pins {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        current_frame.second_throw = Some(pins);
                        // This is not the last frame
                        if current_frame.frame_type != FrameType::Last {
                            // If all 10 pins are down as combined result of both throws
                            // then the frame is a spare.
                            if pins + first_throw_score == 10 {
                                current_frame.frame_type = FrameType::Spare;
                            }
                            // Second throw on any non-last frame completes it.
                            current_frame.completed = true;
                        }
                        // This is the last frame
                        else {
                            let spare_in_last_frame = first_throw_score + pins == 10;
                            // As long as we don't have two strike or spare in the last frame,
                            // the second throw in the last frame completes both the frame and the
                            // game.
                            if !(x_in_last_frame || spare_in_last_frame || pins == 10) {
                                current_frame.completed = true;
                                self.completed = true;
                            }
                        }
                    }
                }
            }
            // This is a completely new frame with no throws recorded yet
            None => {
                if pins > remaining_pins {
                    return Err(Error::NotEnoughPinsLeft);
                }
                current_frame.first_throw = Some(pins);
                // All the pins were down after the first throw. If the current frame
                // is NOT the last frame, then this constitutes a Strike and completes
                // the frame.
                if current_frame.frame_type != FrameType::Last && pins == 10 {
                    current_frame.frame_type = FrameType::Strike;
                    current_frame.completed = true;
                }
            }
        }
        Ok(current_frame)
    }

    fn compute_frame_scores(&mut self) {
        if self.frames.len() != 10 {
            panic!("Cannot compute score until all 10 frames are completed");
        }
        for idx in 0..10 {
            let second_frame = self.frames.get(idx + 1).cloned();
            let third_frame = self.frames.get(idx + 2).cloned();
            let mut frame = self.frames.get_mut(idx).unwrap();
            match frame.frame_type {
                FrameType::Last | FrameType::Open => {
                    frame.score = frame.first_throw.unwrap_or(0)
                        + frame.second_throw.unwrap_or(0)
                        + frame.fill_throw.unwrap_or(0);
                }
                FrameType::Spare => {
                    frame.score = 10 + second_frame.and_then(|f| f.first_throw).unwrap_or(0);
                }
                FrameType::Strike => {
                    let mut base_score = 10;
                    if is_strike(second_frame.as_ref()) {
                        base_score += 10;
                        if is_strike(third_frame.as_ref()) {
                            base_score += 10;
                        }
                    }
                    if base_score == 10 {
                        frame.score = base_score
                            + get_first_throw(second_frame.as_ref())
                            + get_second_throw(second_frame.as_ref());
                    } else if base_score == 20 {
                        frame.score = base_score
                            + get_first_throw(third_frame.as_ref());
                    } else {
                        frame.score = base_score;
                    }
                }
            }
            self.score += frame.score;
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.completed {
            None
        } else {
            Some(self.score)
        }
    }
}
