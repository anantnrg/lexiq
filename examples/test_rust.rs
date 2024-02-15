use lexiq::grammars::rust::RustLang;

fn main() {
    let mut grammar = RustLang::grammar();
    grammar.sort();
    let regexes = grammar.compile().tokenize(
        "// this is a comment"
    );
    println!("{regexes:?}");
}
