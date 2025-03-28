use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(ServiceSubcommand, attributes(service_subcommand))]
pub fn derive_service_subcommand(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let struct_name = &input.ident;
	let attrs = &input.attrs;

	// Extract the service frontend type from the attribute
	let frontend_type =
		match input.attrs.iter().find(|attr| attr.path().is_ident("service_subcommand")) {
			Some(attr) => match attr.parse_args::<Type>() {
				Ok(ty) => ty,
				Err(e) => return e.to_compile_error().into(),
			},
			None => {
				return quote! {
					compile_error!("The service_subcommand attribute must specify the service frontend type");
				}
				.into();
			}
		};

	// Filter out the service_subcommand attribute and ServiceSubcommand derive
	let filtered_attrs = attrs.iter().filter(|attr| {
		!attr.path().is_ident("service_subcommand")
			&& !(attr.path().is_ident("derive")
				&& attr
					.parse_args::<syn::Path>()
					.map(|p| p.is_ident("ServiceSubcommand"))
					.unwrap_or(false))
	});

	let expanded = quote! {
		pub mod service_subcommand {
			use super::*;

			#[derive(Debug, Clone, Subcommand)]
			#(#filtered_attrs)*
			pub enum #struct_name {
				Up(<#frontend_type as service::ServiceFrontend>::UpFrontend),
				Down(<#frontend_type as service::ServiceFrontend>::DownFrontend),
			}

			impl #struct_name {
				pub async fn execute(&self) -> std::result::Result<<<#frontend_type as service::ServiceOperations>::Upper as service::UpOperations>::OutputArtifacts, service::ServiceError> {
					match self {
						Self::Up(config) => #frontend_type::up(config).await,
						Self::Down(config) => #frontend_type::down(config).await,
						_ => Err(service::ServiceError::Unsupported("unsupported subcommand".into())),
					}
				}
			}
		}
	};

	TokenStream::from(expanded)
}
