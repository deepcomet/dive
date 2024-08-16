use miette::Diagnostic;
use thiserror::Error;

/// Result type with default values.
pub type DiveResult<T=()> = Result<T, DiveError>;

/// Dive DNS errors.
#[derive(Error, Debug, Diagnostic)]
#[diagnostic(url(docsrs))]
pub enum DiveError {
  /// Domain validation errors.
  #[error("Domain name is not valid: {0}")]
  DomainNotValid(#[from] DomainValidationError),
}

/// Domain validation errors.
#[derive(Error, Debug, Diagnostic)]
pub enum DomainValidationError {
  /// IDNA parsing failed. Specific errors are not returned, see [`idna::Errors`].
  #[error("IDNA domain name parsing failed; unable to convert Unicode to ASCII")]
  #[diagnostic(
    url("https://url.spec.whatwg.org/#concept-domain-to-ascii"),
    help("Domain names must comply to UTS-46 encoding standards")
  )]
  IdnaToAsciiFailed(#[from] idna::Errors),

  /// Domain does not match expected root.
  #[error("Root mismatch: expected {expect:?}, got {actual:?}")]
  RootMismatch { expect: String, actual: String },

  /// Domain segments splitted failed.
  #[error("Unable to parse domain segments")]
  SegmentsNotValid {},

  /// Domain levels out of exoected range.
  #[error("Domain levels out of range: {actual:?} is not in {expect:?}")]
  #[diagnostic(help("Levels are defined by the number of '.' chars in your domain name"))]
  LevelsOutOfRange { expect: String, actual: String },
}
