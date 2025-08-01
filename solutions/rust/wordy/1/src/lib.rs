fn decode(num_0: i32, operation: &str, num_1: i32) -> Option<i32> {
    match operation {
        "+" => Some(num_0 + num_1),
        "-" => Some(num_0 - num_1),
        "/" => Some(num_0 / num_1),
        "*" => Some(num_0 * num_1),
        _ => None,
    }
}

pub fn answer(command: &str) -> Option<i32> {
    // println!("Return the result of the command '{command}' or None, if the command is invalid.");
    let command = command.replace("?", "")
                         .replace("multiplied by", "*")
                         .replace("divided by", "/")
                         .replace("plus", "+")
                         .replace("minus", "-");
    
    let command_array: Vec<_> = command.split_whitespace().collect();
    // println!("command array {command_array:?}");

    // remove the "What is" bits
    let command_array = &command_array[2..];

    if command_array.is_empty() {
        return None;
    }
        
    // println!("command array {command_array:?}");

    
    let n_chunks = command_array.len();
    
    // single number, just return it
    if n_chunks == 1 {
        if let Ok(num) = command_array.first().unwrap().parse::<i32>() {
            return Some(num);
        } else {
            return None;
        }
    }

    let valid_ops = ["+", "-", "/", "*"];
    let mut num0: i32 = 0;
    let mut num1: i32 = 0;
    let mut op: &str;

    let mut index = 0;
    // the first object in the command has to be a number!
    if let Ok(num) = command_array[index].parse::<i32>() {
        num0 = num;
    } else {
        return None;
    }

    index += 1;
    
    loop {
        op = command_array[index];
        
        // the operator is supported
        if ! valid_ops.iter().any(|&i| i==op) {
            return None;
        }
        
        index += 1;
        // we ended up in an operator, this is not valid
        if index >= n_chunks {
            return None;
        }

        // check if it's a valid number
        if let Ok(num) = command_array[index].parse::<i32>() {
            num1 = num;
        } else {
            return None;
        }

        // notice the ? at the end, if we didn't get a valid op, we'll return a None
        let result = decode(num0, op, num1)?;
        // println!("result {num0} {op} {num1} = {result}");

        // keep the result in the "stack"
        num0 = result;

        // advance index
        index += 1;
        
        // are we done?
        if index >= n_chunks {
            break;
        }
    }
     
    
    Some(num0)
}
