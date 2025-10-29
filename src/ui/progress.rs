use indicatif::ProgressBar;
use std::borrow::Cow;

pub const PROGRESS_BARS_STYLE: &str = "\x1b]9;4;1;{percent}\x1b\\{prefix:>12.bold.cyan} {percent:03}% [{bar:25}] {msg:01} ({per_sec}, {elapsed} ago)";

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
