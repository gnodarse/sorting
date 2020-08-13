use std::cmp::Ordering;

#[derive(Clone)]
pub struct IndexedVal {
    pub idx: usize,
    pub val: i32,
}

impl PartialOrd for IndexedVal {
    fn partial_cmp(&self, other: &IndexedVal) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for IndexedVal {
    fn cmp(&self, other: &IndexedVal) -> Ordering {
        return self.val.cmp(&other.val);
    }
}

impl PartialEq for IndexedVal {
    fn eq(&self, other: &IndexedVal) -> bool {
        return self.val == other.val;
    }
}
impl Eq for IndexedVal {}
