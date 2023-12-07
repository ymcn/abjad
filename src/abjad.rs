use std::{
  collections::HashSet,
  cmp::Ordering,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mark {
  Fatha, Kasra,
  Dumma, Shadda,
  Sukun,
}

/// Represents a specific Arabic letter, or "abjad".
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Abjad {
  Alif,
  Beh, Teh, Theh,
  Jeem, Ha, Kha,
  Del, Thel,
  Reh, Zayn,
  Seen, Sheen,
  Saad, Daad, Ta, Tha,
  Ayn, Ghayn,
  Feh, Qaf, Kaf,
  Lam, Meem, Nun,
  Heh,
  Waw,
  Yeh,
}

/// Forms of an abjad will determine the order it was written in, and thus help to establish the foundational
/// logic for forming "awzaan" ("patterns") and "juthur" ("roots", a basic collection of abjads to form sets of related
/// words and meanings).
/// 
/// [`FormedAbjad`](self::FormedAbjad) contain how an abjad was written (i.e, which abjad and which form
/// it was written in), and allow for comparison to later establish meaning.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Form {
  /// Stands on its own; does not, as written, connect into the next cursive letter.
  Isolated,

  /// Written as beginning without a preceding connection to another cursive letter.
  Initial,

  /// Connects between two non-`Isolated` forms.
  Medial,

  /// A letter written in its final form with only one preceding connection.
  Final,
}

/// The ordered structure in which an abjad was processed from the input parser.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FormedAbjad {
  pub abjad: Abjad,
  pub form: Form,
  pub marks: Vec<Mark>,
}

impl PartialOrd for FormedAbjad {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.form.partial_cmp(&other.form)
  }
}

impl Ord for FormedAbjad {
  fn cmp(&self, other: &Self) -> Ordering {
    // There is no case where the ordering fails, because the order which `Form` defines each variation
    // already implies the ordering and derived the implementations.
    self.partial_cmp(other).unwrap()
  }
}

impl Abjad {
  pub fn all_cases() -> HashSet<Abjad> {
    use Abjad::*;

    HashSet::from_iter(vec![
      Alif,
      Beh, Teh, Theh,
      Jeem, Ha, Kha,
      Del, Thel,
      Reh, Zayn,
      Seen, Sheen,
      Saad, Daad, Ta, Tha,
      Ayn, Ghayn,
      Feh, Qaf, Kaf,
      Lam, Meem, Nun,
      Heh,
      Waw,
      Yeh,
    ].into_iter())
  }

  pub fn is_triple(self) -> Option<Self> {
    if Triples::cases().contains(&self) {
      Some(self)
    } else {
      None
    }
  }

  pub fn is_dual(self) -> Option<Self> {
    if Duals::cases().contains(&self) {
      Some(self)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct Duals;

impl Duals {
  pub fn cases() -> HashSet<Abjad> {
    use Abjad::*;

    HashSet::from_iter(vec![
      Alif,
      Del, Thel,
      Reh, Zayn,
      Waw,
    ].into_iter())
  }
}

#[derive(Debug)]
pub struct Triples;

impl Triples {
  pub fn cases() -> HashSet<Abjad> {
    &Abjad::all_cases() - &Duals::cases()
  }
}
