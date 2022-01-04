use read_input::prelude::*;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg_1 = match args.get(1) {
        Some(v) => Some(v.parse::<u32>().unwrap()),
        None => None,
    };

    let arg_2 = match args.get(2) {
        Some(v) => Some(v.parse::<u32>().unwrap()),
        None => None,
    };

    match (arg_1, arg_2) {
        (Some(amount), None) => {
            let [stack, items] = total_to_stack(64, amount);
            println!("{} stacks and {} items", stack, items);
        }
        (Some(amount), Some(stack_size)) => {
            println!("lol");
            let [stack, items] = total_to_stack(stack_size, amount);
            println!("{} stacks and {} items", stack, items);
        }
        (None, _) => {
            let amount = input::<u32>().msg("Enter amount of items\n").get();
            let stack_size = input::<u32>()
                .msg("Enter stack size\n(Default is 64)\n")
                .default(64)
                .get();
            let [stack, items] = total_to_stack(stack_size, amount);
            println!("{} stacks and {} items", stack, items);
            pause()
        }
    }
}

fn total_to_stack(stack_size: u32, total: u32) -> [u32; 2] {
    [total / stack_size, total % stack_size]
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
