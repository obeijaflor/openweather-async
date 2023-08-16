pub enum Units {
    Standard,
    Metric,
    Imperial,
}

impl Units {
    pub fn value(&self) -> &str {
        match *self {
            Units::Standard => "standard",
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    }
}
