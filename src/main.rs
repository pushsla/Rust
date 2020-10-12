use std::io;
use std::fmt::Display;
use std::time::Instant;

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

    while (!finished){
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
    let mut min = &vector[0];
    let mut min_index = 0;

    for _i in 0..vec.len(){
        min = &vector[0];
        min_index = 0;
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
    //let numbers = read_matrix(1)[0].to_vec();
    //let mut letters = String::new();

    //io::stdin().read_line(&mut letters).unwrap();
    let numbers = vec![0,2,3,6,5,4,8,9,8,6,5,4,3,4,5,6,7,6,1,2,3,0,9,9,1,7,2,3,6,5,4,8,4,0,1,2,3,4,5,6,7,8,9,0,9,8,7,6,5,4,3,2,1];
    let letters = "q a z x s w e d c v f r t g b n h y u j m k i o p l p o i u y t r e w q a s d f g h j k l m n b v c x z a b c d e f g h";
    let letters = letters.trim().split(" ").collect();

    let now = Instant::now();
    print_vector(&bubble_sort(&numbers));
    print_vector(&bubble_sort(&letters));
    println!("Bubble: Time (microsecs): {}", now.elapsed().as_micros());

    let now = Instant::now();
    print_vector(&shaker_sort(&numbers));
    print_vector(&shaker_sort(&letters));
    println!("Shaker: Time (microsecs): {}", now.elapsed().as_micros());

    let now = Instant::now();
    print_vector(&selection_sort(&numbers));
    print_vector(&selection_sort(&letters));
    println!("Selection: Time (microsecs): {}", now.elapsed().as_micros());
}