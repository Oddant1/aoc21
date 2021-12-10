use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];
const PAREN_SCORES: (usize, usize) = (3, 1);
const SQUARE_SCORES: (usize, usize) = (57, 2);
const CURLY_SCORES: (usize, usize) = (1197, 3);
const ANGLE_SCORES: (usize, usize) = (25137, 4);

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let lines: Vec<&str> = contents.lines().collect();

        let scores = do_work(&lines);
        println!("Result 1 for file at path {} is {}", fp, scores.0);
        println!("Result 2 for file at path {} is {}", fp, scores.1);
    }
}

fn do_work(lines: &[&str]) -> (usize, usize) {
    let mut part1_score = 0;
    let mut part2_scores: Vec<usize> = Vec::new();

    'outer: for line in lines {
        let mut opens: Vec<char> = Vec::new();

        for character in line.chars() {
            if character == '(' || character == '[' || character == '{' || character == '<' {
                opens.push(character)
            } else {
                let last = *opens.last().unwrap();

                if (last == '(' && character == ')')
                    || (last == '[' && character == ']')
                    || (last == '{' && character == '}')
                    || (last == '<' && character == '>')
                {
                    opens.pop();
                } else {
                    if character == ')' {
                        part1_score += PAREN_SCORES.0;
                    } else if character == ']' {
                        part1_score += SQUARE_SCORES.0;
                    } else if character == '}' {
                        part1_score += CURLY_SCORES.0;
                    } else if character == '>' {
                        part1_score += ANGLE_SCORES.0;
                    }

                    continue 'outer;
                }
            }
        }

        let mut part2_score = 0;
        for open in opens.iter().rev() {
            part2_score *= 5;

            if *open == '(' {
                part2_score += PAREN_SCORES.1;
            } else if *open == '[' {
                part2_score += SQUARE_SCORES.1;
            } else if *open == '{' {
                part2_score += CURLY_SCORES.1;
            } else if *open == '<' {
                part2_score += ANGLE_SCORES.1;
            }
        }

        part2_scores.push(part2_score);
    }

    part2_scores.sort_by(|a, b| b.cmp(a));
    (part1_score, part2_scores[part2_scores.len() / 2])
}
