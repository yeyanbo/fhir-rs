use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum ValidateStatus {
    Success,
    Error,
    Warn,
    Fatal,
    Info,
    Skip,
}

impl Into<CodeDt> for ValidateStatus {
    fn into(self) -> CodeDt {
        match self {
            ValidateStatus::Success => CodeDt::new("success"),
            ValidateStatus::Error => CodeDt::new("error"),
            ValidateStatus::Warn => CodeDt::new("warning"),
            ValidateStatus::Fatal => CodeDt::new("fatal"),
            ValidateStatus::Info => CodeDt::new("information"),
            ValidateStatus::Skip => CodeDt::new("information"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidateResultItem {
    pub status: ValidateStatus,
    pub path: String,
    pub expression: String,
    pub message: String,
}

impl ValidateResultItem {
    pub fn new(status: ValidateStatus, path: &String, expression: &String, message: String) -> Self {
        Self {
            status,
            path: path.clone(),
            expression: expression.clone(),
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
            severity: Some(self.status.into()),
            code: Some(CodeDt::new("processing")),
            details: None,
            diagnostics: Some(StringDt::new(self.message)),
            location: Some(vec![StringDt::new(self.path)]),
            expression: Some(vec![StringDt::new(self.expression)]),
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
