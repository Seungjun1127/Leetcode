impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        let l1 = nums1.clone();
        let l2 = nums2.clone();
        let mut i = 0;
        let mut j = 0;

        while i + j < m + n {
            if i == m {
                nums1[i + j] = l2[j];
                j += 1;
            } else if j == n {
                nums1[i + j] = l1[i];
                i += 1; 
            } else if l1[i] >= l2[j] {
                nums1[i + j] = l2[j];
                j += 1;
            } else {
                nums1[i + j] = l1[i];
                i += 1;
            }
        }
    }
}