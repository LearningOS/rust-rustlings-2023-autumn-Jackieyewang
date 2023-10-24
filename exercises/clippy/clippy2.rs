// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut res = 42;
    for x in 1..=12 {
        res += x;
    }
    println!("{}", res);
}
