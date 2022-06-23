use std::{env, process::exit};
use urlexpand;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("pass a url to expand: (eg: ./unhorten https://bit.ly/3alqLKi)");
        exit(1);
    }
    let url = args[1].to_owned();

    let x = urlexpand::is_shortened(&url);
    if x {
        match urlexpand::unshorten(&url, None).await {
            Ok(u) => println!("{}\nis_shortened? {}\nExpanded URL = {:?}", url, x, u),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("{} not a short url", url)
    }

    println!();
}
