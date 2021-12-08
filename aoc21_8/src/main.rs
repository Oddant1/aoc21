use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let contents = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let contents: Vec<&str> = contents.lines().collect();

        let mut inputs: Vec<Vec<&str>> = Vec::new();
        let mut outputs: Vec<Vec<&str>> = Vec::new();

        for line in contents {
            let mut input: Vec<&str> = Vec::new();
            let mut output: Vec<&str> = Vec::new();

            let mut line = line.split("|");

            for i in line.next().unwrap().split(" ") {
                input.push(i);
            }

            for o in line.next().unwrap().split(" ") {
                output.push(o);
            }

            inputs.push(input);
            outputs.push(output);
        }

        println!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work1(&outputs)
        );
    }
}

fn do_work1(outputs: &[Vec<&str>]) -> usize {
    let mut unique_counts = 0;

    for output in outputs {
        for num in output {
            let len = num.len();

            if len == 2 || len == 3 || len == 4 || len == 7 {
                unique_counts += 1;
            }
        }
    }

    unique_counts
}
