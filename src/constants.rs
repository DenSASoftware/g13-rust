// some constants used by the program
// Since these constants were to be evaluated at runtime i can't exactly use pub const here
// Therefore I use the computed value and left a comment on how these values are constructed
// The compiler said something about nightly support for constant functions, until this is
// available in stable I'll leave this as it is.

pub const G13_LED_MODE_ENDPOINT: u8 = 0x21; // request_type(Direction::Out, RequestType::Class, Recipient::Interface);
pub const G13_LED_ENDPOINT: u8      = G13_LED_MODE_ENDPOINT;
pub const G13_KEY_INTERFACE: u8     = 1;
pub const G13_LCD_INTERFACE: u8     = 2;
pub const G13_KEYS_ENDPOINT: u8     = 0x80 | G13_KEY_INTERFACE; // request_type(Direction::In, RequestType::Standard, Recipient::Device) | G13_KEY_INTERFACE;
pub const G13_LCD_ENDPOINT: u8      = 0x00 | G13_LCD_INTERFACE;

// width and height of the G13-lcd-screen. Actually the height is smaller, but thanks to how the
// pixels are laid out in the buffer we reserve memory for 48 rows. The G13-lcd-screen is 160x43
// pixels big, each pixel only being on or off. Each byte in the buffer maps to 8 consecutive
// pixels on the display, each of them in the same column. The first byte in the buffer maps to
// pixels (0, 0) to (0, 7) in the lcd-screen (where positions are denoted by (X, Y), X being the
// column index and Y the row index). The second byte maps to pixels (1, 0) to (1, 7) and so on.
// With that continuing, the first 160 bytes of the buffer map to the first 8 rows, the next 160
// bytes to the next 8 rows and so on. For the last 160 bytes the bits representing the nonexisting
// rows are ignored, that means in every byte the last 5 bits will be ignored.
pub const G13_LCD_WIDTH: usize = 160;
pub const G13_LCD_HEIGHT: usize = 48;
pub const G13_LCD_HEIGHT_IN_BYTES: usize = G13_LCD_HEIGHT / 8;
pub const G13_LCD_HEIGHT_REAL: usize = 43;
// The array-length is the length of the array you pass to the method. An byte-array with this size
// is big enough to fit information for the whole lcd-screen if laid out as described above.
pub const G13_LCD_ARRAY_LEN: usize = G13_LCD_WIDTH * G13_LCD_HEIGHT_IN_BYTES;
// The usb-data sent to the G13 starts with a buffer of 32 bytes, for whatever reason. Thanks to
// this the usb-data for the lcd-screen is slightly longer
pub const G13_LCD_BUFFER_PADDING: usize = 32;
pub const G13_LCD_BUFFER_LEN: usize = G13_LCD_BUFFER_PADDING + G13_LCD_ARRAY_LEN;

pub const G13_VENDOR_ID:  u16 = 0x046d;
pub const G13_PRODUCT_ID: u16 = 0xc21c;

pub const G13_KEYS_LENGTH: usize = 40;

