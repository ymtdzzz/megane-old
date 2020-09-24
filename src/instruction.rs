pub enum Instruction {
    FetchLogGroups,
    FetchLogEvents(String, String),
}
