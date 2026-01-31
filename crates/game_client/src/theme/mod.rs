//! UI theme utilities and presets.

use bevy::prelude::*;

pub mod interaction;
pub mod palette;
pub mod widget;

pub fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}
