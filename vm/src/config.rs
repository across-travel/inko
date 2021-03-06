//! Virtual Machine Configuration
//!
//! Various virtual machine settings that can be changed by the user, such as
//! the number of threads to run.
use crate::immix::block::BLOCK_SIZE;
use std::cmp::min;
use std::env;

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

const DEFAULT_YOUNG_THRESHOLD: u32 = (2 * 1024 * 1024) / (BLOCK_SIZE as u32);
const DEFAULT_MATURE_THRESHOLD: u32 = (4 * 1024 * 1024) / (BLOCK_SIZE as u32);
const DEFAULT_GROWTH_FACTOR: f64 = 1.5;
const DEFAULT_GROWTH_THRESHOLD: f64 = 0.9;
const DEFAULT_REDUCTIONS: usize = 1000;

/// Structure containing the configuration settings for the virtual machine.
pub struct Config {
    /// The number of primary process threads to run.
    ///
    /// This defaults to the number of CPU cores.
    pub primary_threads: usize,

    /// The number of blocking process threads to run.
    ///
    /// This defaults to the number of CPU cores.
    pub blocking_threads: usize,

    /// The number of garbage collector threads to run.
    ///
    /// This defaults to half the number of CPU cores, with a minimum of two.
    pub gc_threads: usize,

    /// The number of threads to run for tracing objects during garbage
    /// collection.
    ///
    /// This defaults to half the number of CPU cores, with a minimum of two.
    pub tracer_threads: usize,

    /// The number of threads to use for parsing bytecode images.
    ///
    /// This defaults to 4 cores, or the number of cores itself if this is lower
    /// than or equal to 4.
    ///
    /// When implementing parallel parsing of bytecode images, we investigated
    /// what default would be best. For this test we loaded the image used to
    /// run all of Inko's standard library tests. The CPUs used for this test
    /// were:
    ///
    /// * Ryzen 1600X, with 6 cores and 12 threads
    /// * Intel i5-8265U, with 4 cores and 8 threads
    ///
    /// On both systems we found that 4 cores gives the best performance, with 8
    /// cores performing slightly worse (but not much). For programs that
    /// involve images containing thousands of modules, increasing the number of
    /// cores may reduce the time it takes to parse the image.
    pub bytecode_threads: usize,

    /// The number of reductions a process can perform before being suspended.
    /// Defaults to 1000.
    pub reductions: usize,

    /// The number of memory blocks that can be allocated before triggering a
    /// young collection.
    pub young_threshold: u32,

    /// The number of memory blocks that can be allocated before triggering a
    /// mature collection.
    pub mature_threshold: u32,

    /// The block allocation growth factor for the heap.
    pub heap_growth_factor: f64,

    /// The percentage of memory in the heap (relative to its threshold) that
    /// should be used before increasing the heap size.
    pub heap_growth_threshold: f64,

    /// When enabled, GC timings will be printed to STDERR.
    pub print_gc_timings: bool,
}

impl Config {
    pub fn new() -> Config {
        let cpu_count = num_cpus::get();

        Config {
            primary_threads: cpu_count,
            blocking_threads: cpu_count,
            gc_threads: cpu_count,
            tracer_threads: cpu_count,
            bytecode_threads: min(4, cpu_count),
            reductions: DEFAULT_REDUCTIONS,
            young_threshold: DEFAULT_YOUNG_THRESHOLD,
            mature_threshold: DEFAULT_MATURE_THRESHOLD,
            heap_growth_factor: DEFAULT_GROWTH_FACTOR,
            heap_growth_threshold: DEFAULT_GROWTH_THRESHOLD,
            print_gc_timings: false,
        }
    }

    /// Populates configuration settings based on environment variables.
    #[cfg_attr(
        feature = "cargo-clippy",
        allow(cyclomatic_complexity, cognitive_complexity)
    )]
    pub fn populate_from_env(&mut self) {
        set_from_env!(self, primary_threads, "PRIMARY_THREADS", usize);
        set_from_env!(self, blocking_threads, "BLOCKING_THREADS", usize);
        set_from_env!(self, gc_threads, "GC_THREADS", usize);
        set_from_env!(self, tracer_threads, "TRACER_THREADS", usize);
        set_from_env!(self, bytecode_threads, "BYTECODE_THREADS", usize);

        set_from_env!(self, reductions, "REDUCTIONS", usize);

        set_from_env!(self, young_threshold, "YOUNG_THRESHOLD", u32);
        set_from_env!(self, mature_threshold, "MATURE_THRESHOLD", u32);
        set_from_env!(self, heap_growth_factor, "HEAP_GROWTH_FACTOR", f64);

        set_from_env!(
            self,
            heap_growth_threshold,
            "HEAP_GROWTH_THRESHOLD",
            f64
        );

        set_from_env!(self, print_gc_timings, "PRINT_GC_TIMINGS", bool);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new() {
        let config = Config::new();

        assert!(config.primary_threads >= 1);
        assert!(config.gc_threads >= 1);
        assert_eq!(config.reductions, 1000);
    }

    #[test]
    fn test_populate_from_env() {
        env::set_var("INKO_PRIMARY_THREADS", "42");
        env::set_var("INKO_HEAP_GROWTH_FACTOR", "4.2");

        let mut config = Config::new();

        config.populate_from_env();

        // Unset before any assertions may fail.
        env::remove_var("INKO_HEAP_GROWTH_FACTOR");

        assert_eq!(config.primary_threads, 42);
        assert_eq!(config.heap_growth_factor, 4.2);
    }
}
