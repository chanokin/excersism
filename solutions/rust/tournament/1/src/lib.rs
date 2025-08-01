use std::collections::{BTreeMap, HashSet};
fn add_or_insert<'a>(key: &'a str, hmap: &mut BTreeMap<&'a str, u32>) {
    if let Some(val) = hmap.get_mut(key) {
        *val += 1;
    } else {
        hmap.insert(key, 1);
    }
}

fn win<'a>(home: &'a str, visitor: &'a str, 
           wins: &mut BTreeMap<&'a str, u32>,
           losses: &mut BTreeMap<&'a str, u32>) {
    add_or_insert(home, wins);
    add_or_insert(visitor, losses);
}

fn loss<'a>(home: &'a str, visitor: &'a str, 
            wins: &mut BTreeMap<&'a str, u32>,
            losses: &mut BTreeMap<&'a str, u32>) {
    add_or_insert(home, losses);
    add_or_insert(visitor, wins);
}


fn draw<'a>(home: &'a str, visitor: &'a str, 
            draws: &mut BTreeMap<&'a str, u32>) {
    add_or_insert(home, draws);
    add_or_insert(visitor, draws);
}

pub fn tally(match_results: &str) -> String {
    let header = vec!["Team                          ", "MP", " W", " D", " L", " P"].join(" | ");
    
    if match_results.is_empty() {
        return header;
    }
    
    let mut table: Vec<Vec<u32>> = Vec::new();
    let rows: Vec<_> = match_results.split('\n').collect();
    let mut played: BTreeMap<&str, u32> = BTreeMap::new();
    let mut wins: BTreeMap<&str, u32> = BTreeMap::new();
    let mut losses: BTreeMap<&str, u32> = BTreeMap::new();
    let mut draws: BTreeMap<&str, u32> = BTreeMap::new();

    // count wins, losses, draw maps
    for row in rows.iter() {
        let words: Vec<_> = row.split(';').collect();
        let home = words.get(0).unwrap();
        let visitor = words.get(1).unwrap();
        let decision = words.get(2).unwrap();

        add_or_insert(home, &mut played);
        add_or_insert(visitor, &mut played);
        
        match *decision {
            "win" => win(home, visitor, &mut wins, &mut losses),
            "loss" => loss(home, visitor, &mut wins, &mut losses),
            _ => draw(home, visitor, &mut draws),
        }
    }

    // count total points
    let mut points: BTreeMap<&str, u32> = BTreeMap::new();
    for (team, _) in &played {
        let n_wins = if wins.contains_key(team) {wins.get(team).unwrap()} else {&0};
        let n_draw = if draws.contains_key(team) {draws.get(team).unwrap()} else {&0};
        let n_points = 3 * n_wins + n_draw;
        points.insert(team, n_points);
    }

    // sort teams by number of points
    let mut vec_points = Vec::from_iter(points);
    vec_points.sort_by(|&(_name_0, points_0), &(_name_1, points_1)|
                       points_0.cmp(&points_1).reverse()
                      );

    // make table string rows
    let mut lines: Vec<String> = Vec::new();
    lines.push(header);
    for (team, n_points) in vec_points {
        let n_played = played.get(team).expect("at least played");
        let n_wins = if wins.contains_key(team) {wins.get(team).unwrap()} else {&0};
        let n_loss = if losses.contains_key(team) {losses.get(team).unwrap()} else {&0};
        let n_draw = if draws.contains_key(team) {draws.get(team).unwrap()} else {&0};
        // println!("{team:30} {n_played:2} {n_wins:2} {n_draw:2} {n_loss:2} {n_points:2}");
        let line = [
            format!("{team:30}"), format!("{n_played:2}"), format!("{n_wins:2}"), 
            format!("{n_draw:2}"), format!("{n_loss:2}"), format!("{n_points:2}"), 
        ].join(" | ");

        lines.push(line);
    }

    lines.join("\n")
}
