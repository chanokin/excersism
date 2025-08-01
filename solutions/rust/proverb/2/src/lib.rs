pub fn build_proverb(list: &[&str]) -> String {
    let mut output: String = String::new();

    if list.is_empty() { return output; }
    
    let verse = "For want of a THING1 the THING2 was lost.\n";
    let ending = "And all for the want of a THING.";

    let n_things = list.len();
    for thing_index in 0..n_things - 1 {
        let thing1 = *list.get(thing_index).unwrap();
        let thing2 = *list.get(thing_index + 1).unwrap();
        let _verse = verse.replace("THING1", thing1).replace("THING2", thing2);
        output.push_str(&_verse);
    }

    output.push_str(&ending.replace("THING", list.first().unwrap()));

    output
}
