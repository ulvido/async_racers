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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Race<'a> {
    pub name: String,
    pub laps: u8,
    pub racers: Vec<&'a Racer>,
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
        self.racers.push(racer);
    }

    pub fn start_race_single_threaded(&mut self) -> Racer {
        for lap in 0..self.laps {
            for racer in self.racers.iter_mut() {
                racer.clone().do_lap(lap);
            }
        }
        Racer::default()
    }
}

impl<'a> Default for Race<'a> {
    fn default() -> Self {
        Self {
            name: "".into(),
            laps: 10,
            racers: Vec::new(),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Racer {
    pub name: String,
    pub completed_lap_count: u8,
    pub lap_times: Vec<u8>,
    pub best_lap_time: Option<u8>,
}

impl Default for Racer {
    fn default() -> Self {
        Self {
            name: "Schumaher".into(),
            completed_lap_count: 0,
            lap_times: Vec::new(),
            best_lap_time: None,
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
    pub fn do_lap(mut self, lap: u8) {
        // let laptime: u8 = rand::thread_rng().gen();
        // tokio::time::sleep(tokio::time::Duration::from_millis(laptime.into())).await;
        let laptime: u8 = rand::thread_rng().gen_range(99..=255);
        std::thread::sleep(std::time::Duration::from_millis(laptime.into()));
        println!("{} finished lap {} in {}ms.", self.name, lap, laptime);
        self.lap_times.push(laptime);
        self.completed_lap_count += 1;
        if let Some(best_lap_time) = self.best_lap_time {
            if laptime < best_lap_time {
                self.best_lap_time = Some(laptime)
            }
        } else {
            self.best_lap_time = Some(laptime)
        }
    }
}

// impl std::future::Future for Race<'_> {
//   type Output = Racer;
//   fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//       std::
//   }
// }
