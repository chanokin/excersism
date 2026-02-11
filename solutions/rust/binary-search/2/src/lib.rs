pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    let mut count = 0;
    loop {
        let middle = (left + right) / 2;
        let val = array[middle];

        if val == key {
            return Some(middle);
        } 

        if middle + 1 == right { 
            left = right;  
        } else if key < val {
            right = middle;
        } else {
            left = middle;
        }

        count += 1;
        if count == 10 {
            break;
            
        }
    }
    
    None
}
