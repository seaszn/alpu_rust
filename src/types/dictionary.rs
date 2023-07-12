use std::ops::Index;

#[derive(Clone)]
pub struct Dictionary<X, Y> {
    data: Vec<(X, Y)>,
}

impl<X, Y> Dictionary<X, Y> {
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn add(&mut self, key: X, value: Y) {
        self.data.push((key, value));
    }

    pub fn new() -> Dictionary<X, Y> {
        return Dictionary { data: vec![] };
    }
}

impl<X, Y> Index<&X> for Dictionary<X, Y>
where
    X: PartialEq<X>,
    Y: Clone,
{
    type Output = Y;
    fn index(&self, index: &X) -> &Y {
        for i in 0..self.len() {
            if &self.data[i].0 == index {
                return &self.data[i].1;
            }
        }

        panic!("{}", "index not found");
    }
}