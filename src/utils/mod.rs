pub mod logger;
pub mod json;
pub mod parse;

pub fn append_one<T>(s: Vec<&'static T>, ele: &'static T) -> Vec<&'static T>
where
    T: Clone,
{
    let mut res = s.clone();
    res.push(ele);

    return res.to_vec();
}

pub fn filter_all<F, T>(source: &Vec<T>, mut predicate: F) -> Vec<T>
where
    F: FnMut(&T) -> bool,
    T: Clone,
{
    let mut res: Vec<T> = vec![];
    for ele in source {
        if predicate(ele) {
            res.push(ele.clone());
        }
    }

    return res;
}
