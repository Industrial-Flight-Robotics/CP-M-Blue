use std::env;
use std::process::ExitCode;

// while in the src dir, to run, carpgo run -- .\binary filename. bin files need to be in the same dir as the src dir, or you can provide the full path to the file.

fn main()-> ExitCode{  // Adding ExitCode as the return type of the main function allows us to return an exit code to the operating system when the program finishes executing. This is useful for indicating whether the program ran successfully or encountered an error.
                       // Also in Windows, run $LASTEXITCODE to check if the program ran successfully or encountered an error. If the program ran successfully, $LASTEXITCODE will be 0. If the program encountered an error, $LASTEXITCODE will be a non-zero value.                


    // Collect the command line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return ExitCode::FAILURE; // Return a failure exit code if the user did not provide a filename argument.
    }

    // Print the command line arguments in Debug Format. {:#?} is the pretty Debug format specifier. Regular Debug format specifier is {:?}. The pretty Debug format specifier prints the output in a more readable format with indentation and line breaks.
    // println!("Arguments: {:#?}", args);

    //println!("Arguments: {}");
    // Iterate over the command line arguments and print each one in a new line.
    //for arg in args.iter(){
    //   println!("Argument: {}", arg);
    //}


    // &args[1] is an example of "borrowing" in Rust. It creates a reference to the second element of the args vector, which is the filename provided by the user. This allows us to use the filename without taking ownership of it, which is important in Rust's ownership model.
    // if we need to just look at some data without taking ownership of it, we can borrow it. Borrowing is a way to access data without taking ownership of it. In this case, we are borrowing the filename from the args vector so that we can use it without taking ownership of it.
    let filename = &args[1];

    println!("Filename: {}", filename);
    ExitCode::SUCCESS // Return a success exit code if the program ran successfully.
}
