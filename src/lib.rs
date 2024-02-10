extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, ItemEnum, Lit, Meta, MetaNameValue};

#[proc_macro_attribute]
pub fn enum_bytes(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone(); // The name of the enum
    let data_enum = if let Data::Enum(data_enum) = input.data.clone() {
        data_enum
    } else {
        // Not an enum, so panic or handle as necessary
        panic!("EnumTextureBytes only works with enums!");
    };

    let args_raw = args.to_string();
    let mut args = args_raw.split(",");
    let path = args.next().unwrap().trim().replace(' ', "");
    let extension = args.next().unwrap().trim();

    let variants = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let file_name = format!("/{}/{}.{}", path, variant_name.to_string().to_case(Case::Snake), extension);
        quote! {
            #name::#variant_name => include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), #file_name)).to_vec(),
        }
    });

    let expanded = quote! {
        #input

        impl Into<Vec<u8>> for #name {
            fn into(&self) -> Vec<u8> {
                match self {
                    #( #variants )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}