fn upto_9(n: u64) -> String {
    let mut output: &str;
    match n {
        1 => output = "one",
        2 => output = "two",
        3 => output = "three",
        4 => output = "four",
        5 => output = "five",
        6 => output = "six",
        7 => output = "seven",
        8 => output = "eight",
        9 => output = "nine",
        _ => output = "",
    }

    output.to_string()
}

fn upto_19(n: u64) -> String {
    if n < 10 {
        return upto_9(n);
    }
    let mut output: &str = "";
    match n {
        10 => output = "ten",
        11 => output = "eleven",
        12 => output = "twelve",
        13 => output = "thir",
        14 => output = "four",
        15 => output = "fif",
        16 => output = "six",
        17 => output = "seven",
        18 => output = "eigh",
        19 => output = "nine",
        _ => output = "",
    }

    if n < 13 {
        return output.to_string();
    }

    (output.to_owned() + "teen").to_string()
}
fn upto_99(n: u64) -> String {
    if n < 20 {
        return upto_19(n);
    }
    let mut output: &str;
    let tens = n / 10;
    let uni = n - tens * 10;
    
    match tens {
        2 => output = "twenty",
        3 => output = "thirty",
        4 => output = "forty",
        5 => output = "fifty",
        6 => output = "sixty",
        7 => output = "seventy",
        8 => output = "eighty",
        9 => output = "ninety",
        _ => output = "",
    }

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
    let mut output: &str;
    match chunk {
        1 => output = "thousand",
        2 => output = "million",
        3 => output = "billion",
        4 => output = "trillion",
        5 => output = "quadrillion",
        6 => output = "quintillion",
        _ => output = "",
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
