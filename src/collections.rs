use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MultiSet<T>(BTreeMap<T, usize>);

impl<T: Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, v: T) {
        *self.0.entry(v).or_default() += 1;
    }

    pub fn remove(&mut self, v: &T) {
        let r = self.0.get_mut(v).unwrap();
        *r -= 1;
        if *r == 0 {
            self.0.remove(v);
        }
    }

    pub fn min(&self) -> Option<&T> {
        self.0.iter().next().map(|r| r.0)
    }
}
