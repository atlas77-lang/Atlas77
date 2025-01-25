use bumpalo::Bump;

pub struct CodeGenArena<'arena> {
    allocator: &'arena Bump,
}

impl<'arena> CodeGenArena<'arena> {
    pub fn new(bump: &'arena Bump) -> Self {
        Self { allocator: bump }
    }

    pub fn _alloc<T>(&self, v: T) -> &'arena mut T {
        self.allocator.alloc(v)
    }

    pub fn _alloc_ref_vec<T>(&self, v: Vec<&'arena T>) -> &'arena [&'arena T] {
        self.allocator.alloc_slice_fill_iter(v)
    }

    pub fn alloc_vec<T>(&self, v: Vec<T>) -> &'arena [&'arena T] {
        let iter = v.into_iter().map(|v| &*self.allocator.alloc(v));
        self.allocator.alloc_slice_fill_iter(iter)
    }
}
