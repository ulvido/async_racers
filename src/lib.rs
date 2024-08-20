use std::{
    cell::{Cell, RefCell},
    sync::{Arc, Mutex},
};

use rand::Rng;

/// An F1 Race Event
/// ```rust
/// // example race
/// let race1 = Race {
///     name: "Monaco Grand Prix".into(),
///     laps: 10,
///     racers: vec![],
///     winner: None,
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Race<'a> {
    pub name: String,
    pub laps: u8,
    pub racers: Arc<Mutex<Vec<&'a Racer>>>,
    pub winner: Option<&'a Racer>,
}

impl<'a> Race<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn add_racer(&mut self, racer: &'a Racer) {
        self.racers.lock().unwrap().push(racer);
    }

    pub async fn start(&mut self) {
        for _ in 0..self.laps {
            for racer in self.racers.lock().unwrap().iter() {
                tokio::spawn(async move {
                    racer.do_lap().await;
                })
                .await
                .unwrap();
            }
        }
    }
}

impl<'a> Default for Race<'a> {
    fn default() -> Self {
        Self {
            name: "".into(),
            laps: 88,
            racers: Arc::new(Mutex::new(Vec::new())),
            winner: None,
        }
    }
}

/// An F1 Racer
/// ```rust
/// // example racer
/// let racer1 = Racer {
///    name: "Schumaher".into(),
///    completed_lap_count: 0,
///    best_lap_time: None,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Racer {
    pub name: String,
    pub current_lap: Arc<Mutex<u8>>,
    pub lap_times: Arc<Mutex<Vec<u8>>>,
    pub best_lap_time: Arc<Mutex<Option<u8>>>,
}

impl Default for Racer {
    fn default() -> Self {
        Self {
            name: "Schumaher".into(),
            current_lap: Arc::new(Mutex::new(0)),
            lap_times: Arc::new(Mutex::new(Vec::new())),
            best_lap_time: Arc::new(Mutex::new(None)),
        }
    }
}

impl Racer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    pub async fn do_lap(&self) {
        let laptime: u8 = rand::thread_rng().gen_range(100..=255);
        self.lap_times.lock().unwrap().push(laptime);
        tokio::time::sleep(tokio::time::Duration::from_millis(laptime.into())).await;
        let mut cl = self.current_lap.lock().unwrap();
        *cl += 1;
        if let Some(best_lap_time) = *self.best_lap_time.lock().unwrap() {
            if laptime < best_lap_time {
                *self.best_lap_time.lock().unwrap() = Some(laptime);
            }
        } else {
            *self.best_lap_time.lock().unwrap() = Some(laptime);
        }

        println!(
            "{} comleted lap {} in {}ms.",
            self.name,
            *self.current_lap.lock().unwrap(),
            laptime
        );
    }
}
