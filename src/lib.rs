use rand::Rng;

/// An F1 Race Event
/// ```rust
/// // example race
/// let race1 = Race {
///     name: "Monaco Grand Prix".into(),
///     laps: 88,
///     racers: vec![],
///     winner: None,
/// }
///
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

    // pub async fn start(&mut self) -> Racer {
    //     for lap in 0..self.laps {
    //         self.racers.iter_mut().for_each(|racer| {
    //             racer.get_mut().do_lap();
    //         });
    //     }
    //     Racer::default()
    // }
}

impl<'a> Default for Race<'a> {
    fn default() -> Self {
        Self {
            name: "".into(),
            laps: 88,
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
///    current_lap: 0,
///    best_lap_time: None,
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Racer {
    pub name: String,
    pub current_lap: u8,
    pub lap_times: Vec<u8>,
    pub best_lap_time: Option<u8>,
}

impl Default for Racer {
    fn default() -> Self {
        Self {
            name: "Schumaher".into(),
            current_lap: 0,
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
    pub async fn do_lap(mut self) {
        let laptime: u8 = rand::thread_rng().gen();
        self.lap_times.push(laptime);
        tokio::time::sleep(tokio::time::Duration::from_millis(laptime.into())).await;
        self.current_lap += 1;
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
