use ljod::word::{Word, PRIM, SEC, NS, Syllable};

fn main() {
    let word = Word(vec![
        Syllable::from(PRIM, "s", "ʉuː", "ɹ"),
        Syllable::from(SEC, "", "eː", "p"),
        Syllable::from(NS, "l", "ɪ", "ɹ"),
    ]);

    println!("/{word:#}/");
}
