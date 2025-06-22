#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code
)]

include!(concat!(env!("OUT_DIR"), "/c-lib-bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/cpp-lib-bindings.rs"));
