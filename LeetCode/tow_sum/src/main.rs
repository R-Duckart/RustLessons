/*
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in nums.iter().enumerate(){
            for (ii, y) in nums.iter().enumerate(){
                if i == ii { continue }
                if x+y == target {
                    return vec![i as i32,ii as i32]
                }
            }
        }
        vec![]
    }
}
*/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, x) in nums.iter().enumerate(){
            match map.get(&(target - x)) {
                Some(&j) => return vec![j as i32, i as i32],
                None => { map.insert(*x, i); }
            }
        }
        vec![]
    }
}
