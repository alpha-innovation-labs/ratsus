use crate::app::test_support::app_fixture::app_fixture;
use crate::app::test_support::dormant_session::dormant_session;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::ui::left_panel::active_content::ActiveLeftPaneContent;
use crate::ui::left_panel::content::LeftPaneContent;

/// Verifies chat content exposes footer shortcuts without status text.
#[test]
fn chat_footer_contract_has_shortcuts_without_status() {
    let mut app =
        app_fixture(vec![dormant_session("one", "one", "/tmp/project-one")]).expect("app fixture");
    app.active_main_pane_tab = MainPaneTab::Chat;
    let content = ActiveLeftPaneContent::for_app(&mut app);

    let items = content.footer_items();

    assert!(items.iter().any(|item| item.key == "Space"));
    assert_eq!(content.footer_status(), None);
}

/// Verifies files content exposes footer shortcuts with selected-path status.
#[test]
fn files_footer_contract_has_shortcuts_and_selected_status() {
    let mut app = app_fixture(Vec::new()).expect("app fixture");
    app.active_main_pane_tab = MainPaneTab::Files;
    let content = ActiveLeftPaneContent::for_app(&mut app);

    let items = content.footer_items();
    let status = content.footer_status().expect("file status");

    assert!(items.iter().any(|item| item.key == "enter"));
    assert!(!status.starts_with("Selected: "));
    assert!(!status.is_empty());
}
