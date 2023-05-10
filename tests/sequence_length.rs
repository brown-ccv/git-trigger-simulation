use collatz::collatz;

fn get_sequence_length(n_start: u32) -> usize {
    let collatz_seq = collatz(n_start);
    collatz_seq.len()
}

#[test]
fn sequence_lengths_are_correct() {
    assert_eq!(get_sequence_length(8), 4);
    assert_eq!(get_sequence_length(16), 5);
    assert_eq!(get_sequence_length(5), 6);
    assert_eq!(get_sequence_length(10), 7);
    assert_eq!(get_sequence_length(20), 8);

}
