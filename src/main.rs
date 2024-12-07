mod day01;
mod day01_part2;
use std::fs;

fn main() {
    let days: Vec<&str> = vec!["01", "01_part2"];
    //"./input/day1.txt";
    for day in days {
        let input_index= day.split("_").collect::<Vec<&str>>()[0];
        let file_path = format!("./input/day{}.txt", input_index);
        println!("In file {file_path}");
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

        let solution: fn(input: String) -> i32 = match day {
            "01" => day01::solution,
            "01_part2" => day01_part2::solution,
            _ => error,
        };

        print!("day{} -> {}\n", day, solution(input));
    }
}

fn error(_input: String) -> i32 {
    return 1;
}
