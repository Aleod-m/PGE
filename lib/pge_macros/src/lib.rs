extern crate proc_macro;
use std::str::FromStr;
use proc_macro::{
    TokenStream,
};
use syn::{
    self,
    parse_macro_input,
};
use quote::{
    quote,
};



/// Struct wrapper generator
/// attibutes:
///  - wrapper_name : String
///  - wrapper_generics : nested unamed String attributes
///  - wrapped_generics : nested unamed String attributes
#[proc_macro_attribute]
pub fn wrap(args : TokenStream, input: TokenStream) -> TokenStream {
    // Generate abstract syntax trees
    let input_ast = parse_macro_input!(input as syn::DeriveInput);
    let args_ast = parse_macro_input!(args as syn::AttributeArgs);
    let item = syn::Item::from(input_ast.clone());
    let input_struct = if let syn::Item::Struct(i) = item {
        i
    }
    else {
        panic!("Expected a struct in argument !")
    };
    
    let wrapper_struct = generate_wrapper_struct(&input_struct, &args_ast);
    let wrapper_struct_ast = syn::Type::Verbatim(wrapper_struct.into());
    let final_code = quote! {
        #input_ast
        #wrapper_struct_ast
    };
    
    final_code.into()
}




fn generate_wrapper_struct(input_struct : &syn::ItemStruct, args : &syn::AttributeArgs) -> TokenStream {
    let args = get_arguments_values(args);
    let wrapper_name = args.wrapper_name;
    let wrapper_name_ast = syn::Type::Verbatim(wrapper_name.into());
    let wrapped_name = &input_struct.ident;
    let wrapped_generics_ast = match args.wrapped_generics {
        Some(gen) => syn::Type::Verbatim(gen.into()),
        None => {
            let temp_tks = TokenStream::new();
            syn::Type::Verbatim(temp_tks.into())
        }
    };
    let new_tks = quote! {
        struct #wrapper_name_ast {
            inner : #wrapped_name#wrapped_generics_ast,
        }
    };
    
    return new_tks.into();
} 

#[derive(Debug)]
struct WrapArgs {
    pub(crate) wrapper_name : TokenStream,
    pub(crate) wrapped_generics : Option<TokenStream>,
}

fn get_arguments_values(args : &syn::AttributeArgs) -> WrapArgs {
    let mut wrapper_name : TokenStream = TokenStream::new();
    let mut wrapped_generics = None;

    for arg in args.iter() {
        // get wrapped name
        if let syn::NestedMeta::Meta(
            syn::Meta::NameValue(
                syn::MetaNameValue{
                    path : syn::Path{
                        segments,
                        ..
                    },
                    lit,
                    ..
                }
            )
        ) = arg {
            for seg in segments.iter() {
                if seg.ident == "wrapper_name"  {
                    match lit {
                        syn::Lit::Str(name) => wrapper_name = TokenStream::from_str(&name.value().to_owned()).unwrap(),
                        _ => panic!("Expected a string for the wrapper struct name!"),
                    };
                }
                else if seg.ident == "wrapped_generics" {
                    match lit {
                        syn::Lit::Str(name) => wrapped_generics = Some(TokenStream::from_str(&name.value().to_owned()).unwrap()),
                        _ => panic!("Expected a string for the wrapped generics!"),
                    }
                }
                else {
                    panic!("Unexpected named argument!");
                }
            }
        }
        else {
            panic!("Unexpected named argument")
        }
    }

    WrapArgs {
        wrapper_name,
        wrapped_generics,
    }
}

// Expose the functions of the wrapped struct in the wapper struct
#[proc_macro_attribute]
pub fn expose_pub_fn(args: TokenStream, input : TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as syn::DeriveInput);
    let args_ast = parse_macro_input!(args as syn::AttributeArgs);
    let item = syn::Item::from(input_ast.clone());
    let input_impl_block = if let syn::Item::Impl(i) = item {
        i
    }
    else {
        panic!("Expected an impl block in argument !")
    };
    
    let wrapper_struct = generate_impl_block(&input_impl_block, &args_ast);
    let wrapper_struct_ast = syn::Type::Verbatim(wrapper_struct.into());
    let final_code = quote! {
        #input_ast
        #wrapper_struct_ast
    };
    
    final_code.into()
}

fn generate_impl_block(input_impl : &syn::ItemImpl, args : &syn::AttributeArgs) -> TokenStream {
    let wrapper_name = get_wrapper_name(args);
    let final_code = quote! {
        impl {

        }
    };

    final_code.into()
}

fn get_wrapper_name(args : &syn::AttributeArgs) /*-> TokenStream */{
    eprintln!("{:#?}", args);
}