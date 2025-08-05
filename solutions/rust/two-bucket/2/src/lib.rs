#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new() -> Self {
        BucketStats {
            moves: 0,
            goal_bucket: Bucket::One,
            other_bucket: 0,
        }
    }
}

struct ActionBucket {
    pub id: Bucket,
    pub capacity: u8,
    pub amount: u8,
}

impl ActionBucket {
    pub fn new(id: Bucket, capacity: u8) -> ActionBucket {
        Self {
            id, 
            capacity,
            amount: 0u8,      
        }
    }

    pub fn pour_to_bucket(&mut self, other: &mut ActionBucket) {
        let need_to_fill = other.needed_to_fill();
        if need_to_fill > self.amount {
            other.amount += self.amount;
            self.empty();
        } else {
            self.amount -= need_to_fill;
            other.fill();
        }
    }

    pub fn needed_to_fill(&self) -> u8 {
        self.capacity - self.amount
    }

    pub fn is_empty(&self) -> bool {
        self.amount == 0
    }
    
    pub fn empty(&mut self) {
        self.amount = 0;
    }

    pub fn fill(&mut self) {
        self.amount = self.capacity;
    }

    pub fn is_full(&self) -> bool {
        self.amount == self.capacity
    }

}


/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }

    println!("capacity 1: {capacity_1}, capacity 2: {capacity_2}, goal: {goal}, start: {start_bucket:?}");
    // After an action, you may not arrive at a state where 
    // the initial starting bucket is empty and the other bucket is full.
    let mut stats = BucketStats::new();
    
    let other_bucket: Bucket = if start_bucket == &Bucket::One {Bucket::Two} else {Bucket::One};
    
    let start_capacity: u8 = if start_bucket == &Bucket::One {capacity_1} else {capacity_2};
    let other_capacity: u8 = if start_bucket == &Bucket::One {capacity_2} else {capacity_1};
    
    let mut start = ActionBucket::new(start_bucket.clone(), start_capacity);
    let mut other = ActionBucket::new(other_bucket, other_capacity);

    start.fill();
    
    for move_num in 1..50 {
        // exit condition
        // if any of the two buckets have reached the desired quantity in them
        // return the stats
        if start.amount == goal {
            stats.moves = move_num;
            stats.goal_bucket = start.id;
            stats.other_bucket = other.amount;
            return Some(stats);
        }
        if other.amount == goal {
            stats.moves = move_num;
            stats.goal_bucket = other.id;
            stats.other_bucket = start.amount;
            return Some(stats);
        }

        // otherwise, we need to do some liquid swapping
        if other.capacity == goal {
            other.fill();
            continue;
        } 
        
        if other.is_full() {
            other.empty();
            continue;
        }

        // make sure we don't end with a start empty and other full scenario
        if start.is_empty() {
            start.fill();
            continue;
        }
        
        start.pour_to_bucket(&mut other);
         
    }
    
    None
}
