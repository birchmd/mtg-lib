mod deck;
mod parallelism;
mod simulation;

fn main() {
    let distribution = parallelism::run(20, 100_000);
    for (n_turns, amount) in distribution.into_iter().enumerate() {
        println!("{n_turns},{amount}");
    }
}
