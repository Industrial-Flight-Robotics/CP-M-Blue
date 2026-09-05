use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    println!("Arguments: {:?}", args);

    for arg in args.iter(){
        println!("Argument: {}", arg);
    }
}
