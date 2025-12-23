use std::fmt;

/// User-facing errors that should exit cleanly without stack traces.
#[derive(Debug)]
pub struct UserError {
    pub message: String,
    pub hint: Option<String>,
}

impl UserError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\n\nTip: {hint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for UserError {}
