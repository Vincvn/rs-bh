use uuid::Uuid;

pub fn v4() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}