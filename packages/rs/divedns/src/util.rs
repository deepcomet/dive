use std::{fmt::Display, str::FromStr};

use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Bound<T: Ord> {
  Include(T),
  Exclude(T),
}

impl<T: Ord+Display> Display for Bound<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Bound::Include(value) => write!(f, "{}", value),
      Bound::Exclude(value) => write!(f, "{}", value),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Range<T: Ord=usize> {
  pub start: Bound<T>,
  pub end: Bound<T>,
}

impl<T: Ord> Range<T> {
  pub fn new(start: Bound<T>, end: Bound<T>) -> Self {
    Self { start, end }
  }

  pub fn includes(&self, value: &T) -> bool {
    match (&self.start, &self.end) {
      (Bound::Include(start), Bound::Include(end)) => start <= value && value <= end,
      (Bound::Include(start), Bound::Exclude(end)) => start <= value && value < end,
      (Bound::Exclude(start), Bound::Include(end)) => start < value && value <= end,
      (Bound::Exclude(start), Bound::Exclude(end)) => start < value && value < end,
    }
  }
}

impl<T: Ord+Display> Display for Range<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match (&self.start, &self.end) {
      (Bound::Include(start), Bound::Include(end)) => write!(f, "[{}..{}]", start, end),
      (Bound::Include(start), Bound::Exclude(end)) => write!(f, "[{}..{})", start, end),
      (Bound::Exclude(start), Bound::Include(end)) => write!(f, "({}..{}]", start, end),
      (Bound::Exclude(start), Bound::Exclude(end)) => write!(f, "({}..{})", start, end),
    }
  }
}

impl FromStr for Range<usize> {
  type Err = miette::Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s_trimmed = s.trim_matches(|c| c == '[' || c == ']' || c == '(' || c == ')');
    let parts: Vec<&str> = s_trimmed.split("..").collect();
    if parts.len() != 2 {
      return Err(miette::miette!("Invalid range format: {}", s));
    }
    let start_bound = if s.starts_with('(') {
      Bound::Exclude(parts[0].parse().into_diagnostic()?)
    } else {
      Bound::Include(parts[0].parse().into_diagnostic()?)
    };
    let end_bound = if s.ends_with(')') {
      Bound::Exclude(parts[1].parse().into_diagnostic()?)
    } else {
      Bound::Include(parts[1].parse().into_diagnostic()?)
    };
    Ok(Self::new(start_bound, end_bound))
  }
}
