impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut chars = word.chars();
        let mut ans = String::new();
        let mut prefix = None;
        let mut prefix_cnt = 0;
        let n_to_char = |n: i32| -> char {
            match n {
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => '0'
            }
        };
        
        while let Some(ch) = chars.next() {
            if let Some(pre) = prefix {
                if pre == ch && prefix_cnt < 9 {
                    prefix_cnt += 1;
                } else {
                    ans.push(n_to_char(prefix_cnt));
                    ans.push(pre);

                    prefix = Some(ch);
                    prefix_cnt = 1;
                }
            } else {
                prefix = Some(ch);
                prefix_cnt = 1;
            }
        }
        
        if let Some(pre) = prefix {
            ans.push(n_to_char(prefix_cnt));
            ans.push(pre);
        }

        ans
    }
}