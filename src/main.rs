use clap::{ArgAction, Command, arg};

fn main() {
  let matches = Command::new("Ruff")
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))

    .arg(arg!(-o --output <dir> "Set output directory").require_equals(true))
    .arg(arg!(-s --silent "Disable printing"))

    .arg(arg!(<file> "Files to compile").action(ArgAction::Append))

    .get_matches();

  match matches.get_many::<String>("file") {
    Some(_) => todo!(),
    None => unreachable!()
  }
}
