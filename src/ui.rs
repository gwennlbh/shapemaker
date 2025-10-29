use chrono::DateTime;
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{self, Duration};

pub const PROGRESS_BARS_STYLE: &str = "\x1b]9;4;1;{percent}\x1b\\{prefix:>12.bold.cyan} {percent:03}% [{bar:25}] {msg:01} ({per_sec}, {elapsed} ago)";

pub struct Spinner {
    pub spinner: ProgressBar,
    pub finished: Arc<Mutex<bool>>,
    pub thread: JoinHandle<()>,
}

impl Spinner {
    pub fn start(verb: &'static str, message: &str) -> Self {
        let spinner = ProgressBar::new(0).with_style(
            ProgressStyle::with_template(&format_log_msg(
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

pub fn setup_progress_bar(total: u64, verb: &'static str) -> ProgressBar {
    indicatif::ProgressBar::new(total)
        .with_prefix(verb)
        .with_style(
            indicatif::ProgressStyle::with_template(PROGRESS_BARS_STYLE)
                .unwrap()
                .progress_chars("=> "),
        )
        .with_finish(indicatif::ProgressFinish::WithMessage(
            "\x1b]9;4;0\x1b\\".into(),
        ))
}

pub trait Log {
    fn log_styled(&self, style: Style, verb: &'static str, message: &str);
    fn log(&self, verb: &'static str, message: &str) {
        self.log_styled(Style::new().bold().green(), verb, message);
    }
    fn log_cyan(&self, verb: &'static str, message: &str) {
        self.log_styled(Style::new().bold().cyan(), verb, message);
    }
    fn log_error(&self, verb: &'static str, message: &str) {
        self.log_styled(Style::new().bold().red(), verb, message);
    }
}

fn format_log_msg(style: Style, verb: &'static str, message: &str) -> String {
    format!("{} {}", style.apply_to(format!("{verb:>12}")), message)
}

impl Log for () {
    fn log_styled(&self, style: Style, verb: &'static str, message: &str) {
        println!("{}", format_log_msg(style, verb, message));
    }
}

impl Log for ProgressBar {
    fn log_styled(&self, style: Style, verb: &'static str, message: &str) {
        self.println(format_log_msg(style, verb, message));
    }
}

impl Log for Option<&ProgressBar> {
    fn log_styled(&self, style: Style, verb: &'static str, message: &str) {
        if let Some(pb) = self {
            pb.println(format_log_msg(style, verb, message));
        }
    }
}

pub trait MaybeProgressBar<'a> {
    fn set_message(&'a self, message: impl Into<Cow<'static, str>>);
    fn set_length(&'a self, length: u64);
    fn inc(&'a self, n: u64);
    fn println(&'a self, message: impl AsRef<str>);
}

impl<'a> MaybeProgressBar<'a> for Option<&'a ProgressBar> {
    fn set_message(&'a self, message: impl Into<Cow<'static, str>>) {
        if let Some(pb) = self {
            pb.set_message(message);
        }
    }

    fn set_length(&'a self, length: u64) {
        if let Some(pb) = self {
            pb.set_length(length);
        }
    }

    fn inc(&'a self, n: u64) {
        if let Some(pb) = self {
            pb.inc(n);
        }
    }

    fn println(&'a self, message: impl AsRef<str>) {
        if let Some(pb) = self {
            pb.println(message);
        }
    }
}

pub fn display_counts(counts: HashMap<impl std::fmt::Display, usize>) -> String {
    counts
        .iter()
        .filter_map(|(name, &count)| {
            if count > 0 {
                Some(format!("{count} {name}"))
            } else {
                None
            }
        })
        .join(", ")
}

pub(crate) fn format_duration(duration: Duration) -> String {
    let (hours, rest) = duration.as_millis().div_rem(&3_600_000);
    let (minutes, rest) = rest.div_rem(&60_000);
    let (seconds, milliseconds) = rest.div_rem(&1_000);

    if hours > 0 {
        format!("{} h {:02} m {:02} s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{} m {:02} s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{:03} s", seconds, milliseconds)
    } else {
        format!("{} ms", milliseconds)
    }
}

trait DivRem<T> {
    fn div_rem(&self, rhs: &T) -> (T, T);
}

impl DivRem<u128> for u128 {
    fn div_rem(&self, rhs: &u128) -> (u128, u128) {
        (self / rhs, self % rhs)
    }
}

pub(crate) fn format_timestamp(ms: usize) -> String {
    format!(
        "{}",
        DateTime::from_timestamp_millis(ms as i64)
            .unwrap()
            .format("%H:%M:%S%.3f")
    )
}

pub(crate) fn format_timestamp_range(ms_range: &Range<usize>) -> String {
    format!(
        "from {} to {}",
        format_timestamp(ms_range.start),
        format_timestamp(ms_range.end)
    )
}

pub(crate) fn format_filepath(path: &std::path::Path) -> String {
    format!(
        "{}{}",
        if path.is_relative() { "./" } else { "" },
        path.to_string_lossy()
    )
}
