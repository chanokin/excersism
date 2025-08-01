use std::cmp::min; 

fn _translate(triad: &str) -> &str {
    match triad {
        "AUG" => "Methionine",
        "UUU" | "UUC" => "Phenylalanine",
        "UUA" | "UUG" => "Leucine",
        "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
        "UAU" | "UAC" => "Tyrosine",
        "UGU" | "UGC" => "Cysteine",
        "UGG" => "Tryptophan",
        "UAA" | "UAG" | "UGA" => "STOP",
        &_ => "INVALID",
    }
}

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    if rna.is_empty() {
        return Some(vec![]);
    }

    let n_chars = rna.len();
    let mut output: Vec<&str> = Vec::new();
    for start in (0..n_chars).step_by(3) {
        let end = min(start + 3, n_chars);
        // print!("chars between {start} to {end}");
        let chunk = &rna[start..end];
        // print!(" is {chunk}");
        let translated = _translate(chunk);
        
        // println!(" which translated is {translated}");
        
        if translated == "INVALID" {
            return None;
        }
        
        if translated == "STOP" {
            break;
        }


        output.push(translated);

        
        
    }
    
    Some(output)
}
