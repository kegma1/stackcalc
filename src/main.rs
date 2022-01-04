fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    
        match args.len() {
            1 => {
                use read_input::prelude::*;
                let amount = input::<u32>().msg("Enter amount of items\n>>> ").get();
                let stack_size = input::<u32>().msg("Enter stack size\n(Default is 64)\n>>> ").default(64).get();
                let [stack, items] = total_to_stack(stack_size, amount);
                println!("{} stacks and {} items",stack,items);
                pause()
            }
            2 => {
                let amount = args.get(1).unwrap().parse::<u32>().unwrap();
                let [stack, items] = total_to_stack(64, amount);
                println!("{} stacks and {} items",stack,items);
            }
            3 => {
                let amount = args.get(1).unwrap().parse::<u32>().unwrap();
                let stack_size = args.get(2).unwrap().parse::<u32>().unwrap();
                let [stack, items] = total_to_stack(stack_size, amount);
                println!("{} stacks and {} items",stack,items);
                
            }
            _ => ()
        }
       
}

fn total_to_stack(stack_size: u32, total: u32) -> [u32; 2] {
    [total / stack_size, total % stack_size]
}

use std::io;
use std::io::prelude::*;

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
