use async_racers::{Race, Racer};

fn main() {
    let mut race1 = Race::with_name("İstanbul Grandprix");

    let racer1 = Racer::with_name("Ulvi Yelen");
    let racer2 = Racer::with_name("Schumaher");
    let racer3 = Racer::with_name("Hakkinen");

    race1.add_racer(&racer1);
    race1.add_racer(&racer2);
    race1.add_racer(&racer3);

    // println!("{race1:#?}");
    // println!("{racer1:#?}");
    
    race1.start_race_single_threaded();
}
