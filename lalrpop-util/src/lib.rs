pub trait Token {
    /// The type of a "location" in the source. Often usize,
    /// indicating the byte offset within the file, but it might be
    /// anything else you like.
    type Loc;

    /// The type of the "enum" that the parser generator will use for
    /// matching.
    type Enum;

    /// Extract the start location of this token. That is, the
    /// position of the first character.
    fn to_start_loc(&self) -> Self::Loc;

    /// Extract the end location of this token. That is, the position
    /// of the last character (inclusive or exclusive, your call).
    fn to_end_loc(&self) -> Self::Loc;

    /// Convert the token into the enum that the parser generator will
    /// use for matching.
    fn as_enum(&self) -> &Self::Enum;
}
