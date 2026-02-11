pub struct RailFence {
    n_rails: u32
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            n_rails: rails
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let text = text.replace(char::is_whitespace, "");
        let mut rails: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.n_rails {
            rails.push(Vec::new());
        }

        let mut sign: i32 = 1;
        let mut index: i32 = 0;
        let n_rails: i32 = self.n_rails as i32;
        for ch in text.chars() {
            if let Some(rail) = rails.get_mut(index as usize) {
                rail.push(ch);
            }

            index += sign;
            if index == (n_rails - 1) || index == 0 {
                sign *= -1;
            }
        }

        let mut output = String::new();
        for rail in rails {
            output += String::from_iter(rail.into_iter()).as_str();
        }

        output 
    }

    pub fn decode(&self, cipher: &str) -> String {
        
        let cipher = cipher.replace(char::is_whitespace, "").to_string();
        let mut sign: i32 = 1;
        let mut index: i32 = 0;
        let n_rails: i32 = self.n_rails as i32;
        
        let mut counts: Vec<usize> = vec![0;n_rails as usize];
        
        for _ in 0..cipher.len() {
            if let Some(count) = counts.get_mut(index as usize) {
                *count += 1;
            }

            index += sign;
            if index == (n_rails - 1) || index == 0 {
                sign *= -1;
            }
        }

        let mut rails: Vec<String> = Vec::new();
        let mut start:usize = 0;
        for count in counts {
            let end = start + count;
            rails.push(cipher.get(start..end).unwrap().to_string());
            start = end;
        }

        let mut output = String::new();
        index = 0;
        sign = 1;
        for _ in 0..cipher.len() {
            
            if let Some(rail) = rails.get_mut(index as usize) {
                let ch = rail.remove(0);
                output.push(ch);
            }
            
            index += sign;
            if index == (n_rails - 1) || index == 0 {
                sign *= -1;
            }
        }
        
        output 
    }
}
