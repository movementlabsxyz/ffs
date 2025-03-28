use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

mod derive;

#[proc_macro_derive(Orfile, attributes(orfile))]
#[proc_macro_error]
pub fn orfile_derive(input: TokenStream) -> TokenStream {
	derive::impl_orfile(input)
}
