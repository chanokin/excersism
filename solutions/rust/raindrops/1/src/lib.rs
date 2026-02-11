pub fn raindrops(n: u32) -> String {
    let mut output = String::new();
    let div_by_3: bool = n % 3 == 0;
    if div_by_3 {
        output.push_str("Pling");
    
    } 
    
    let div_by_5: bool = n % 5 == 0;
    if div_by_5 {
        output.push_str("Plang");
    
    } 
    let div_by_7: bool = n % 7 == 0;
    if div_by_7 {
        output.push_str("Plong");
    
    }
    
    if ! (div_by_3 || div_by_5 || div_by_7){
        output.push_str(&n.to_string());
    }
    
    output
}
