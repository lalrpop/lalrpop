#[derive(Clone, Copy, Debug, PartialEq)]
struct UserErrorPrivate;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UserErrorCrate;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UserErrorPublic;

lalrpop_util::lalrpop_mod!(
    #[allow(unused)]
    user_defined_error_private
);
