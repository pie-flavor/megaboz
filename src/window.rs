use crate::*;

impl ZMachine {
    /// Returns the value of a window property.
    pub fn window_property(&self, property_id: WindowProperty) {
        unimplemented!()
    }
}

/// All the window properties understood by the Z-machine.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WindowProperty {
    CoordinateY,
    CoordinateX,
    SizeY,
    SizeX,
    CursorY,
    CursorX,
    MarginSizeLeft,
    MarginSizeRight,
    NewlineInterruptRoutine,
    InterruptCountdown,
    TextStyle,
    ColorData,
    FontNumber,
    FontSize,
    Attributes,
    LineCount,
    TrueForegroundColor,
    TrueBackgroundColor,
}

impl WindowProperty {
    /// A convenience array of all the [`WindowProperty`] values.
    pub const VALUES: [WindowProperty; 18] = [
        WindowProperty::CoordinateY,
        WindowProperty::CoordinateX,
        WindowProperty::SizeY,
        WindowProperty::SizeX,
        WindowProperty::CursorY,
        WindowProperty::CursorX,
        WindowProperty::MarginSizeLeft,
        WindowProperty::MarginSizeRight,
        WindowProperty::NewlineInterruptRoutine,
        WindowProperty::InterruptCountdown,
        WindowProperty::TextStyle,
        WindowProperty::ColorData,
        WindowProperty::FontNumber,
        WindowProperty::FontSize,
        WindowProperty::Attributes,
        WindowProperty::LineCount,
        WindowProperty::TrueForegroundColor,
        WindowProperty::TrueBackgroundColor,
    ];
}
