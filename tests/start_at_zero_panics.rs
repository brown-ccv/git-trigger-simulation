use collatz::collatz;


#[test]
#[should_panic]
fn starting_at_zero_panics() {

    let _collatz_seq = collatz(0);

}
