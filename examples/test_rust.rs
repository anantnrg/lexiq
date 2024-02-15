use lexiq::grammars::rust::RustLang;

fn main() {
    let mut grammar = RustLang::grammar();
    grammar.sort();
}
