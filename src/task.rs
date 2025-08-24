use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Task {
    name: String,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(name: &str, description: &str) -> Task {
        Task {
            name: name.into(),
            description: description.into(),
            completed: false,
        }
    }

    pub fn new_from_name(name: &str) -> Task {
        Task {
            name: String::from(name),
            description: String::new(),
            completed: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let completed = if self.completed() { '☑' } else { '☐' };
        write!(
            f,
            "{}  - {}: {}",
            completed,
            self.name(),
            self.description()
        )
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

    #[test]
    fn test_display() {
        let mut task = super::Task::new("test", "test");
        let formatted_task = format!("{}", task);
        assert_eq!(formatted_task, "☐ test - test");

        task.complete();
        let formatted_task = format!("{}", task);
        assert_eq!(formatted_task, "☑ test - test");
    }
}
