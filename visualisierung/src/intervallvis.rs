use std::time::{Duration, Instant};

pub struct LivePlot {
    pub data: Vec<f64>,
    pub last_update: Instant,
    pub counter: f64,
    pub interval: Duration,
}

impl LivePlot {
    /// Neues LivePlot-Objekt, Intervalldauer in Sekunden
    pub fn new(interval_secs: u64) -> Self {
        Self {
            data: Vec::new(),
            last_update: Instant::now(),
            counter: 0.0,
            interval: Duration::from_secs(interval_secs),
        }
    }

    /// Wert an die Daten anhängen
    pub fn append(&mut self, value: f64) {
        self.data.push(value);
        if self.data.len() > 200 {
            self.data.remove(0); // Begrenze die Länge für Performance
        }
    }

    /// Prüft, ob das Intervall abgelaufen ist; wenn ja, generiere neuen Wert
    pub fn update(&mut self) -> Option<f64> {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.interval {
            self.counter += 0.1;
            let value = (self.counter).sin(); // Beispielwert
            self.append(value);
            self.last_update = now;
            Some(value)
        } else {
            None
        }
    }
}
