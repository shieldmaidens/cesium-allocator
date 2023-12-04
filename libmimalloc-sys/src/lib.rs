// Copyright 2019 Octavian Oncescu, MIT License

#![allow(nonstandard_style)]

use core::ffi::c_void;
use std::ffi::{c_char, c_int, c_ulonglong};

use crate::heap::*;

pub mod options;
pub mod heap;
pub mod allocator;

/// The maximum number of bytes which may be used as an argument to a function
/// in the `_small` family ([`mi_malloc_small`](allocator::mi_malloc_small), [`mi_zalloc_small`](allocator::mi_zalloc_small), etc).
pub const MI_SMALL_SIZE_MAX: usize = 128 * core::mem::size_of::<*mut c_void>();

/// An output callback. Must be thread-safe.
///
/// See [`mi_stats_print_out`](allocator::mi_stats_print_out), [`mi_thread_stats_print_out`](allocator::mi_thread_stats_print_out), [`mi_register_output`](allocator::mi_register_output)
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;

/// Type of deferred free functions. Must be thread-safe.
///
/// - `force`: If true, all outstanding items should be freed.
/// - `heartbeat` A monotonically increasing count.
/// - `arg` Argument that was passed at registration to hold extra state.
///
/// See [`mi_register_deferred_free`](allocator::mi_register_deferred_free)
pub type mi_deferred_free_fun =
Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;

/// Type of error callback functions. Must be thread-safe.
///
/// - `err`: Error code (see [`mi_register_error`](allocator::mi_register_error) for a list).
/// - `arg`: Argument that was passed at registration to hold extra state.
///
/// See [`mi_register_error`](allocator::mi_register_error)
pub type mi_error_fun = Option<unsafe extern "C" fn(code: c_int, arg: *mut c_void)>;

/// Runtime options. All options are false by default.
pub type mi_option_t = c_int;

// Note: mimalloc doc website seems to have the order of show_stats and
// show_errors reversed as of 1.6.3, however what I have here is correct:
// https://github.com/microsoft/mimalloc/issues/266#issuecomment-653822341

/// Print error messages to `stderr`.
pub const mi_option_show_errors: mi_option_t = 0;

/// Print statistics to `stderr` when the program is done.
pub const mi_option_show_stats: mi_option_t = 1;

/// Print verbose messages to `stderr`.
pub const mi_option_verbose: mi_option_t = 2;

/// ### The following options are experimental

/// Option (experimental) Use large OS pages (2MiB in size) if possible.
///
/// Use large OS pages (2MiB) when available; for some workloads this can
/// significantly improve performance. Use mi_option_verbose to check if
/// the large OS pages are enabled -- usually one needs to explicitly allow
/// large OS pages (as on Windows and Linux). However, sometimes the OS is
/// very slow to reserve contiguous physical memory for large OS pages so
/// use with care on systems that can have fragmented memory (for that
/// reason, we generally recommend to use mi_option_reserve_huge_os_pages
/// instead whenever possible).
pub const mi_option_large_os_pages: mi_option_t = 6;

/// Option (experimental) The number of huge OS pages (1GiB in size) to reserve at the start of the program.
///
/// This reserves the huge pages at startup and sometimes this can give a large (latency) performance
/// improvement on big workloads. Usually it is better to not use MIMALLOC_LARGE_OS_PAGES in
/// combination with this setting. Just like large OS pages, use with care as reserving contiguous
/// physical memory can take a long time when memory is fragmented (but reserving the huge pages is
/// done at startup only once). Note that we usually need to explicitly enable huge OS pages (as on
/// Windows and Linux)). With huge OS pages, it may be beneficial to set the setting
/// mi_option_eager_commit_delay=N (N is 1 by default) to delay the initial N segments (of 4MiB) of
/// a thread to not allocate in the huge OS pages; this prevents threads that are short lived and
/// allocate just a little to take up space in the huge OS page area (which cannot be reset).
pub const mi_option_reserve_huge_os_pages: mi_option_t = 7;

/// Option (experimental) Reserve huge OS pages at node N.
///
/// The huge pages are usually allocated evenly among NUMA nodes.
/// You can use mi_option_reserve_huge_os_pages_at=N where `N` is the numa node (starting at 0) to allocate all
/// the huge pages at a specific numa node instead.
pub const mi_option_reserve_huge_os_pages_at: mi_option_t = 8;

/// Option (experimental) Reserve specified amount of OS memory at startup, e.g. "1g" or "512m".
pub const mi_option_reserve_os_memory: mi_option_t = 9;

/// Option (experimental) the first N segments per thread are not eagerly committed (=1).
pub const mi_option_eager_commit_delay: mi_option_t = 14;

/// Option (experimental) Pretend there are at most N NUMA nodes; Use 0 to use the actual detected NUMA nodes at runtime.
pub const mi_option_use_numa_nodes: mi_option_t = 16;

/// Option (experimental) If set to 1, do not use OS memory for allocation (but only pre-reserved arenas)
pub const mi_option_limit_os_alloc: mi_option_t = 17;

/// Option (experimental) OS tag to assign to mimalloc'd memory
pub const mi_option_os_tag: mi_option_t = 18;

/// Option (experimental)
pub const mi_option_max_errors: mi_option_t = 19;

/// Option (experimental)
pub const mi_option_max_warnings: mi_option_t = 20;

/// Option (experimental)
pub const mi_option_max_segment_reclaim: mi_option_t = 21;

/// Last option.
pub const _mi_option_last: mi_option_t = 26;

/// Visitor function passed to [`mi_heap_visit_blocks`]
///
/// Should return `true` to continue, and `false` to stop visiting (i.e. break)
///
/// This function is always first called for every `area` with `block` as a null
/// pointer. If `visit_all_blocks` was `true`, the function is then called for
/// every allocated block in that area.
pub type mi_block_visit_fun = Option<
    unsafe extern "C" fn(
        heap: *const mi_heap_t,
        area: *const mi_heap_area_t,
        block: *mut c_void,
        block_size: usize,
        arg: *mut c_void,
    ) -> bool,
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_stable_option() {
        unsafe {
            assert_eq!(mi_option_get(mi_option_show_errors), 0);
            mi_option_set(mi_option_show_errors, 1);
            assert_eq!(mi_option_get(mi_option_show_errors), 1);

            assert_eq!(mi_option_get(mi_option_show_stats), 0);
            mi_option_set(mi_option_show_stats, 1);
            assert_eq!(mi_option_get(mi_option_show_stats), 1);

            assert_eq!(mi_option_get(mi_option_verbose), 0);
            mi_option_set(mi_option_verbose, 1);
            assert_eq!(mi_option_get(mi_option_verbose), 1);
        }
    }
}