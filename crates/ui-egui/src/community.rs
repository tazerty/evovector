//! Project links (About, Home): the app's source code on GitHub. Icons are Lucide's (ISC).
//!
//! Evora (EVORA-SIN-03, patch E1): the Upstream Discord button and the Upstream website and app
//! page links are removed. A modified version must not carry the Upstream marks
//! (docs/brand/LICENSE-brand.txt).

use egui::Ui;

use crate::theme::Tokens;
use crate::{EvoVectorApp, icons};

/// One link row: icon, label, opens the command's URL.
fn link(app: &mut EvoVectorApp, ui: &mut Ui, icon: &str, label: &str, cmd: &str, url: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        icons::icon(ui, icon, 16.0, t.icon);
        let r = ui.add(egui::Button::new(egui::RichText::new(label).color(t.accent).size(13.0)).frame(false)).on_hover_text(url);
        if r.clicked() {
            app.open_link(cmd);
        }
    });
}

/// The project links.
pub fn links(app: &mut EvoVectorApp, ui: &mut Ui) {
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let s = |k: &str| l[k].as_str().unwrap_or("").to_string();
    link(app, ui, "git-branch", tl!("Source code on GitHub"), "help.github", &s("github"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_clicks_open_the_right_urls() {
        use std::sync::{Arc, Mutex};
        let opened = Arc::new(Mutex::new(vec![]));
        let o = opened.clone();
        let services = crate::Services { open_url: Some(Box::new(move |u: &str| o.lock().unwrap().push(u.to_string()))), ..Default::default() };
        let mut app = EvoVectorApp::new(evovector_engine::Session::new(), services);
        crate::menus::invoke(&mut app, "help.github", serde_json::json!({}));
        assert_eq!(*opened.lock().unwrap(), ["https://github.com/tazerty/evovector"]);
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.github", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://github.com/tazerty/evovector");
        assert_eq!(opened.lock().unwrap().len(), 1);
        // Evora (E1): the Upstream links are not commands any more.
        assert!(app.run("help.discord", serde_json::json!({})).is_err());
        let ctx = egui::Context::default();
        crate::theme::install_fonts(&ctx);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| links(&mut app, ui));
        out.textures_delta.clear();
    }
}
