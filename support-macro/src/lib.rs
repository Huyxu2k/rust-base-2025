use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Token, ItemFn, LitStr};


#[derive(Debug)]
enum Method {
    Get,
    Post,
    Put,
    Delete
}

impl Method {
    fn from_ident(ident: &Ident)->Option<Self>{
        match ident.to_string().as_str() {
            "Get"=> Some(Self::Get),
            "Post"=> Some(Self::Post),
            "Put"=> Some(Self::Put),
            "Delete"=> Some(Self::Delete),
            _ => None,
        }
    }
    fn to_token(&self)-> proc_macro2::TokenStream{
        match self {
            Method::Get => quote! { get },
            Method::Post => quote! { post },
            Method::Put => quote! { put },
            Method::Delete => quote! { delete },
        }
    }
}

struct MacroInput{
    method: Method,
    path: Option<LitStr>,
}

impl Parse for  MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident= input.parse()?;
        let method= Method::from_ident(&ident).ok_or_else(||{
            syn::Error::new(ident.span(), "Invalid HTTP method, expected one of: Get, Post, Put, Delete")
        })?;
        let path= if input.peek(Token![,]){
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        }else{
            None
        };
        Ok(MacroInput { method, path })
    }
}

#[proc_macro_attribute]
pub fn openapi(attr: TokenStream, item: TokenStream)-> TokenStream{
    let input= parse_macro_input!(item as ItemFn);
    let func_name= &input.sig.ident;

    let method_input= parse_macro_input!(attr as MacroInput);
    let method= method_input.method.to_token();

    let path= if let Some(path_str)= method_input.path{
        quote! {#path_str}
    }else{
        quote! { concat!("/", stringify!(#func_name))}
    };

    let generated= quote! {
        #[utoipa::path(
            #method,
            path=#path ,
            responses(
                (status=200, description="Success"),
                (status=404, description="Not Found")
            )
        )]
        #input
    };
    TokenStream::from(generated)
}
