use std::io;
use std::fmt::Display;
use std::time::Instant;
use rand::Rng;

fn read_matrix(n: i64) -> Vec<Vec<i64>>{
    let mut result = Vec::new();

    for _i in 0..n{
        let mut line = String::new();
        let mut vline = Vec::new();

        io::stdin().read_line(&mut line).expect("reading err");
        for d in line.trim().split(" "){
            vline.push(d.parse::<i64>().unwrap());
        }
        result.push(vline);
    }

    result
}

fn print_matrix<T: Display>(matr: &Vec<Vec<T>>){
    for line in matr{
        print_vector(line);
    }
}

fn print_vector<T: Display>(vec: &Vec<T>){
    print!("[ ");
    for d in vec{
        print!("{} ", d);
    }
    println!("]");
}

fn bubble_sort<T: Ord + Clone>(vec: &Vec<T>) -> Vec<T>{
    let mut vec = vec.to_vec();
    let mut r = 1;

    for _i in 0..vec.len(){
        for i in 0..(vec.len()-r){
            if vec[i] > vec[i+1]{
                vec.swap(i, i+1);
            }
        }
        r = r + 1;
    }

    vec
}

fn shaker_sort<T: Ord + Clone>(vec: &Vec<T>) -> Vec<T>{
    let mut vec = vec.to_vec();
    let mut r = 1;
    let mut l = 1;
    let mut finished = false;

    while !finished{
        finished = true;
        for i in (l-1)..(vec.len()-r){
            if vec[i] > vec[i+1]{
                finished = false;
                vec.swap(i, i+1);
            }
        }
        if finished{
            break;
        }

        finished = true;
        for i in (l..(vec.len()-r)).rev(){
            if vec[i] < vec[i-1]{
                finished = false;
                vec.swap(i, i-1);
            }
        }
        r = r + 1;
        l = l + 1;
    }

    vec
}

fn selection_sort<T: Ord + Clone>(vec: &Vec<T>) -> Vec<T>{
    let mut vector = vec.to_vec();
    let mut result = Vec::new();

    for _i in 0..vec.len(){
        let mut min = &vector[0];
        let mut min_index = 0;
        for i in 0..vector.len(){
            if vector[i] < *min{
                min = &vector[i];
                min_index = i;
            }
        }
        result.push(min.clone());
        vector.remove(min_index);
    }

    result
}

fn main(){
    let mut rng = rand::thread_rng();

    let items_count: usize = 10000;
    let original_numbers: Vec<usize> = (0..items_count).collect();
    let mut numbers = original_numbers.to_vec();

    for i in 0..items_count{
        numbers.swap(rng.gen_range(0, items_count), i);
    }

    println!("Start sorting!");
    let now = Instant::now();
    bubble_sort(&numbers);
    println!("Bubble: Time (microsecs): {}; (secs): {}", now.elapsed().as_micros(), now.elapsed().as_secs());

    let now = Instant::now();
    shaker_sort(&numbers);
    println!("Shaker: Time (microsecs): {}; (secs): {}", now.elapsed().as_micros(), now.elapsed().as_secs());

    let now = Instant::now();
    selection_sort(&numbers);
    println!("Selection: Time (microsecs): {}; (secs): {}", now.elapsed().as_micros(), now.elapsed().as_secs());
}