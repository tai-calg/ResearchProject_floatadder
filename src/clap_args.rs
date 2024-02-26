
#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum AdderType {
    Default = 0,
    WithoutRound = 1,
    ByCmpl = 2,
    TenAdderLG = 3,
    TenAdderOPA = 4,
}

impl std::fmt::Display for AdderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AdderType::Default => "Default",
            AdderType::WithoutRound => "WithoutRound",
            AdderType::ByCmpl => "ByCmpl",
            AdderType::TenAdderLG => "TenAdderLG",
            AdderType::TenAdderOPA => "TenAdderOPA",
        })
    }
}
