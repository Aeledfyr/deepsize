/// A basic DeepSizeOf Derive implementation
///
/// Mainly from `syn`'s heap_size derive example:
/// https://github.com/dtolnay/syn/commits/master/examples/heapsize/heapsize_derive/src/lib.rs

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

#[proc_macro_derive(DeepSizeOf)]
pub fn derive_deep_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: DeepSizeOf` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the size of each field.
    let sum = deepsize_sum(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ::deepsize::DeepSizeOf for #name #ty_generics #where_clause {
            fn deep_size_of_children(&self, context: &mut ::deepsize::Context) -> usize {
                #sum
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: DeepSizeOf` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::deepsize::DeepSizeOf));
        }
    }
    generics
}

// Generate an expression to sum up the size of each field.
fn deepsize_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            ::deepsize::DeepSizeOf::deep_size_of_children(&self.#name, context)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::deepsize::DeepSizeOf::deep_size_of_children(&self.#index, context)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}