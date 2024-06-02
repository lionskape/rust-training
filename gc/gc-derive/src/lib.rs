use proc_macro::{TokenStream, TokenTree::Group, TokenTree::Ident};

#[proc_macro_derive(Scan)]
pub fn derive_scan(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    iter.next(); // trash
    if let Some(Ident(type_name)) = iter.next() {
        let default_res: TokenStream = format!(
            "impl Scan for {} {{ fn scan(&self) -> Vec<usize> {{ vec![] }} }}",
            type_name
        ).parse().unwrap();
        if let Some(Group(group)) = iter.next() {
            if let Some(Ident(field_name)) = group.stream().into_iter().next() {
                format!(
                    "impl Scan for {} {{ fn scan(&self) -> Vec<usize> {{ self.{}.scan() }} }}",
                    type_name,
                    field_name
                ).parse().unwrap()
            } else {
                default_res
            }
        } else {
            default_res
        }
    } else {
        TokenStream::new()
    }
}
