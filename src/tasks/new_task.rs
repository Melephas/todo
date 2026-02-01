use bon::Builder;

#[derive(Debug, Hash, Clone, Builder)]
pub struct NewTask {
    name: String,
    description: Option<String>,
}

impl NewTask {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
