pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::<Vec<u32>>::new();

        for row_index in 0 .. self.row_count {
            if row_index == 0 {
                rows.push([1].to_vec());
                continue;
            }

            let n_cols = row_index + 1;
            let top_row = &rows[row_index as usize - 1];
            let len_top = top_row.len() as u32;
            let mut row = Vec::<u32>::new();
            for col_index in 0..n_cols {
                let top_left = if col_index == 0 {0} 
                               else {top_row[col_index as usize - 1]};
                let top_right = if col_index == len_top {0} 
                                else {top_row[col_index as usize]};
                row.push(top_left + top_right);
            }
            rows.push(row);
        }

        
        rows
    }
}
