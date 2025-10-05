#[derive(Debug, Hash, Clone)]
pub struct NewTask {
    pub name: String,
    pub description: Option<String>,
}
