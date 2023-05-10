// This function computes the Collatz sequence of numbers. This is sometimes also known as 
// the "hailstone sequence". Starting from any number `n`, by applying the function below,
// we gnerate a sequence that eventually ends with `n` equal to 1.

pub fn collatz(mut n: u32) -> Vec<u32> {

    if n == 0 {
        panic!("Starting value of `n` cannot be equal to zero!");
    }

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


#[cfg(test)]
mod tests {
    use super::collatz;

    #[test]
    fn collatz_return_vec() {
        let collatz_seq = collatz(100_000_000);

        assert!(collatz_seq.len() > 0);
    }
}
