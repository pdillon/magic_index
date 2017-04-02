extern crate time;
extern crate rand;

use std::env;
use time::PreciseTime;
use rand::distributions::{IndependentSample, Range};
use std::collections::VecDeque;

fn rand_arr(size: i32) -> VecDeque<i32> {
    let mut rand_vec = VecDeque::new();
    let lower_range: i32 = -(size / 2);
    let upper_range: i32 = size * 2 + lower_range;

    println!("Creating random array of length {} with values between {} and {} ...", size, lower_range, upper_range);

    let between = Range::new(lower_range, upper_range);

    while rand_vec.len() < size as usize {
        let mut rng = rand::thread_rng();
        let sample = between.ind_sample(&mut rng);
        if !rand_vec.contains(&sample) {
            rand_vec.push_back(sample);
        }
    }

    rand_vec
}

fn find_magic_index(vec: VecDeque<i32>) -> i32 {
    let len = vec.len();
    let mut magic_index = -1;
    for x in 0..len {
        if vec[x] == x as i32 {
            magic_index = x as i32;
            break;
        }
    }

    magic_index
}

fn main() {
    let mut size = 10000;
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        size = args[1].parse().unwrap();
    }

    let gen_start = PreciseTime::now();
    let magic_array = rand_arr(size);
    let gen_end = PreciseTime::now();
    let gen_dur = gen_start.to(gen_end);
    println!("Created random array of length {} in {} milliseconds.", magic_array.len(), gen_dur.num_milliseconds());

    let find_start = PreciseTime::now();
    let magic_index = find_magic_index(magic_array);
    let find_end = PreciseTime::now();
    let find_dur = find_start.to(find_end);

    println!("Magic index of {} found in {} microseconds.", magic_index, find_dur.num_microseconds().unwrap());
}