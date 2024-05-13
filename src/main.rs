use tide::Request;
use tide::prelude::*;
use std::env;
use clap::{ arg, Arg, ArgAction, command, Command, value_parser };

#[derive(Debug, Deserialize)]
struct ArdReq {
    rgb: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("debug").short('d').action(ArgAction::SetTrue)
        )
        .subcommand(
            Command::new("run")
                .about("specify which fucntionality you'd like to run")
                .arg(arg!(-l --list "lists test values")
                    .action(ArgAction::SetTrue))
                .arg(arg!(<SETUP>)
                    .value_parser(value_parser!(u16).range(1..)))
        )
        .get_matches();

    let setup: u16 = *matches
    .get_one::<u16>("SETUP")
    .expect("'SETUP' is required and parsing will fail if its missing");
    println!("PORT = {setup}");

    println!("name: {:?}", matches.get_one::<String>("name"));

    let path: std::path::PathBuf = env::current_dir()?;
    println!("The current directory is {}", path.display());
    let mut app = tide::new();
    app.at("/:identifier/rgb").post(set_rgb);
    let _ = app.at("/www").serve_dir("./www");
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
 
async fn set_rgb(mut req: Request<()>) -> tide::Result {
    let ArdReq { rgb } = req.body_json().await?;
    Ok(format!("Hello, {}! ", rgb).into())
}

