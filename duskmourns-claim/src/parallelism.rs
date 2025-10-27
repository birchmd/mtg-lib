use std::sync::mpsc;

pub fn run(n_threads: usize, n_sims_per_thread: usize) -> Vec<usize> {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::with_capacity(n_threads);
    for _ in 0..n_threads {
        let thread_tx = tx.clone();
        let handle = std::thread::spawn(move || run_single(n_sims_per_thread, thread_tx));
        handles.push(handle);
    }
    drop(tx);

    let mut result = vec![0; 20];
    while let Ok(n_turns) = rx.recv() {
        if n_turns >= result.len() {
            result.extend(std::iter::repeat_n(0, n_turns - result.len() + 1));
        }
        result[n_turns] += 1;
    }

    for handle in handles {
        handle.join().expect("All threads finish");
    }

    result
}

fn run_single(n_sims: usize, tx: mpsc::Sender<usize>) {
    for _ in 0..n_sims {
        let n_turns = crate::simulation::simulation_run();
        tx.send(n_turns).expect("Receiver exists");
    }
}
