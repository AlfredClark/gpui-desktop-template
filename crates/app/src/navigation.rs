#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Destination {
    Home,
    Settings,
    About,
}

impl Destination {
    pub const ALL: [Self; 3] = [Self::Home, Self::Settings, Self::About];

    pub fn label(self) -> &'static str {
        match self {
            Self::Home => "首页",
            Self::Settings => "设置",
            Self::About => "关于",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Home => 0,
            Self::Settings => 1,
            Self::About => 2,
        }
    }

    pub fn from_index(ix: usize) -> Self {
        match ix {
            1 => Self::Settings,
            2 => Self::About,
            _ => Self::Home,
        }
    }
}
