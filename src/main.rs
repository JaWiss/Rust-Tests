use rand::Rng;
use std::time::{Instant};


fn main() {
    let limit: i32 = 10000;
    let mut rng = rand::thread_rng();

    let mut listofnumbers: Vec<i32> = Vec::new();
    let mut listofsortednumbers: Vec<i32> = Vec::new();

    for _ in 0..limit {
        listofnumbers.push(rng.gen_range(1..100001));
    }
    let start_time = Instant::now();
    listofsortednumbers = rust_sort(listofnumbers);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Elapsed Time: {:?}", elapsed_time);

}

fn rust_sort(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    return list;
}