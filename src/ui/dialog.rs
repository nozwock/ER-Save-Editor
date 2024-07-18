pub fn show_dialog(modal: &egui_modal::Modal, body: impl std::fmt::Display, icon: egui_modal::Icon) {
    modal.dialog().with_body(body).with_icon(icon).open();
}

pub fn show_err<T>(modal: &egui_modal::Modal, inner: anyhow::Result<T>) {
    if let Err(err) = inner {
        show_dialog(modal, err, egui_modal::Icon::Error);
    }
}