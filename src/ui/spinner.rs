use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time;

pub struct Spinner {
    pub spinner: ProgressBar,
    pub finished: Arc<Mutex<bool>>,
    pub thread: JoinHandle<()>,
}

impl Spinner {
    pub fn start(verb: &'static str, message: &str) -> Self {
        let spinner = ProgressBar::new(0).with_style(
            ProgressStyle::with_template(&super::format_log_msg(
                Style::new().bold().cyan(),
                verb,
                &(message.to_owned() + "  {spinner:.cyan}"),
            ))
            .unwrap(),
        );
        spinner.tick();

        let thread_spinner = spinner.clone();
        let finished = Arc::new(Mutex::new(false));
        let thread_finished = Arc::clone(&finished);
        let spinner_thread = thread::spawn(move || {
            while !*thread_finished.lock().unwrap() {
                thread_spinner.tick();
                thread::sleep(time::Duration::from_millis(100));
            }
            thread_spinner.finish_and_clear();
        });

        Self {
            spinner: spinner.clone(),
            finished,
            thread: spinner_thread,
        }
    }

    pub fn end(self, message: &str) {
        self.spinner.finish_and_clear();
        *self.finished.lock().unwrap() = true;
        self.thread.join().unwrap();
        println!("{}", message);
    }
}
