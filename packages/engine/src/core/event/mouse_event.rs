use strum::Display;
use winit::event::MouseButton;

/// Events produced by a mouse.
#[derive(Debug, Display)]
pub enum MouseEvent {
    /// The attached `MouseButton` was pressed.
    Pressed(MouseButton),
    /// The attached `MouseButton` was released.
    Released(MouseButton),
    /// The mouse was moved
    Moved(u32, u32),
}
