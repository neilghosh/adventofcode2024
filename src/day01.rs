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
    vec1.sort();
    vec2.sort();
    //println!("{:?}", vec1);

    let mut i = 0;
    let mut sum = 0;
    while i < vec1.len() {
        sum += (vec1[i] - vec2[i]).abs();
        i += 1;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solution() {
        let test_string = "1 5\n\
              2 6\n\
              7 3\n\
              10 20";

        assert_eq!(solution(String::from(test_string)), 16);
    }
}
