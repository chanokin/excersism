pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let mut output = Vec::<String>::new();

    if minefield.is_empty() {
        return output;
    }
    
    let width = (*minefield).get(0).unwrap().len();
    let height = minefield.len();
    
    for row in 0 .. height {
        let top = if row == 0 {0} else {row - 1};
        let bottom = if row < height - 1 {row + 2} else {height};
        let mut output_row = String::new();
        
        for col in 0 .. width {
            let left = if col == 0 {0} else {col - 1};
            let right = if col < width - 1 {col + 2} else {width};
            let mut count = 0;
            // println!("[{row}, {col}] : {top},{left} => {bottom},{right}");
            for in_row in top .. bottom {
                let row_vals = (*minefield).get(in_row);
                if row_vals.is_none() {
                    return output;
                }

                let row_vals = row_vals.unwrap();
                for in_col in left .. right {
                    let val_at = (*row_vals).bytes().nth(in_col);
                    
                    // let x = if val_at.is_none() {'_'} else {val_at.unwrap()};
                    // println!("{in_row}, {in_col} : {x}");

                    if val_at.is_none() {
                        break;
                    }
                    
                    if val_at.unwrap() == b'*' {
                        count += 1;
                    }
                }
            }


            let row_vals = (*minefield).get(row);
            let val_at = row_vals.unwrap().bytes().nth(col);

            // println!("count {row},{col} = {count}");

            if val_at.is_none() {
                break;
            }
            else if val_at.unwrap() == b'*' {
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
