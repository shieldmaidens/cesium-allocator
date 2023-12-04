use std::ffi::{c_char, c_int, c_void};

use crate::{allocator, mi_deferred_free_fun, mi_error_fun, mi_output_fun, MI_SMALL_SIZE_MAX};

extern "C" {
    /// Allocate zero-initialized `size` bytes.
    ///
    /// Returns a pointer to newly allocated zero-initialized memory, or null if
    /// out of memory.
    pub fn mi_zalloc(size: usize) -> *mut c_void;

    /// Allocate `size` bytes.
    ///
    /// Returns pointer to the allocated memory or null if out of memory.
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_malloc(size: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// Return pointer to the allocated memory or null if out of memory. If null
    /// is returned, the pointer `p` is not freed. Otherwise the original
    /// pointer is either freed or returned as the reallocated result (in case
    /// it fits in-place with the new size).
    ///
    /// If `p` is null, it behaves as [`mi_malloc`]. If `newsize` is larger than
    /// the original `size` allocated for `p`, the bytes after `size` are
    /// uninitialized.
    pub fn mi_realloc(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment`, initialized to zero.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    ///
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_zalloc_aligned(size: usize, alignment: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    ///
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes, aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory. If null
    /// is returned, the pointer `p` is not freed. Otherwise the original
    /// pointer is either freed or returned as the reallocated result (in case
    /// it fits in-place with the new size).
    ///
    /// If `p` is null, it behaves as [`mi_malloc_aligned`]. If `newsize` is
    /// larger than the original `size` allocated for `p`, the bytes after
    /// `size` are uninitialized.
    pub fn mi_realloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;

    /// Free previously allocated memory.
    ///
    /// The pointer `p` must have been allocated before (or be null).
    pub fn mi_free(p: *mut c_void);

    /// Return the available bytes in a memory block.
    ///
    /// The returned size can be used to call `mi_expand` successfully.
    pub fn mi_usable_size(p: *const c_void) -> usize;

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory.
    ///
    /// All items are initialized to zero.
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory,
    /// otherwise returns the same as [`mi_malloc(count * size)`].
    /// Equivalent to [`mi_calloc`], but returns uninitialized (and not zeroed)
    /// bytes.
    pub fn mi_mallocn(count: usize, size: usize) -> *mut c_void;

    /// Re-allocate memory to `count` elements of `size` bytes.
    ///
    /// The realloc equivalent of the [`mi_mallocn`] interface. Returns `null`
    /// if `count * size` overflows or on out-of-memory, otherwise returns the
    /// same as [`mi_realloc(p, count * size)`].
    pub fn mi_reallocn(p: *mut c_void, count: usize, size: usize) -> *mut c_void;

    /// Try to re-allocate memory to `newsize` bytes _in place_.
    ///
    /// Returns null on out-of-memory or if the memory could not be expanded in
    /// place. On success, returns the same pointer as `p`.
    ///
    /// If `newsize` is larger than the original `size` allocated for `p`, the
    /// bytes after `size` are uninitialized.
    ///
    /// If null is returned, the original pointer is not freed.
    ///
    /// Note: Conceptually, this is a realloc-like which returns null if it
    /// would be forced to reallocate memory and copy. In practice it's
    /// equivalent testing against [`mi_usable_size`].
    pub fn mi_expand(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// This differs from [`mi_realloc`] in that on failure,
    /// `p` is freed.
    pub fn mi_reallocf(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Allocate and duplicate a nul-terminated C string.
    ///
    /// This can be useful for Rust code when interacting with the FFI.
    pub fn mi_strdup(s: *const c_char) -> *mut c_char;

    /// Allocate and duplicate a nul-terminated C string, up to `n` bytes.
    ///
    /// This can be useful for Rust code when interacting with the FFI.
    pub fn mi_strndup(s: *const c_char, n: usize) -> *mut c_char;

    /// Resolve a file path name, producing a `C` string which can be passed to
    /// [`mi_free`].
    ///
    /// `resolved_name` should be null, but can also point to a buffer of at
    /// least `PATH_MAX` bytes.
    ///
    /// If successful, returns a pointer to the resolved absolute file name, or
    /// `null` on failure (with `errno` set to the error code).
    ///
    /// If `resolved_name` was `null`, the returned result should be freed with
    /// [`mi_free`].
    ///
    /// This can rarely be useful in FFI code, but is mostly included for
    /// completeness.
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;

    /// Allocate `size * count` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory or if
    /// `size * count` overflows.
    ///
    /// Returns a unique pointer if called with `size * count` 0.
    pub fn mi_calloc_aligned(count: usize, size: usize, alignment: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`.
    ///
    /// Note that the resulting pointer itself is not aligned by the alignment,
    /// but after `offset` bytes it will be. This can be useful for allocating
    /// data with an inline header, where the data has a specific alignment
    /// requirement.
    ///
    /// Specifically, if `p` is the returned pointer `p.add(offset)` is aligned
    /// to `alignment`.
    pub fn mi_malloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`,
    /// zero-initialized.
    ///
    /// This is a [`mi_zalloc`] equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_zalloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;

    /// Allocate `size` of bytes aligned by `alignment` and place the address of the
    /// allocated memory to `ptr`.
    ///
    /// Returns zero on success, invalid argument for invalid alignment, or out-of-memory.
    pub fn mi_posix_memalign(ptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;

    /// Allocate `size` bytes aligned by `alignment` with alignment as the first
    /// parameter.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    pub fn mi_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;

    /// Allocate `size * count` bytes aligned by `alignment` at a specified
    /// `offset`, zero-initialized.
    ///
    /// This is a [`calloc`](mi_calloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_calloc_aligned_at(
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes aligned by `alignment` at a
    /// specified `offset`.
    ///
    /// This is a [`realloc`](mi_realloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_realloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Zero initialized [re-allocation](allocator::mi_realloc).
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`mi_calloc`],
    /// [`mi_zalloc`],
    /// [`mi_zalloc_aligned`], ...
    pub fn mi_rezalloc(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Zero initialized [re-allocation](allocator::mi_realloc), following `calloc`
    /// paramater conventions.
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`mi_calloc`],
    /// [`mi_zalloc`],
    /// [`mi_zalloc_aligned`], ...
    pub fn mi_recalloc(p: *mut c_void, newcount: usize, size: usize) -> *mut c_void;

    /// Aligned version of [`mi_rezalloc`].
    pub fn mi_rezalloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;

    /// Offset-aligned version of [`mi_rezalloc`].
    pub fn mi_rezalloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Aligned version of [`mi_recalloc`].
    pub fn mi_recalloc_aligned(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Offset-aligned version of [`mi_recalloc`].
    pub fn mi_recalloc_aligned_at(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Allocate an object of no more than [`MI_SMALL_SIZE_MAX`] bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`mi_malloc`] checks if `size` is
    /// small and calls this if
    /// so at runtime, so its' only worth using if you know for certain.
    pub fn mi_malloc_small(size: usize) -> *mut c_void;

    /// Allocate an zero-initialized object of no more than
    /// [`MI_SMALL_SIZE_MAX`] bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`mi_zalloc`] checks if `size` is
    /// small and calls this if so at runtime, so its' only worth using if you
    /// know for certain.
    pub fn mi_zalloc_small(size: usize) -> *mut c_void;

    /// Return the used allocation size.
    ///
    /// Returns the size `n` that will be allocated, where `n >= size`.
    ///
    /// Generally, `mi_usable_size(mi_malloc(size)) == mi_good_size(size)`. This
    /// can be used to reduce internal wasted space when allocating buffers for
    /// example.
    ///
    /// See [`mi_usable_size`].
    pub fn mi_good_size(size: usize) -> usize;

    /// Eagerly free memory.
    ///
    /// If `force` is true, aggressively return memory to the OS (can be
    /// expensive!)
    ///
    /// Regular code should not have to call this function. It can be beneficial
    /// in very narrow circumstances; in particular, when a long running thread
    /// allocates a lot of blocks that are freed by other threads it may improve
    /// resource usage by calling this every once in a while.
    pub fn mi_collect(force: bool);

    /// Checked free: If `p` came from mimalloc's heap (as decided by
    /// [`mi_is_in_heap_region`]), this is [`mi_free(p)`], but
    /// otherwise it is a no-op.
    pub fn mi_cfree(p: *mut c_void);

    /// Returns true if this is a pointer into a memory region that has been
    /// reserved by the mimalloc heap.
    ///
    /// This function is described by the mimalloc documentation as "relatively
    /// fast".
    ///
    /// See also [`mi_heap_check_owned`](allocator::mi_is_in_heap_region), which is (much) slower and slightly
    /// more precise, but only concerns a single `mi_heap`.
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;

    /// Layout-aware deallocation: Like [`mi_free`], but accepts
    /// the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function, and you are not required
    /// to use this to deallocate memory from an aligned allocation function.
    pub fn mi_free_size_aligned(p: *mut c_void, size: usize, alignment: usize);

    /// Size-aware deallocation: Like [`mi_free`], but accepts
    /// the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function.
    pub fn mi_free_size(p: *mut c_void, size: usize);

    /// Alignment-aware deallocation: Like [`mi_free`], but
    /// accepts the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function.
    pub fn mi_free_aligned(p: *mut c_void, alignment: usize);

    /// Print the main statistics.
    ///
    /// Ignores the passed in argument, and outputs to the registered output
    /// function or stderr by default.
    ///
    /// Most detailed when using a debug build.
    pub fn mi_stats_print(_: *mut c_void);

    /// Print the main statistics.
    ///
    /// Pass `None` for `out` to use the default. If `out` is provided, `arc` is
    /// passed as it's second parameter.
    ///
    /// Most detailed when using a debug build.
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);

    /// Reset statistics.
    ///
    /// Note: This function is thread safe.
    pub fn mi_stats_reset();

    /// Merge thread local statistics with the main statistics and reset.
    ///
    /// Note: This function is thread safe.
    pub fn mi_stats_merge();

    /// Return the mimalloc version number.
    ///
    /// For example version 1.6.3 would return the number `163`.
    pub fn mi_version() -> c_int;

    /// Initialize mimalloc on a thread.
    ///
    /// Should not be used as on most systems (pthreads, windows) this is done
    /// automatically.
    pub fn mi_thread_init();

    /// Initialize the process.
    ///
    /// Should not be used on most systems, as it's called by thread_init or the
    /// process loader.
    pub fn mi_process_init();

    /// Return process information (time and memory usage). All parameters are
    /// optional (nullable) out-params:
    ///
    /// | Parameter        | Description |
    /// | :-               | :- |
    /// | `elapsed_msecs`  | Elapsed wall-clock time of the process in milli-seconds. |
    /// | `user_msecs`     | User time in milli-seconds (as the sum over all threads). |
    /// | `system_msecs`   | System time in milli-seconds. |
    /// | `current_rss`    | Current working set size (touched pages). |
    /// | `peak_rss`       | Peak working set size (touched pages). |
    /// | `current_commit` | Current committed memory (backed by the page file). |
    /// | `peak_commit`    | Peak committed memory (backed by the page file). |
    /// | `page_faults`    | Count of hard page faults. |
    ///
    /// The `current_rss` is precise on Windows and MacOSX; other systems
    /// estimate this using `current_commit`. The `commit` is precise on Windows
    /// but estimated on other systems as the amount of read/write accessible
    /// memory reserved by mimalloc.
    pub fn mi_process_info(
        elapsed_msecs: *mut usize,
        user_msecs: *mut usize,
        system_msecs: *mut usize,
        current_rss: *mut usize,
        peak_rss: *mut usize,
        current_commit: *mut usize,
        peak_commit: *mut usize,
        page_faults: *mut usize,
    );

    /// Uninitialize mimalloc on a thread.
    ///
    /// Should not be used as on most systems (pthreads, windows) this is done
    /// automatically. Ensures that any memory that is not freed yet (but will
    /// be freed by other threads in the future) is properly handled.
    ///
    /// Note: This function is thread safe.
    pub fn mi_thread_done();

    /// Print out heap statistics for this thread.
    ///
    /// Pass `None` for `out` to use the default. If `out` is provided, `arc` is
    /// passed as it's second parameter
    ///
    /// Most detailed when using a debug build.
    ///
    /// Note: This function is thread safe.
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);

    /// Register an output function.
    ///
    /// - `out` The output function, use `None` to output to stderr.
    /// - `arg` Argument that will be passed on to the output function.
    ///
    /// The `out` function is called to output any information from mimalloc,
    /// like verbose or warning messages.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);

    /// Register a deferred free function.
    ///
    /// - `deferred_free` Address of a deferred free-ing function or `None` to
    ///   unregister.
    /// - `arg` Argument that will be passed on to the deferred free function.
    ///
    /// Some runtime systems use deferred free-ing, for example when using
    /// reference counting to limit the worst case free time.
    ///
    /// Such systems can register (re-entrant) deferred free function to free
    /// more memory on demand.
    ///
    /// - When the `force` parameter is `true` all possible memory should be
    ///   freed.
    ///
    /// - The per-thread `heartbeat` parameter is monotonically increasing and
    ///   guaranteed to be deterministic if the program allocates
    ///   deterministically.
    ///
    /// - The `deferred_free` function is guaranteed to be called
    ///   deterministically after some number of allocations (regardless of
    ///   freeing or available free memory).
    ///
    /// At most one `deferred_free` function can be active.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_deferred_free(out: mi_deferred_free_fun, arg: *mut c_void);

    /// Register an error callback function.
    ///
    /// The `errfun` function is called on an error in mimalloc after emitting
    /// an error message (through the output function).
    ///
    /// It as always legal to just return from the `errfun` function in which
    /// case allocation functions generally return null or ignore the condition.
    ///
    /// The default function only calls abort() when compiled in secure mode
    /// with an `EFAULT` error. The possible error codes are:
    ///
    /// - `EAGAIN` (11): Double free was detected (only in debug and secure
    ///   mode).
    /// - `EFAULT` (14): Corrupted free list or meta-data was detected (only in
    ///   debug and secure mode).
    /// - `ENOMEM` (12): Not enough memory available to satisfy the request.
    /// - `EOVERFLOW` (75): Too large a request, for example in `mi_calloc`, the
    ///   `count` and `size` parameters are too large.
    /// - `EINVAL` (22): Trying to free or re-allocate an invalid pointer.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_error(out: mi_error_fun, arg: *mut c_void);
}
