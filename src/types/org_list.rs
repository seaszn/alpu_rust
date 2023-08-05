use std::{ops::*, vec};

use crate::exchanges::UniswapV2MarketState;

#[derive(Clone, Debug)]
pub struct OrganizedList<T>
where
    T: Send,
{
    internal: Vec<OrgValue<T>>,
}

impl<T> IntoIterator for OrganizedList<T>
where
    T: Send
{
    type Item = OrgValue<T>;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.internal.into_iter();
    }
}

impl<T> Deref for OrganizedList<T>
where
    T: Send,
{
    type Target = Vec<OrgValue<T>>;
    
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T> DerefMut for OrganizedList<T>
where
T: Send,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.internal;
    }
}


impl PartialEq for OrganizedList<UniswapV2MarketState> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.internal.len() != other.internal.len(){
            return false;
        }

        for i in 0..other.internal.len(){
            if !self.internal[i].value.0.eq(&other.internal[i].value.0) || !self.internal[i].value.1.eq(&other.internal[i].value.1) {
                return false;
            }
        }

        return true;
    }
}

impl<T> OrganizedList<T>
where
    T: Send,
{
    pub fn new() -> OrganizedList<T> {
        return OrganizedList { internal: vec![] };
    }

    #[inline(always)]
    pub fn add_value(&mut self, value: T) {
        self.internal.push(OrgValue {
            id: self.internal.len(),
            value,
        })
    }

    #[inline(always)]
    pub fn add_pair(&mut self, value: OrgValue<T>) {
        if !self.contains_key(value.id) {
            self.internal.push(value);
        }
    }

    #[inline(always)]
    pub fn sort(&mut self) {
        self.internal.sort_by_cached_key(|x| x.id);
    }

    #[inline(always)]
    pub fn update_value_at<P>(&mut self, id: usize, mut predicate: P)
    where
        P: FnMut(&mut OrgValue<T>),
    {
        predicate(&mut self.internal[id]);
    }

    #[inline(always)]
    pub fn filter<P>(&self, predicate: P) -> Vec<&OrgValue<T>>
    where
        P: FnMut(&&OrgValue<T>) -> bool,
    {
        return self.internal.iter().filter(predicate).collect();
    }

    #[inline(always)]
    pub fn to_vec(&self) -> Vec<&OrgValue<T>> {
        return self.internal.iter().map(|x| x).collect();
    }

    #[inline(always)]
    pub fn to_raw_vec(&self) -> &Vec<OrgValue<T>> {
        return &self.internal;
    }

    #[inline(always)]
    pub fn contains_key(&mut self, id: usize) -> bool {
        self.sort();
        return self.contains_key_unsorted(id);
    }
    
    #[inline(always)]
    pub fn update_all(&mut self, other: &mut OrganizedList<T>){
        self.internal.clear();
        self.internal.append(&mut other.internal);
    } 

    #[inline(always)]
    pub fn contains_key_unsorted(&self, id: usize) -> bool {
        for i in 0..self.internal.len() {
            if self.internal[i].id == id {
                return true;
            }
        }

        return false;
        // return self.internal.par_iter().any(|x| x.id == id);
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
    #[inline(always)]
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
