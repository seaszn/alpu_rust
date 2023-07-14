use ethers::types::H160;

pub type Reserves = (u128, u128);

#[derive(Clone)]
pub struct ReserveTable {
    internal: Vec<([u8; 20], Reserves)>,
}
impl ReserveTable {
    pub fn new() -> ReserveTable {
        return ReserveTable { internal: vec![] };
    }

    #[doc = r"Adds the current value to key. If key is already present, it updates it's values"]
    pub fn add(&mut self, key: &H160, value: Reserves) {
        if self.contains_key(&key) {
            self.update_at(key, |_| value);
        } else {
            self.internal.push((key.0, value));
        }
    }

    #[doc = r"Updates the value at key if present, and returns the old value"]
    pub fn update_at<F>(&mut self, key: &H160, predicate: F) -> Option<Reserves>
    where
        F: Fn(&mut Reserves) -> Reserves,
    {
        if let Some(position) = self.internal.iter().position(|x| x.0.eq(&key.0)) {
            let old_value = self.internal[position].1;
            self.internal[position].1 = predicate(&mut self.internal[position].1);

            return Some(old_value);
        }

        return None;
    }

    #[doc = r"Checks if the key is present"]
    pub fn contains_key(&self, key: &H160) -> bool {
        return self.internal.iter().any(|x| x.0.eq(&key.0));
    }

    pub fn get_value(&self, key: &H160) -> Option<Reserves> {
        if let Some(element) = self.internal.iter().find(|x| x.0.eq(&key.0)) {
            return Some(element.1);
        }

        return None;
    }

    pub fn len(&self) -> usize {
        return self.internal.len();
    }
}
