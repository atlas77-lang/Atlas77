//The first view versions of the type checking will be quite simple.
//As there will only be primitive types to check.
//A rework of the type checker will be done when structs, classes, enums and unions are added.

use super::arena::TypeArena;

pub(crate) struct TypeChecker<'hir> {
    arena_ty: TypeArena<'hir>,
}

impl<'hir> TypeChecker<'hir> {
    pub fn new(arena_ty: TypeArena<'hir>) -> Self {
        Self { arena_ty }
    }
}
