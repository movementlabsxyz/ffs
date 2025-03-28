use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn impl_orfile(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let vis = &input.vis;
	let struct_prefix = struct_name.to_string().to_uppercase();

	let mod_or_file = format_ident!("or_file");
	let mod_using = format_ident!("using");

	let lower_case_struct_prefix = struct_name.to_string().to_lowercase();
	let doc_where = Literal::string(&format!(
		"Run {} with all parameters passed explicitly as CLI flags.",
		lower_case_struct_prefix
	));
	let doc_using = Literal::string(&format!(
		"Run {} with parameters from environment variables, config files, and CLI flags.",
		lower_case_struct_prefix
	));

	let (config_fields, other_fields): (Vec<_>, Vec<_>) = match &input.data {
		Data::Struct(data) => data.fields.iter().partition(|f| {
			f.attrs.iter().any(|attr| {
				attr.path().is_ident("orfile")
					&& attr.parse_args::<syn::Path>().map(|p| p.is_ident("config")).unwrap_or(false)
			})
		}),
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
			let attrs = f.attrs.iter().filter(|attr| {
				!(attr.path().is_ident("orfile")
					&& attr
						.parse_args::<syn::Path>()
						.map(|p| p.is_ident("config"))
						.unwrap_or(false))
			});
			quote! {
				#(#attrs)*
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
				pub #id: Option<String>,
			}
		})
		.collect();

	let env_and_file_mergers: Vec<_> = config_path_idents
		.iter()
		.zip(config_types.iter())
		.zip(config_idents.iter())
		.map(|((path_ident, ty), config_ident)| {
			let env_prefix = format!("{}_", struct_prefix);

			quote! {
				let mut config_map = serde_json::Map::new();

				// Merge from ENV
				for (key, val) in std::env::vars() {
					if let Some(suffix) = key.strip_prefix(#env_prefix) {
						let field_name = suffix.to_ascii_lowercase().replace("__", "_");
						config_map.insert(field_name, serde_json::Value::String(val));
					}
				}

				// Merge from file
				if let Some(file_path) = &self.#path_ident {
					let file_contents = tokio::fs::read_to_string(file_path).await
					.with_context(|| format!("Failed to read file at {}", file_path))?;
					let file_value: serde_json::Value = serde_json::from_str(&file_contents)
						.context("Failed to parse config file")?;

					if let Some(map) = file_value.as_object() {
						config_map.extend(map.clone());
					}
				}

				// Merge from CLI extra args
				for pair in self.extra_args.chunks(2) {
					if pair.len() == 2 {
						let key = pair[0]
							.trim_start_matches("--")
							.replace("-", "_")
							.to_ascii_lowercase(); // optional for safety
						let val = pair[1].to_string();
						config_map.insert(key, serde_json::Value::String(val));
					}
				}

				let #config_ident: #ty = serde_json::from_value(serde_json::Value::Object(config_map))
					.context("Failed to deserialize merged config")?;
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
			use anyhow::{Context, Error};
			use serde_json;

			#[derive(clap::Parser, Debug, Clone)]
			#[clap(trailing_var_arg = true)]
			pub struct #struct_name {
				#(#config_path_fields)*

				#(#other_field_defs)*

				pub extra_args: Vec<String>,
			}

			impl #struct_name {
				pub async fn resolve(self) -> Result<super::#struct_name, Error> {
					#(#env_and_file_mergers)*

					Ok(super::#struct_name {
						#(#construct_config_fields,)*
						#(#construct_other_fields,)*
					})
				}
			}
		}

		pub mod #mod_or_file {
			use super::*;
			use anyhow::Error;
			use #mod_using;

			#[derive(clap::Subcommand, Debug, Clone)]
			#vis enum #struct_name {
				#[doc = #doc_where]
				Where(super::#struct_name),

				#[doc = #doc_using]
				Using(#mod_using::#struct_name),
			}

			impl #struct_name {
				pub async fn resolve(self) -> Result<super::#struct_name, Error> {
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
