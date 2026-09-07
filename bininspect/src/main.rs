use std::env;
use std::fs;
use std::process::ExitCode;

// while in the src dir, to run, carpgo run -- .\binary filename. bin files need to be in the same dir as the src dir, or you can provide the full path to the file.

fn main()-> ExitCode{  // Adding ExitCode as the return type of the main function allows us to return an exit code to the operating system when the program finishes executing. This is useful for indicating whether the program ran successfully or encountered an error.
                       // Also in Windows, run $LASTEXITCODE to check if the program ran successfully or encountered an error. If the program ran successfully, $LASTEXITCODE will be 0. If the program encountered an error, $LASTEXITCODE will be a non-zero value.                


    // Collect the command line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
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


    let file =  fs::read(filename); // Read the contents of the file specified by the filename argument. The read function returns a Result type, which can be either Ok or Err. If the file is read successfully, it returns Ok with the contents of the file as a vector of bytes. If there is an error reading the file, it returns Err with an error message.



    match file {
        Ok(data) => {
            println!("File: {}", filename); // Print the filename.
            println!("Size: {} bytes", data.len()); // Print the size of the file.
            println!(); // print a newline

            let mut addr: usize = 0; // Initialize the address variable to 0. This variable will be used to keep track of the current address in the file as we iterate over the bytes.
            
            for chunk in data.chunks(16)
            {
                print!("{:04X}: ", addr);  // print the current address in hexadecimal format, padded with zeros to 4 digits. The {:04X} format specifier is used to format the address as a hexadecimal number with at least 4 digits, padding with zeros if necessary.
 
                for byte in chunk // iterate over the bytes in the current chunk and print each byte in hexadecimal format, padded with zeros to 2 digits. The {:02X} format specifier is used to format the byte as a hexadecimal number with at least 2 digits, padding with zeros if necessary.
                {
                    print!("{:02X} ", byte);
                }

                for _ in chunk.len()..16  //
                {
                    print!("   ");
                }

                print!(" |");

                for byte in chunk // iterate over the bytes in the current chunk and print each byte as a character if it is a printable ASCII character, or a dot (.) if it is not. The is_ascii_graphic() method is used to check if the byte is a printable ASCII character, and the *byte == b' ' condition is used to check if the byte is a space character.
                {
                    if byte.is_ascii_graphic() || *byte == b' ' // check if the byte is a printable ASCII character or a space character
                    {
                        print!("{}", *byte as char);
                    }
                    else
                    {
                        print!(".");
                    }
                }

                println!("|");

                addr   = addr + 16; // increment the address variable by 16 for the next chunk of bytes.
            }

            return ExitCode::SUCCESS; // Return a success exit code if the file was read successfully.
        }
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error); // Print an error message if there was an error reading the file. The eprintln! macro is used to print the error message to the standard error stream.
            return ExitCode::FAILURE; // Return a failure exit code if there was an error reading the file.
        }
    }


    println!("Filename: {}", filename);
    ExitCode::SUCCESS // Return a success exit code if the program ran successfully.
}
