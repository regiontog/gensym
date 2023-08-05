//! Creates unique identifiers for macros using procedural macros and [UUID](https://crates.io/crates/uuid)
//! # Examples
//! ```
//!
//! macro_rules! gen_fn {
//!     ($a:ty, $b:ty) => {
//!         gensym::gensym!{ _gen_fn!{ $a, $b } }
//!     };
//! }
//!
//! macro_rules! _gen_fn {
//!     ($gensym:ident, $a:ty, $b:ty) => {
//!         fn $gensym(a: $a, b: $b) {
//!             unimplemented!()
//!         }
//!     };
//! }
//!
//! mod test {
//!     gen_fn!{ u64, u64 }
//!     gen_fn!{ u64, u64 }
//! }
//! ```
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{parse_macro_input, parse_quote};
use uuid::Uuid;

#[proc_macro]
pub fn gensym(input: TokenStream) -> TokenStream {
    //! Generate a unique identifier with a span of `Span::call_site` and
    //! insert it as the first argument to a macro call followed by a comma.

    let mcall = parse_macro_input!(input as syn::Macro);

    proc_macro::TokenStream::from(
        alter_macro(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

fn alter_macro(mut mcall: syn::Macro) -> Result<proc_macro2::TokenStream, syn::Error> {
    use quote::ToTokens;

    let sym = syn::Ident::new(
        &format!("__gensym_{}", Uuid::new_v4().simple()),
        Span::call_site(),
    );

    let mut inserted_gensym: proc_macro2::TokenStream = parse_quote!(#sym, );

    inserted_gensym.extend(mcall.tokens);
    mcall.tokens = inserted_gensym;

    Ok(mcall.into_token_stream())
}
