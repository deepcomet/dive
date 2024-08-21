use derive_more::{Display, Error, From};
use miette::Diagnostic;

use crate::util::Range;

/// Result type with default values.
pub type DiveResult<T=()> = Result<T, DiveError>;

/// Dive DNS errors.
#[derive(Debug, Diagnostic, Display, Error, From)]
#[diagnostic(url(docsrs))]
pub enum DiveError {
  /// Domain validation errors.
  #[display("Domain name is not valid")]
  DomainNotValid { source: DomainValidationError },
}

impl DiveError {
  pub fn idna_to_ascii_failed(source: idna::Errors) -> Self {
    DomainValidationError::IdnaToAsciiFailed { source }.into()
  }

  pub fn domain_root_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
    DomainValidationError::RootMismatch { expected: expected.into(), actual: actual.into() }.into()
  }

  pub fn domain_segments_not_valid() -> Self {
    DomainValidationError::SegmentsNotValid {}.into()
  }

  pub fn domain_levels_out_of_range(expected_range: Range, actual: usize) -> Self {
    DomainValidationError::LevelsOutOfRange { expected_range, actual }.into()
  }

  pub fn domain_length_out_of_range(expected_range: Range, actual: usize) -> Self {
    DomainValidationError::LengthOutOfRange { actual, expected_range }.into()
  }
}

/// Domain validation errors.
#[derive(Debug, Diagnostic, Display, Error, From)]
pub enum DomainValidationError {
  /// IDNA parsing failed. Specific errors are not returned, see [`idna::Errors`].
  #[display("IDNA domain name parsing failed; unable to convert Unicode to ASCII")]
  #[diagnostic(
    url("https://url.spec.whatwg.org/#concept-domain-to-ascii"),
    help("Domain names must comply to UTS-46 encoding standards")
  )]
  #[from]
  IdnaToAsciiFailed { source: idna::Errors },

  /// Domain does not match expected root.
  #[display("Root mismatch: expected {expected}, got {actual}")]
  RootMismatch { expected: String, actual: String },

  /// Domain segments splitted failed.
  #[display("Unable to parse domain segments")]
  SegmentsNotValid {},

  /// Domain levels out of exoected range.
  #[display("Domain levels out of range: {actual} is not in {expected_range}")]
  #[diagnostic(help("Levels are defined by the number of '.' chars in your domain name"))]
  LevelsOutOfRange { actual: usize, expected_range: Range },

  /// Domain length out of expected range.
  #[display("Domain length out of expected range: {actual} is not in {expected_range}")]
  LengthOutOfRange { actual: usize, expected_range: Range },
}
