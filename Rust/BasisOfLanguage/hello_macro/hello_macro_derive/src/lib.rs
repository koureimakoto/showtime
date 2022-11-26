use proc_macro::TokenStream;
use quote::quote;
use syn;


/*
Da para entender por que este assunto ficou no final. Você pode
manipular a AST. Na documentação até informaram que pretendem no
futuro desenvolver algo mais simples, mas honestamente, apesar 
de ser um pouco complexo, faz muito sentido.

Rust já é complexo em um todo, para quem domina a linguagem, esse
detalhe aqui não vai atrapalhar, até ajuda, já que você pode com-
preender melhor o funcionamento da lógica do próprio compilador.
*/
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Ola MACRO... trabalhoso, mas legal, seu nome é {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
