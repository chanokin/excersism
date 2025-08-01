use rand::prelude::*;
use std::sync::Mutex;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGISTERED_NAMES: Mutex<HashSet<String>> = {
        Mutex::new(HashSet::new())
    };
}


fn _generate_name() -> String {
    let mut name = String::new();
    let mut rng = rand::rng();
    
    for _ in 0..2 {
        name.push(
            (rng.sample(rand::distr::Alphabetic) as char)
            .to_uppercase()
            .next()
            .unwrap()
        );
    }

    let mut numbers: Vec<u32> = (0..10).collect();
    numbers.shuffle(&mut rng);
    rng.reseed();
    
    for _ in 0..3 {
        if let Some(num) = numbers.choose(&mut rng) {
            if let Some(char_num) = char::from_digit(*num, 10) {
                name.push(char_num);
            }
        }
    }
    
    name
}

fn generate_name() -> String {
    let n_tries = 100;
    let mut name = _generate_name();
    for _ in 0..n_tries {
        let already_registered = REGISTERED_NAMES.lock().unwrap().contains(&name);
        if ! already_registered {
            let register_name = name.clone();
            REGISTERED_NAMES.lock().unwrap().insert(register_name);
            
            break;
        }
        
        name = _generate_name();
    }

    name
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: generate_name(),
        }
    }
    
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }
}
