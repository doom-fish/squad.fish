pub type UInt8 = ::std::os::raw::c_uchar;
pub type SInt8 = ::std::os::raw::c_schar;
pub type UInt16 = ::std::os::raw::c_ushort;
pub type SInt16 = ::std::os::raw::c_short;
pub type UInt32 = ::std::os::raw::c_uint;
pub type SInt32 = ::std::os::raw::c_int;
pub type SInt64 = ::std::os::raw::c_longlong;
pub type UInt64 = ::std::os::raw::c_ulonglong;
pub type Float32 = f32;
pub type Float64 = f64;
pub type Ptr = *mut ::std::os::raw::c_char;
pub type Handle = *mut Ptr;
pub type Size = ::std::os::raw::c_long;
pub type OSErr = SInt16;
#[allow(clippy::upper_case_acronyms)]
pub type BOOL = ::std::os::raw::c_schar;
pub type FourCharCode = UInt32;
pub type OSType = FourCharCode;
pub type OSTypePtr = *mut OSType;
pub type Boolean = ::std::os::raw::c_uchar;
pub type SizeT = ::std::os::raw::c_ulong;
pub type PidT = ::std::os::raw::c_int;
pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub const CMTIME_FLAGS_K_CMTIME_FLAGS_VALID: CMTimeFlags = 1;
pub const CMTIME_FLAGS_K_CMTIME_FLAGS_HAS_BEEN_ROUNDED: CMTimeFlags = 2;
pub const CMTIME_FLAGS_K_CMTIME_FLAGS_POSITIVE_INFINITY: CMTimeFlags = 4;
pub const CMTIME_FLAGS_K_CMTIME_FLAGS_NEGATIVE_INFINITY: CMTimeFlags = 8;
pub const CMTIME_FLAGS_K_CMTIME_FLAGS_INDEFINITE: CMTimeFlags = 16;
pub const CMTIME_FLAGS_K_CMTIME_FLAGS_IMPLIED_VALUE_FLAGS_MASK: CMTimeFlags = 28;
pub type CMTimeFlags = u32;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}
