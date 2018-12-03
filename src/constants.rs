// some constants used by the program
// Since these constants were to be evaluated at runtime i can't exactly use pub const here
// Therefore I use the computed value and left a comment on how these values are constructed
// The compiler said something about nightly support for constant functions, until this is
// available in stable I'll leave this as it is.

pub const G13_LED_MODE_ENDPOINT: u8 = 0x21; // request_type(Direction::Out, RequestType::Class, Recipient::Interface);
pub const G13_LED_ENDPOINT: u8      = G13_LED_MODE_ENDPOINT;
pub const G13_KEY_INTERFACE: u8     = 1;
pub const G13_KEYS_ENDPOINT: u8     = 0x80 | G13_KEY_INTERFACE; // request_type(Direction::In, RequestType::Standard, Recipient::Device) | G13_KEY_INTERFACE;

