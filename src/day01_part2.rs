use std::collections::HashMap;

//use std::env;
pub fn solution(contents: String) -> i32 {
    //println!("With text:\n{contents}");
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    for part in contents.lines() {
        // println!("{}", part)
        let vec3 = part.split_whitespace().collect::<Vec<&str>>();
        vec1.push(vec3[0].parse().unwrap());
        vec2.push(vec3[1].parse().unwrap());
    }
    assert!(
        vec1.len() == vec2.len(),
        "Both columns should be of same size"
    );
    // vec1.sort();
    // vec2.sort();
    //println!("{:?}", vec1);

    let mut i = 0;
    let mut freq_vec2: HashMap<i32, i32> = HashMap::new();
    while i < vec2.len() {
        //let current_freq = freq_vec2.entry(vec1[i]).or_insert(1);
        freq_vec2.entry(vec2[i]).and_modify(|current_freq| *current_freq += 1).or_insert(1);
        i += 1;
    }

    let mut sum: i32 = 0;

    let mut j = 0;
    while j < vec1.len() {
        //let current_freq = freq_vec2.entry(vec1[i]).or_insert(1);
        sum += vec1[j] * freq_vec2.get(&vec1[j]).unwrap_or(&0);
        j += 1;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solution() {
        let test_string = "1 1\n\
              2 1\n\
              7 2\n\
              10 2";

        assert_eq!(solution(String::from(test_string)), 6);
    }
}
