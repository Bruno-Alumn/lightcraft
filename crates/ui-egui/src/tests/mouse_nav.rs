//! Headless tests of the mouse's Back and Forward buttons (issue #3303).

use std::time::Duration;

use serde_json::json;

use crate::headless::Headless;
use crate::state::{Dialog, ViewMode};
use crate::{LightcraftApp, Services};

const T: Duration = Duration::from_secs(20);
const SETTLE: Duration = Duration::from_secs(120);

/// Press and release one mouse button over the photo, with real egui pointer events.
fn press(h: &mut Headless, button: egui::PointerButton) {
    let pos = h.app.canvas_rect.map_or(egui::pos2(600.0, 400.0), |r| r.center());
    h.app.synthetic.push(egui::Event::PointerMoved(pos));
    h.app.synthetic.push(egui::Event::PointerButton { pos, button, pressed: true, modifiers: Default::default() });
    h.app.synthetic.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
    h.settle(SETTLE);
}

/// Forward (Extra2) shows the next photo and Back (Extra1) the previous one, as → and ← do; a
/// dialog keeps them, as it keeps the keys.
#[test]
fn mouse_back_and_forward_buttons_step_through_the_photos() {
    let app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), Services { png: None, ..Default::default() });
    let mut h = Headless::new(app, [1200.0, 800.0], 1.0);
    let ids = h.app.session.visible_cloned();
    let (first, second) = (ids[2], ids[3]);
    let r = h.request("engine.execute", json!({"command": "library.select", "params": {"ids": [first.0], "active": first.0}}), T);
    assert_eq!(r["ok"], true, "{r}");
    let r = h.request("ui.set", json!({"view": "detail"}), T);
    assert_eq!(r["ok"], true, "{r}");
    h.settle(SETTLE);
    assert_eq!((h.app.ui.view, h.app.session.active()), (ViewMode::Detail, Some(first)));

    press(&mut h, egui::PointerButton::Extra2);
    assert_eq!(h.app.session.active(), Some(second), "Forward shows the next photo");
    press(&mut h, egui::PointerButton::Extra1);
    assert_eq!(h.app.session.active(), Some(first), "Back shows the previous one");

    // an open dialog keeps the buttons
    h.app.ui.dialog = Some(Dialog::About);
    press(&mut h, egui::PointerButton::Extra2);
    assert_eq!(h.app.session.active(), Some(first));
}
