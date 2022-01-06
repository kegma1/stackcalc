use read_input::prelude::*;
use std::env;
use std::io;
use std::io::prelude::*;

enum ArgErr {
    NotPassed,
    Failed
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg_1 = get_arg(args.get(1));

    // If the first argument is ok the value is stored in amount, if it fails in any way it goes into tui mode.
    match arg_1 {
        Ok(amount) => {
                    let arg_2 = get_arg(args.get(2));
                    // If args_2 if ok it return the value, else it returns 64 if no value was passed, and it goes into tui mode if the value failed to parse.
                    let stack_size = match arg_2 {
                        Ok(x) => x,
                        Err(e) => match e {
                            ArgErr::NotPassed => 64,
                            ArgErr::Failed => {tui(); 0} // Idk why i need this 0, but it does not mater since tui mode exits the program early so this wont be an issue.
                        }
                    };
                    let [stack, items] = total_to_stack(stack_size, amount);
                    println!("{} stacks and {} items", stack, items);
                }
        Err(_) => tui()
    }
}

// Text-based user interface
fn tui()->() {
    let amount = input::<u32>().msg("Enter amount of items\n").get();
    let stack_size = input::<u32>()
        .msg("Enter stack size\n(Default is 64)\n")
        .default(64)
        .get();
    let [stack, items] = total_to_stack(stack_size, amount);
    println!("{} stacks and {} items", stack, items);
    pause();
    std::process::exit(0)
}

// Takes and optional string and either parses it to a u32, returns a NotPassed error if there is no argument or returns a Failed error if the parsing fail
fn get_arg(arg:Option<&String>)->Result<u32, ArgErr> {
    match arg {
        Some(v) => {
            let parsed_arg = v.parse::<u32>();
            match parsed_arg {
                Ok(x) => Ok(x),
                Err(_) => Err(ArgErr::Failed)
            }
        },
        None => Err(ArgErr::NotPassed)
    }
}

// Calculates how many stacks of stack_size total is and return an array of tow elements with the first being the number of stacks and the second being the rest.
fn total_to_stack(stack_size: u32, total: u32) -> [u32; 2] {
    [total / stack_size, total % stack_size]
}

// To pause the program and wait for the user to press a key. This allows the user to see the outcome of the program if its in tui mode.
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
