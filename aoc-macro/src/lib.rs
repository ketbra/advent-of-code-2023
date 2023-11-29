use proc_macro::TokenStream;
use quote::quote;
// use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    // let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
    //     [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/{}.in", day.token()),
    //     _ => panic!("Expected one integer argument"),
    // };
    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      // const INPUT: &str = include_str!(#input_path);
      #aoc_solution
      fn _aoc_main_main() -> anyhow::Result<()> {
          // let this_file = file!();

          let year = 2023;
          // println!("defined in file: {this_file}");

          let (_, day) = lazy_regex::regex_captures!(r#"day(\d+).rs$"#, file!())
              .context("File must be named in day\\d+.rs")?;
          let day = day.parse::<usize>().unwrap();

          let home_dir = std::env::var("HOME")?;
          let cache_fn = format!("{}/.aoc/{}/{}.inp", home_dir, year, day);
          println!("Cache file={}", cache_fn);
          let input_cache_path = std::path::Path::new(&cache_fn);
          if !input_cache_path.exists() {
              use std::fs::File;
              use std::io::{Write, BufReader, BufRead, Error};
              // Create parent folder
              println!("Creating {:?}", input_cache_path.parent().unwrap());
              std::fs::create_dir_all(input_cache_path.parent().unwrap()).unwrap();

              // Download and save the data
              let mut f = std::fs::File::create(input_cache_path)?;
              let input = aoc::get_input(day, year)?;
              std::fs::write(input_cache_path, input);
          }

          // Slurp the cache file
          let input = std::fs::read_to_string(input_cache_path)?;
          println!("Day is {:?}", day);

          // let now = ::std::time::Instant::now();
          aoc_solution(&input)?;
        // let elapsed = now.elapsed();
        // println!("Part one: {}", p1);
        // println!("Part two: {}", p2);
        // if elapsed.as_millis() > 0 {
        //   println!("Time: {}ms", elapsed.as_millis());
        // } else {
        //   println!("Time: {}μs", elapsed.as_micros());
          // }

          Ok(())
      }
      fn main() {

          // _aoc_main_main().unwrap();
      }
    };
    TokenStream::from(tokens)
}
