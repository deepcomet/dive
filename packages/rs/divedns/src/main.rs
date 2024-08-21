use std::str::FromStr;

use clap::{Parser, Subcommand};
use divedns::util::Range;
use divedns::DomainValidator;
use miette::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
  /// Manipulate domains
  #[command(alias = "d")]
  Domain {
    #[command(subcommand)]
    command: CliDomainCommand,
  },
}

#[derive(Subcommand, Debug)]
enum CliDomainCommand {
  /// Validate name
  #[command(alias = "v")]
  Validate {
    /// Domain name
    domain: String,
    /// Expected domain root
    #[arg(long)]
    root: Option<String>,
    /// Expected domain levels range (e.g. "1..3" (default inclusive), "(1..3]", etc)
    #[arg(long)]
    levels: Option<String>,
    /// Expected domain length range
    #[arg(long)]
    length: Option<String>,
  },
  /// View domain information
  #[command(alias = "i")]
  Info {
    /// Domain name
    domain: String,
  },
}

pub fn main() -> Result<()> {
  let cli = Cli::parse();
  match cli.command {
    CliCommand::Domain { command } => match command {
      CliDomainCommand::Validate { domain, root, levels, length } => {
        let validator = DomainValidator::new(
          root,
          levels.map(|lvl| Range::from_str(lvl.as_str())).transpose()?,
          length.map(|len| Range::from_str(len.as_str())).transpose()?,
        );
        let domain = validator.validate(&domain)?;
        println!("{}", domain);
      }
      CliDomainCommand::Info { domain } => {
        let domain = divedns::Domain::new(&domain)?;
        println!("{}:", domain);
        println!("  ASCII: {}", domain.to_punycode()?);
        println!("  Root: {}", domain.root().unwrap_or_default());
        println!("  Levels: {}", domain.levels());
      }
    },
  };
  Ok(())
}
