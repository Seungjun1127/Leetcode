impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        let mut index = 0;
        while index < arr.len() - 1 && arr[index] < arr[index + 1] {
            index += 1;
        }

        if index == 0 || index == arr.len() - 1 {
            return false;
        }

        while index < arr.len() - 1 && arr[index] > arr[index + 1] {
            index += 1;
        }

        index == arr.len() - 1
    }
}