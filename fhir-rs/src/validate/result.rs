use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ValidateResultItem {
    pub severity: String,
    pub location: String,
    pub message: String,
}

impl ValidateResultItem {
    pub fn new(severity: &str, location: &String, message: String) -> Self {
        Self {
            severity: String::from(severity),
            location: location.clone(),
            message
        }
    }
}

impl Into<OperationOutcomeIssueBackboneElement> for ValidateResultItem {

    fn into(self) -> OperationOutcomeIssueBackboneElement {
        OperationOutcomeIssueBackboneElement {
            id: None,
            extension: None,
            modifier_extension: None,
            severity: Some(CodeDt::new(self.severity)),
            code: None,
            details: None,
            diagnostics: Some(StringDt::new(self.message)),
            location: None,
            expression: Some(vec![StringDt::new(self.location)]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidateResult(Vec<ValidateResultItem>);

impl ValidateResult {

    pub fn new() -> Self {
        ValidateResult(vec![])
    }

    pub fn add_result_item(&mut self, items: Vec<ValidateResultItem>) {
        self.0.extend(items)
    }
}
impl Into<OperationOutcome> for ValidateResult {
    fn into(self) -> OperationOutcome {
        let mut outcome = OperationOutcome::default();

        for item in self.0 {
            outcome = outcome.add_issue(item.into());
        }

        outcome
    }
}
