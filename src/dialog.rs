pub use native_dialog::*;
pub fn message(title: &str, message: &str, message_type: MessageType, is_confirm: bool)-> bool{
    let dialog = MessageDialog::new()
    .set_type(message_type)
    .set_title(&title)
    .set_text(&message);
    if is_confirm {
        let result = dialog.show_confirm().unwrap();
        return result
    }else{
        dialog.show_alert().unwrap();
        return false
    };
}

pub fn error(msg: &str, title: Option<&str>) {
    let title = title.unwrap_or("Lỗi");
    message(title, msg, MessageType::Error, false);
}

pub fn info(msg: &str, title: Option<&str>) {
    let title = title.unwrap_or("Thông Báo");
    message(title, msg, MessageType::Info, false);
}

pub fn warn(msg: &str, title: Option<&str>) {
    let title = title.unwrap_or("Cảnh Báo");
    message(title, msg, MessageType::Warning, false);
}