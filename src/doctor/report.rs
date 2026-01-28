use crate::doctor::checks::{CheckResult, CheckStatus};
use crate::output::OutputMode;
use serde::Serialize;

#[derive(Serialize)]
pub struct DoctorReport {
    pub checks: Vec<CheckResultJson>,
    pub summary: Summary,
}

#[derive(Serialize)]
pub struct CheckResultJson {
    pub id: String,
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

#[derive(Serialize)]
pub struct Summary {
    pub pass: usize,
    pub warn: usize,
    pub fail: usize,
}

impl From<&CheckResult> for CheckResultJson {
    fn from(result: &CheckResult) -> Self {
        CheckResultJson {
            id: result.id.to_string(),
            status: match result.status {
                CheckStatus::Pass => "pass".to_string(),
                CheckStatus::Warn => "warn".to_string(),
                CheckStatus::Fail => "fail".to_string(),
            },
            message: result.message.clone(),
            hint: result.hint.clone(),
        }
    }
}

pub fn format_report(results: &[CheckResult], mode: OutputMode) -> String {
    match mode {
        OutputMode::Json => {
            let summary = Summary {
                pass: results.iter().filter(|r| r.status == CheckStatus::Pass).count(),
                warn: results.iter().filter(|r| r.status == CheckStatus::Warn).count(),
                fail: results.iter().filter(|r| r.status == CheckStatus::Fail).count(),
            };
            let report = DoctorReport {
                checks: results.iter().map(|r| r.into()).collect(),
                summary,
            };
            serde_json::to_string_pretty(&report).unwrap_or_default()
        }
        OutputMode::Human => {
            let mut output = String::new();
            output.push_str("PortaQEMU Diagnostics\n");
            output.push_str("=====================\n\n");
            
            for result in results {
                let symbol = match result.status {
                    CheckStatus::Pass => "✓",
                    CheckStatus::Warn => "⚠",
                    CheckStatus::Fail => "✗",
                };
                output.push_str(&format!("{} {}: {}\n", symbol, result.id, result.message));
                if let Some(hint) = &result.hint {
                    output.push_str(&format!("  Hint: {}\n", hint));
                }
            }
            
            let pass = results.iter().filter(|r| r.status == CheckStatus::Pass).count();
            let warn = results.iter().filter(|r| r.status == CheckStatus::Warn).count();
            let fail = results.iter().filter(|r| r.status == CheckStatus::Fail).count();
            
            output.push_str(&format!("\nSummary: {} pass, {} warn, {} fail\n", pass, warn, fail));
            
            output
        }
    }
}
