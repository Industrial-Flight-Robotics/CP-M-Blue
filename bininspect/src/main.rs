use std::env;


// while in the src dir, to run, carpgo run -- .\binary filename. bin files need to be in the same dir as the src dir, or you can provide the full path to the file.
fn main() {

    // Collect the command line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();


    // Print the command line arguments in Debug Format. {:#?} is the pretty Debug format specifier. Regular Debug format specifier is {:?}. The pretty Debug format specifier prints the output in a more readable format with indentation and line breaks.
    // println!("Arguments: {:#?}", args);


    // Iterate over the command line arguments and print each one in a new line.
    for arg in args.iter(){
        println!("Argument: {}", arg);
    }
}
