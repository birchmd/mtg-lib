use std::sync::mpsc;

pub fn run(n_threads: usize, n_sims_per_thread: usize) -> (Vec<usize>, Vec<usize>) {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::with_capacity(n_threads);
    for _ in 0..n_threads {
        let thread_tx = tx.clone();
        let handle = std::thread::spawn(move || run_single(n_sims_per_thread, thread_tx));
        handles.push(handle);
    }
    drop(tx);

    let mut wins = vec![0; 20];
    let mut losses = vec![0; 20];
    while let Ok(outcome) = rx.recv() {
        // A negative value represents a loss
        if outcome < 0 {
            let n_turns = -outcome;
            increment(&mut losses, n_turns as usize);
        } else {
            increment(&mut wins, outcome as usize);
        }
    }

    for handle in handles {
        handle.join().expect("All threads finish");
    }

    (wins, losses)
}

fn increment(result: &mut Vec<usize>, index: usize) {
    if index >= result.len() {
        result.extend(std::iter::repeat_n(0, index - result.len() + 1));
    }
    result[index] += 1;
}

fn run_single(n_sims: usize, tx: mpsc::Sender<isize>) {
    for _ in 0..n_sims {
        let n_turns = crate::simulation::simulation_run();
        tx.send(n_turns).expect("Receiver exists");
    }
}
