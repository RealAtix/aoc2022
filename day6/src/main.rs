use std::{io::BufRead, time::Instant};

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input).unwrap();

    input
}

fn main() {
    let time = Instant::now();

    let input = input();
    let bytes = input.as_bytes();

    part1(bytes);

    part2_arr(bytes);
    part2_bits(bytes);
    part2_bits_backward(bytes);

    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &[u8]) {
    let result = input
        .windows(4)
        .position(|w| {
            let mut vec = Vec::with_capacity(4);
            for x in w {
                if vec.contains(x) {
                    return false;
                }

                vec.push(*x);
            }
            return true;
        })
        .map(|x| x + 4)
        .unwrap();

    println!("Part 1 answer: {}", result);
}

fn part2_arr(input: &[u8]) {
    // faster version using array instead of vec
    let time = Instant::now();
    let result = input
        .windows(14)
        .position(|w| {
            let mut arr = [0u8; 14];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();

    println!("Part 2 answer: {} - arr {:?}", result, time.elapsed());
}

fn part2_bits(input: &[u8]) {
    let time = Instant::now();
    let mut filter = 0u32; // 32 bit filter to keep state
                           // grab first 13 characters in the sequence
    input
        .iter()
        .take(14 - 1)
        .for_each(|c| filter ^= 1 << (c % 32)); // xor operation

    let result = input
        .windows(14)
        .position(|w| {
            let first = w[0]; // first char in window
            let last = w[w.len() - 1]; // last char in window
            filter ^= 1 << (last % 32); // toggle 14th char in window
            let res = filter.count_ones() == 14 as u32;
            filter ^= 1 << (first % 32); // remove first char in window
            res
        })
        .unwrap(); // Returns the first index of the window
    println!("Part 2 answer: {} - bits {:?}", result + 14, time.elapsed());
}

fn part2_bits_backward(input: &[u8]) {
    let time = Instant::now();
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        // constant size window
        let mut state = 0u32; // 32 bit state variable

        if let Some(pos) = slice.iter().rposition(|byte| {
            // iterate in reverse
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0; // check if bit is already set to one (duplicate)
            state |= 1 << bit_idx; // shift on that bit
            ret // return wether or not we've seen this byte already
        }) {
            idx += pos + 1; // if we have seen this byte, and we are going backwards, jump all the
                            // way to this position + 1, for optimization
        } else {
            println!(
                "Part 2 answer: {} - bits backward {:?}",
                idx + 14,
                time.elapsed()
            );
            break;
        }
    }
}
