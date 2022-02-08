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

#[derive(Debug, Clone)]
pub struct Syllable<C, V> {
    stress: Stress,
    onset: Vec<C>,
    nucleus: V,
    coda: Vec<C>,
}

impl<C, V> Syllable<C, V> {
    pub fn new(stress: Stress, onset: Vec<C>, nucleus: V, coda: Vec<C>) -> Self {
        Syllable { stress, onset, nucleus, coda }
    }
}
impl Syllable<char, String> {
    pub fn from(stress: Stress, onset: &str, nucleus: &str, coda: &str) -> Self {
        Syllable { stress, onset: onset.chars().collect(), nucleus: nucleus.to_owned(), coda: coda.chars().collect() }
    }
}

impl<C: Display, V: Display> Display for Syllable<C, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.stress {
            Stress::None if f.sign_plus() => write!(f, ".")?,
            Stress::None => (),
            Stress::PrimaryStress => write!(f, "ˈ")?,
            Stress::SecondaryStress => write!(f, "ˌ")?,
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
pub struct Word<C, V>(pub Vec<Syllable<C, V>>);

impl<C: Display, V: Display> Display for Word<C, V> {
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
