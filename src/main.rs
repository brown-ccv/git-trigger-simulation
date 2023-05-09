


fn collatz(mut n: i32) -> Vec<i32> {

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
    let starting_num = 54321;
    let collatz_seq = collatz(starting_num);
    println!("{:?}", collatz_seq);
}
