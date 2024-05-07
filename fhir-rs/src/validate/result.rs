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
pub struct ValidateResult{
    pub fatal_count: usize,
    pub error_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
    pub skip_count: usize,
    pub success_count: usize,
    items: Vec<ValidateResultItem>
}

impl ValidateResult {

    pub fn new() -> Self {
        Self{
            fatal_count: 0,
            error_count: 0,
            warn_count: 0,
            info_count: 0,
            skip_count: 0,
            success_count: 0,
            items: vec![],
        }
    }

    pub fn add_result_item(&mut self, items: Vec<ValidateResultItem>) {
        for item in items {
            match item.status {
                ValidateStatus::Success => self.success_count += 1,
                ValidateStatus::Error => self.error_count += 1,
                ValidateStatus::Warn => self.warn_count += 1,
                ValidateStatus::Fatal => self.fatal_count += 1,
                ValidateStatus::Info => self.info_count += 1,
                ValidateStatus::Skip => self.skip_count += 1,
            }
            self.items.push(item)
        }
    }
}

impl Into<OperationOutcome> for ValidateResult {
    fn into(self) -> OperationOutcome {
        let mut outcome = OperationOutcome::default();

        for item in self.items {
            outcome = outcome.add_issue(item.into());
        }

        outcome
    }
}
