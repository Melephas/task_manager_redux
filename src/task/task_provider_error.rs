use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
    },
};

#[derive(Debug)]
pub struct TaskProviderError {
    inner: Option<&'static (dyn Error + 'static)>,
    message: String,
}

impl Display for TaskProviderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(inner_source) = self.inner.source() {
            writeln!(f, "TaskProviderError: {}", self.message);
            writeln!(f, "   inner: {}", inner_source)
        } else {
            writeln!(f, "TaskProviderError: {}", self.message)
        }
    }
}

impl Error for TaskProviderError {

}