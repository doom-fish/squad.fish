use std::{
    ascii,
    fmt::{self, Write},
};

/// A four-character code.
///
/// The characters are stored in big-endian byte order.
///
/// See [documentation](https://developer.apple.com/documentation/kernel/fourcharcode?language=objc).
///
/// # Examples
///
/// Use [`from_chars`](Self::from_chars) to create an instance from a byte
/// string literal:
///
/// ```
/// use fruity::core::FourCharCode;
///
/// const APPL: FourCharCode = FourCharCode::from_chars(*b"APPL");
/// ```
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FourCharCode(u32);

impl fmt::Debug for FourCharCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format as escaped ASCII string.

        write!(f, "\"")?;

        for ch in self
            .into_chars()
            .iter()
            .flat_map(|&b| ascii::escape_default(b))
        {
            f.write_char(ch as char)?;
        }

        write!(f, "\"")
    }
}

impl FourCharCode {
    /// Returns an instance from the integer value.
    #[inline]
    pub const fn from_int(int: u32) -> Self {
        Self(int)
    }

    /// Returns an instance from the 4-character code.
    #[inline]
    pub const fn from_chars(chars: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(chars))
    }

    /// Returns this descriptor's integer value.
    #[inline]
    pub const fn into_int(self) -> u32 {
        self.0
    }

    /// Returns this descriptor's 4-character code.
    #[inline]
    pub const fn into_chars(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Returns `true` if all of the characters in `self` are ASCII.
    #[inline]
    pub const fn is_ascii(&self) -> bool {
        const NON_ASCII: u32 = u32::from_be_bytes([128; 4]);

        self.0 & NON_ASCII == 0
    }

    /// Returns `true` if all of the characters in `self` are ASCII graphic
    /// characters: U+0021 '!' ..= U+007E '~'.
    #[inline]
    pub const fn is_ascii_graphic(&self) -> bool {
        matches!(
            self.into_chars(),
            [b'!'..=b'~', b'!'..=b'~', b'!'..=b'~', b'!'..=b'~'],
        )
    }
}
