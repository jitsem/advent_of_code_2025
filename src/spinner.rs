use std::io::Write;
use std::thread::sleep;
use std::{io, thread};
use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};

#[derive(Clone, Copy)]
struct SpinnerState {
    should_show: bool,
    should_stop: bool,
}

pub struct Spinner {
    state: Arc<Mutex<SpinnerState>>,
    handle: JoinHandle<()>,
}

impl Spinner {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(SpinnerState {
            should_show: false,
            should_stop: false,
        }));
        let show_spin = state.clone();
        let handle = thread::spawn(move || {
            let spinner = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
            let mut spinner_index = 0;
            loop {
                {
                    let state = *show_spin.lock().expect("Failed to lock spinner mutex");
                    if state.should_stop {
                        print!("\r");
                        io::stdout().flush().expect("Expected being able to flush");
                        break;
                    }
                    if state.should_show {
                        print!("{}", spinner[spinner_index]);
                        spinner_index += 1;
                        if spinner_index == spinner.len() {
                            spinner_index = 0;
                        }

                        io::stdout().flush().expect("Expected being able to flush");
                        print!("\r");
                    }
                }
                sleep(Duration::from_millis(500));
            }
        });
        Self { state, handle }
    }

    pub fn resume_spining(&self) {
        let mut lock = self.state.lock().expect("Failed to lock spinner mutex");
        lock.should_show = true;
    }

    pub fn pause_spining(&self) {
        let mut lock = self.state.lock().expect("Failed to lock spinner mutex");
        lock.should_show = false;
    }

    pub fn stop_spinner(self) {
        {
            let mut lock = self.state.lock().expect("Failed to lock spinner mutex");
            lock.should_stop = true;
        }
        self.handle.join().expect("Failed to join spinner");
    }
}
