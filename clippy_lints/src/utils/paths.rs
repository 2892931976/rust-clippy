// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains paths to types and functions Clippy needs to know
//! about.

pub const ANY_TRAIT: [&str; 3] = ["std", "any", "Any"];
pub const ARC: [&str; 3] = ["alloc", "sync", "Arc"];
pub const ASMUT_TRAIT: [&str; 3] = ["core", "convert", "AsMut"];
pub const ASREF_TRAIT: [&str; 3] = ["core", "convert", "AsRef"];
pub const BEGIN_PANIC: [&str; 3] = ["std", "panicking", "begin_panic"];
pub const BEGIN_PANIC_FMT: [&str; 3] = ["std", "panicking", "begin_panic_fmt"];
pub const BINARY_HEAP: [&str; 4] = ["alloc", "collections", "binary_heap", "BinaryHeap"];
pub const BORROW_TRAIT: [&str; 3] = ["core", "borrow", "Borrow"];
pub const BTREEMAP: [&str; 5] = ["alloc", "collections", "btree", "map", "BTreeMap"];
pub const BTREEMAP_ENTRY: [&str; 5] = ["alloc", "collections", "btree", "map", "Entry"];
pub const BTREESET: [&str; 5] = ["alloc", "collections", "btree", "set", "BTreeSet"];
pub const CLONE_TRAIT: [&str; 3] = ["core", "clone", "Clone"];
pub const CLONE_TRAIT_METHOD: [&str; 4] = ["core", "clone", "Clone", "clone"];
pub const CMP_MAX: [&str; 3] = ["core", "cmp", "max"];
pub const CMP_MIN: [&str; 3] = ["core", "cmp", "min"];
pub const COW: [&str; 3] = ["alloc", "borrow", "Cow"];
pub const CSTRING_NEW: [&str; 5] = ["std", "ffi", "c_str", "CString", "new"];
pub const C_VOID: [&str; 3] = ["core", "ffi", "c_void"];
pub const C_VOID_LIBC: [&str; 2] = ["libc", "c_void"];
pub const DEFAULT_TRAIT: [&str; 3] = ["core", "default", "Default"];
pub const DEFAULT_TRAIT_METHOD: [&str; 4] = ["core", "default", "Default", "default"];
pub const DEREF_TRAIT_METHOD: [&str; 5] = ["core", "ops", "deref", "Deref", "deref"];
pub const DISPLAY_FMT_METHOD: [&str; 4] = ["core", "fmt", "Display", "fmt"];
pub const DOUBLE_ENDED_ITERATOR: [&str; 4] = ["core", "iter", "traits", "DoubleEndedIterator"];
pub const DROP: [&str; 3] = ["core", "mem", "drop"];
pub const DURATION: [&str; 3] = ["core", "time", "Duration"];
pub const EARLY_CONTEXT: [&str; 4] = ["rustc", "lint", "context", "EarlyContext"];
pub const FMT_ARGUMENTS_NEWV1FORMATTED: [&str; 4] = ["core", "fmt", "Arguments", "new_v1_formatted"];
pub const FROM_FROM: [&str; 4] = ["core", "convert", "From", "from"];
pub const FROM_TRAIT: [&str; 3] = ["core", "convert", "From"];
pub const HASH: [&str; 2] = ["hash", "Hash"];
pub const HASHMAP: [&str; 5] = ["std", "collections", "hash", "map", "HashMap"];
pub const HASHMAP_ENTRY: [&str; 5] = ["std", "collections", "hash", "map", "Entry"];
pub const HASHSET: [&str; 5] = ["std", "collections", "hash", "set", "HashSet"];
pub const INDEX: [&str; 3] = ["core", "ops", "Index"];
pub const INDEX_MUT: [&str; 3] = ["core", "ops", "IndexMut"];
pub const INIT: [&str; 4] = ["core", "intrinsics", "", "init"];
pub const INTO: [&str; 3] = ["core", "convert", "Into"];
pub const INTO_ITERATOR: [&str; 4] = ["core", "iter", "traits", "IntoIterator"];
pub const IO_READ: [&str; 3] = ["std", "io", "Read"];
pub const IO_WRITE: [&str; 3] = ["std", "io", "Write"];
pub const ITERATOR: [&str; 4] = ["core", "iter", "iterator", "Iterator"];
pub const LATE_CONTEXT: [&str; 4] = ["rustc", "lint", "context", "LateContext"];
pub const LINKED_LIST: [&str; 4] = ["alloc", "collections", "linked_list", "LinkedList"];
pub const LINT: [&str; 3] = ["rustc", "lint", "Lint"];
pub const LINT_PASS: [&str; 3] = ["rustc", "lint", "LintPass"];
pub const MEM_DISCRIMINANT: [&str; 3] = ["core", "mem", "discriminant"];
pub const MEM_FORGET: [&str; 3] = ["core", "mem", "forget"];
pub const MEM_REPLACE: [&str; 3] = ["core", "mem", "replace"];
pub const MEM_UNINIT: [&str; 3] = ["core", "mem", "uninitialized"];
pub const MEM_ZEROED: [&str; 3] = ["core", "mem", "zeroed"];
pub const MUTEX: [&str; 4] = ["std", "sync", "mutex", "Mutex"];
pub const OPEN_OPTIONS: [&str; 3] = ["std", "fs", "OpenOptions"];
pub const OPS_MODULE: [&str; 2] = ["core", "ops"];
pub const OPTION: [&str; 3] = ["core", "option", "Option"];
pub const OPTION_NONE: [&str; 4] = ["core", "option", "Option", "None"];
pub const OPTION_SOME: [&str; 4] = ["core", "option", "Option", "Some"];
pub const ORD: [&str; 3] = ["core", "cmp", "Ord"];
pub const OS_STRING: [&str; 4] = ["std", "ffi", "os_str", "OsString"];
pub const OS_STR_TO_OS_STRING: [&str; 5] = ["std", "ffi", "os_str", "OsStr", "to_os_string"];
pub const PARTIAL_ORD: [&str; 3] = ["core", "cmp", "PartialOrd"];
pub const PATH_BUF: [&str; 3] = ["std", "path", "PathBuf"];
pub const PATH_TO_PATH_BUF: [&str; 4] = ["std", "path", "Path", "to_path_buf"];
pub const PTR_NULL: [&str; 2] = ["ptr", "null"];
pub const PTR_NULL_MUT: [&str; 2] = ["ptr", "null_mut"];
pub const RANGE: [&str; 3] = ["core", "ops", "Range"];
pub const RANGE_ARGUMENT_TRAIT: [&str; 3] = ["core", "ops", "RangeBounds"];
pub const RANGE_FROM: [&str; 3] = ["core", "ops", "RangeFrom"];
pub const RANGE_FROM_STD: [&str; 3] = ["std", "ops", "RangeFrom"];
pub const RANGE_FULL: [&str; 3] = ["core", "ops", "RangeFull"];
pub const RANGE_FULL_STD: [&str; 3] = ["std", "ops", "RangeFull"];
pub const RANGE_INCLUSIVE_NEW: [&str; 4] = ["core", "ops", "RangeInclusive", "new"];
pub const RANGE_INCLUSIVE_STD_NEW: [&str; 4] = ["std", "ops", "RangeInclusive", "new"];
pub const RANGE_STD: [&str; 3] = ["std", "ops", "Range"];
pub const RANGE_TO: [&str; 3] = ["core", "ops", "RangeTo"];
pub const RANGE_TO_INCLUSIVE: [&str; 3] = ["core", "ops", "RangeToInclusive"];
pub const RANGE_TO_INCLUSIVE_STD: [&str; 3] = ["std", "ops", "RangeToInclusive"];
pub const RANGE_TO_STD: [&str; 3] = ["std", "ops", "RangeTo"];
pub const RC: [&str; 3] = ["alloc", "rc", "Rc"];
pub const REGEX: [&str; 3] = ["regex", "re_unicode", "Regex"];
pub const REGEX_BUILDER_NEW: [&str; 5] = ["regex", "re_builder", "unicode", "RegexBuilder", "new"];
pub const REGEX_BYTES_BUILDER_NEW: [&str; 5] = ["regex", "re_builder", "bytes", "RegexBuilder", "new"];
pub const REGEX_BYTES_NEW: [&str; 4] = ["regex", "re_bytes", "Regex", "new"];
pub const REGEX_BYTES_SET_NEW: [&str; 5] = ["regex", "re_set", "bytes", "RegexSet", "new"];
pub const REGEX_NEW: [&str; 4] = ["regex", "re_unicode", "Regex", "new"];
pub const REGEX_SET_NEW: [&str; 5] = ["regex", "re_set", "unicode", "RegexSet", "new"];
pub const REPEAT: [&str; 3] = ["core", "iter", "repeat"];
pub const RESULT: [&str; 3] = ["core", "result", "Result"];
pub const RESULT_ERR: [&str; 4] = ["core", "result", "Result", "Err"];
pub const RESULT_OK: [&str; 4] = ["core", "result", "Result", "Ok"];
pub const SERDE_DE_VISITOR: [&str; 3] = ["serde", "de", "Visitor"];
pub const SLICE_INTO_VEC: [&str; 4] = ["alloc", "slice", "<impl [T]>", "into_vec"];
pub const SLICE_ITER: [&str; 3] = ["core", "slice", "Iter"];
pub const STRING: [&str; 3] = ["alloc", "string", "String"];
pub const TO_OWNED: [&str; 3] = ["alloc", "borrow", "ToOwned"];
pub const TO_OWNED_METHOD: [&str; 4] = ["alloc", "borrow", "ToOwned", "to_owned"];
pub const TO_STRING: [&str; 3] = ["alloc", "string", "ToString"];
pub const TO_STRING_METHOD: [&str; 4] = ["alloc", "string", "ToString", "to_string"];
pub const TRANSMUTE: [&str; 4] = ["core", "intrinsics", "", "transmute"];
pub const TRY_INTO_RESULT: [&str; 4] = ["std", "ops", "Try", "into_result"];
pub const UNINIT: [&str; 4] = ["core", "intrinsics", "", "uninit"];
pub const VEC: [&str; 3] = ["alloc", "vec", "Vec"];
pub const VEC_DEQUE: [&str; 4] = ["alloc", "collections", "vec_deque", "VecDeque"];
pub const VEC_FROM_ELEM: [&str; 3] = ["alloc", "vec", "from_elem"];
pub const WEAK_ARC: [&str; 3] = ["alloc", "sync", "Weak"];
pub const WEAK_RC: [&str; 3] = ["alloc", "rc", "Weak"];
