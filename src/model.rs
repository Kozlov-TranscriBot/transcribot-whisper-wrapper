pub enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
    Large
}

const TINY_LITERAL: &str = "tiny";
const BASE_LITERAL: &str = "base";
const SMALL_LITERAL: &str = "small";
const MEDIUM_LITERAL: &str = "medium";
const LARGE_LITERAL: &str = "large";

impl ModelSize {
    pub fn from(str: &str) -> Result<Self, String> {
        match str {
            TINY_LITERAL => Ok(ModelSize::Tiny),
            BASE_LITERAL => Ok(ModelSize::Base),
            SMALL_LITERAL => Ok(ModelSize::Small),
            MEDIUM_LITERAL => Ok(ModelSize::Medium),
            LARGE_LITERAL => Ok(ModelSize::Large),
            _ => Err(String::from("Invalid model size name"))
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ModelSize::Tiny => TINY_LITERAL,
            ModelSize::Base => BASE_LITERAL,
            ModelSize::Small => SMALL_LITERAL,
            ModelSize::Medium => MEDIUM_LITERAL,
            ModelSize::Large => LARGE_LITERAL,
        }
    }
}