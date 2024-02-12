use rand::Rng;
use std::time::{Instant};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("test_numbers/test_numbers.txt").expect("error creating file");
    for limit in (1_000..=76_000).step_by(5000) {
        for length in (10_000_000..=100_000_000).step_by(30_000_000) {
            let intro = format!("Number of Number: {}, Highest possible number: {}\n",limit, length);
            file.write_all(intro.as_bytes()).expect("error writing");;
            time_stopper(bubblesort, limit, length, &mut file, "Bubblesort");
            time_stopper(quicksort, limit, length, &mut file, "Quicksort ");
            time_stopper(rust_sort, limit, length, &mut file, "Rust Sort ");
            time_stopper(mergesort, limit, length, &mut file, "Mergesort ");
            file.write_all("\n".as_bytes()).expect("error writing3");;
        }
    }
    
}

fn time_stopper(sorting_algorithm: fn(Vec<i32>) -> Vec<i32>, limit: i32, length: i32, fd: &mut File, name: &str) {


    let mut rng = rand::thread_rng();

    let mut listofnumbers: Vec<i32> = Vec::new();

    let mut listofsortednumbers: Vec<i32> = Vec::new();

    for _ in 0..limit {
        listofnumbers.push(rng.gen_range(1..=length));
    }
    let mut start_time = Instant::now();
    listofsortednumbers = sorting_algorithm(listofnumbers.clone());
    let mut end_time = Instant::now();
    let mut elapsed_time = end_time.duration_since(start_time);
    let time_data = format!("{}:    {:?}\n", name, elapsed_time);
    fd.write_all(time_data.as_bytes()).expect("error writing2");;

}

fn rust_sort(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    return list;
}

fn bubblesort(mut list: Vec<i32>) -> Vec<i32> {
    let mut temp: i32;
    for _ in 0..list.len() {
        for j in 1..list.len() {
            if list[j-1] > list[j] {
                temp = list[j-1];
                list[j-1] = list[j];
                list[j] = temp;
            }
        }
    }
    return list;
}

fn quicksort(list: Vec<i32>) -> Vec<i32> {
    let maximum: usize = highestnumberinlist(list.clone());
    let mut placeholderlist: Vec<i32> =  vec![0; maximum+1];
    let mut orderedlist: Vec<i32> = Vec::new();
    for i in 0..list.len() {
        placeholderlist[list[i] as usize] += 1;
    }
    for j in 0..placeholderlist.len() {
        for _ in 0..placeholderlist[j] {
            orderedlist.push(j as i32);
        }
    }

    return orderedlist;
}

fn highestnumberinlist(list: Vec<i32>) -> usize {
    let mut max: i32 = list[0];
    for i in 1..list.len() {
        if list[i] > max {
            max = list[i];
        }
    }
    return max as usize;
}

fn mergesort(list: Vec<i32>) -> Vec<i32> {
    let mut ordered_list: Vec<i32> = Vec::new();
    if list.len() > 1 {
        let list_mid = (list.len()+1) / 2;
        let mut curr_l_index = 0;
        let mut curr_r_index = 0;
        let mut l_list = mergesort(list[..list_mid].to_vec().clone());
        let mut r_list = mergesort(list[list_mid..].to_vec().clone());
        while curr_l_index < l_list.len() && curr_r_index < r_list.len() {
            if l_list[curr_l_index] < r_list[curr_r_index] {
                ordered_list.push(l_list[curr_l_index]);
                curr_l_index+=1;
            } else {
                ordered_list.push(r_list[curr_r_index]);
                curr_r_index+=1;
            }
        }
        if curr_l_index == l_list.len() {
            for i in curr_r_index..r_list.len() {
                ordered_list.push(r_list[curr_r_index]);
            }
        } else {
            for i in curr_l_index..l_list.len() {
                ordered_list.push(l_list[curr_l_index]);
            }
        }
    }
    return list
}
