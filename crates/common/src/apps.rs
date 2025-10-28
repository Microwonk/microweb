use strum::EnumIter;
pub use strum::IntoEnumIterator;

#[derive(EnumIter, Clone, PartialEq, Eq)]
pub enum Apps {
    Blog,
    Www,
    Auth,
    Files,
    SandBox,
}

impl Apps {
    pub fn prefix(&self) -> &'static str {
        match self {
            Apps::Blog => "blog.",
            Apps::Www => "www.",
            Apps::Auth => "auth.",
            Apps::Files => "files.",
            Apps::SandBox => "sandbox.",
        }
    }

    pub fn starts_with<S: AsRef<str>>(&self, string: S) -> bool {
        string.as_ref().starts_with(self.prefix())
    }

    pub fn is_leptos(&self) -> bool {
        !matches!(self, Apps::Files | Apps::SandBox)
    }

    pub fn url(&self) -> String {
        format!("{}://{}{}", crate::PROTOCOL, self.prefix(), crate::DOMAIN)
    }
}
