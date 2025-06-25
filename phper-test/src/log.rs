//! Logging utilities for phper tests.
//!
//! This module provides a centralized logging setup specifically designed for
//! test environments. It configures the env_logger with custom formatting to
//! display key-value pairs in a structured format.

use env_logger::fmt::Formatter;
use log::kv::{self, Key, Value};
use std::sync::Once;

/// Sets up the logger for test environments.
///
/// This function initializes the env_logger with custom formatting that
/// displays key-value pairs in a structured format with separators. The setup
/// is guaranteed to run only once using `std::sync::Once`.
///
/// # Features
/// - Uses default environment configuration
/// - Enables test mode formatting
/// - Custom key-value formatter that displays each pair with visual separators
pub fn setup() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        env_logger::Builder::from_default_env()
            .default_format()
            .is_test(true)
            .format_key_values(|buf, args| {
                use std::io::Write as _;

                /// A visitor implementation for formatting key-value pairs.
                ///
                /// This visitor formats each key-value pair with visual
                /// separators, making log output more readable
                /// in test environments.
                struct Visitor<'a>(&'a mut Formatter);

                impl<'kvs> kv::VisitSource<'kvs> for Visitor<'kvs> {
                    /// Visits and formats a single key-value pair.
                    ///
                    /// # Arguments
                    /// * `key` - The key of the key-value pair
                    /// * `value` - The value of the key-value pair
                    ///
                    /// # Returns
                    /// Returns `Ok(())` on successful formatting, or a
                    /// `kv::Error` if formatting fails.
                    fn visit_pair(
                        &mut self, key: Key<'kvs>, value: Value<'kvs>,
                    ) -> Result<(), kv::Error> {
                        writeln!(self.0).unwrap();
                        writeln!(self.0, "===== {} =====", key).unwrap();
                        writeln!(self.0, "{}", value).unwrap();
                        Ok(())
                    }
                }
                args.visit(&mut Visitor(buf)).unwrap();
                Ok(())
            })
            .init();
    });
}
