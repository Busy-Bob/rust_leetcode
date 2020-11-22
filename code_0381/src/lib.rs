use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
#[derive(Default,Debug)]
struct RandomizedCollection {
    vec: Vec<i32>,
    idx: HashMap<i32, HashSet<usize>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    
    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        self.vec.push(val);
        self.idx.entry(val).or_default().insert(self.vec.len() - 1);
        self.idx[&val].len() == 1
    }
    
    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        // println!("{:?}", self);
        // vec 交换该元素与尾元素位置
        if self.idx.contains_key(&val) && self.idx[&val].len() > 0
        {
            let index = *self.idx[&val].iter().nth(0).unwrap();
            self.idx.get_mut(&val).unwrap().remove(&index);
            self.vec[index] = *self.vec.last().unwrap();
            // 改变交换元素索引
            self.idx.get_mut(&self.vec[index]).unwrap().insert(index);
            self.idx.get_mut(&self.vec.pop().unwrap()).unwrap().remove(&self.vec.len());
            true
        } else {false}
        


    }
    
    /** Get a random element from the collection. */
    fn get_random(&self) -> i32 {
        *self.vec.choose(&mut rand::thread_rng()).unwrap()
    }
}

