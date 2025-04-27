# LeetCode - Most Common Word

This repository contains my solution for the [LeetCode problem "Most Common Word"](https://leetcode.com/problems/most-common-word/submissions/1619202660/).

## Problem Description

Given a paragraph and a list of banned words, return the most frequent word that is not banned.

The answer is guaranteed to be unique, and it may be in lowercase.

## Solution

- Used a `HashMap` to count the frequency of each valid word.
- Ignored all characters except English alphabets (a-z, A-Z).
- Converted the paragraph to lowercase.
- Skipped any banned words.
- Found the word with the maximum frequency.

## Code

Implemented in Rust.

```rust
use std::collections::HashMap;
impl Solution {
pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut scores:HashMap<String, i16> = HashMap::new();
        let mut str: String = "".to_string();

        for ch in paragraph.to_lowercase().chars() {
            if !ch.is_alphabetic() {
                
                let mut cnt: &i16 = &0;
                if let Some(value) = scores.get(&str) {
                    cnt = value;
                } else {
                    cnt = &0;
                }

                println!("{str} + {cnt}");
                if !str.is_empty() && !banned.contains(&str) {
                    scores.insert(str, *cnt + 1);
                }
                
                str = "".to_string();
                continue;
            }
            str.push(ch);           
        }

        let mut cnt = &0;
        if let Some(value) = scores.get(&str) {
            cnt = value;
        } else {
            cnt = &0;
        }

        if !str.is_empty() && !banned.contains(&str) {
            scores.insert(str, *cnt + 1);
        }

        let mut res = String::new();
        let mut max = 0;
        
        for (k, v) in scores {
            if v > max {
                max = v;
                res = k;
            }
        }        
        res
}
}
