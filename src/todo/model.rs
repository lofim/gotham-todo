#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    content: String,
    id: u32
}

impl Task {
    pub fn new(content: String, id: u32) -> Task {
        Task {
            content,
            id,
        }
    }
}
