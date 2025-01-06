use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use syn::{DeriveInput, LitStr}; // Use proc_macro2 for quote

#[proc_macro]
pub fn my_funtion_like_macro(_input: TokenStream) -> TokenStream {
    "fn say_hello() { println!(\"Hello, world!\"); }"
        .parse()
        .unwrap()
}

#[proc_macro_derive(MyDebug)]
pub fn my_derive_macro(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // extract fields from the struct
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    // generate the code for each field
    let field_code = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            write!(f, ", {}: {:?}", stringify!(#name), self.#name)?;
        }
    });

    let gen = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {{", stringify!(#name))?;
                #(#field_code)*
                write!(f, "}}")
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn my_attribute_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the function from the input
    let input = parse_macro_input!(input as ItemFn);

    // Extract the function name, signature, and block
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;

    // Generate the modified function with logging
    let expanded = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            println!("Entering function: {}", stringify!(#fn_name));

            let result = (|| #fn_block)();

            println!("Exiting function: {}", stringify!(#fn_name));
            println!("Execution duration: {:?} ms", start.elapsed().as_millis());

            result
        }
    };

    // Return the generated TokenStream
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn sql_query(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal
    let query = parse_macro_input!(input as LitStr);

    // Validate that the query starts with "SELECT"
    if !query
        .value()
        .trim_start()
        .to_uppercase()
        .starts_with("SELECT")
    {
        return syn::Error::new_spanned(query, "Invalid query: Only SELECT queries are allowed")
            .to_compile_error()
            .into();
    }

    // Generate the validated query as output
    let expanded = quote! {
        #query
    };

    expanded.into()
}
