pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let order = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    let mut output = Vec::<&'static str>::new();
    let position = order.iter().position(|&x| x.to_lowercase() == student.to_lowercase());

    if position.is_none() {
        return output;
    }

    // multipy by 2 here because each student has 2 slots per row
    let position = position.unwrap() * 2;
    let rows: Vec<&str> = diagram.split('\n').collect();
    for row in rows {
        for ch in row.get(position .. position + 2).unwrap().chars() {
            output.push( match ch {
                    'V' => "violets",
                    'C' => "clover",
                    'R' => "radishes",
                    'G' => "grass",
                    _ => "ZA",
                }
            )
        }
    }

    output
}
