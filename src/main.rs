use evil::Supervillain;

fn main() {
    println!("Hello, world!");
    let _ = Supervillain::try_from("Doctor Doom").ok();
}
