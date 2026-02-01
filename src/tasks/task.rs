use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Hash, Clone, Serialize, Deserialize, FromRow, Builder)]
pub struct Task {
    id: i32,
    name: String,
    description: Option<String>,
    completed: bool,
}


impl Task {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn set_completed(&mut self) {
        self.completed = true;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let completed = if self.completed() { '☑' } else { '☐' };

        if let Some(description) = self.description() {
            write!(
                f,
                "{}  - {}: {}",
                completed,
                self.name(),
                description,
            )
        } else {
            write!(f, "{}  - {}", completed, self.name())
        }
    }
}

mod tests {
    #[test]
    const fn test_send() {
        const fn is_send<T: Send>() {}
        is_send::<super::Task>();
    }

    #[test]
    const fn test_sync() {
        const fn is_sync<T: Sync>() {}
        is_sync::<super::Task>();
    }
}
