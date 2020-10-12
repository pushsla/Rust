use std::io;
use std::fmt::Display;

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
            println!("force: {}, {}", i, i+1);
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
            println!("reverse: {}, {}", i, i-1);
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

fn main(){
    let numbers = read_matrix(1)[0].to_vec();
    let mut letters = String::new();

    io::stdin().read_line(&mut letters).unwrap();
    let letters = letters.trim().split(" ").collect();

    print_vector(&bubble_sort(&numbers));
    print_vector(&bubble_sort(&letters));

    print_vector(&shaker_sort(&numbers));
    print_vector(&shaker_sort(&letters));
}