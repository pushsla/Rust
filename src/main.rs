use std::fmt::Display;

fn print_vec<T: Display>(v: &Vec<T>){
    print!("\n[ ");
    for e in v{
        print!("{} ", e);
    }
    println!("]");
}

fn map<T: , F: Fn(&T) -> T>(v: &mut Vec<T>, f: F){
    for i in 0..v.len(){
        v[i] = f(&v[i]);
    }
}

fn main(){
    let mut vec = vec![1, 2, 3,4 ,5, 6];

    print_vec(&vec);
    map(&mut vec, |x| {x*2});
    print_vec(&vec);

}