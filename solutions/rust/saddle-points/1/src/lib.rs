pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut good_trees: Vec<(usize, usize)> = Vec::new();
    if input.is_empty() {
        return good_trees;
    }


    let mut max_per_row: Vec<(usize, usize, u64)> = Vec::new();
    for (row_index, row) in input.iter().enumerate() {
        let mut max_val: u64 = 0;
        for (col, val) in row.iter().enumerate() {
            if *val > max_val {
                max_val = *val;
            }
        }
        for (col, val) in row.iter().enumerate() {
            if *val == max_val {
                max_per_row.push((row_index, col, max_val));
            }
        }

    }

    for (row_index, col_index, max_row_val) in max_per_row {
        // if we can't get them proper values (top/bottom rows)
        // make them be super tall trees
        let mut south_val: u64 = u64::MAX;
        let mut north_val: u64 = u64::MAX;
        
        if let Some(south_row) = input.get(row_index + 1) {
            south_val = *south_row.get(col_index)
                                  .expect("column wrong");
        }

        if row_index > 0 {
           if let Some(north_row) = input.get(row_index - 1) {

                north_val = *north_row.get(col_index)
                                      .expect("column wrong");
           }
        }

        if south_val >= max_row_val &&
           north_val >= max_row_val {

            good_trees.push((row_index, col_index));
        } 

    }
    
    good_trees
}
