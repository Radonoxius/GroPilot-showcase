use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Abi, Ident, ItemFn, LitStr, Signature, Token};

#[proc_macro_attribute]
pub fn jvm(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut package_name = String::new();
    
    if !attrs.is_empty() {
        let attr_ast = parse_macro_input!(attrs as LitStr);
        package_name = attr_ast.value().replace(".", "_").to_string();
        if package_name != "" {
            package_name.push('_');
        }
    }
    
    let ast = parse_macro_input!(input as ItemFn);
    let vis = &ast.vis;
    let sig = &ast.sig;
    let name = &ast.sig.ident;
    let block = &ast.block;
    
    let new_name = 
        Ident::new(&format!("Java_{}{}", package_name, name), name.span());

    #[cfg(target_family = "unix")]
    let new_sig = Signature {
        abi: Some(Abi {
            extern_token: Token![extern](sig.abi.span()),
            name: Some(LitStr::new("C", sig.abi.span()))
        }),
        ident: new_name,
        ..sig.clone()
    };
    
    #[cfg(target_family = "windows")]
    let new_sig = Signature {
        abi: Some(Abi {
            extern_token: Token![extern](sig.abi.span()),
            name: Some(LitStr::new("system", sig.abi.span()))
        }),
        ident: new_name,
        ..sig.clone()
    };

    quote! {
        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        #vis #new_sig #block
    }.into()
}