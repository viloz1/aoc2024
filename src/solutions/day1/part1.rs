use crate::utils::loadfile::read_file;

pub fn part1(file: &str) {
    let file = read_file(file).unwrap();
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in file {
        println!("{}", line);
        let split: Vec<&str> = line.split_whitespace().collect();
        left_list.push(split[0].parse::<i64>().unwrap());
        right_list.push(split[1].parse::<i64>().unwrap());
    }

    left_list.sort();
    right_list.sort();
    
    let mut total_distance = 0;

    for (l, r) in left_list.iter().zip(right_list.iter()) {
        total_distance += (l - r).abs();
    }
    println!("Total distance: {}", total_distance);
}
