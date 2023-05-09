use rand::{self, Rng};
use time::Instant;

// This function computes the Collatz sequence of numbers. This is sometimes also known as 
// the "hailstone sequence" 
fn collatz(mut n: u32) -> Vec<u32> {

    let mut collatz_sequence = vec![n]; // initialize the vector with the starting number
    
    while n != 1 {
        match n % 2 {
            0 => n /= 2,
            1 => n = n * 3 + 1,
            _ => unreachable!(),
        }
        collatz_sequence.push(n);       // add the current number to the vector
    }

    collatz_sequence
}



fn main() {
    let mut rng = rand::thread_rng();
    let starting_num: u32 = rng.gen_range(1..100_000_000);
   
    let t1 = Instant::now();

    let collatz_seq = collatz(starting_num);

    let t2 = Instant::now();

    println!("{:?}, {:?}", collatz_seq, t2 - t1);
}
