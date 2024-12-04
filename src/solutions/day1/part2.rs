use crate::utils::loadfile::read_file;

pub fn part2(file: &str) {
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
    
    let mut sim_score = 0;

    for l in left_list {
        let occurences = right_list.iter().filter(|x| **x == l).count() as i64;
        sim_score += l * occurences;
    }
    println!("Simularity score: {}", sim_score);
}
