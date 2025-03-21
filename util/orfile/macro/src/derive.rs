use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn impl_orfile(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let vis = &input.vis;

	let mod_or_file = format_ident!("or_file");
	let mod_using = format_ident!("using");

	let (config_fields, other_fields): (Vec<_>, Vec<_>) = match &input.data {
		Data::Struct(data) => data
			.fields
			.iter()
			.partition(|f| f.ident.as_ref().map_or(false, |id| id.to_string().ends_with("config"))),
		_ => panic!("Orfile can only be derived for structs"),
	};

	let config_idents: Vec<_> = config_fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
	let config_path_idents: Vec<_> =
		config_idents.iter().map(|id| format_ident!("{}_path", id)).collect();
	let config_types: Vec<_> = config_fields.iter().map(|f| &f.ty).collect();

	let other_field_defs: Vec<_> = other_fields
		.iter()
		.map(|f| {
			let id = &f.ident;
			let ty = &f.ty;
			quote! {
				#[clap(long)]
				pub #id: #ty,
			}
		})
		.collect();

	let config_path_fields: Vec<_> = config_path_idents
		.iter()
		.map(|id| {
			quote! {
				#[clap(long)]
				pub #id: String,
			}
		})
		.collect();

	let resolve_config_lines: Vec<_> = config_path_idents
		.iter()
		.zip(config_types.iter())
		.zip(config_idents.iter())
		.map(|((path_ident, ty), config_ident)| {
			quote! {
				let contents = tokio::fs::read_to_string(&self.#path_ident).await?;
				let #config_ident: #ty = serde_json::from_str(&contents)?;
			}
		})
		.collect();

	let construct_config_fields: Vec<_> = config_idents.iter().map(|id| quote! { #id }).collect();
	let construct_other_fields: Vec<_> = other_fields
		.iter()
		.map(|f| {
			let id = &f.ident;
			quote! { #id: self.#id.clone() }
		})
		.collect();

	let expanded = quote! {
		pub mod #mod_using {
			use super::*;
			use serde_json;

			#[derive(clap::Parser, Debug, Clone)]
			pub struct #struct_name {
				#(#config_path_fields)*

				#(#other_field_defs)*
			}

			impl #struct_name {
				pub async fn resolve(self) -> Result<super::#struct_name, anyhow::Error> {
					#(#resolve_config_lines)*

					Ok(super::#struct_name {
						#(#construct_config_fields,)*
						#(#construct_other_fields,)*
					})
				}
			}
		}

		pub mod #mod_or_file {
			use super::*;
			use #mod_using;

			#[derive(clap::Subcommand, Debug, Clone)]
			#vis enum #struct_name {
				/// Run $struct_name where flags provide parameters
				Where(super::#struct_name),
				/// Run $struct_name using config files to load parameters
				Using(#mod_using::#struct_name),
			}

			impl #struct_name {
				pub async fn resolve(self) -> Result<super::#struct_name, anyhow::Error> {
					match self {
						Self::Where(inner) => Ok(inner),
						Self::Using(inner) => inner.resolve().await,
					}
				}
			}
		}
	};

	TokenStream::from(expanded)
}
