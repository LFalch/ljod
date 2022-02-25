use std::fmt::{self, Display};

pub const PRIM: Stress = Stress::PrimaryStress;
pub const SEC: Stress = Stress::SecondaryStress;
pub const NS: Stress = Stress::None;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stress {
    PrimaryStress,
    SecondaryStress,
    None
}

impl Display for Stress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stress::None if f.sign_plus() => write!(f, "."),
            Stress::None => Ok(()),
            Stress::PrimaryStress => write!(f, "ˈ"),
            Stress::SecondaryStress => write!(f, "ˌ"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NumberedTone(u8);

impl NumberedTone {
    // `n` is a number from 0 to 9, 0 is no tone/unmarked tone and the rest are that number in superscript.
    pub fn new(n: u8) -> Self {
        match n {
            0..=10 => NumberedTone(n),
            _ => panic!("Invalid value, `n` has to be in 0..=9"),
        }
    }
}

impl Display for NumberedTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 if f.sign_plus() => write!(f, "."),
            0 => Ok(()),
            1 => write!(f, "¹"),
            2 => write!(f, "²"),
            3 => write!(f, "³"),
            4 => write!(f, "⁴"),
            5 => write!(f, "⁵"),
            6 => write!(f, "⁶"),
            7 => write!(f, "⁷"),
            8 => write!(f, "⁸"),
            9 => write!(f, "⁹"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Syllable<C, V, S = Stress> {
    stress: S,
    onset: Vec<C>,
    nucleus: V,
    coda: Vec<C>,
}

impl<C, V, S> Syllable<C, V, S> {
    pub fn new(stress: S, onset: Vec<C>, nucleus: V, coda: Vec<C>) -> Self {
        Syllable { stress, onset, nucleus, coda }
    }
}
impl<S> Syllable<char, String, S> {
    pub fn from(stress: S, onset: &str, nucleus: &str, coda: &str) -> Self {
        Syllable { stress, onset: onset.chars().collect(), nucleus: nucleus.to_owned(), coda: coda.chars().collect() }
    }
}

impl<C: Display, V: Display, S: Display> Display for Syllable<C, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.sign_plus() {
            write!(f, "{:+}", self.stress)?;
        } else {
            write!(f, "{}", self.stress)?;
        }

        for c in &self.onset {
            write!(f, "{c}")?;
        }
        write!(f, "{}", self.nucleus)?;
        for c in &self.coda {
            write!(f, "{c}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Word<C, V, S = Stress>(pub Vec<Syllable<C, V, S>>);

impl<C: Display, V: Display, S: Display> Display for Word<C, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter();

        // Do not write syllable boundaries for the alternate style
        if f.alternate() {
            for s in iter {
                write!(f, "{s}")?;
            }
        } else {
            if let Some(s) = iter.next() {
                write!(f, "{s}")?;
            }

            for s in iter {
                write!(f, "{s:+}")?;
            }
        }

        Ok(())
    }
}

// Make trait for parsing a string into a syllable.
// Make function that can take one of those traits and parse a word
