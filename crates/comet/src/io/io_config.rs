pub struct IOConfig {
    data_dir: String,
}

impl IOConfig {
    pub fn builder() -> IOConfigBuilder {
        IOConfigBuilder::new()
    }

    pub fn data_dir(&self) -> Box<str> {
        Box::from(self.data_dir.as_str())
    }
}

pub struct IOConfigBuilder {
    data_dir: Option<String>,
}

impl IOConfigBuilder {
    pub fn new() -> Self {
        Self { data_dir: None }
    }

    pub fn data_dir(mut self, data_dir: String) -> Self {
        self.data_dir = Some(data_dir);
        self
    }

    pub fn build(self) -> IOConfig {
        IOConfig {
            data_dir: self.data_dir.unwrap(),
        }
    }
}
