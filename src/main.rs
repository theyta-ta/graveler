// on my shitty hardware it gets ~0.11s for 1million. 
// so expecting ~110s = ~2min for 1bn

// for random
use rand::distributions::{Bernoulli, Distribution};
use rand::thread_rng;

// for multithreading
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::sync::Arc;

// for timing
use std::time::Instant;

const NO_OF_THREADS: u64 = 20;
const NO_OF_SIMS: u64 = 1_000_000_000;
// make sure no_of_sims is a multiple of no_of_threads. else youll get less than expected simulations
const NO_OF_SIMS_PER_THREAD: u64 = NO_OF_SIMS / NO_OF_THREADS;

const PARA_PROCS: u64 = 177;

fn main() {
    let now = Instant::now();
    do_the_thang();
    let elapsed_time = now.elapsed();
    println!("{:?}", elapsed_time);
}

fn do_the_thang() {
    // 1 in 4 chance of succeeding
    let d = Bernoulli::from_ratio(1, 4).unwrap();
    let mut handles = Vec::new();
    // save the max. will stop generating random numbers if impossible to pass
    // each process will update the max ecery 1000 simulations
    let max_ones = Arc::new(AtomicU64::new(0));
    // spawn 20 threads. aka get it to run 20 simulations at the same time
    for _ in 0..NO_OF_THREADS {
        let max_ones = max_ones.clone();
        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            //let mut next_update = 1;
            let mut max_ones_thread: u64 = max_ones.load(Ordering::Relaxed);
            for _ in 0..NO_OF_SIMS_PER_THREAD {
                let mut count: u64 = 0;
                let mut poss = true;
                for j in 0..PARA_PROCS {
                    // check if its possible to beat max
                    if j + max_ones_thread >= PARA_PROCS + count { poss = false; break; }
                    if d.sample(&mut rng) { count += 1 }
                }
                // we have a new max
                if poss {
                    max_ones_thread = max_ones
                        .fetch_max(count, Ordering::Relaxed)
                        .max(count);
                }
                //println!("{}", count);

                /*if i >= next_update {
                    next_update <<= 1;
                    max_ones_thread = max_ones
                        .fetch_max(max_ones_thread, Ordering::Relaxed)
                        .max(max_ones_thread);
                }*/
                //let _ = max_ones.fetch_max(max_ones_thread, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    println!("{}", max_ones.load(Ordering::Relaxed));
}
