use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    rc::Rc,
};

use bumpalo::Bump;

use super::ty::{
    HirBooleanTy, HirFloatTy, HirIntegerTy, HirTy, HirTyId, HirUninitializedTy, HirUnitTy,
    HirUnsignedIntTy,
};

pub(crate) struct HirArena<'arena> {
    allocator: Rc<Bump>,
    type_arena: TypeArena<'arena>,
    name_arena: HirNameArena<'arena>,
    phantom: PhantomData<&'arena ()>,
}

impl<'arena> HirArena<'arena> {
    pub(crate) fn new() -> Self {
        let allocator = Rc::new(Bump::new());
        Self {
            type_arena: TypeArena::new(allocator.clone()),
            name_arena: HirNameArena::new(allocator.clone()),
            allocator,
            phantom: PhantomData,
        }
    }

    pub fn intern<T>(&'arena self, v: T) -> &'arena mut T {
        self.allocator.alloc(v)
    }

    pub fn names(&'arena self) -> &'arena HirNameArena<'arena> {
        &self.name_arena
    }

    pub fn types(&'arena self) -> &'arena TypeArena<'arena> {
        &self.type_arena
    }
}

pub(crate) struct HirNameArena<'arena> {
    allocator: Rc<Bump>,
    intern: RefCell<HashSet<&'arena str>>,
}

impl<'arena> HirNameArena<'arena> {
    pub(crate) fn new(allocator: Rc<Bump>) -> Self {
        Self {
            allocator,
            intern: RefCell::new(HashSet::new()),
        }
    }

    pub(crate) fn get(&'arena self, name: &str) -> &'arena str {
        if let Some(interned) = self.intern.borrow().get(name) {
            return interned;
        }
        let id = self.allocator.alloc_str(name);
        self.intern.borrow_mut().insert(id);
        id
    }
}

pub(crate) struct TypeArena<'arena> {
    allocator: Rc<Bump>,
    intern: RefCell<HashMap<HirTyId, &'arena HirTy<'arena>>>,
}

impl<'arena> TypeArena<'arena> {
    pub(crate) fn new(allocator: Rc<Bump>) -> Self {
        Self {
            allocator,
            intern: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_type(&'arena self, id: HirTyId) -> Option<&'arena HirTy<'arena>> {
        self.intern.borrow().get(&id).copied()
    }

    pub fn get_integer64_ty(&'arena self) -> &'arena HirTy<'arena> {
        let id = HirTyId::compute_integer64_ty_id();
        self.intern
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| self.allocator.alloc(HirTy::Int64(HirIntegerTy {})))
    }

    pub fn get_float64_ty(&'arena self) -> &'arena HirTy<'arena> {
        let id = HirTyId::compute_float64_ty_id();
        self.intern
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| self.allocator.alloc(HirTy::Float64(HirFloatTy {})))
    }

    pub fn get_uint64_ty(&'arena self) -> &'arena HirTy<'arena> {
        let id = HirTyId::compute_uint64_ty_id();
        self.intern
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| self.allocator.alloc(HirTy::UInt64(HirUnsignedIntTy {})))
    }

    pub fn get_boolean_ty(&'arena self) -> &'arena HirTy<'arena> {
        let id = HirTyId::compute_boolean_ty_id();
        self.intern
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| self.allocator.alloc(HirTy::Boolean(HirBooleanTy {})))
    }

    pub fn get_unit_ty(&'arena self) -> &'arena HirTy<'arena> {
        let id = HirTyId::compute_unit_ty_id();
        self.intern
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| self.allocator.alloc(HirTy::Unit(HirUnitTy {})))
    }

    pub fn get_uninitialized_ty(&'arena self) -> &'arena HirTy<'arena> {
        let id = HirTyId::compute_uninitialized_ty_id();
        self.intern.borrow_mut().entry(id).or_insert_with(|| {
            self.allocator
                .alloc(HirTy::Uninitialized(HirUninitializedTy {}))
        })
    }
}
