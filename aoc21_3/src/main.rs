use std::fs;

const FPS: [&str; 2] = ["./data/test.txt", "./data/input.txt"];

fn main() {
    for fp in FPS {
        let reads = fs::read_to_string(fp).expect("Something went wrong reading the file");
        let num_bits = reads.lines().next().unwrap().len();
        let reads: Vec<u32> = reads
            .lines()
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect();
        let bit_counts = get_bit_counts(num_bits, &reads);

        println!(
            "Result 1 for file at path {} is {}",
            fp,
            do_work_1(&bit_counts, &reads)
        );
        println!(
            "Result 2 for file at path {} is {}",
            fp,
            do_work_2(num_bits, &bit_counts, &reads)
        );
    }
}

fn do_work_1(bit_counts: &[u32], reads: &[u32]) -> u32 {
    let half_num_reads = reads.len() as u32 / 2;

    let mut gamma_read: u32 = 0;
    let mut epsilon_read: u32 = 0;

    for (idx, bit_count) in bit_counts.iter().enumerate() {
        if *bit_count > half_num_reads {
            gamma_read |= 1 << idx;
        } else {
            epsilon_read |= 1 << idx;
        }
    }

    println!("Gamma: {}\nEpsilon: {}", gamma_read, epsilon_read);
    gamma_read * epsilon_read
}

fn do_work_2(num_bits: usize, bit_counts: &[u32], reads: &[u32]) -> u32 {
    let num_reads = reads.len();

    let mut oxygen: Vec<u32> = vec![0; num_reads];
    let mut oxygen_counts: Vec<u32> = vec![0; num_bits];

    let mut co2: Vec<u32> = vec![0; num_reads];
    let mut co2_counts: Vec<u32> = vec![0; num_bits];

    oxygen.clone_from_slice(reads);
    oxygen_counts.clone_from_slice(&bit_counts);

    co2.clone_from_slice(reads);
    co2_counts.clone_from_slice(&bit_counts);

    for idx in (0..num_bits).rev() {
        let mut oxygen_remove: Vec<u32> = Vec::new();
        let oxygen_bit_count = oxygen_counts[idx] as f32;
        let half_oxygen_len = oxygen.len() as f32 / 2.0;

        let mut co2_remove: Vec<u32> = Vec::new();
        let co2_bit_count = co2_counts[idx] as f32;
        let half_co2_len = co2.len() as f32 / 2.0;

        let offset = num_bits - (num_bits - idx);

        if oxygen.len() > 1 {
            if oxygen_bit_count < half_oxygen_len {
                for (read_idx, read) in oxygen.iter().enumerate() {
                    if read << (31 - offset) >> 31 == 1 {
                        oxygen_remove.push(read_idx as u32);

                        for count_idx in 0..num_bits {
                            oxygen_counts[count_idx] -= read << (31 - count_idx) >> 31;
                        }
                    }
                }
            } else if oxygen_bit_count == half_oxygen_len {
                for (read_idx, read) in oxygen.iter().enumerate() {
                    if read << (31 - offset) >> 31 == 0 {
                        oxygen_remove.push(read_idx as u32);

                        for count_idx in 0..num_bits {
                            oxygen_counts[count_idx] -= read << (31 - count_idx) >> 31;
                        }
                    }
                }
            } else {
                for (read_idx, read) in oxygen.iter().enumerate() {
                    if read << (31 - offset) >> 31 == 0 {
                        oxygen_remove.push(read_idx as u32);

                        for count_idx in 0..num_bits {
                            oxygen_counts[count_idx] -= read << (31 - count_idx) >> 31;
                        }
                    }
                }
            }

            for remove_idx in oxygen_remove.iter().rev() {
                oxygen.remove(*remove_idx as usize);
            }
        }

        if co2.len() > 1 {
            if co2_bit_count < half_co2_len {
                for (read_idx, read) in co2.iter().enumerate() {
                    if read << (31 - offset) >> 31 == 0 {
                        co2_remove.push(read_idx as u32);

                        for count_idx in 0..num_bits {
                            co2_counts[count_idx] -= read << (31 - count_idx) >> 31;
                        }
                    }
                }
            } else if co2_bit_count == half_co2_len {
                for (read_idx, read) in co2.iter().enumerate() {
                    if read << (31 - offset) >> 31 == 1 {
                        co2_remove.push(read_idx as u32);

                        for count_idx in 0..num_bits {
                            co2_counts[count_idx] -= read << (31 - count_idx) >> 31;
                        }
                    }
                }
            } else {
                for (read_idx, read) in co2.iter().enumerate() {
                    if read << (31 - offset) >> 31 == 1 {
                        co2_remove.push(read_idx as u32);

                        for count_idx in 0..num_bits {
                            co2_counts[count_idx] -= read << (31 - count_idx) >> 31;
                        }
                    }
                }
            }

            for remove_idx in co2_remove.iter().rev() {
                co2.remove(*remove_idx as usize);
            }
        }
    }

    println!("Oxygen: {}\nCO2: {}", oxygen[0], co2[0]);
    oxygen[0] * co2[0]
}

fn get_bit_counts(num_bits: usize, reads: &[u32]) -> Vec<u32> {
    let mut bit_counts: Vec<u32> = vec![0; num_bits];

    for read in reads.iter() {
        for idx in 0..num_bits {
            bit_counts[idx] += read << (31 - idx) >> 31;
        }
    }

    bit_counts
}
