const COLORLESS: u8 = 0b0000_0000;
const WHITE: u8 = 0b0000_0001;
const BLUE: u8 = 0b0000_0010;
const BLACK: u8 = 0b0000_0100;
const RED: u8 = 0b0000_1000;
const GREEN: u8 = 0b0001_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color(u8);

impl Color {
    pub const fn colorless() -> Self {
        Self(COLORLESS)
    }

    pub const fn white() -> Self {
        Self(WHITE)
    }

    pub const fn blue() -> Self {
        Self(BLUE)
    }

    pub const fn black() -> Self {
        Self(BLACK)
    }

    pub const fn red() -> Self {
        Self(RED)
    }

    pub const fn green() -> Self {
        Self(GREEN)
    }

    pub const fn selesnya() -> Self {
        Self::green().and(Self::white())
    }

    pub const fn golgari() -> Self {
        Self::black().and(Self::green())
    }

    pub const fn izzet() -> Self {
        Self::red().and(Self::blue())
    }

    pub const fn and(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn is_white(&self) -> bool {
        self.0 & WHITE > 0
    }

    pub const fn is_blue(&self) -> bool {
        self.0 & BLUE > 0
    }

    pub const fn is_black(&self) -> bool {
        self.0 & BLACK > 0
    }

    pub const fn is_red(&self) -> bool {
        self.0 & RED > 0
    }

    pub const fn is_green(&self) -> bool {
        self.0 & GREEN > 0
    }
}

#[test]
fn test_color() {
    assert!(Color::white().is_white());
    assert!(!Color::white().is_blue());
    assert!(!Color::white().is_black());
    assert!(!Color::white().is_red());
    assert!(!Color::white().is_green());

    assert!(!Color::blue().is_white());
    assert!(Color::blue().is_blue());
    assert!(!Color::blue().is_black());
    assert!(!Color::blue().is_red());
    assert!(!Color::blue().is_green());

    assert!(!Color::black().is_white());
    assert!(!Color::black().is_blue());
    assert!(Color::black().is_black());
    assert!(!Color::black().is_red());
    assert!(!Color::black().is_green());

    assert!(!Color::red().is_white());
    assert!(!Color::red().is_blue());
    assert!(!Color::red().is_black());
    assert!(Color::red().is_red());
    assert!(!Color::red().is_green());

    assert!(!Color::green().is_white());
    assert!(!Color::green().is_blue());
    assert!(!Color::green().is_black());
    assert!(!Color::green().is_red());
    assert!(Color::green().is_green());

    assert!(!Color::colorless().is_white());
    assert!(!Color::colorless().is_blue());
    assert!(!Color::colorless().is_black());
    assert!(!Color::colorless().is_red());
    assert!(!Color::colorless().is_green());

    assert!(!Color::izzet().is_white());
    assert!(Color::izzet().is_blue());
    assert!(!Color::izzet().is_black());
    assert!(Color::izzet().is_red());
    assert!(!Color::izzet().is_green());
}
