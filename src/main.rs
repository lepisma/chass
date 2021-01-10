fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("chass")
        .version("0.1.0")
        .author("Abhinav Tushar <abhinav@lepisma.xyz>")
        .about("Basic lichess assistant")
        .arg(clap::Arg::with_name("name")
             .short("n")
             .long("name")
             .takes_value(true)
             .required(true)
             .help("Lichess user name"))
        .get_matches();

    let username = matches.value_of("name").unwrap();
    let url = "https://lichess.org/api/user/".to_string() + username + &"/current-game".to_string();
    let body = reqwest::blocking::get(&url)?.text()?;
    println!("Body:\n{}", body);

    Ok(())
}
