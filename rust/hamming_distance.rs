impl Solution {
    pub fn hamming_distance(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}