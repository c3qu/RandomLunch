// use std::env;
use std::fs;
use rand::seq::SliceRandom; 

fn main() {
    // // --snip--
    // let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);

    let contents = fs::read_to_string("eat.ini")
        .expect("Something went wrong reading the file");


    let vec: Vec<&str> = contents.split("\n").collect();
    let someone=vec.choose(&mut rand::thread_rng()).expect("ERROR");
    println!("{}", someone);
}
