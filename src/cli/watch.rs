use super::run;
use std::{path::PathBuf, time::Duration};
use watchexec::Watchexec;
use watchexec_events::{Event, FileType, Tag};
use watchexec_signals::Signal;

pub async fn watch_project(project_dir: PathBuf) -> anyhow::Result<()> {
    let pathset = vec![project_dir.clone()];

    let watched = |path: &PathBuf| {
        path.extension()
            .map_or(false, |ext| vec!["rs"].contains(&ext.to_str().unwrap()))
    };

    let wx = Watchexec::new(move |mut action| {
        // print any events
        for event in action.events.iter() {
            match event {
                Event { tags, .. } => {
                    for tag in tags {
                        match tag.clone() {
                            Tag::Path {
                                path,
                                file_type: Some(FileType::File),
                            } if watched(&path) => {
                                run::run_project(&project_dir).unwrap()
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        // if Ctrl-C is received, quit
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit();
        }

        action
    })?;

    wx.config.pathset(pathset);
    wx.config.throttle(Duration::from_secs(1));

    // TODO handle miette diagnostics
    wx.main().await?;
    Ok(())
}
