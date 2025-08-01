pub fn encrypt(input: &str) -> String {
    let mut translated = String::new();
    if input.is_empty() {
        return translated;
    }

    let normalized = input.chars()
                          .filter(|ch| ch.is_alphanumeric())
                          .map(|x| x.to_lowercase().next().unwrap())
                          .collect::<String>();

    if normalized.is_empty() {
        return translated;
    }
    
    // println!("{:?}", normalized);
    let n_columns = (normalized.len() as f32).sqrt().ceil() as usize;
    let n_rows = (normalized.len() + n_columns - 1)/ n_columns;

    // println!("cols {n_columns} rows {n_rows}");

    let mut squared = Vec::new();
    for start in (0..normalized.len()).step_by(n_columns) {
        let end = std::cmp::min(start + n_columns, normalized.len());

        let mut substr = String::from(normalized.get(start..end).unwrap());

        let extra = start + n_columns - end;
        for _ in  0..extra {
            substr.push(' ');
        }
        
        squared.push(substr);
        
    }

    for col in 0..n_columns {
        for row in 0..n_rows {
            let char_at = squared[row].chars().nth(col).unwrap();
            // println!("({row}, {col}) = {char_at}");
            
            translated.push(char_at);
        }

        if col < (n_columns - 1) {
            translated.push(' ');
        }
        
    }    
    translated
}
