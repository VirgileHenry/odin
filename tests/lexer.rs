use krark::*;
use odin::*;

fn main() -> Result<(), String> {
    let mut krark_harness = KrarkHarness::new("Lexer test: all cards".to_string());
    krark_harness.run(|card, mut results| {
        if card.set != "fdn" {
            return results;
        }
        match card.oracle_text {
            Some(text) => {
                let oracle_text = lexer::preprocess(card.name, text);
                let lexed = lexer::lex(&oracle_text);
                results.assert_ok(lexed, format!("Check the oracle text has been parsed"));
            }
            None => { /* Pass */ }
        }
        results
    });

    Ok(())
}
