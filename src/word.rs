use std::{
  collections::HashSet,
  str::FromStr,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
  abjad::FormedAbjad,
};

pub struct Word {
  pub letters: HashSet<FormedAbjad>,
}

impl FromStr for Word {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let graphs = s.graphemes(true);
    
  }
}
