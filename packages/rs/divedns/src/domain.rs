use std::fmt::Display;

use idna::{
  domain_to_ascii_cow,
  uts46::{Hyphens, Uts46},
  AsciiDenyList,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::DomainValidationError, util::Range, DiveError, DiveResult};

/// Domain struct. Should never be instantiated directly; use [`Domain::new`] for default
/// validation or [`DomainValidator`] for custom options.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema)]
pub struct Domain(String);

impl Domain {
  /// Create a new domain with default validation settings.
  pub fn new(domain: &str) -> DiveResult<Self> {
    DomainValidator::default().validate(domain)
  }

  /// Get the Punycode ASCII representation of the domain. See [`parse_domain_ascii`].
  pub fn to_punycode(&self) -> DiveResult<String> {
    parse_domain_ascii(&self.0)
  }

  /// Get domain root (last level).
  pub fn root(&self) -> Option<String> {
    self.0.split('.').last().map(String::from)
  }

  /// Count domain levels (number of '.' chars).
  pub fn levels(&self) -> usize {
    self.0.matches('.').count()
  }
}

impl Display for Domain {
  /// Print domain as Unicode.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// Configurable domain validator.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema)]
pub struct DomainValidator {
  /// Required root domain.
  pub root: Option<String>,
  /// Required range of levels/subdomains. Counts base but not root (e.g. "hello.dive" has 1).
  pub levels: Option<Range>,
  /// Range of allowed total domain length.
  pub length: Option<Range>,
}

impl DomainValidator {
  /// Create a domain validator.
  pub const fn new(root: Option<String>, levels: Option<Range>, length: Option<Range>) -> Self {
    Self { root, levels, length }
  }

  /// Validate a domain name.
  pub fn validate(&self, maybe_domain: &str) -> DiveResult<Domain> {
    let normalized = parse_domain_unicode(maybe_domain)?;
    let parts = normalized.split('.').collect::<Vec<&str>>();
    if let Some(expected_root) = self.root.as_ref() {
      let actual_root = *parts.last().ok_or_else(|| DomainValidationError::SegmentsNotValid {})?;
      if actual_root != expected_root {
        return Err(DiveError::domain_root_mismatch(expected_root, actual_root));
      }
    }
    if let Some(expected_levels) = self.levels.as_ref() {
      let actual_level = parts.len();
      if !expected_levels.includes(&actual_level) {
        return Err(DiveError::domain_levels_out_of_range(
          expected_levels.to_owned(),
          actual_level,
        ));
      }
    }
    if let Some(expected_length) = self.length.as_ref() {
      let actual_length = normalized.len();
      if !expected_length.includes(&actual_length) {
        return Err(DiveError::domain_length_out_of_range(
          expected_length.to_owned(),
          actual_length,
        ));
      }
    };
    Ok(Domain(normalized))
  }
}

impl Default for DomainValidator {
  /// Defaukt validation settings:
  /// - Any root.
  /// - Any number of levels.
  /// - Any length (that can parse as a valid domain name).
  fn default() -> Self {
    Self::new(None, None, None)
  }
}

/// Parse domain name and get Unicode normalized respresentation. Uses
/// [`idna::uts46::Uts64::to_unicode`].
pub fn parse_domain_unicode(domain: &str) -> DiveResult<String> {
  let (normalized, res) =
    Uts46::default().to_unicode(domain.as_bytes(), AsciiDenyList::URL, Hyphens::Allow);
  res.map_err(DomainValidationError::from)?;
  Ok(normalized.to_string())
}

/// Parse domain name and get ASCII Punycode normalized representation. Uses
/// [`idna::domain_to_ascii_cow`].
pub fn parse_domain_ascii(domain: &str) -> DiveResult<String> {
  domain_to_ascii_cow(domain.as_bytes(), AsciiDenyList::URL)
    .map(String::from)
    .map_err(|e| DomainValidationError::from(e).into())
}
