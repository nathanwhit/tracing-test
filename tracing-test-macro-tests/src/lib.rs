//! This crate is only here to test the `tracing-test-macro` crate (because proc macros cannot be
//! tested from within the crate itself).

#[cfg(test)]
mod tests {
    use tracing::{info, warn};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_logs_are_captured() {
        // Local log
        info!("This is being logged on the info level");

        info!("CountMe");
        // Log from a spawned task (which runs in a separate thread)
        tokio::spawn(async {
            warn!("This is being logged on the warn level from a spawned task");
            info!("CountMe");
        })
        .await
        .unwrap();

        // Ensure that `logs_contain` works as intended
        assert!(logs_contain("logged on the info level"));
        assert!(logs_contain("logged on the warn level"));
        assert!(!logs_contain("logged on the error level"));

        // Ensure that `logs_assert` works as intended (with a closure)
        logs_assert(|lines: &[&str]| {
            match lines.iter().filter(|line| line.contains("CountMe")).count() {
                2 => Ok(()),
                n => Err(format!("Count should be 2, but was {}", n)),
            }
        });

        // Ensure that `logs_assert` works as intended (with a function)
        fn assert_fn(lines: &[&str]) -> Result<(), String> {
            match lines.iter().filter(|line| line.contains("CountMe")).count() {
                2 => Ok(()),
                n => Err(format!("Count should be 2, but was {}", n)),
            }
        }
        logs_assert(assert_fn);
    }

    #[tokio::test]
    #[traced_test(filter_crate)]
    async fn filters_crate() {
        info!("This is being logged on the info level");
        assert!(logs_contain("logged on the info level"));
    }

    #[traced_test]
    #[test]
    fn annotate_sync_test() {
        assert!(!logs_contain("Logging from a non-async test"));
        info!("Logging from a non-async test");
        assert!(logs_contain("Logging from a non-async test"));
        assert!(!logs_contain("This was never logged"));
    }
}
