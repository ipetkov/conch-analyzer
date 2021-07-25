use conch_parser::lexer::Lexer;
use conch_parser::parse::DefaultParser;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Write;

use wasm_bindgen::prelude::*;

mod histogram;
mod utils;
mod visit;

type ParseResult<T> = conch_parser::parse::ParseResult<T, void::Void>;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn build_histogram(map: ParseResult<HashMap<String, usize>>) -> String {
    let map = match map {
        Ok(m) => m,
        Err(e) => return format!("parse error: {}", e),
    };

    let do_format = || {
        let mut data = map.into_iter().collect::<Vec<(String, usize)>>();

        // First sort by frequency (higher first), then sort alphabetically
        data.sort_by(|(first_name, first_count), (second_name, second_count)| {
            let cmp = first_count.cmp(second_count).reverse();

            if let Ordering::Equal = cmp {
                first_name.cmp(second_name)
            } else {
                cmp
            }
        });

        let mut ret = String::from("histogram (top):\n");
        for (name, count) in &data {
            writeln!(&mut ret, "{}: {}", count, name)?;
        }

        data.sort_by(|(first_name, _), (second_name, _)| first_name.cmp(second_name));
        writeln!(&mut ret, "\nhistogram (alpha):")?;
        for (name, count) in &data {
            writeln!(&mut ret, "{}: {}", name, count)?;
        }

        Ok(ret)
    };

    do_format().unwrap_or_else(|e: std::fmt::Error| format!("oh no: {}", e))
}

#[wasm_bindgen]
pub fn cmd_histogram(s: &str) -> String {
    let lex = Lexer::new(s.chars());
    let parser = DefaultParser::new(lex);

    build_histogram(histogram::Histogram::histogram(parser))
}
