pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let total = size * size;
    let mut output: Vec<Vec<u32>> = Vec::new();

    for _x in 0..size {
        output.push(vec![0; size as usize]);
    }

    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut drow: i32 = 0;
    let mut dcol: i32 = 1;
    let mut row_max: i32 = size as i32;
    let mut col_max: i32 = size as i32;
    let mut row_min: i32 = 1;
    let mut col_min: i32 = 0;
    let mut is_row_major: bool = true;
    for (idx, val) in (1..total + 1).enumerate() {
        println!("[{idx}]: ({row}, {col}) = {val}   d({drow}, {dcol})");
        if let Some(row_vec) = output.get_mut(row as usize) {
            if let Some(cell) = row_vec.get_mut(col as usize) {
                *cell = val;
            }
        }

        row += drow;
        col += dcol;
        

        // reached right
        if col == col_max && is_row_major && dcol > 0 {
            println!("col reached right {col_max}");
            is_row_major = false;
            dcol = 0;
            drow = 1;
            col_max -= 1;
            col = col_max;
            row += 1;

        } 
        // reached down
        else if row == row_max && !is_row_major && drow > 0 {
            println!("row reached down {row_max}");
            is_row_major = true;
            dcol = -1;
            drow = 0;
            row_max -= 1;
            row = row_max ;
            col -= 1;
        }
        // reached left
        else if col < col_min && is_row_major && dcol < 0 {
            println!("col reached left {col_min}");
            is_row_major = false;
            dcol = 0;
            drow = -1;
            col = col_min;
            col_min += 1;
            row -= 1;
        } 
        // reached up
        else if row < row_min && !is_row_major && drow < 0 {
            println!("row reached up {row_min}");
            is_row_major = true;
            dcol = 1;
            drow = 0;
            row = row_min ;
            row_min += 1;
            col += 1;
        }

        

        
    }

    output
}
