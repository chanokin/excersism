pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let mut output = Vec::<String>::new();

    if minefield.is_empty() {
        return output;
    }
    
    let width = (*minefield).first().unwrap().len();
    let height = minefield.len();
    
    for row in 0 .. height {
        let top = if row == 0 {0} else {row - 1};
        let bottom = if row < height - 1 {row + 2} else {height};
        let mut output_row = String::new();
        
        for col in 0 .. width {
            let left = if col == 0 {0} else {col - 1};
            let right = if col < width - 1 {col + 2} else {width};
            let mut count = 0;

            for in_row in top .. bottom {
                let row_vals = (*minefield).get(in_row);
                if row_vals.is_none() {
                    return output;
                }

                let row_vals = row_vals.unwrap();
                for in_col in left .. right {
                    let val_at = (*row_vals).as_bytes().get(in_col);

                    if val_at.is_none() {
                        break;
                    }
                    
                    if *val_at.unwrap() == b'*' {
                        count += 1;
                    }
                }
            }


            let row_vals = (*minefield).get(row);
            let val_at = row_vals.unwrap().as_bytes().get(col);

            if val_at.is_none() {
                break;
            }
            else if *val_at.unwrap() == b'*' {
                output_row.push('*');
                
            } else if count == 0 {
                output_row.push(' ');
                
            } else {
                output_row.push(char::from_digit(count, 10).unwrap());
            }
            
        }
        output.push(output_row);
    }

    output
}
