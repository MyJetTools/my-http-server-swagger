pub enum SourceToRead {
    FormData,
    QueryString,
}
impl SourceToRead {
    pub fn get_source_variable(&self) -> &str {
        match self {
            SourceToRead::FormData => "form_data",
            SourceToRead::QueryString => "query_string",
        }
    }
}
