use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Jsonl)]
pub fn derive_jsonl(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;

	let fields = match &input.data {
		Data::Struct(s) => &s.fields,
		_ => panic!("Jsonl can only be derived for structs"),
	};

	// Extract fields for parsing from JSONL
	let field_extracts = fields.iter().map(|field| {
		let field_name = field.ident.as_ref().unwrap();
		let field_str = field_name.to_string();

		quote! {
			#field_name: serde_json::from_value(
				parsed_data.get(#field_str)
					.ok_or_else(|| jsonlvar::JsonlError::MissingField(#field_str.to_string()))?
					.clone()
			).map_err(jsonlvar::JsonlError::Json)?,
		}
	});

	// Generate JSONL field serialization (flat)
	let field_serializations = fields.iter().map(|field| {
		let field_name = field.ident.as_ref().unwrap();
		let field_str = field_name.to_string();

		quote! {
			let field_value = serde_json::to_string(&self.#field_name)?;
			let prefixed_name = match &var_prefix {
				Some(prefix) => format!("{}_{}", prefix, #field_str),
				None => #field_str.to_string(),
			};
			jsonl_entries.push(format!("JSONL {} = {}", prefixed_name, field_value));
		}
	});

	let expanded = quote! {
		use jsonlvar::{JsonlError, serde_json};

		impl Jsonl for #struct_name {
			fn try_from_jsonl_map(parsed_data: &std::collections::HashMap<String, serde_json::Value>)
				-> Result<Self, JsonlError> {
				Ok(Self {
					#(#field_extracts)*
				})
			}

			fn try_to_jsonl_flat_vec(&self, var_prefix: Option<String>) -> Result<Vec<String>, JsonlError> {
				let mut jsonl_entries = Vec::new();
				#(#field_serializations)*
				Ok(jsonl_entries)
			}
		}
	};

	TokenStream::from(expanded)
}
