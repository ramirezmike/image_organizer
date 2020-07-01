#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizeMode {
    Copy,
    Move,
    Link,
}

impl OrganizeMode {
    // TODO force this to be the same size as the number of enum variants
    const MODES: [OrganizeMode; 3] = [OrganizeMode::Copy, OrganizeMode::Move, OrganizeMode::Link];

    pub fn next(self: &mut Self) {
        if let Some(mut current_mode) = OrganizeMode::MODES.iter().position(|x| x == self) {
            current_mode = current_mode + 1;
            if current_mode > 2 { current_mode = 0 }
            *self = OrganizeMode::MODES[current_mode];
        }
    }
}
