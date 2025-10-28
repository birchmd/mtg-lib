mod deck;
mod parallelism;
mod simulation;

fn main() {
    let (wins, losses) = parallelism::run(20, 100_000);
    let mean_win = {
        let (weighted_total, total) = wins
            .iter()
            .enumerate()
            .fold((0, 0), |(wt, t), (n_turns, &freq)| {
                (wt + (n_turns * freq), t + freq)
            });
        (weighted_total as f64) / (total as f64)
    };
    println!("Average {mean_win}");
    for (n_turns, amount) in wins.into_iter().enumerate() {
        println!("{n_turns},{amount}");
    }
    for (n_turns, amount) in losses.into_iter().enumerate() {
        println!("{n_turns},{amount}");
    }
}
