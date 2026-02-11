fn upto_9(n: u64) -> String {
    let output: &str = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    };

    output.to_string()
}

fn upto_19(n: u64) -> String {
    if n < 10 {
        return upto_9(n);
    }
    
    let output: &str = match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    };


    output.to_string()
}
fn upto_99(n: u64) -> String {
    if n < 20 {
        return upto_19(n);
    }
    
    let tens = n / 10;
    let uni = n - tens * 10;
    
    let output: &str = match tens {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    };

    if uni == 0 {
        return output.to_string();
    }

    (output.to_owned() + "-" + upto_9(uni).as_str()).to_string()
}

fn upto_999(n: u64) -> String {
    
    let hundreds = n / 100;
    let rest = n - hundreds * 100; 
    let mut output: Vec<String> = Vec::new();
    
    if hundreds > 0 {
        output.push(upto_9(hundreds));
        output.push("hundred".to_string());
    }

    if rest != 0 {
        output.push(upto_99(rest));
    }

    output.join(" ")
}

fn chunk_name(chunk: u64) -> String {
    let output: &str = match chunk {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => "",
    };

    output.to_string()
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    
    if n < 1000 {
        return upto_999(n);
    }

    let l10: u64 = n.ilog10() as u64;
    let n_chunks = l10 / 3;
    
    let mut output: Vec<String> = Vec::new();
    let mut remainder = n;
    for chunk in (0..n_chunks).rev() {
        let section = 1000u64.pow(chunk as u32 + 1);
        let current = remainder / section;
        let name = chunk_name(chunk + 1);
        remainder -= current * section;
        
        if current == 0 {
            continue;
        }
        
        output.push(upto_999(current));
        output.push(name);
        
    }

    if remainder > 0 {
        output.push(upto_999(remainder));
    }
    


    output.join(" ")
}
