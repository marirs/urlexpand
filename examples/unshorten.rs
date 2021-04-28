use std::{env, process::exit, time::Duration};
use urlexpand;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("pass a url to expand: (eg: ./unhorten https://bit.ly/3alqLKi)");
        exit(1);
    }
    let url = args[1].to_owned();

    println!(
        "{}\nis_shortened? {}\nExpanded URL = {:?}",
        url,
        urlexpand::is_shortened(&url),
        urlexpand::unshorten(&url, Some(Duration::from_secs(20)))
    );
    println!();
}
