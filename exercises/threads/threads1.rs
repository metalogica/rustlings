// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let new_job = JobStatus { jobs_completed: 0 };
    // TO DO: Change this line
    let status = Arc::new(Mutex::new(new_job));
    let status_shared = Arc::clone(&status);
    // spawn a new thread that will run a for-loop code block 10 times
    thread::spawn(move || {
        // in this code block, the new thread will sleep 250ms, and then increment the counter in JobStatus
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            // TO DO: Change this line
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    // whilst the spawned thread has begun running, we will peek into the JobStatus on 500ms intervals to check the counter
    // if the counter has reached 10, we stop running the code block below.
    // TO DO: Change this line
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... Current Count: {:#?}", status.lock().unwrap().jobs_completed);
        thread::sleep(Duration::from_millis(500));
    }
}
