use proc_macro::TokenStream;
use quote::quote;
// use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut solve_function = parse_macro_input!(input as ItemFn);
    solve_function.sig.ident = Ident::new("solve", solve_function.sig.ident.span());

    let tokens = quote! {
      // const INPUT: &str = include_str!(#input_path);
      #solve_function
      fn main() -> anyhow::Result<()> {
          let input = aoc::get_input_for_script(file!())?;

          let now = ::std::time::Instant::now();
          let elapsed = now.elapsed();
          let answer = solve(&input)?;
          println!("answer = {}", answer);
        // println!("Part one: {}", p1);
        // println!("Part two: {}", p2);
          if elapsed.as_millis() > 0 {
              println!("Time: {}ms", elapsed.as_millis());
          } else {
              println!("Time: {}μs", elapsed.as_micros());
          }

          Ok(())
      }
    };
    TokenStream::from(tokens)
}
