#[derive(PartialEq)]
pub(crate) enum SchemaRule {
    All,
    NoneExcept {
        whitelist: Vec<String>,
    }
}

impl SchemaRule {
    pub(crate) fn from_whitelist(whitelist: Vec<String>) -> Self {
        Self::NoneExcept { whitelist }
    }

    pub(crate) fn check(&self, identifier: impl Into<String>) -> bool {
        match self {
            Self::All => true,
            Self::NoneExcept { whitelist } => whitelist.contains(&identifier.into()),
        }
    }
}
