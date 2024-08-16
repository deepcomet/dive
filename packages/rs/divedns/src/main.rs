use clap::{Parser, Subcommand};
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
    #[arg(long, visible_alias = "root")]
    expect_root: Option<String>,
    /// Expected domain levels range (e.g. "1..3" (default inclusive), "(1..3]", etc)
    #[arg(long, visible_alias = "levels")]
    expect_levels: Option<String>,
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
      CliDomainCommand::Validate { domain, expect_root, expect_levels } => {
        let validator = if expect_root.is_none() && expect_levels.is_none() {
          DomainValidator::default()
        } else {
          DomainValidator::new(expect_root, expect_levels.map(|range| range.parse()).transpose()?)
        };
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
