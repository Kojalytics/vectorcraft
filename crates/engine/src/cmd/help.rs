//! Help → project links (the website and this app's page). The commands return the URL; the
//! frontend opens it.

use serde_json::{Value, json};

use super::*;

/// The app's id on the website.
pub const APP_ID: &str = "vectorcraft";
pub const WEBSITE_URL: &str = "https://kojalytics.com";

/// This app's page on the website.
pub fn app_page_url() -> String {
    format!("{WEBSITE_URL}/apps/{APP_ID}")
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.website", "Kojalytics Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "VectorKoja on kojalytics.com", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {website, appPage}", always, |_, _| {
            Ok(json!({ "website": WEBSITE_URL, "appPage": app_page_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://www.kojalytics.com/koja-apps");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://kojalytics.com");
    }
}
