#[derive(Debug)]
pub enum PrintFormat {
    Json,
    Table,
    List,
}

impl PrintFormat {
    pub fn variants() -> &'static [&'static str] {
        &["table", "list", "json"]
    }
}

impl std::str::FromStr for PrintFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "table" => Ok(Self::Table),
            "list" => Ok(Self::List),
            "json" => Ok(Self::Json),
            format => Err(format!("I don't know what {:?} means", format)),
        }
    }
}
