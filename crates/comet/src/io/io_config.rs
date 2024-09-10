pub struct IoConfig {
    data_dir: String,
}

impl IoConfig {
    pub fn builder() -> IoConfigBuilder {
        IoConfigBuilder::new()
    }

    pub fn data_dir(&self) -> Box<str> {
        Box::from(self.data_dir.as_str())
    }
}

pub struct IoConfigBuilder {
    data_dir: Option<String>,
}

impl IoConfigBuilder {
    pub fn new() -> Self {
        Self { data_dir: None }
    }

    pub fn data_dir(mut self, data_dir: String) -> Self {
        self.data_dir = Some(data_dir);
        self
    }

    pub fn build(self) -> IoConfig {
        IoConfig {
            data_dir: self.data_dir.unwrap(),
        }
    }
}
