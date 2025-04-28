impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let first = edges[0];
    let second = edges[1];
    if first[0] == second[0] || first[0] == first[1] {
        first[1]
    }else {
        first[1]
    }

    }
}