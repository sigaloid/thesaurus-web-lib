#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    // clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    clippy::all,
    // missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(dead_code, clippy::too_many_lines)]

use classes::SynonymInfo;

mod classes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
pub fn lookup_word(word: &str) -> Option<SynonymInfo>{
    let request = ureq::get(&format!("https://tuna.thesaurus.com/pageData/{}", urlencoding::encode(word))).call().ok()?.into_string().ok()?;
    let object: SynonymInfo = nanoserde::DeJson::deserialize_json(&request).unwrap();
    Some(object)
}


