impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut index = 0;
        while index < arr.len() {
            if arr[index] == 0 {
                for i in (index + 1..arr.len()).rev() {
                    arr[i] = arr[i-1];
                }
                index += 1;
            }
            index +=1;
        }
    }
}