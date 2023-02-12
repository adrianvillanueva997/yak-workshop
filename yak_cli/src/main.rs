use yak::Yak;

fn main() {
    println!("Hello, world!");

    let shaved = Yak::new(4.0).can_be_shaved(13.0);
    println!("{}", shaved)
}
