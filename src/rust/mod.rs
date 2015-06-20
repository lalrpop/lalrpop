//! Simple Rust AST. This is what the various code generators create,
//! which then gets serialized.

use intern;
use repr;

pub use repr::TypeRepr;
pub use repr::ActionFnDefn;

pub struct ModuleDefn {
    pub items: Vec<ItemDefn>,
}

pub enum ItemDefn {
    Use(Use),
    Module(ModuleDefn),
    Enum(EnumDefn),
    Fn(FnDefn),
    ActionFn(ActionFn, ActionFnDefn),
}

pub struct Use {
    pub path: Path, // use foo::bar [as baz]
    pub as_name: Option<InternedString>,
}

pub struct EnumDefn {
    pub name: InternedString,
    pub variants: Vec<VariantDefn>
}

pub struct StructDefn {
    pub fields: Vec<FieldDefn>
}

pub struct FieldDefn {
    pub name: InternedString,
    pub ty: TypeRepr,
}

pub struct VariantDefn {
    pub name: InternedString,
    pub arguments: Vec<TypeRepr>,
}

pub struct FnDefn {
    pub name: InternedString,
    pub type_parameters: Vec<TypeParameterDefn>,
    pub arg_patterns: Vec<InternedString>,
    pub arg_types: Vec<TypeRepr>,
    pub ret_type: TypeRepr,
    pub code: Block,
}

// <X: Bound0+Bound1>
pub struct TypeParameterDefn {
    pub name: InternedString,
    pub bounds: Vec<Bound>,
}

// Foo<X=Ty>
pub struct Bound {
    pub path: Path,
    pub assoc_bindings: Vec<AssocBinding>,
}

// X=Ty
pub struct AssocBinding {
    pub name: InternedString,
    pub ty: TypeRepr,
}

pub struct Block {
    pub statements: Vec<Statement>,
    pub tail: Option<Expr>
}

pub enum Statement {
    Expr(Expr),                       // X;
    Let(Pattern, Expr),               // let P = X;
}

pub enum Expr {
    Tuple(Vec<Expr>),                 // (), (X,), (X,Y), etc
    Field(Box<Expr>, InternedString), // X.f
    Return(Box<Expr>),                // return X;
    Match(Box<Expr>, Vec<Arm>),       // match X { ARM }
    Variable(InternedString),         // x
    Call(Box<Expr>, Vec<Expr>),       // X(Y,Z)
}

pub struct Arm {
    pattern: Pattern,
    body: Block,
}

pub struct Pattern {
    Variable(InternedString)
    Tuple(Vec<Pattern>),
    Variant(Path, InternedString, Option<Vec<Pattern>>), // Enum::Foo(A,B,C) or Enum::Foo(..)
}

pub type Path = Vec<InternedString>;
