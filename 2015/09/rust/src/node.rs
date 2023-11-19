#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Node {
    pub city: String,
    pub remaining: Vec<String>,
    pub dist: i32
}

impl PartialOrd for Node {
    fn lt(&self, other: &Self) -> bool {
        self.dist.lt(&other.dist)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.city.partial_cmp(&other.city) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.remaining.partial_cmp(&other.remaining) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}
