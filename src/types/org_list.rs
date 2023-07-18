use std::ops::*;

#[derive(Clone)]
pub struct OrganizedList<T>
where
    T: Send,
{
    internal: Vec<OrgValue<T>>,
}

impl<T> Deref for OrganizedList<T>
where
    T: Send,
{
    type Target = Vec<OrgValue<T>>;
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T> OrganizedList<T>
where
    T: Send,
{
    pub fn new() -> OrganizedList<T> {
        return OrganizedList { internal: vec![] };
    }

    pub fn add_value(&mut self, value: T) {
        self.internal.push(OrgValue {
            id: self.internal.len(),
            value,
        })
    }

    pub fn add_pair(&mut self, value: OrgValue<T>) {
        if !self.contains_key(value.id) {
            self.internal.push(value);
        }
    }

    pub fn sort(&mut self) {
        self.internal.sort_by_cached_key(|x| x.id);
    }

    pub fn update_value_at<P>(&mut self, id: usize, mut predicate: P)
    where
        P: FnMut(&mut OrgValue<T>),
    {
        predicate(&mut self.internal[id]);
    }

    pub fn filter<P>(&self, predicate: P) -> Vec<&OrgValue<T>>
    where
        P: FnMut(&&OrgValue<T>) -> bool,
    {
        return self.internal.iter().filter(predicate).collect();
    }

    pub fn to_vec(&self) -> Vec<&OrgValue<T>> {
        return self.internal.iter().map(|x| x).collect();
    }

    pub fn contains_key(&mut self, id: usize) -> bool {
        self.sort();
        return self.contains_key_unsorted(id);
    }

    pub fn contains_key_unsorted(&self, id: usize) -> bool {
        return self.internal.iter().any(|x| x.id == id);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OrgValue<T>
where
    T: Send,
{
    pub id: usize,
    pub value: T,
}

trait OrganizedFilter<T> {
    fn filter<P>(&self, predicate: P) -> Vec<&OrgValue<T>>
    where
        T: Send,
        P: Fn(&OrgValue<T>) -> bool;
}

impl<T> OrganizedFilter<T> for Vec<&OrgValue<T>>
where
    T: Send,
{
    fn filter<P>(&self, predicate: P) -> Vec<&OrgValue<T>>
    where
        P: Fn(&OrgValue<T>) -> bool,
    {
        let mut f: Vec<&OrgValue<T>> = vec![];

        for &s in self {
            if predicate(s) {
                f.push(s);
            }
        }

        return f;
        // return false;
        // todo!()
    }
}
