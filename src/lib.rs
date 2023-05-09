// This function computes the Collatz sequence of numbers. This is sometimes also known as 
// the "hailstone sequence" 

pub fn collatz(mut n: u32) -> Vec<u32> {

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

