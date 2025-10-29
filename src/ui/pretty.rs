use chrono::DateTime;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Range;
use std::time::Duration;

use crate::Timestamp;

pub(crate) trait Pretty {
    fn pretty(&self) -> String;
}

impl<K: std::fmt::Display> Pretty for HashMap<K, usize> {
    fn pretty(&self) -> String {
        self.iter()
            .filter_map(|(name, &count)| {
                if count > 0 {
                    Some(format!("{count} {name}"))
                } else {
                    None
                }
            })
            .join(", ")
    }
}

impl Pretty for Duration {
    fn pretty(&self) -> String {
        let (hours, rest) = self.as_millis().div_rem(&3_600_000);
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
}

trait DivRem<T> {
    fn div_rem(&self, rhs: &T) -> (T, T);
}

impl DivRem<u128> for u128 {
    fn div_rem(&self, rhs: &u128) -> (u128, u128) {
        (self / rhs, self % rhs)
    }
}

impl Pretty for Timestamp {
    fn pretty(&self) -> String {
        format!(
            "{}",
            DateTime::from_timestamp_millis(self.ms() as i64)
                .unwrap()
                .format("%H:%M:%S%.3f")
        )
    }
}

impl Pretty for Range<Timestamp> {
    fn pretty(&self) -> String {
        format!("from {} to {}", self.start.pretty(), self.end.pretty())
    }
}

impl Pretty for std::path::PathBuf {
    fn pretty(&self) -> String {
        format!(
            "{}{}",
            if self.is_relative() { "./" } else { "" },
            self.to_string_lossy()
        )
    }
}
