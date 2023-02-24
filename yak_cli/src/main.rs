use std::env;

use yak_cli::read_xml_content;

fn main() {
    let xml_path = env::args()
        .nth(1)
        .expect("Please provide a path to the xml file");
    let days = env::args().nth(2).expect("Days are missing");
    let herd = read_xml_content(xml_path);
    for yak in herd.yak {
        println!("{yak:?}");
        if yak.is_alive() {
            println!("{} is alive", yak.name());
        } else {
            println!("{} is dead", yak.name());
        }
    }
}
