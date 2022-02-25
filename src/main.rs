use ljod::word::{Word, PRIM, SEC, NS, NumberedTone, Syllable};

fn main() {
    let word = Word(vec![
        Syllable::from(PRIM, "s", "ʉuː", "ɹ"),
        Syllable::from(SEC, "", "eː", "p"),
        Syllable::from(NS, "l", "ɪ", "ɹ"),
    ]);

    println!("/{word}/");

    let word = Word(vec![
        Syllable::from(NumberedTone::new(2), "k", "oː", ""),
        Syllable::from(NumberedTone::new(0), "n", "a", ""),
    ]);

    println!("/{word:#}/");
}
