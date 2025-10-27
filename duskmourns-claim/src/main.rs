mod deck;
mod parallelism;
mod simulation;

fn main() {
    let (wins, losses) = parallelism::run(20, 100_000);
    for (n_turns, amount) in wins.into_iter().enumerate() {
        println!("{n_turns},{amount}");
    }
    for (n_turns, amount) in losses.into_iter().enumerate() {
        println!("{n_turns},{amount}");
    }
}
