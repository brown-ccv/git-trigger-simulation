use rand::{self, Rng};
use time::Instant;
use collatz::collatz;


fn main() {
    let mut rng = rand::thread_rng();
    let starting_num: u32 = rng.gen_range(1..100_000_000);
   
    let t1 = Instant::now();

    let collatz_seq = collatz(starting_num);

    let t2 = Instant::now();

    println!("{:?}, {:?}", collatz_seq, t2 - t1);
}
