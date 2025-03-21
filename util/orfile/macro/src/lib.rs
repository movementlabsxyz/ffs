use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod derive;

#[proc_macro_derive(Orfile)]
#[proc_macro_error]
pub fn orfile_derive(input: TokenStream) -> TokenStream {
	derive::impl_orfile(input)
}
