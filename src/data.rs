use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Label(String);
impl Label {
    pub fn new(label: String) -> Result<Self, &'static str> {
        if label.is_empty() {
            return Err("Label cannot be empty.");
        }
        if label.chars().count() >= 256 {
            return Err("Label is too long.");
        }
        // TODO escape/sanitize html?
        Ok(Self(label))
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
enum Goal {
    Increase,
    #[default]
    Decrease,
    Maintain {
        target: Option<i32>,
    },
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
enum OutputType {
    #[default]
    Summary,
    Statistics,
    Graph,
    Logs,
    Options,
}

#[derive(Default, Debug, Clone)]
enum InputType {
    #[default]
    Occurance,
    Quantity(i32),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Category {
    id: Uuid,
    label: Label,
    goal: Option<Goal>,
    active_output: OutputType,
}

#[derive(Clone, Debug, Default, Store, Deserialize, Serialize)]
pub struct State {
    categories: Vec<Category>,
    active_category: Option<Category>,
    interaction_stage: UserLifecycleStage,
}

#[derive(PartialEq, Clone, Debug, Default, Deserialize, Serialize)]
pub enum UserLifecycleStage {
    #[default]
    Onboarding,
    AddingCategoryModal,
    Regular,
}

#[cfg(test)]
mod tests {
    use crate::data::Label;

    #[test]
    fn test_label() {
        // Non-empty
        assert!(Label::new("".into()).is_err());
        // Not too long
        assert!(Label::new("test".repeat(120).into()).is_err());
        // Ok
        assert!(Label::new("some string".into()).is_ok());
    }
}
