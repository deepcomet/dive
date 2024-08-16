use std::fmt::Display;

use idna::{
  domain_to_ascii_cow,
  uts46::{Hyphens, Uts46},
  AsciiDenyList,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::DomainValidationError, util::Range, DiveResult};

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
  pub expect_root: Option<String>,
  /// Required range of levels/subdomains. Counts base but not root (e.g. "hello.dive" has 1).
  pub expect_levels: Option<Range>,
}

impl DomainValidator {
  /// Create a domain validator.
  pub const fn new(expect_root: Option<String>, expect_levels: Option<Range>) -> Self {
    Self { expect_root, expect_levels }
  }

  /// Validate a domain name.
  pub fn validate(&self, maybe_domain: &str) -> DiveResult<Domain> {
    let normalized = parse_domain_unicode(maybe_domain)?;
    let parts = normalized.split('.').collect::<Vec<&str>>();
    if let Some(expect_root) = self.expect_root.as_ref() {
      let actual_root = parts.last().ok_or(DomainValidationError::SegmentsNotValid {})?;
      if actual_root != expect_root {
        return Err(
          DomainValidationError::RootMismatch {
            expect: expect_root.to_string(),
            actual: actual_root.to_string(),
          }
          .into(),
        );
      }
    }
    if let Some(expect_levels) = self.expect_levels.as_ref() {
      let actual_level = parts.len();
      if !expect_levels.includes(&actual_level) {
        return Err(
          DomainValidationError::LevelsOutOfRange {
            expect: expect_levels.to_string(),
            actual: actual_level.to_string(),
          }
          .into(),
        );
      }
    }
    Ok(Domain(normalized))
  }
}

impl Default for DomainValidator {
  /// Defaukt validation settings:
  /// - No expected root.
  /// - Any number of levels.
  fn default() -> Self {
    Self { expect_root: None, expect_levels: None }
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
