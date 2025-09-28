use krark::*;
use odin::*;

fn main() -> Result<(), String> {
    let mut krark_harness = KrarkHarness::new("Lexer full tests".to_string());
    krark_harness.run(|card, mut results| {
        match card.oracle_text {
            Some(text) => {
                let oracle_text = text.to_ascii_lowercase();
                let lexed = lexer::lex(&oracle_text);
                results.assert_ok(lexed, format!("Check the oracle text has been parsed"));
            }
            None => { /* Pass */ }
        }
        results
    });

    Ok(())
}
