use std::collections::HashMap;

pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in digits.iter() {
        *map.entry(*num).or_default() += 1;
        println!("{:?}",map.get(num));
    }

    for i in 100..999 {
        if i % 2 == 0 {
            
            let mut temp = i;
            let mut all_in_digits = true;
            let mut digits_mapped: HashMap<i32, i32> = HashMap::new();
            while temp > 0 {
                let digit = temp % 10;
                *digits_mapped.entry(digit).or_default() += 1;
                

                if !digits.contains(&(digit as i32)) {
                    all_in_digits = false;
                    break;
                }

                if digits_mapped.get(&digit) > map.get(&digit) {
                    all_in_digits = false;
                    break;
                }
                temp /= 10;
            }
            if all_in_digits {
                result.push(i);
            }
        }
    }

    result
}


fn main() {
    let digits = vec![2,1,3,0];
    let result = find_even_numbers(digits);
    println!("{:?}", result);
}