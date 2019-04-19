#[macro_use]
extern crate clap;
mod cert;
mod cmd;

fn main() {
    let matches = cmd::get_cmd_app().get_matches();

    match matches.subcommand() {
        ("cert", Some(cert_match)) => match cert_match.subcommand() {
            ("install", Some(install_match)) => {
                cert::install_cert(install_match.values_of("path").unwrap().collect())
            }
            _ => {}
        },
        _ => {}
    }
}
