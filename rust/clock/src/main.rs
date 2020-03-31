use clock::Clock;

fn main() {
    let c2 = Clock::new(2, 20).add_minutes(-3000);
    println!("{}", c2);
}