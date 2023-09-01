use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test_with_logger(_metadata: TokenStream, test_fn: TokenStream) -> TokenStream {
	let ItemFn {
		attrs,
		vis,
		sig,
		block,
	} = parse_macro_input!(test_fn as ItemFn);

	let attrs = attrs.iter().map(|attr| quote!(#attr)).collect::<Vec<_>>();
	let attrs = TokenStream2::from_iter(attrs);

	let expanded = quote! {
		#[test]
		#attrs
		#vis #sig {
			use env_logger;

			let _ = env_logger::builder().is_test(true).try_init();
			#block
		}
	};

	TokenStream::from(expanded)
}
