use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableEnum {
    Users,
    Roles,
    Permissions,
}

impl fmt::Display for TableEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table_str = match self {
            TableEnum::Users => "Users",
            TableEnum::Roles => "Roles",
            TableEnum::Permissions => "Permissions",
        };
        write!(f, "{}", table_str)
    }
}
