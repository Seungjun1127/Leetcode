impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    // 1단계: 가능한 모든 부분집합의 비트 OR 최대값 찾기
    let mut max_value = 0;
    let mut count = 0;
    let n = nums.len();

    // 가능한 모든 부분집합을 탐색 (2^n 개)
    for i in 0..(1 << n) {
        let mut current_or = 0;

        // 부분집합에 속하는 원소들에 대해 OR 연산
        for j in 0..n {
            if i & (1 << j) != 0 {
                current_or |= nums[j];
            }
        }

        // 최대 OR 값을 갱신하고, 그 값을 가지는 부분집합의 개수를 셈
        if current_or > max_value {
            max_value = current_or;
            count = 1;
        } else if current_or == max_value {
            count += 1;
        }
    }
    count
    }
}