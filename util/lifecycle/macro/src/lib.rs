use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(LifecycleSubcommand, attributes(lifecycle_subcommand))]
pub fn derive_lifecycle_subcommand(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let attrs = &input.attrs;

	// Extract the lifecycle frontend type from the attribute
	let frontend_type =
		match input.attrs.iter().find(|attr| attr.path().is_ident("lifecycle_subcommand")) {
			Some(attr) => match attr.parse_args::<Type>() {
				Ok(ty) => ty,
				Err(e) => return e.to_compile_error().into(),
			},
			None => {
				return quote! {
					compile_error!("The lifecycle_subcommand attribute must specify the lifecycle frontend type");
				}
				.into();
			}
		};

	// Filter out the lifecycle_subcommand attribute and LifecycleSubcommand derive
	let filtered_attrs = attrs.iter().filter(|attr| {
		!attr.path().is_ident("lifecycle_subcommand")
			&& !(attr.path().is_ident("derive")
				&& attr
					.parse_args::<syn::Path>()
					.map(|p| p.is_ident("LifecycleSubcommand"))
					.unwrap_or(false))
	});

	let expanded = quote! {
		pub mod lifecycle_subcommand {
			use super::*;

			#[derive(Debug, Clone, Subcommand)]
			#(#filtered_attrs)*
			pub enum #struct_name {
				Apply(<#frontend_type as lifecycle::LifecycleFrontend>::ApplyFrontend),
				Destroy(<#frontend_type as lifecycle::LifecycleFrontend>::DestroyFrontend),
			}

			impl #struct_name {
				pub async fn execute(&self) -> std::result::Result<<<#frontend_type as lifecycle::LifecycleOperations>::Applier as lifecycle::ApplyOperations>::OutputArtifacts, lifecycle::LifecycleError> {
					match self {
						Self::Apply(config) => #frontend_type::apply(config).await,
						Self::Destroy(config) => #frontend_type::destroy(config).await,
						_ => Err(lifecycle::LifecycleError::Unsupported("unsupported subcommand".into())),
					}
				}
			}
		}
	};

	TokenStream::from(expanded)
}
