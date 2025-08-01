pub fn get_diamond(c: char) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    if c == 'A' {
        output.push(String::from("A"));
        return output;
    }

    // here 'A' ascii code is 65 but we want the
    // number of rows, so we add a 1
    let n_rows = (c as u8) - 64;
    let total_spaces = 2 * n_rows - 1;
    // println!("n_rows {n_rows} => spaces {total_spaces}");
    
    for row_up in 0..n_rows {
        let row_char = (65 + row_up) as char;
        let n_chars = if row_up == 0 {1} else {2};
        let n_external_spaces = n_rows - row_up - 1;
        let n_internal_spaces = total_spaces - n_external_spaces * 2 - n_chars;
        // println!(
        //     "going up: row {} / {}\n\tnum chars: {}\n\tspaces: external {} internal {}",
        //     row_up,
        //     row_char,
        //     n_chars,
        //     n_external_spaces,
        //     n_internal_spaces,
        // );

        let mut row_out = String::new();
        for _ in 0..n_external_spaces {
            row_out.push(' ');
        }

        row_out.push(row_char);

        for _ in 0..n_internal_spaces {
            row_out.push(' ');
        }

        if n_chars == 2 {
            row_out.push(row_char);
        }

        for _ in 0..n_external_spaces {
            row_out.push(' ');
        }
        
        output.push(row_out);
    }

    for row_down in (0..n_rows - 1).rev() {
        let row = output[row_down as usize].clone();

        output.push(row);
    }

    output
}
