use loco_rs::prelude::*;

/// Render a single contacts view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "contacts/show.html", serde_json::json!({}))
}

/// Render a contacts create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "contacts/create.html", serde_json::json!({}))
}
