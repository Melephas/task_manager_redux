#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Task {
    name: String,
    time_elapsed: f64,
}

impl Task {
    /* Construction functions */
    pub fn new() -> Task {
        Default::default()
    }

    pub fn with_name(name: &str) -> Task {
        Task {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn with_values(name: &str, time_elapsed: f64) -> Task {
        Task {
            name: String::from(name),
            time_elapsed,
        }
    }


    /* Object methods */
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn time_elapsed(&self) -> &f64 {
        &self.time_elapsed
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn set_time_elapsed(&mut self, new_time_elapsed: f64) {
        self.time_elapsed = new_time_elapsed;
    }
}