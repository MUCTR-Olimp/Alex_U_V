use proc_macro::TokenStream;
use std::iter::FromIterator;
//use std::iter::IntoIterator;
use proc_macro::TokenTree;
#[proc_macro]
pub fn generate_primes(up_to: TokenStream) -> TokenStream {
    let input = up_to.into_iter().collect::<std::vec::Vec<proc_macro::TokenTree>>();
    let value = match &input.get(0) {
        Some(proc_macro::TokenTree::Literal(literal)) => literal.to_string(),
        _ => panic!("wrong macro argument")//compile_error!("wrong macro argument!")
    };
    let border = value.parse::<usize>().unwrap();
    let mut list: std::vec::Vec<i32> = (2..=border+1).map(|_| 1).collect();
    let mut index: usize = 2;
    while index < border/2{
        if list[index - 2] == 1 {
            for i in ((2*index-2)..border).step_by(index) {
                list[i] = 0;
            }
        }
        index += 1;
    }
    let mut ans_list = std::vec::Vec::new();
    //ans_list.push(proc_macro::TokenTree::Punct(proc_macro::Punct::new('[',proc_macro::Spacing::Alone)));
    for (i,x) in list.iter().enumerate() {
        if *x == 1 {
            ans_list.push(proc_macro::TokenTree::Literal(proc_macro::Literal::i32_suffixed((i+2) as i32)));
            ans_list.push(proc_macro::TokenTree::Punct(proc_macro::Punct::new(',',proc_macro::Spacing::Alone)));
        }
    }
    ans_list.pop();
    //ans_list.push(proc_macro::TokenTree::Punct(proc_macro::Punct::new(']',proc_macro::Spacing::Joint)));
    let tmp = TokenStream::from_iter(ans_list);
    TokenStream::from(TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Bracket,tmp)))
}