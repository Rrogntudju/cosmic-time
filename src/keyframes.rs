pub mod container;
pub mod button;
pub mod space;

use iced::Length;
use iced_native::widget;

pub use container::Container;
pub use button::Button;
pub use space::Space;

use std::time::Instant;

use crate::Timeline;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Repeat {
    Never,
    Forever,
}

impl std::default::Default for Repeat {
    fn default() -> Self {
        Repeat::Never
    }
}

pub trait IsChain {
    fn repeat(&self) -> Repeat;
}

pub fn get_length(
    id: &widget::Id,
    timeline: &Timeline,
    now: &Instant,
    index: usize,
    default: Length,
) -> Length {
    timeline.get(id, now, index)
        .map(Length::Fixed)
        .unwrap_or(default)
}
