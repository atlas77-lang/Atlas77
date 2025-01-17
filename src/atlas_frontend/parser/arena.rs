use bumpalo::Bump;

/// An arena allocator for AST nodes.
///
/// The AST is a fully immutable representation of the input source, so we can safely intern all
/// nodes in the AST. That also means we can drop the entire AST at the end of AST lowering.
pub(crate) struct AstArena<'arena> {
    allocator: &'arena Bump,
}

impl<'arena> AstArena<'arena> {
    pub(crate) fn new(bump: &'arena Bump) -> Self {
        Self { allocator: bump }
    }

    pub(crate) fn alloc<T>(&self, v: T) -> &'arena mut T {
        self.allocator.alloc(v)
    }

    /// Intern a slice of already-interned values
    pub(crate) fn _alloc_ref_vec<T>(&self, v: Vec<&'arena T>) -> &'arena [&'arena T] {
        self.allocator.alloc_slice_fill_iter(v)
    }

    /// Intern a slice of values and intern them
    pub(crate) fn alloc_vec<T>(&self, v: Vec<T>) -> &'arena [&'arena T] {
        let iter = v.into_iter().map(|v| &*self.allocator.alloc(v));
        self.allocator.alloc_slice_fill_iter(iter)
    }
}
