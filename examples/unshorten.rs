use std::time::Duration;
use urlexpand;

fn main() {
    let url = "https://bit.ly/3alqLKi";
    println!("{}\nis_shortened? {}\nExpanded URL = {:?}",
        url,
        urlexpand::is_shortened(url),
        urlexpand::unshorten(url, Some(Duration::from_secs(10)))
    );
    println!();
}
