#[derive(Debug, Copy, Clone)]
pub enum Icon {
    Language,
    DraftingCompass,
}

impl Icon {
    pub fn to_class(self) -> &'static str {
        match self {
            Icon::Language => "fa-language",
            Icon::DraftingCompass => "fa-drafting-compass",
        }
    }
}
