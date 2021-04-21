use core::time::Duration;
use urlexpand;

fn main() {
    // let url = "https://bit.ly/3alqLKi";
    // println!("{}\nis_shortened? {}\nExpanded URL = {:?}",
    //     url,
    //     urlexpand::is_shortened(url),
    //     urlexpand::unshorten(url, Some(Duration::new(30,0)))
    // );
    // println!();

    let url = "http://goo.gl/cvSjeY";
    println!(
        "{}\nis_shortened? {}\nExpanded URL = {:?}",
        url,
        urlexpand::is_shortened(url),
        urlexpand::unshorten(url, Some(Duration::new(30, 0)))
    );
    println!();
}
