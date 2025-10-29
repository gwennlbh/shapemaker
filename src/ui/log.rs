use console::Style;
use indicatif::ProgressBar;

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

pub(super) fn format_log_msg(
    style: Style,
    verb: &'static str,
    message: &str,
) -> String {
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
