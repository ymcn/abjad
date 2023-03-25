pub mod abjad {
  /// Identifiers for Arabic letters.
  #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum Name {
    Alif {
      top_hamza: bool,
      bottom_hamza: bool,
    },
    Beh, Teh, Theh,
    Jeem, Ha, Kha,
    Del, Thel,
    Reh, Zayn,
    Seen, Sheen,
    Saad, Daad,
    Ta, Tha,
    Ain, Ghain,
    Feh, Qaf,
    Kaf,
    Lam, Meem, Nun,
    Heh, Waw,
    Yeh {
      maqsura: bool,
    },
  }

  /// Identifiers for identifying which form an abjad takes.
  #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum Form {
    Isolated,
    Initial,
    Medial,
    Final,
  }

  impl Name {
    /// Checks whether this `Abjad` is one that is marked with dots.
    pub fn dotable(&self) -> bool {
      use Name::{Alif, Kaf, Lam, Meem, Waw};

      matches!(self, Alif {..} | Kaf | Lam | Meem | Waw)
    }
  }
}

pub mod harakat {
  /// Symbols for identifying the marks/"harakat" of Arabic diacritics over and under each letter.
  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
  pub enum Name {
    /// Value varies by whether it's on top or bottom of the abjad.
    Fatha(bool),

    /// Value varies by whether it's on top of bottom of the abjad.
    Kasra(bool),

    /// Doubles the pronunciation of an abjad.
    Dumma,

    /// "Closes" the pronunciation of an abjad, instead of leaving it "open".
    Sukun,
  }
}
