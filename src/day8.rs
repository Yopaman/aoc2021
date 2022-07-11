use crate::inputs;

fn parse_inputs() -> Vec<(Vec<String>, Vec<String>)> {
    let lines = inputs::read_lines("./src/inputs/day8.txt");
    lines.iter().map(|l| {
        let parse: Vec<&str> = l.split(" | ").collect();
        let left_parse: Vec<&str> = parse[0].split(" ").collect();
        let right_parse: Vec<&str> = parse[1].split(" ").collect();
        (left_parse.into_iter().map(|n| String::from(n)).collect(), right_parse.into_iter().map(|n| String::from(n)).collect())
    }).collect()
}

pub fn part1() {
    let data = parse_inputs();
    let output: Vec<&Vec<String>> = data.iter().map(|line| &line.1).collect();

    let result = output.iter().fold(0, |acc, e| 
        acc + e.iter().filter(|digit| 
            digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
        ).collect::<Vec<&String>>().len()
    );

    println!("Result: {}", result);
}