use async_racers::{Race, Racer};

fn main() {
    let mut race1 = Race::with_name("İstanbul Grandprix");

    let racer1 = Racer::with_name("Ulvi Yelen");

    race1.add_racer(&racer1);
    println!("{race1:#?}");
    println!("{racer1:#?}");
}
