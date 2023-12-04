use std::ffi::{c_char, c_void};

use crate::{allocator, mi_block_visit_fun};

/// First-class heaps that can be destroyed in one go.
///
/// Note: The pointers allocated out of a heap can be be freed using
/// [`mi_free`](allocator::mi_free) -- there is no `mi_heap_free`.
///
/// # Example
///
/// ```
/// use cesium_libmimalloc_sys as mi;
/// unsafe {
///     let h = mi::mi_heap_new();
///     assert!(!h.is_null());
///     let p = mi::mi_heap_malloc(h, 50);
///     assert!(!p.is_null());
///
///     // use p...
///     mi::mi_free(p);
///
///     // Clean up the heap. Note that pointers allocated from `h`
///     // are *not* invalided by `mi_heap_delete`. You would have
///     // to use (the very dangerous) `mi_heap_destroy` for that
///     // behavior
///     mi::mi_heap_delete(h);
/// }
/// ```
pub enum mi_heap_t {}

/// An area of heap space contains blocks of a single size.
///
/// The bytes in freed blocks are `committed - used`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mi_heap_area_t {
    /// Start of the area containing heap blocks.
    pub blocks: *mut c_void,
    /// Bytes reserved for this area.
    pub reserved: usize,
    /// Current committed bytes of this area.
    pub committed: usize,
    /// Bytes in use by allocated blocks.
    pub used: usize,
    /// Size in bytes of one block.
    pub block_size: usize,
    /// Size in bytes of a full block including padding and metadata.
    pub full_block_size: usize,
}

extern "C" {
    /// Create a new heap that can be used for allocation.
    pub fn mi_heap_new() -> *mut mi_heap_t;

    /// Delete a previously allocated heap.
    ///
    /// This will release resources and migrate any still allocated blocks in
    /// this heap (efficienty) to the default heap.
    ///
    /// If `heap` is the default heap, the default heap is set to the backing
    /// heap.
    pub fn mi_heap_delete(heap: *mut mi_heap_t);

    /// Destroy a heap, freeing all its still allocated blocks.
    ///
    /// Use with care as this will free all blocks still allocated in the heap.
    /// However, this can be a very efficient way to free all heap memory in one
    /// go.
    ///
    /// If `heap` is the default heap, the default heap is set to the backing
    /// heap.
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);

    /// Set the default heap to use for [`mi_malloc`](allocator::mi_malloc) et al.
    ///
    /// Returns the previous default heap.
    pub fn mi_heap_set_default(heap: *mut mi_heap_t) -> *mut mi_heap_t;

    /// Get the default heap that is used for [`mi_malloc`](allocator::mi_malloc) et al.
    pub fn mi_heap_get_default() -> *mut mi_heap_t;

    /// Get the backing heap.
    ///
    /// The _backing_ heap is the initial default heap for a thread and always
    /// available for allocations. It cannot be destroyed or deleted except by
    /// exiting the thread.
    pub fn mi_heap_get_backing() -> *mut mi_heap_t;

    /// Release outstanding resources in a specific heap.
    ///
    /// See also [`mi_collect`](allocator::mi_collect).
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);

    /// Equivalent to [`mi_malloc`](allocator::mi_malloc), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_zalloc`](allocator::mi_zalloc), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_calloc`](allocator::mi_calloc), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_calloc(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_mallocn`](allocator::mi_mallocn), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_mallocn(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_malloc_small`](allocator::mi_malloc_small), but allocates out of the specific
    /// heap instead of the default.
    ///
    /// `size` must be smaller or equal to [`MI_SMALL_SIZE_MAX`](crate::MI_SMALL_SIZE_MAX).
    pub fn mi_heap_malloc_small(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_realloc`](allocator::mi_realloc), but allocates out of
    /// the specific heap instead of the default.
    pub fn mi_heap_realloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_reallocn`](allocator::mi_reallocn), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_reallocn(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        count: usize,
        size: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_reallocf`](allocator::mi_reallocf), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_reallocf(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_strdup`](allocator::mi_strdup), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_strdup(heap: *mut mi_heap_t, s: *const c_char) -> *mut c_char;

    /// Equivalent to [`mi_strndup`](allocator::mi_strndup), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_strndup(heap: *mut mi_heap_t, s: *const c_char, n: usize) -> *mut c_char;

    /// Equivalent to [`mi_realpath`](allocator::mi_realpath), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_realpath(
        heap: *mut mi_heap_t,
        fname: *const c_char,
        resolved_name: *mut c_char,
    ) -> *mut c_char;

    /// Equivalent to [`mi_malloc_aligned`](allocator::mi_malloc_aligned), but
    /// allocates out of the specific heap instead of the default.
    pub fn mi_heap_malloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_malloc_aligned_at`](allocator::mi_malloc_aligned_at), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_malloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_aligned`](allocator::mi_zalloc_aligned), but
    /// allocates out of the specific heap instead of the default.
    pub fn mi_heap_zalloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_aligned_at`](allocator::mi_zalloc_aligned_at), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_zalloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_calloc_aligned`](allocator::mi_calloc_aligned), but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_calloc_aligned(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_calloc_aligned_at`](allocator::mi_calloc_aligned_at), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_calloc_aligned_at(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_realloc_aligned`](allocator::mi_realloc_aligned), but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_realloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_realloc_aligned_at`](allocator::mi_realloc_aligned_at), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_realloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc`](allocator::mi_rezalloc), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_rezalloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_recalloc`](allocator::mi_recalloc), but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_recalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc_aligned`](allocator::mi_rezalloc_aligned), but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_rezalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc_aligned_at`](allocator::mi_rezalloc_aligned_at), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_rezalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_recalloc_aligned`](allocator::mi_realloc_aligned), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_recalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_recalloc_aligned_at`](allocator::mi_realloc_aligned_at), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_recalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Does a heap contain a pointer to a previously allocated block?
    ///
    /// `p` must be a pointer to a previously allocated block (in any heap) -- it cannot be some
    /// random pointer!
    ///
    /// Returns `true` if the block pointed to by `p` is in the `heap`.
    ///
    /// See [`mi_heap_check_owned`].
    pub fn mi_heap_contains_block(heap: *mut mi_heap_t, p: *const c_void) -> bool;

    /// Check safely if any pointer is part of a heap.
    ///
    /// `p` may be any pointer -- not required to be previously allocated by the
    /// given heap or any other mimalloc heap. Returns `true` if `p` points to a
    /// block in the given heap, false otherwise.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    ///
    /// See [`mi_heap_contains_block`], [`mi_heap_get_default`], and
    /// [`mi_is_in_heap_region`](allocator::mi_is_in_heap_region)
    pub fn mi_heap_check_owned(heap: *mut mi_heap_t, p: *const c_void) -> bool;

    /// Check safely if any pointer is part of the default heap of this thread.
    ///
    /// `p` may be any pointer -- not required to be previously allocated by the
    /// default heap for this thread, or any other mimalloc heap. Returns `true`
    /// if `p` points to a block in the default heap, false otherwise.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    ///
    /// See [`mi_heap_contains_block`], [`mi_heap_get_default`]
    pub fn mi_check_owned(p: *const c_void) -> bool;

    /// Visit all areas and blocks in `heap`.
    ///
    /// If `visit_all_blocks` is false, the `visitor` is only called once for
    /// every heap area. If it's true, the `visitor` is also called for every
    /// allocated block inside every area (with `!block.is_null()`). Return
    /// `false` from the `visitor` to return early.
    ///
    /// `arg` is an extra argument passed into the `visitor`.
    ///
    /// Returns `true` if all areas and blocks were visited.
    ///
    /// Passing a `None` visitor is allowed, and is a no-op.
    pub fn mi_heap_visit_blocks(
        heap: *const mi_heap_t,
        visit_all_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;
}
