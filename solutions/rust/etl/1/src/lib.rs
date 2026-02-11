use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // todo!("How will you transform the tree {h:?}?")

    let mut output = BTreeMap::<char, i32>::new();
    for (value, uc_chars) in h {
        for ch in uc_chars {
            output.insert(ch.to_lowercase().next().unwrap(), *value);
        }
    }
    
    output 
}
