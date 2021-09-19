use uuid::Uuid;

pub fn get_uuid_v4() -> String {
    let new_uuid = Uuid::new_v4().to_hyphenated().to_string();
    new_uuid
}
