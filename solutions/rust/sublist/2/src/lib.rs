#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // both lists empty
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }

    // one list has items, the other is empty
    if !first_list.is_empty() && second_list.is_empty() {
        return Comparison::Superlist;
    }

    if first_list.is_empty() && !second_list.is_empty() {
        return Comparison::Sublist;
    }

    // both lists have items
    let first_smaller = first_list.len() <= second_list.len();
    let equal_length = first_list.len() == second_list.len();
    
    let large_list = if first_smaller {
        second_list
    } else {
        first_list
    };

    let short_list = if first_smaller {
        first_list
    } else {
        second_list
    };

    let mut iter_short = short_list.iter().peekable();
    let mut iter_large = large_list.iter().peekable();

    let mut num_equal = 0;
    let mut short_v = iter_short.next().unwrap();
    loop {
        let _large = iter_large.next();
        if _large.is_none() {
            // unequal
            break;
        }
        
        let large_v = _large.unwrap();

        if large_v == short_v {
            num_equal += 1;
            if num_equal == short_list.len() {
                if equal_length {
                    return Comparison::Equal;

                } else if first_smaller {
                    return Comparison::Sublist;

                } else {
                    return Comparison::Superlist;
                }
            }
            let _large = iter_large.peek();
            let _short = iter_short.peek();
            if _large.is_none() || _short.is_none() {
                // unequal
                break;
            }

            if iter_large.peek().unwrap() == iter_short.peek().unwrap() {
                short_v = iter_short.next().unwrap();
            } else {
                num_equal -= 1;
            }
        }
        

    }

    
    Comparison::Unequal
}
