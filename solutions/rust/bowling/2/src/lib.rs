use std::fmt;

const MAX_FRAMES_STD: u8 = 10;
const START_PINS: u16 = 10;
const START_BALLS: u16 = 2;


#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FrameCase {
    Start,
    Open,
    Strike,
    Spare,
    FinalFrameStrike,
    FinalFrameSpare,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum GameState {
    Start,
    TenFrames,
    BonusFromStrike,
    BonusFromSpare,
    Finished,
}


#[derive(Copy, Clone)]
struct Frame {
    number: u8,
    n_balls: u16,
    n_pins: u16,
    rolls: [u16; 2],
    score: u16,
    case: FrameCase,
    previous_case: FrameCase,
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
         .field("number", &self.number)
         .field("balls", &self.n_balls)
         .field("pins", &self.n_pins)
         .field("score", &self.score)
         .field("case", &self.case)
         .field("previous_case", &self.previous_case)
         .field("done", &self.is_finished())
         .finish()
    }
}

impl Frame {
    pub fn new (_number: u8) -> Self {
        Self {
            number: _number,
            n_balls: START_BALLS,
            n_pins: START_PINS,
            rolls: [0, 0],
            score: 0,
            case: FrameCase::Start,
            previous_case: FrameCase::Unknown,
        }
    }

    fn in_bonus_round(&self) -> bool {
        self.previous_case == FrameCase::FinalFrameSpare || 
        self.previous_case == FrameCase::FinalFrameStrike
    }
    
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        println!("F #{}: pins {} / balls {}", 
                 self.number, self.n_pins, self.n_balls);
        self.rolls[(2 - self.n_balls) as usize] = pins;
        
        if self.n_pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        let strike_roll = pins == START_PINS && (self.case == FrameCase::Start || self.in_bonus_round());
        let in_bonus_round = self.in_bonus_round();
        
        if strike_roll && !in_bonus_round {
            self.n_balls -= 1; // no need to reduce to 0
        } 

        self.score += pins;
        self.n_pins -= pins;
        self.n_balls -= 1;

        if strike_roll && in_bonus_round {
            self.n_pins += START_PINS;
            self.case = FrameCase::Strike; 
        }
        
        if self.is_finished() {
            if strike_roll {
                self.case = FrameCase::Strike;
            }
            else if self.score == START_PINS {
                self.case = FrameCase::Spare;
            } 
            else {
                self.case = FrameCase::Open;
            }
        }         
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.n_balls == 0
    }
    
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    current_frame: Frame,
    state: GameState,
}


impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::<Frame>::new(),
            current_frame: Frame::new(1),
            state: GameState::Start,
        }
        
    }
    pub fn n_played(&self) -> u8 {
        self.frames.len() as u8
    }

    
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {

        if self.state == GameState::Finished {
            return Err(Error::GameComplete);
        }

        if self.state == GameState::Start {
            self.state = GameState::TenFrames;
        }
        
        let roll_result = self.current_frame.roll(pins)?;

        println!("Game state: {:#?}", self.state);
        println!("{:#?}", self.current_frame);
        
        if self.current_frame.is_finished() {
            if self.state == GameState::BonusFromStrike ||
               self.state == GameState::BonusFromSpare {
                println!("Finished a frame in a bonus round");    
                self.state = GameState::Finished;
            } 
            
            self.finish_frame();

        }

        
        Ok(())    
    }

    pub fn finish_frame(&mut self) {
        println!("Finishing frame");
        let final_spare = self.final_frame_spare();
        let final_strike = self.final_frame_strike();

        self.frames.push(self.current_frame);
        
        // no bonus and final frame
        if !(final_strike || final_spare) && 
            self.current_frame.number == MAX_FRAMES_STD {

            println!("finish game if no bonus round");
            self.state = GameState::Finished;
            return;
        }

        let prev_case = self.current_frame.case;
        let number = self.current_frame.number + 1;
        self.current_frame = Frame::new(number);
        self.current_frame.previous_case = prev_case;
        
        if final_spare {
            self.current_frame.n_balls = 1;
            self.current_frame.previous_case = FrameCase::FinalFrameSpare;
            self.state = GameState::BonusFromSpare;
        } 

        if final_strike {
            self.current_frame.previous_case = FrameCase::FinalFrameStrike;
            self.state = GameState::BonusFromStrike;
        }
        
    }

    pub fn score_bonus_spare(&self, frame_index: usize) -> Option<u16> {
        let frame = self.frames[frame_index];
        if frame.previous_case == FrameCase::FinalFrameStrike {
            return Some(0);
        }
        
        if frame_index + 1 >= self.frames.len() {
            return None;
        }
        
        let next_frame = self.frames[frame_index + 1];
       
        Some(next_frame.rolls[0])
    }
    pub fn score_bonus_strike(&self, frame_index: usize) -> Option<u16> {
        let frame = self.frames[frame_index];
        if frame.previous_case == FrameCase::FinalFrameSpare ||
           frame.previous_case == FrameCase::FinalFrameStrike {
            return Some(0);
        } 
        
        if frame_index + 1 >= self.frames.len() {
            return None;
        }
        
        let mut score = 0;

        let next_frame = self.frames[frame_index + 1];

        if next_frame.previous_case == FrameCase::FinalFrameStrike {
            return Some(0);
        } 
        
        if next_frame.case != FrameCase::Strike {
            for ball_idx in 0..2 {
                score += next_frame.rolls[ball_idx];
            }
        } else {
            score += next_frame.rolls[0];
            let next_next = self.frames[frame_index + 2];
            score += next_next.rolls[0];
        }

        
        
        Some(score)
    }
    
    pub fn score(&self) -> Option<u16> {
        if self.state == GameState::Finished {
            println!("game finished - scoring:");
            let mut score = 0u16;
            println!("length of frames {}", self.frames.len());
            for frame_index in 0..self.frames.len() {
                let frame = self.frames[frame_index];
                let frame_score = frame.score;
                print!("({frame_index}: {score} + {frame_score}");
                score += frame_score;
                
                if frame.case == FrameCase::Strike {
                    let bonus = self.score_bonus_strike(frame_index)?;
                    print!("| strike {bonus}");
                    score += bonus;
                }

                if frame.case == FrameCase::Spare {
                    let bonus = self.score_bonus_spare(frame_index)?;
                    print!("| spare {bonus}");
                    score += bonus;
                }

                println!("),");
            }
            return Some(score);
        } 

        
        println!("not all frames have been played");
        None
    }

    pub fn final_frame_spare(&mut self) -> bool {
        if self.state == GameState::TenFrames &&
           self.current_frame.case == FrameCase::Spare && 
           self.current_frame.number == MAX_FRAMES_STD {

            println!("final frame spare =============");
            self.state = GameState::BonusFromSpare;
            return true;
        }

        false
    }

    pub fn final_frame_strike(&mut self) -> bool {
        if self.state == GameState::TenFrames &&
           self.current_frame.case == FrameCase::Strike && 
           self.current_frame.number == MAX_FRAMES_STD {
            
            println!("final frame strike !!!!!!!!!!!!!!!!!! ");
            self.state = GameState::BonusFromStrike;
            return true;
        }

        false
    }

    
    

}
