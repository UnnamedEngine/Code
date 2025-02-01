use strum::Display;
use winit::keyboard::PhysicalKey;

#[derive(Debug, Display)]
pub enum KeyboardEvent {
    /// The attached `PhysicalKey` was pressed.
    Pressed(PhysicalKey),
    /// The attached `PhysicalKey` was released.
    Released(PhysicalKey),
}
