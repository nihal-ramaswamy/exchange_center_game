use std::cmp::Ordering;


pub struct Price {
    val: f32
}

impl Price {
    const EPSILON: f32 = 1e-9;
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.val < other.val + Price::EPSILON {
            true => Ordering::Less,
            false => {
                match self.val > other.val - Price::EPSILON {
                    true => Ordering::Greater,
                    false => Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Price {}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        let diff = self.val - other.val; 
        let diff = diff.abs();
        diff < Price::EPSILON
    }
}
