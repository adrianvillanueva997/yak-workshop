use std::env;

use yak_cli::read_xml_content;

fn main() {
    let xml_path = env::args()
        .nth(1)
        .expect("Please provide a path to the xml file");
    let days = env::args()
        .nth(2)
        .expect("Days are missing")
        .parse::<u32>()
        .unwrap();
    let herd = read_xml_content(xml_path);
    let mut total_whool = 0;
    let mut total_milk = 0.0;
    for yak in herd.yak {
        let mut current_day = 0;
        let mut whool_production = 0;
        let mut milk_production = 0.0;
        while current_day < days && yak.is_alive() {
            if yak.can_be_shaved(current_day) {
                whool_production += 1;
            }
            milk_production += yak.calculate_milk_production(current_day);
            current_day += 1;
        }
        total_whool += whool_production;
        total_milk += milk_production;
    }
    println!("Whool: {total_whool} Milk: {total_milk}")
}
