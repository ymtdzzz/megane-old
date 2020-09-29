pub enum Instruction {
    FetchLogGroups,
    FetchLogEvents(String, String, i64, i64),
}
