use std::collections::{
    HashMap,
    HashSet,
};

use itertools::Itertools;

fn parse_trial(chars: &Vec<char>, trial: &Vec<u32>, cant_be_zero: &Vec<char>, map: &mut HashMap<char, u32>) -> bool {
    
    for (ch, val) in std::iter::zip(chars, trial) {
        
        if cant_be_zero.contains(ch) && *val == 0 {
            // print!("i:{ch} ");
            return false;
        }
        map.insert(*ch, *val);
    }

    true
}


fn is_valid(puzzle: &Vec<&str>, chars: &Vec<char>, trial: &Vec<u32>, cant_be_zero: &Vec<char>, map: &mut HashMap<char, u32>) -> bool {
    let valid = parse_trial(chars, trial, cant_be_zero, map);
    if !valid {
        return false;
    }

    let mut numbers = Vec::<u32>::new();
    for word in puzzle {
        let mut mult: u32 = 1;
        let mut number: u32 = 0;
        for ch in word.chars().rev() {
            let digit: u32 = *map.get(&ch).unwrap();
            number += mult * digit;
            mult *= 10;            

        }

        numbers.push(number);
    }
    let sol = *numbers.last().unwrap();
    let sum: u32 = numbers.iter().sum::<u32>() - sol;
    if  sum != sol {
        return false;
    }

    // println!("sum {sum} == {sol}");
    true
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut solution = HashMap::<char, u32>::new();
    if input.is_empty() {
        return None;
    }

    if ! input.contains("+") {
        return None;
    }

    let single_equals = input.replace("==", "="); // binding
    let words: Vec<&str> = single_equals.split(&['+', '='][..])
                                        .map(|w| w.trim())
                                        .collect();
    let mut max_in_word_size = 0;
    for (i, w) in words.iter().enumerate() {
        if i == words.len() - 1 {
            break;
        }
        
        if w.len() > max_in_word_size {
            max_in_word_size = w.len();
        }
    }
    let result_size = words.last().unwrap().len();
    if max_in_word_size > result_size {
        
        // println!("odd sum {max_in_word_size} > {result_size} ");
        return None;
    }
    
    for w in &words {
        if w.is_empty() {
            return None;
        }
    }

    let initials: HashSet<char> = words.clone().into_iter().map(|w| w.chars().nth(0).unwrap()).collect();
    let initials: Vec<char> = initials.into_iter().collect();
    
    
    // for ch in &initials {
    //     println!("initial {ch}");
    // }

    
    let _chars: HashSet<char> = input.chars().filter(|x| x.is_alphabetic()).collect();
    let _chars: Vec<char> = _chars.into_iter().collect();
    
    for ch in &_chars {
        solution.insert(*ch, 0);
    }

    let n_digits = _chars.len();
    let perms = (0u32..10u32).rev().permutations(n_digits);

    // let mut count: u64 = 0;
    for perm in perms {
        // for p in &perm {
        //     print!("{p} ");
        // }
        // println!();
        
        let solved = is_valid(&words, &_chars, &perm, &initials, &mut solution);
        if solved {
            println!("solution found!");
            break;
        }

        // count += 1;
        // if count > 2520000 {
        // if count > 100000000 {
        //     break;
        // }
    }
    // let solution = perms.find_map(
    //                     |x| is_valid(&words, &_chars, &x, &initials));
    let mut sol: HashMap<char, u8> = HashMap::new();
    for (k, v) in solution {
        // println!("{k} => {v}");
        sol.insert(k, v as u8);
    }
    Some(sol)
}
