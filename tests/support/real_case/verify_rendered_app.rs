use anyhow::ensure;

/// Verifies the rendered app contains stable user-visible shell regions.
pub fn verify_rendered_app(rendered: &str) -> anyhow::Result<()> {
    ensure!(
        rendered.contains("Sessions"),
        "rendered app must show the Sessions tab"
    );
    ensure!(
        !rendered.contains("panicked"),
        "rendered app must not panic"
    );
    Ok(())
}
