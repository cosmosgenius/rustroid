use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_cmd_app() -> clap::App<'static, 'static> {
    let app = App::new("rustroid")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("cert")
                .about("Commands related to cert")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("install")
                        .about("Install certificate")
                        .arg(
                            Arg::with_name("path")
                                .multiple(true)
                                .takes_value(true)
                                .required(true)
                                .short("p")
                                .long("path")
                                .help("Path to the certificate file"),
                        ),
                ),
        );
    return app;
}
