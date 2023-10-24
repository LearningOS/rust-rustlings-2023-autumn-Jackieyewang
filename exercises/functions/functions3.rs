// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(1);
}

fn call_me(num: usize) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
