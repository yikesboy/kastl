use indicatif::{ProgressBar, ProgressStyle};
use std::future::Future;
use std::io::IsTerminal;
use std::time::Duration;

pub async fn with_spinner<F, T, E>(message: impl Into<String>, fut: F) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
{
    let progress_bar = if std::io::stderr().is_terminal() {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner} {msg}").expect("valid spinner template"),
        );
        pb.set_message(message.into());
        pb.enable_steady_tick(Duration::from_millis(100));

        pb
    } else {
        ProgressBar::hidden()
    };

    let result = fut.await;
    progress_bar.finish_and_clear();
    result
}
