use crate::app::diagnostics::app_diagnostics::AppDiagnostics;

/// Formats app diagnostics for compact status-bar display.
pub fn app_diagnostics_status_line(diagnostics: &AppDiagnostics) -> String {
    format!("FPS {} | Redraws {}", diagnostics.fps, diagnostics.redraws)
}

#[cfg(test)]
mod tests {
    use crate::app::diagnostics::app_diagnostics_status_line::app_diagnostics_status_line;
    use crate::app::diagnostics::new_app_diagnostics::new_app_diagnostics;

    /// Verifies the displayed diagnostics order and separators.
    #[test]
    fn formats_diagnostics_status_line() {
        let mut diagnostics = new_app_diagnostics();
        diagnostics.fps = 12;
        diagnostics.redraws = 34;
        assert_eq!(
            app_diagnostics_status_line(&diagnostics),
            "FPS 12 | Redraws 34"
        );
    }
}
