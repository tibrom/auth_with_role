use crate::domain::error::service::ErrorService;
pub struct ErrorHandler;

impl ErrorService for ErrorHandler {
    fn critical_error(&self, err: &dyn std::error::Error) -> String {
        let mut message = String::from("[CRITICAL ERROR] ");
        self.format_error(err, &mut message);
        // Например, можно тут залогировать или отправить в мониторинг
        eprintln!("{}", message);
        message
    }

    fn not_critical_error(&self, err: &dyn std::error::Error) -> String {
        let mut message = String::from("[WARNING] ");
        self.format_error(err, &mut message);
        eprintln!("{}", message); // Можно просто логировать
        message
    }
}

impl ErrorHandler {
    fn format_error(&self, err: &dyn std::error::Error, message: &mut String) {
        let _ = write!(message, "{}", err);

        let mut source = err.source();
        while let Some(e) = source {
            let _ = write!(message, "\nCaused by: {}", e);
            source = e.source();
        }
    }
}