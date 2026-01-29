#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BioDomain {
    Blood,
    Sugar,
    Smart,
    Wave,
    Brain,
    Oxygen,
}

impl BioDomain {
    pub fn hex_tag(&self) -> &'static str {
        match self {
            BioDomain::Blood  => "0xB10oD001",
            BioDomain::Sugar  => "0x5U6AR002",
            BioDomain::Smart  => "0x5MART003",
            BioDomain::Wave   => "0xWA4E004",
            BioDomain::Brain  => "0x8RAIN005",
            BioDomain::Oxygen => "0x0XY6EN06",
        }
    }
}
