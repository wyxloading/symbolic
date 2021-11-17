//! The raw SymCache binary file format internals.
//!
//! TODO: actually write some docs ;-)

/// The magic file preamble as individual bytes.
use crate::{Index, LineNumber, RelativeAddress};

const SYMCACHE_MAGIC_BYTES: [u8; 4] = *b"SYMC";

/// The magic file preamble to identify SymCache files.
///
/// Serialized as ASCII "SYMC" on little-endian (x64) systems.
pub const SYMCACHE_MAGIC: u32 = u32::from_be_bytes(SYMCACHE_MAGIC_BYTES);
/// The byte-flipped magic, which indicates an endianness mismatch.
pub const SYMCACHE_MAGIC_FLIPPED: u32 = SYMCACHE_MAGIC.swap_bytes();

/// The latest version of the file format.
pub const SYMCACHE_VERSION: u32 = 1_000;

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    /// The file magic representing the file format and endianness.
    pub magic: u32,
    /// The SymCache Format Version.
    pub version: u32,
    /// Number of included [`String`]s.
    pub num_strings: u32,
    /// Number of included [`File`]s.
    pub num_files: u32,
    /// Number of included [`Function`]s.
    pub num_functions: u32,
    /// Number of included [`SourceLocation`]s.
    pub num_source_locations: u32,
    /// Number of included [`Range`]s.
    pub num_ranges: u32,
    /// Total number of bytes used for string data.
    pub string_bytes: u32,

    pub range_threshold: u64,
}

/// Serialized Function metadata in the SymCache.
#[derive(Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Function {
    /// The functions name (reference to a [`String`]).
    pub name_idx: Index,
    /// The first address covered by this function.
    pub entry_pc: Option<RelativeAddress>,
    /// The language of the function.
    pub lang: u8,
}

/// Serialized File in the SymCache.
#[derive(Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct File {
    /// The optional compilation directory prefix (reference to a [`String`]).
    pub comp_dir_idx: Option<Index>,
    /// The optional directory prefix (reference to a [`String`]).
    pub directory_idx: Option<Index>,
    /// The file path (reference to a [`String`]).
    pub path_name_idx: Option<Index>,
}

/// A location in a source file, comprising a file, a line, a function, and
/// the index of the source location this was inlined into, if any.
///
/// Note that each time a function is inlined, as well as the non-inlined
/// version of the function, is represented by a distinct `SourceLocation`.
/// These `SourceLocation`s will all point to the same file, line, and function,
/// but have different inline information.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct SourceLocation {
    /// The optional source file (reference to a [`File`]).
    pub file_idx: Option<Index>,
    /// The line number.
    pub line: Option<LineNumber>,
    /// The function (reference to a [`Function`]).
    pub function_idx: Option<Index>,
    /// The caller source location in case this location was inlined
    /// (reference to another [`SourceLocation`]).
    pub inlined_into_idx: Option<Index>,
}

/// Serialized String in the SymCache.
#[derive(Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct String {
    /// The offset into the `string_bytes`.
    pub string_offset: u32,
    /// Length of the string.
    pub string_len: u32,
}

/// A representation of a code range in the SymCache.
///
/// We only save the start address, the end is implicitly given
/// by the next range's start.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Range(pub RelativeAddress);

/// Returns the amount left to add to the remainder to get 8 if
/// `to_align` isn't a multiple of 8.
pub fn align_to_eight(to_align: usize) -> usize {
    let remainder = to_align % 8;
    if remainder == 0 {
        remainder
    } else {
        8 - remainder
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_sizeof() {
        assert_eq!(mem::size_of::<Header>(), 32);
        assert_eq!(mem::align_of::<Header>(), 4);

        assert_eq!(mem::size_of::<Function>(), 12);
        assert_eq!(mem::align_of::<Function>(), 4);

        assert_eq!(mem::size_of::<File>(), 12);
        assert_eq!(mem::align_of::<File>(), 4);

        assert_eq!(mem::size_of::<SourceLocation>(), 16);
        assert_eq!(mem::align_of::<SourceLocation>(), 4);

        assert_eq!(mem::size_of::<String>(), 8);
        assert_eq!(mem::align_of::<String>(), 4);

        assert_eq!(mem::size_of::<Range>(), 4);
        assert_eq!(mem::align_of::<Range>(), 4);
    }
}
