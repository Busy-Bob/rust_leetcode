struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut totalGas = 0;
        let mut minGas = i32::MAX;
        let mut index = 0;
        for i in 0..gas.len() {
            totalGas += gas[i] - cost[i];
            if totalGas < minGas {
                index = i;
                minGas = totalGas;
            }
        }

        if totalGas >= 0 {
            ((index + 1) % gas.len()) as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
