use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(
	LifecycleSubcommand,
	attributes(lifecycle, lifecycle_apply_is_subcommand, lifecycle_destroy_is_subcommand)
)]
pub fn derive_lifecycle_subcommand(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let attrs = &input.attrs;

	// Extract the lifecycle frontend type and check for subcommand flags
	let (frontend_type, apply_is_subcommand, destroy_is_subcommand) = {
		let mut frontend_type = None;
		let mut apply_is_subcommand = false;
		let mut destroy_is_subcommand = false;

		for attr in attrs.iter() {
			if attr.path().is_ident("lifecycle") {
				if let Ok(ty) = attr.parse_args::<Type>() {
					frontend_type = Some(ty);
				}
			} else if attr.path().is_ident("lifecycle_apply_is_subcommand") {
				apply_is_subcommand = true;
			} else if attr.path().is_ident("lifecycle_destroy_is_subcommand") {
				destroy_is_subcommand = true;
			}
		}

		let frontend_type = frontend_type.ok_or_else(|| {
			quote! {
				compile_error!("The lifecycle attribute must specify the lifecycle frontend type");
			}
		});

		match frontend_type {
			Ok(ty) => (ty, apply_is_subcommand, destroy_is_subcommand),
			Err(e) => return e.into(),
		}
	};

	// Filter out the lifecycle attributes
	let filtered_attrs = attrs.iter().filter(|attr| {
		!attr.path().is_ident("lifecycle")
			&& !attr.path().is_ident("lifecycle_apply_is_subcommand")
			&& !attr.path().is_ident("lifecycle_destroy_is_subcommand")
	});

	let apply_variant = if apply_is_subcommand {
		quote! {
			#[clap(subcommand)]
			Apply(<#frontend_type as lifecycle::LifecycleFrontend>::ApplyFrontend),
		}
	} else {
		quote! {
			Apply(<#frontend_type as lifecycle::LifecycleFrontend>::ApplyFrontend),
		}
	};

	let destroy_variant = if destroy_is_subcommand {
		quote! {
			#[clap(subcommand)]
			Destroy(<#frontend_type as lifecycle::LifecycleFrontend>::DestroyFrontend),
		}
	} else {
		quote! {
			Destroy(<#frontend_type as lifecycle::LifecycleFrontend>::DestroyFrontend),
		}
	};

	let expanded = quote! {
		pub mod lifecycle_subcommand {
			use super::*;
			use clap::Subcommand;

			#[derive(Debug, Clone, Subcommand)]
			#(#filtered_attrs)*
			pub enum #struct_name {
				#apply_variant
				#destroy_variant
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
