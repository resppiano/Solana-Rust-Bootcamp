// TwoSum.rs
// This program finds two indices in a vector of numbers that add up to a target value.
// There is always one unique solution, and you can't use the same number twice.

fn two_sum(nums: &Vec<i32>, target: i32) -> (usize, usize) {
    // We'll use a simple approach: check every pair of numbers
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            // Check if the sum of nums[i] and nums[j] equals the target
            if nums[i] + nums[j] == target {
                // Return the indices as a tuple
                return (i, j);
            }
        }
    }
    // If no solution is found (shouldn't happen with the given rules), panic
    panic!("No two sum solution found");
}

fn main() {
    // Example 1
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(&nums, target);
    println!("Input: nums = {:?}, target = {}", nums, target);
    println!("Output: [{}, {}]", result.0, result.1);
    println!("Explanation: Because nums[{}] + nums[{}] == {}, we return [{}, {}].", result.0, result.1, target, result.0, result.1);
}
