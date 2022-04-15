pub enum AsStr<'s> {
    AsStr(&'s str),
    AsString(String),
}

impl<'s> AsStr<'s> {
    pub fn create_as_str(src: &'s str) -> Self {
        Self::AsStr(src)
    }

    pub fn create_as_string(src: String) -> Self {
        Self::AsString(src)
    }

    pub fn get_str(&self) -> &str {
        match self {
            AsStr::AsStr(src) => src,
            AsStr::AsString(src) => src,
        }
    }
}

impl<'s> std::fmt::Display for AsStr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsStr::AsStr(src) => write!(f, "{}", src),
            AsStr::AsString(src) => write!(f, "{}", src),
        }
    }
}
