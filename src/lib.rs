use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};
/// A macro attribute to mark the kernel entry point.
///
/// # Example
///
/// ```ignore
/// #![no_std]
///
/// use axmacro::main;

/// #[axmacro::main]
/// pub fn main() {
///     ax_println!("hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let main_fn = parse_macro_input!(item as ItemFn);
    let main_block = &main_fn.block;

    quote!(
        #[unsafe(no_mangle)]
        fn main(){
            #main_block
        }
    )
    .into()
}

/// A macro attribute for the panic handler.
///
/// The attributed function will be used to override Arceos's default
/// implementation of Rust's `#[panic_handler]`. The function takes a single
/// parameter of type `&core::panic::PanicInfo` and does not return.
#[proc_macro_attribute]
pub fn panic_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let handler_fn = parse_macro_input!(item as ItemFn);
    let handler_fn_name = &handler_fn.sig.ident;

    quote!(
        #[unsafe(no_mangle)]
        extern "Rust" fn __arceos_panic_handler(info: &core::panic::PanicInfo) -> ! {
            #handler_fn_name(info);
        }

        #[expect(unused)]
        #handler_fn
    )
    .into()
}
