impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();

        for i in 0..words.len() {
            let mut flag = false;
            for j in 0..words.len() {
                if i == j {
                    continue;
                }

                if words[j].contains(&words[i]) {
                    flag = true;
                    break;
                }
            }

            if flag {
                ans.push(words[i].clone());
            }
        }
        ans
    }
}