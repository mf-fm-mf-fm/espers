use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Error, Fields,
};

#[proc_macro_derive(ToIcedColumn)]
pub fn to_iced_column_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let x = match input.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = format!("{:?}", f.ident);
                    let foo = &f.ident;
                    quote! {
                        ::toiced::iced::widget::Container::new(::toiced::iced::widget::text(#name).size(30)).padding(10).into(),
                        self.#foo.to_iced(args)
                    }
                });
                quote! {
                    let elems: Vec<::toiced::iced::Element<'a, M>> = vec![#(#recurse),*];
                    ::toiced::iced::widget::Column::with_children(elems).into()
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let num = syn::Index::from(i);
                    quote! {
                        ::toiced::iced::widget::Container::new(::toiced::iced::widget::text(#i).size(30)).padding(10).into(),
                        self.#num.to_iced(args)
                    }
                });
                quote! {
                    let elems: Vec<::toiced::iced::Element<'a, M>> = vec![#(#recurse),*];
                    ::toiced::iced::widget::Column::with_children(elems).into()
                }
            }
            Fields::Unit => quote! { ::toiced::iced::widget::text(format!("{}", self)) },
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    let expanded = quote! {
        // The generated impl.
        impl<'a, M: 'a> ::toiced::ToIced<'a, M> for #name {
            fn to_iced(&self) -> ::toiced::iced::Element<'a, M> {
                #x
            }
        }
    };
    TokenStream::from(expanded)
}
