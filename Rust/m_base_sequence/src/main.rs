fn next_non_decreasing(seq: &mut [u8], base: u8) -> Option<()> {
    let mut index = seq.len();
    loop {
        index = index.checked_sub(1)?;
        seq[index] += 1;
        let si = seq[index];
        if si < base {
            seq[index..].fill(si);
            break Some(());
        }
    }
}
const SIZE: usize = 6;
const BASE: u8 = 3;

fn main() {
    let mut seq = [0_u8; SIZE / 2];

    let mut aggregator: Vec<usize> = vec![0; (SIZE / 2) as usize * (BASE as usize - 1) + 1];
    aggregator[0] = 1;
    while let Some(_) = next_non_decreasing(&mut seq, BASE) {
        println!("{seq:?}");
        aggregator[seq.iter().fold(0_usize, |acc, x| acc + *x as usize)] += 1;
        println!("{aggregator:?}");
    }

    println!("done: {aggregator:?}");

    println!(
        "number of subsequence: {}",
        aggregator
            .into_iter()
            .fold(0_usize, |acc, x| acc + x.pow(2))
    );
}
