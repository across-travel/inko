//! Virtual Machine Configuration
//!
//! Various virtual machine settings can be changed by the user, such as the
//! directories to search for bytecode files and the number of operating system
//! threads to use for running VM processes.
//!
//! These settings are all stored in the Config struct, allowing various parts
//! of the VM to easily access these configuration details.
#![cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]

use immix::block::BLOCK_SIZE;
use num_cpus;
use std::env;
use std::path::PathBuf;

/// Sets a configuration field based on an environment variable.
macro_rules! set_from_env {
    ($config:expr, $field:ident, $key:expr, $value_type:ty) => {{
        if let Ok(raw_value) = env::var(concat!("INKO_", $key)) {
            if let Ok(value) = raw_value.parse::<$value_type>() {
                $config.$field = value;
            }
        };
    }};
}

const DEFAULT_YOUNG_THRESHOLD: usize = (8 * 1024 * 1024) / BLOCK_SIZE;
const DEFAULT_MATURE_THRESHOLD: usize = (16 * 1024 * 1024) / BLOCK_SIZE;
const DEFAULT_MAILBOX_THRESHOLD: usize = 1;
const DEFAULT_GROWTH_FACTOR: f64 = 1.5;
const DEFAULT_GROWTH_THRESHOLD: f64 = 0.9;
const DEFAULT_SUSPENSION_CHECK_INTERVAL: f64 = 0.1;
const DEFAULT_REDUCTIONS: usize = 1000;

/// Structure containing the configuration settings for the virtual machine.
pub struct Config {
    /// The directories to search in for extra bytecode files to run.
    pub directories: Vec<PathBuf>,

    /// The number of primary process threads to run.
    pub primary_threads: u8,

    /// The number of secondary process threads to run.
    pub secondary_threads: u8,

    /// The number of garbage collector threads to run. Defaults to 2 threads.
    pub gc_threads: u8,

    /// The number of finalizer threads to run. Defaults to 2 threads.
    pub finalizer_threads: u8,

    /// The number of threads to use for various generic parallel tasks such as
    /// scanning stack frames during garbage collection. Defaults to the number
    /// of physical CPU cores.
    pub generic_parallel_threads: u8,

    /// The number of reductions a process can perform before being suspended.
    /// Defaults to 1000.
    pub reductions: usize,

    /// The number of seconds to wait between checking for suspended processes.
    pub suspension_check_interval: f64,

    /// The number of memory blocks that can be allocated before triggering a
    /// young collection.
    pub young_threshold: usize,

    /// The number of memory blocks that can be allocated before triggering a
    /// mature collection.
    pub mature_threshold: usize,

    /// The block allocation growth factor for the heap.
    pub heap_growth_factor: f64,

    /// The percentage of memory in the heap (relative to its threshold) that
    /// should be used before increasing the heap size.
    pub heap_growth_threshold: f64,

    /// The number of memory blocks that can be allocated before triggering a
    /// mailbox collection.
    pub mailbox_threshold: usize,

    /// The block allocation growth factor for the mailbox heap.
    pub mailbox_growth_factor: f64,

    /// The percentage of memory in the mailbox heap that should be used before
    /// increasing the size.
    pub mailbox_growth_threshold: f64,
}

impl Config {
    pub fn new() -> Config {
        let cpu_count = num_cpus::get() as u8;

        Config {
            directories: Vec::new(),
            primary_threads: cpu_count,
            gc_threads: cpu_count,
            finalizer_threads: cpu_count,
            secondary_threads: cpu_count,
            generic_parallel_threads: cpu_count,
            reductions: DEFAULT_REDUCTIONS,
            suspension_check_interval: DEFAULT_SUSPENSION_CHECK_INTERVAL,
            young_threshold: DEFAULT_YOUNG_THRESHOLD,
            mature_threshold: DEFAULT_MATURE_THRESHOLD,
            heap_growth_factor: DEFAULT_GROWTH_FACTOR,
            heap_growth_threshold: DEFAULT_GROWTH_THRESHOLD,
            mailbox_threshold: DEFAULT_MAILBOX_THRESHOLD,
            mailbox_growth_factor: DEFAULT_GROWTH_FACTOR,
            mailbox_growth_threshold: DEFAULT_GROWTH_THRESHOLD,
        }
    }

    /// Populates configuration settings based on environment variables.
    #[cfg_attr(feature = "cargo-clippy", allow(cyclomatic_complexity))]
    pub fn populate_from_env(&mut self) {
        set_from_env!(self, primary_threads, "CONCURRENCY", u8);
        set_from_env!(self, secondary_threads, "CONCURRENCY", u8);
        set_from_env!(self, gc_threads, "CONCURRENCY", u8);
        set_from_env!(self, finalizer_threads, "CONCURRENCY", u8);
        set_from_env!(self, generic_parallel_threads, "CONCURRENCY", u8);

        set_from_env!(self, reductions, "REDUCTIONS", usize);
        set_from_env!(
            self,
            suspension_check_interval,
            "SUSPENSION_CHECK_INTERVAL",
            f64
        );

        set_from_env!(self, young_threshold, "YOUNG_THRESHOLD", usize);
        set_from_env!(self, mature_threshold, "MATURE_THRESHOLD", usize);
        set_from_env!(self, heap_growth_factor, "HEAP_GROWTH_FACTOR", f64);

        set_from_env!(
            self,
            heap_growth_threshold,
            "HEAP_GROWTH_THRESHOLD",
            f64
        );

        set_from_env!(self, mailbox_threshold, "MAILBOX_THRESHOLD", usize);

        set_from_env!(
            self,
            mailbox_growth_factor,
            "MAILBOX_GROWTH_FACTOR",
            f64
        );

        set_from_env!(
            self,
            mailbox_growth_threshold,
            "MAILBOX_GROWTH_THRESHOLD",
            f64
        );
    }

    pub fn add_directory(&mut self, path: String) {
        self.directories.push(PathBuf::from(path));
    }

    pub fn set_primary_threads(&mut self, threads: u8) {
        if threads == 0 {
            self.primary_threads = 1;
        } else {
            self.primary_threads = threads;
        }
    }

    pub fn set_secondary_threads(&mut self, threads: u8) {
        if threads == 0 {
            self.secondary_threads = 1;
        } else {
            self.secondary_threads = threads;
        }
    }

    pub fn set_gc_threads(&mut self, threads: u8) {
        if threads == 0 {
            self.gc_threads = 1;
        } else {
            self.gc_threads = threads;
        }
    }

    pub fn set_reductions(&mut self, reductions: usize) {
        if reductions > 0 {
            self.reductions = reductions;
        }
    }

    pub fn set_suspension_check_interval(&mut self, interval: f64) {
        if interval > 0.0 {
            self.suspension_check_interval = interval;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new() {
        let config = Config::new();

        assert_eq!(config.directories.len(), 0);
        assert!(config.primary_threads >= 1);
        assert!(config.gc_threads >= 1);
        assert_eq!(config.reductions, 1000);
    }

    #[test]
    fn test_populate_from_env() {
        env::set_var("INKO_CONCURRENCY", "42");
        env::set_var("INKO_HEAP_GROWTH_FACTOR", "4.2");

        let mut config = Config::new();

        config.populate_from_env();

        // Unset before any assertions may fail.
        env::remove_var("INKO_CONCURRENCY");
        env::remove_var("INKO_HEAP_GROWTH_FACTOR");

        assert_eq!(config.primary_threads, 42);
        assert_eq!(config.heap_growth_factor, 4.2);
    }

    #[test]
    fn test_add_directory() {
        let mut config = Config::new();

        config.add_directory("foo".to_string());

        assert_eq!(config.directories.len(), 1);
    }

    #[test]
    fn test_set_primary_threads() {
        let mut config = Config::new();

        config.set_primary_threads(5);

        assert_eq!(config.primary_threads, 5);
    }

    #[test]
    fn test_set_gc_threads() {
        let mut config = Config::new();

        config.set_gc_threads(5);

        assert_eq!(config.gc_threads, 5);
    }

    #[test]
    fn test_set_reductions() {
        let mut config = Config::new();

        config.set_reductions(5);

        assert_eq!(config.reductions, 5);
    }

    #[test]
    fn test_set_secondary_threads() {
        let mut config = Config::new();

        config.set_secondary_threads(2);

        assert_eq!(config.secondary_threads, 2);
    }
}
