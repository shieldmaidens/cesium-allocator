use std::ffi::{
    c_char,
    c_void,
};

use cesium_libmimalloc_sys as mi;
use cesium_libmimalloc_sys::allocator::mi_free;
use mi::{
    heap::*,
    mi_block_visit_fun,
};

/// A general-purpose memory allocator. It's recommended to use the Allocator
/// Pool to manage allocator instances. It is important to be aware that `*mut
/// u8` return types are type-friendly wrappers on top of
/// [`libc::c_void`](libc::c_void), which is just a `void*` in C.
pub struct Allocator {
    id: u32,
    heap: *mut mi_heap_t,
}

impl Default for Allocator {
    /// Create an allocator that uses the default heap.
    ///
    /// Note: If called multiple times, it will contain the same reference to
    /// the same underlying heap. There are not multiple heaps.
    fn default() -> Self {
        Allocator {
            id: 0,
            heap: unsafe { mi_heap_get_default() },
        }
    }
}

impl Allocator {
    pub fn new(id: u32, heap: *mut mi_heap_t) -> Self {
        Allocator { id, heap }
    }

    pub fn id(self) -> u32 {
        self.id
    }

    /// Release outstanding resources in a specific heap.
    pub fn collect(&self, force: bool) {
        unsafe {
            mi_heap_collect(self.heap, force);
        }
    }

    /// Allocate `size` bytes.
    ///
    /// Returns pointer to the allocated memory or null if out of memory.
    /// Returns a unique pointer if called with `size` 0.
    pub fn malloc(&self, size: usize) -> *mut u8 {
        unsafe { mi_heap_malloc(self.heap, size) as *mut u8 }
    }

    pub fn free(&self, p: *mut u8) {
        unsafe { mi_free(p as *mut c_void) }
    }

    /// Allocate zero-initialized `size` bytes.
    ///
    /// Returns a pointer to newly allocated zero-initialized memory, or null if
    /// out of memory.
    pub fn zalloc(&self, size: usize) -> *mut u8 {
        unsafe { mi_heap_zalloc(self.heap, size) as *mut u8 }
    }

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `0` if `count * size` overflows or on out-of-memory.
    ///
    /// All items are initialized to zero.
    pub fn calloc(&self, count: usize, size: usize) -> *mut u8 {
        unsafe { mi_heap_calloc(self.heap, count, size) as *mut u8 }
    }

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `0` if `count * size` overflows or on out-of-memory,
    /// otherwise returns the same as [`malloc(count *
    /// size)`](Allocator::malloc). Equivalent to
    /// [`calloc`](Allocator::calloc), but returns uninitialized (and not
    /// zeroed) bytes.
    pub fn mallocn(&self, count: usize, size: usize) -> *mut u8 {
        unsafe { mi_heap_mallocn(self.heap, count, size) as *mut u8 }
    }

    /// Allocate an object of no more than [`SMALL_SIZE_MAX`](MI_SMALL_SIZE_MAX)
    /// bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`malloc_small`](Allocator::malloc_small) checks if
    /// `size` is small and calls this if
    /// so at runtime, so its' only worth using if you know for certain.
    pub fn malloc_small(&self, size: usize) -> *mut u8 {
        unsafe { mi_heap_malloc_small(self.heap, size) as *mut u8 }
    }

    /// Zero initialized re-allocation.
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`calloc`](Allocator::calloc),
    /// [`zalloc`](Allocator::zalloc),
    /// [`zalloc_aligned`](Allocator::zalloc_aligned), ...
    pub fn realloc(&self, p: *mut u8, newsize: usize) -> *mut u8 {
        unsafe { mi_heap_realloc(self.heap, p as *mut c_void, newsize) as *mut u8 }
    }

    /// Re-allocate memory to `count` elements of `size` bytes.
    ///
    /// The realloc equivalent of the [`mallocn`](Allocator::mallocn) interface.
    /// Returns `null` if `count * size` overflows or on out-of-memory,
    /// otherwise returns the same as [`realloc(p, count *
    /// size)`](Allocator::realloc).
    pub fn reallocn(&self, p: *mut u8, count: usize, size: usize) -> *mut u8 {
        unsafe { mi_heap_reallocn(self.heap, p as *mut c_void, count, size) as *mut u8 }
    }

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// This differs from [`realloc`](Allocator::realloc) in that on failure,
    /// `p` is freed.
    pub fn reallocf(&self, p: *mut u8, newsize: usize) -> *mut u8 {
        unsafe { mi_heap_reallocf(self.heap, p as *mut c_void, newsize) as *mut u8 }
    }

    /// Allocate and duplicate a nul-terminated C string. Because this could be
    /// either an i8 or u8, the original type is left unwrapped.
    pub fn strdup(&self, s: *const c_char) -> *mut c_char {
        unsafe { mi_heap_strdup(self.heap, s) }
    }

    /// Allocate and duplicate a nul-terminated C string, up to `n` bytes.
    /// Because this could be either an i8 or u8, the original type is left
    /// unwrapped.
    pub fn strndup(&self, s: *const c_char, n: usize) -> *mut c_char {
        unsafe { mi_heap_strndup(self.heap, s, n) }
    }

    /// Resolve a file path name, producing a `C` string which can be passed to
    /// [`free`](Allocator::free).
    ///
    /// `resolved_name` should be null, but can also point to a buffer of at
    /// least `PATH_MAX` bytes.
    ///
    /// If successful, returns a pointer to the resolved absolute file name, or
    /// `null` on failure (with `errno` set to the error code).
    ///
    /// If `resolved_name` was `null`, the returned result should be freed with
    /// [`free`](Allocator::free).
    ///
    /// This can rarely be useful in FFI code, but is mostly included for
    /// completeness.
    pub fn realpath(&self, fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char {
        unsafe { mi_heap_realpath(self.heap, fname, resolved_name) }
    }

    /// Allocate `size` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    ///
    /// Returns a unique pointer if called with `size` 0.
    pub fn malloc_aligned(&self, size: usize, alignment: usize) -> *mut u8 {
        unsafe { mi_heap_malloc_aligned(self.heap, size, alignment) as *mut u8 }
    }

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`.
    ///
    /// Note that the resulting pointer itself is not aligned by the alignment,
    /// but after `offset` bytes it will be. This can be useful for allocating
    /// data with an inline header, where the data has a specific alignment
    /// requirement.
    ///
    /// Specifically, if `p` is the returned pointer `p.add(offset)` is aligned
    /// to `alignment`.
    pub fn malloc_aligned_at(&self, size: usize, alignment: usize, offset: usize) -> *mut u8 {
        unsafe { mi_heap_malloc_aligned_at(self.heap, size, alignment, offset) as *mut u8 }
    }

    /// Allocate `size` bytes aligned by `alignment`, initialized to zero.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    ///
    /// Returns a unique pointer if called with `size` 0.
    pub fn zalloc_aligned(&self, size: usize, alignment: usize) -> *mut u8 {
        unsafe { mi_heap_zalloc_aligned(self.heap, size, alignment) as *mut u8 }
    }

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`,
    /// zero-initialized.
    ///
    /// This is a [`zalloc`](Allocator::zalloc) equivalent of
    /// [`malloc_aligned_at`](Allocator::malloc_aligned_at).
    pub fn zalloc_aligned_at(&self, size: usize, alignment: usize, offset: usize) -> *mut u8 {
        unsafe { mi_heap_zalloc_aligned_at(self.heap, size, alignment, offset) as *mut u8 }
    }

    /// Allocate `size * count` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory or if
    /// `size * count` overflows.
    ///
    /// Returns a unique pointer if called with `size * count` 0.
    pub fn calloc_aligned(&self, count: usize, size: usize, alignment: usize) -> *mut u8 {
        unsafe { mi_heap_calloc_aligned(self.heap, count, size, alignment) as *mut u8 }
    }

    /// Allocate `size * count` bytes aligned by `alignment` at a specified
    /// `offset`, zero-initialized.
    ///
    /// This is a [`calloc`](Allocator::calloc) equivalent of
    /// [`malloc_aligned_at`](Allocator::malloc_aligned_at).
    pub fn calloc_aligned_at(
        &self,
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut u8 {
        unsafe { mi_heap_calloc_aligned_at(self.heap, count, size, alignment, offset) as *mut u8 }
    }

    /// Re-allocate memory to `newsize` bytes, aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory. If null
    /// is returned, the pointer `p` is not freed. Otherwise the original
    /// pointer is either freed or returned as the reallocated result (in case
    /// it fits in-place with the new size).
    ///
    /// If `p` is null, it behaves as
    /// [`malloc_aligned`](Allocator::malloc_aligned). If `new_size` is
    /// larger than the original `size` allocated for `p`, the bytes after
    /// `size` are uninitialized.
    pub fn realloc_aligned(&self, p: *mut u8, new_size: usize, alignment: usize) -> *mut u8 {
        unsafe {
            mi_heap_realloc_aligned(self.heap, p as *mut c_void, new_size, alignment) as *mut u8
        }
    }

    /// Re-allocate memory to `newsize` bytes aligned by `alignment` at a
    /// specified `offset`.
    ///
    /// This is a [`realloc`](Allocator::realloc) equivalent of
    /// [`malloc_aligned_at`](Allocator::malloc_aligned_at).
    pub fn realloc_aligned_at(
        &self,
        p: *mut u8,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut u8 {
        unsafe {
            mi_heap_realloc_aligned_at(self.heap, p as *mut c_void, newsize, alignment, offset)
                as *mut u8
        }
    }

    /// Zero initialized [re-allocation](Allocator::realloc).
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`calloc`](Allocator::calloc),
    /// [`zalloc`](Allocator::zalloc),
    /// [`zalloc_aligned`](Allocator::zalloc_aligned), ...
    pub fn rezalloc(&self, p: *mut u8, newsize: usize) -> *mut u8 {
        unsafe { mi_heap_rezalloc(self.heap, p as *mut c_void, newsize) as *mut u8 }
    }

    /// Zero initialized [re-allocation](Allocator::realloc), following `calloc`
    /// paramater conventions.
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`calloc`](Allocator::calloc),
    /// [`zalloc`](Allocator::zalloc),
    /// [`zalloc_aligned`](Allocator::zalloc_aligned), ...
    pub fn recalloc(&self, p: *mut u8, newcount: usize, size: usize) -> *mut u8 {
        unsafe { mi_heap_recalloc(self.heap, p as *mut c_void, newcount, size) as *mut u8 }
    }

    /// Aligned version of [`rezalloc`](Allocator::rezalloc).
    pub fn rezalloc_aligned(&self, p: *mut u8, newsize: usize, alignment: usize) -> *mut u8 {
        unsafe {
            mi_heap_rezalloc_aligned(self.heap, p as *mut c_void, newsize, alignment) as *mut u8
        }
    }

    /// Offset-aligned version of [`rezalloc`](Allocator::rezalloc).
    pub fn rezalloc_aligned_at(
        &self,
        p: *mut u8,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut u8 {
        unsafe {
            mi_heap_rezalloc_aligned_at(self.heap, p as *mut c_void, newsize, alignment, offset)
                as *mut u8
        }
    }

    /// Aligned version of [`recalloc`](Allocator::recalloc).
    pub fn recalloc_aligned(
        &self,
        p: *mut u8,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut u8 {
        unsafe {
            mi_heap_recalloc_aligned(self.heap, p as *mut c_void, newcount, size, alignment)
                as *mut u8
        }
    }

    /// Offset-aligned version of [`recalloc`](Allocator::recalloc).
    pub fn recalloc_aligned_at(
        &self,
        p: *mut u8,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut u8 {
        unsafe {
            mi_heap_recalloc_aligned_at(
                self.heap,
                p as *mut c_void,
                newcount,
                size,
                alignment,
                offset,
            ) as *mut u8
        }
    }

    /// Does a heap contain a pointer to a previously allocated block?
    ///
    /// `p` must be a pointer to a previously allocated block (in any heap) --
    /// it cannot be some random pointer!
    ///
    /// Returns `true` if the block pointed to by `p` is in the `heap`.
    ///
    /// See [`check_owned`](Allocator::check_owned).
    pub fn contains_block(&self, p: *const u8) -> bool {
        unsafe { mi_heap_contains_block(self.heap, p as *const c_void) }
    }

    /// Check safely if any pointer is part of a heap.
    ///
    /// `p` may be any pointer -- not required to be previously allocated by the
    /// given heap or any other known heap. Returns `true` if `p` points to a
    /// block in the given heap, false otherwise.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    ///
    /// See [`contains_block`](Allocator::contains_block), [`get_default`], and
    /// [`is_in_region`]
    pub fn check_owned(&self, p: *const u8) -> bool {
        unsafe { mi_heap_check_owned(self.heap, p as *const c_void) }
    }

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
    pub fn visit_blocks(
        &self,
        visit_all_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut u8,
    ) -> bool {
        unsafe { mi_heap_visit_blocks(self.heap, visit_all_blocks, visitor, arg as *mut c_void) }
    }
}
