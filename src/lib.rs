mod allocator;

use std::{
    collections::BTreeMap,
    sync::Arc,
};

use cesium_libmimalloc_sys::heap::mi_heap_new;

use crate::allocator::Allocator;

pub struct AllocatorPool {
    lowest_id: u32,
    heaps: BTreeMap<u32, Arc<Allocator>>,
}

/// A pool of general allocators.
impl AllocatorPool {
    pub fn new() -> Self {
        AllocatorPool {
            lowest_id: 0,
            heaps: BTreeMap::new(),
        }
    }

    /// Create a new allocator
    pub fn new_allocator(&mut self) -> Arc<Allocator> {
        let heap = unsafe { mi_heap_new() };
        let id = self.lowest_id + 1;
        self.lowest_id = id;

        let alloc = Arc::new(Allocator::new(id, heap));
        self.heaps.insert(id, alloc.clone());

        alloc
    }

    /// Gets or creates an allocator
    pub fn get_allocator(&mut self, id: u32, create: Option<bool>) -> Option<Arc<Allocator>> {
        match self.heaps.get(&id) {
            | None => match create {
                | None => None,
                | Some(v) => Some(self.new_allocator()),
            },
            | Some(v) => Some(v.clone()),
        }
    }
}
