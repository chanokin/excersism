pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    let numbers = ["no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    let key = "NUMBER";
    let plural = "PLURAL";
    let sentence_1 = String::from("NUMBER green bottlePLURAL hanging on the wall,\n");
    let sentence_2 = String::from("And if one green bottle should accidentally fall,\n");
    let sentence_3 = String::from("There'll be NUMBER green bottlePLURAL hanging on the wall.\n");
    
    println!("{start_bottles}: 0..{take_down}");
    for current in 0 .. take_down {
        let current: usize = (start_bottles - current) as usize;
        let name = numbers.get(current).unwrap();
        let minus_one = current - 1;
        let minus_one_name = numbers.get(minus_one).unwrap().to_lowercase();
        let minus_one_name = minus_one_name.as_str();

        let current_ch = if current > 1 {"s"} else {""};
        let minus_one_ch = if minus_one > 1 || minus_one == 0 {"s"} else {""};
        song += &sentence_1.replace(key, name).replace(plural, current_ch);
        song += &sentence_1.replace(key, name).replace(plural, current_ch);
        song += &sentence_2;
        song += &sentence_3.replace(key, minus_one_name).replace(plural, minus_one_ch);
        song.push('\n');
        
    }

    song
}
