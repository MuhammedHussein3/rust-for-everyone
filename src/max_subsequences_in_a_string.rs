impl Solution {
pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let mut cn_pa1: i64 = 0;
    let mut cn_pa2: i64 = 0;
    let mut sum: i64 = 0;
    for  ch in text.chars()  {
        if ch == pattern.chars().nth(0).unwrap() {         
            cn_pa1 += 1;
            continue;
        }
        if ch == pattern.chars().nth(1).unwrap() {
            println!("ch: {:?}", ch);
            cn_pa2 += 1;
            sum += cn_pa1 as i64;
        }
     
    }

    if pattern.chars().nth(0).unwrap() == pattern.chars().nth(1).unwrap() {
        return (cn_pa1 as i64 + 1 as i64) * cn_pa1 as i64 / 2 as i64;
    }
    if cn_pa2 > cn_pa1{
        sum += cn_pa2 as i64;
    } else {
        sum += cn_pa1 as i64;
    }

    sum
}
}