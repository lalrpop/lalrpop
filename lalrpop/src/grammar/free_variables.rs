use grammar::parse_tree::{self, TypeParameter};
use grammar::repr;

/// Finds the set of "free variables" in something -- that is, the
/// type/lifetime parameters that appear and are not bound. For
/// example, `T: Foo<U>` would return `[T, U]`.
pub trait FreeVariables {
    fn free_variables(&self) -> Vec<TypeParameter>;
}

impl<T: FreeVariables> FreeVariables for Option<T> {
    fn free_variables(&self) -> Vec<TypeParameter> {
        match self {
            None => vec![],
            Some(t) => t.free_variables(),
        }
    }
}

impl<T: FreeVariables> FreeVariables for Vec<T> {
    fn free_variables(&self) -> Vec<TypeParameter> {
        self.into_iter()
            .flat_map(|e| e.free_variables())
            .collect()
    }
}

impl FreeVariables for repr::TypeRepr {
    fn free_variables(&self) -> Vec<TypeParameter> {
        match self {
            repr::TypeRepr::Tuple(tys) => tys.free_variables(),
            repr::TypeRepr::Nominal(data) => data.free_variables(),
            repr::TypeRepr::Associated {
                type_parameter, id: _
            } => vec![TypeParameter::Id(type_parameter.clone())],
            repr::TypeRepr::Lifetime(l) => vec![TypeParameter::Lifetime(l.clone())],
            repr::TypeRepr::Ref {
                lifetime,
                mutable: _,
                referent,
            } => lifetime
                .iter()
                .map(|id| TypeParameter::Lifetime(id.clone()))
                .chain(referent.free_variables())
                .collect(),
        }
    }
}

impl FreeVariables for parse_tree::Path {
    fn free_variables(&self) -> Vec<TypeParameter> {
        // A path like `foo::Bar` is considered no free variables; a
        // single identifier like `T` is a free variable `T`. Note
        // that we can't distinguish type parameters from random names
        // like `String`.
        match self.as_id() {
            Some(id) => vec![TypeParameter::Id(id)],
            None => vec![],
        }
    }
}

impl FreeVariables for repr::NominalTypeRepr {
    fn free_variables(&self) -> Vec<TypeParameter> {
        let repr::NominalTypeRepr { path, types } = self;
        path.free_variables()
            .into_iter()
            .chain(types.free_variables())
            .collect()
    }
}
