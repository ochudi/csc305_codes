use litrs::StringLit;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn my_attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    //Get the original function
    let input = parse_macro_input!(item as ItemFn);
    //Get the original function name
    let name = input.sig.ident;
    //Get the arguments to the original function
    let args = input.sig.inputs.clone();
    //Get the function block {}
    let block = input.block;
    //Get the attributes
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    /*
    Below is illustration for taking an attribute and adding it to outcome of function
    #[my_attribute_macro(6)]
    fn my_ordinary_function(x: i32) -> i32 {
    x * 3
    }

    fn main() {
    println!("{}", my_ordinary_function(3));
    }
    */
    let n = attr_args[0].clone(); //assuming that you have an attribute number, you want to add to original statement.
    let output = quote! {
    fn #name(#args) -> i32 {
    (#block) + #n
    }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    //Get the original function
    let input = parse_macro_input!(item as ItemFn);
    //Get the original function name
    let name = input.sig.ident;
    //Get the arguments to the original function
    let args = input.sig.inputs.clone();
    //Get the function block {}
    let block = input.block;
    //Get the attributes
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    //let stmts = block.stmts; //ToDo: examine statements
    //let stmts0 = stmts[0].clone();

    /*
    Below is illustration for taking an attribute capturing
    parameters e.g. in route definition when creating a Web framework
    and using them in the logic. i.e.
    #[route(GET, "/")]
    */
    let http_verb = attr_args[0].clone(); //assuming that you have an attribute like "GET"
    let url_route = attr_args[1].clone();

    /*
    //Get the attributes
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let http_verb = attr_args[0].clone(); //assuming that you have an attribute like "GET"
    let url_route = attr_args[1].clone();

    let ItemFn {
    attrs,
    vis,
    sig,
    block,
    } = input;
    let stmts = &block.stmts;
    let output = quote! {
    #(#attrs)* #vis #sig -> (&'static str){
    let x = 1; // <- your extra code
    let verb = #http_verb;
    let url_route = #url_route;
    #(#stmts)*
    }
    };
    */

    let output = quote! {
    fn #name<'a>() -> (&'a str, &'a str, &'a str) {
    //https://github.com/smoqadam/rust-router
    let x = 1; // <- your extra code, if you want
    //do what you want with the variables passed
    let http_verb = #http_verb;
    let url_route = #url_route;
    // println!("Testing http_verb: {:}", http_verb);
    // println!("Testing url_route: {:}", url_route);

    //#(#stmts)* //call the original statements and return the original outcome
    (#block, #http_verb, #url_route) //return the original outcome
    }
    };
    output.into()
}

//Let's do function-like macro illustration here too
//they are similar to declarative macros but macro_rules!
//is not used for matching.
//Similar but more powerful than declarative macros
//Function-like macros are executed not at runtime
//but at compile time.
//Function-like macros take a TokenStream parameter and
//their definition manipulates that TokenStream using
//Rust code as the other two types of procedural macros do.

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    //input //Return same if nothing to be done. Obviously useless!

    //idea from https://stackoverflow.com/questions/61169932/how-do-i-get-the-value-and-type-of-a-literal-in-a-procedural-macro
    let input = input.into_iter().collect::<Vec<_>>();
    if input.len() != 1 {
        let msg = format!("expected exactly one input token, got {}", input.len());
        return quote! { compile_error!(#msg) }.into();
    }

    let string_lit = match StringLit::try_from(&input[0]) {
        // Error if the token is not a string literal
        Err(e) => return e.to_compile_error(),
        Ok(lit) => lit,
    };

    // `StringLit::value` returns the actual string value represented by the
    // literal. Quotes are removed and escape sequences replaced with the
    // corresponding value.
    let sql = string_lit.value();

    // Validate the sql statement using some logic
    // For example, check if it contains a SELECT condition
    if sql.contains("select") {
        // Return the valid sql statement as a new TokenStream
        TokenStream::from(quote! {
        #sql
        })
    } else {
        // Return an error as a compile_error macro invocation
        syn::Error::new(
            sql.span(),
            "Invalid sql statement: missing SELECT condition",
        )
        .to_compile_error()
        .into()
    }
}
