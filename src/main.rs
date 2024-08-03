use async_racers::{Race, Racer};

fn main() {
    let mut race1 = Race::with_name("İstanbul Grandprix");

    let racer1 = Racer::with_name("Ulvi Yelen");
    let racer2 = Racer::with_name("Michael Schumaher");
    let racer3 = Racer::with_name("Hakkinen");

    race1.add_racer(&racer1);
    race1.add_racer(&racer2);
    race1.add_racer(&racer3);
    
    println!("{race1:#?}");
    println!("{racer1:#?}");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = race1.start();

    rt.block_on(future);

    // println!("{race1:#?}");
    // println!("{racer1:#?}");
}
