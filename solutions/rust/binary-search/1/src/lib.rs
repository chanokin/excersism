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
        println!("l:{left} m:{middle} r:{right} => {val}");
        if val == key {
            return Some(middle);
        } 

        if middle + 1 == right {
            let val = array[right];
            if val == key {
                return Some(right);
            }
        }

        if key < val {
            right = middle;
        } else {
            left = middle;
        }

        count += 1;
        if count == 10 {
            break;
            
        }
    }
    
    // todo!(
    //     "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
    // );
    None
}
