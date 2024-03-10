#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]


#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct ConnectErr_Unrecognized {
    pub f1: roc_std::RocStr,
    pub f0: i32,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum discriminant_ConnectErr {
    AddrInUse = 0,
    AddrNotAvailable = 1,
    ConnectionRefused = 2,
    Interrupted = 3,
    PermissionDenied = 4,
    TimedOut = 5,
    Unrecognized = 6,
    Unsupported = 7,
}

impl core::fmt::Debug for discriminant_ConnectErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AddrInUse => f.write_str("discriminant_ConnectErr::AddrInUse"),
            Self::AddrNotAvailable => f.write_str("discriminant_ConnectErr::AddrNotAvailable"),
            Self::ConnectionRefused => f.write_str("discriminant_ConnectErr::ConnectionRefused"),
            Self::Interrupted => f.write_str("discriminant_ConnectErr::Interrupted"),
            Self::PermissionDenied => f.write_str("discriminant_ConnectErr::PermissionDenied"),
            Self::TimedOut => f.write_str("discriminant_ConnectErr::TimedOut"),
            Self::Unrecognized => f.write_str("discriminant_ConnectErr::Unrecognized"),
            Self::Unsupported => f.write_str("discriminant_ConnectErr::Unsupported"),
        }
    }
}

#[repr(C, align(8))]
pub union union_ConnectErr {
    AddrInUse: (),
    AddrNotAvailable: (),
    ConnectionRefused: (),
    Interrupted: (),
    PermissionDenied: (),
    TimedOut: (),
    Unrecognized: core::mem::ManuallyDrop<ConnectErr_Unrecognized>,
    Unsupported: (),
}

// const _SIZE_CHECK_union_ConnectErr: () = assert!(core::mem::size_of::<union_ConnectErr>() == 40);
const _ALIGN_CHECK_union_ConnectErr: () = assert!(core::mem::align_of::<union_ConnectErr>() == 8);

const _SIZE_CHECK_ConnectErr: () = assert!(core::mem::size_of::<ConnectErr>() == 40);
const _ALIGN_CHECK_ConnectErr: () = assert!(core::mem::align_of::<ConnectErr>() == 8);

impl ConnectErr {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_ConnectErr {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_ConnectErr>(*bytes.as_ptr().add(32))
        }
    }

    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_ConnectErr) {
        let discriminant_ptr: *mut discriminant_ConnectErr = (self as *mut ConnectErr).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

#[repr(C)]
pub struct ConnectErr {
    payload: union_ConnectErr,
    discriminant: discriminant_ConnectErr,
}

impl Clone for ConnectErr {
    fn clone(&self) -> Self {
        use discriminant_ConnectErr::*;

        let payload = unsafe {
            match self.discriminant {
                AddrInUse => union_ConnectErr {
                    AddrInUse: self.payload.AddrInUse.clone(),
                },
                AddrNotAvailable => union_ConnectErr {
                    AddrNotAvailable: self.payload.AddrNotAvailable.clone(),
                },
                ConnectionRefused => union_ConnectErr {
                    ConnectionRefused: self.payload.ConnectionRefused.clone(),
                },
                Interrupted => union_ConnectErr {
                    Interrupted: self.payload.Interrupted.clone(),
                },
                PermissionDenied => union_ConnectErr {
                    PermissionDenied: self.payload.PermissionDenied.clone(),
                },
                TimedOut => union_ConnectErr {
                    TimedOut: self.payload.TimedOut.clone(),
                },
                Unrecognized => union_ConnectErr {
                    Unrecognized: self.payload.Unrecognized.clone(),
                },
                Unsupported => union_ConnectErr {
                    Unsupported: self.payload.Unsupported.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

impl core::fmt::Debug for ConnectErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use discriminant_ConnectErr::*;

        unsafe {
            match self.discriminant {
                AddrInUse => {
                    let field: &() = &self.payload.AddrInUse;
                    f.debug_tuple("ConnectErr::AddrInUse").field(field).finish()
                }
                AddrNotAvailable => {
                    let field: &() = &self.payload.AddrNotAvailable;
                    f.debug_tuple("ConnectErr::AddrNotAvailable")
                        .field(field)
                        .finish()
                }
                ConnectionRefused => {
                    let field: &() = &self.payload.ConnectionRefused;
                    f.debug_tuple("ConnectErr::ConnectionRefused")
                        .field(field)
                        .finish()
                }
                Interrupted => {
                    let field: &() = &self.payload.Interrupted;
                    f.debug_tuple("ConnectErr::Interrupted")
                        .field(field)
                        .finish()
                }
                PermissionDenied => {
                    let field: &() = &self.payload.PermissionDenied;
                    f.debug_tuple("ConnectErr::PermissionDenied")
                        .field(field)
                        .finish()
                }
                TimedOut => {
                    let field: &() = &self.payload.TimedOut;
                    f.debug_tuple("ConnectErr::TimedOut").field(field).finish()
                }
                Unrecognized => {
                    let field: &ConnectErr_Unrecognized = &self.payload.Unrecognized;
                    f.debug_tuple("ConnectErr::Unrecognized")
                        .field(field)
                        .finish()
                }
                Unsupported => {
                    let field: &() = &self.payload.Unsupported;
                    f.debug_tuple("ConnectErr::Unsupported")
                        .field(field)
                        .finish()
                }
            }
        }
    }
}

impl Eq for ConnectErr {}

impl PartialEq for ConnectErr {
    fn eq(&self, other: &Self) -> bool {
        use discriminant_ConnectErr::*;

        if self.discriminant != other.discriminant {
            return false;
        }

        unsafe {
            match self.discriminant {
                AddrInUse => self.payload.AddrInUse == other.payload.AddrInUse,
                AddrNotAvailable => self.payload.AddrNotAvailable == other.payload.AddrNotAvailable,
                ConnectionRefused => {
                    self.payload.ConnectionRefused == other.payload.ConnectionRefused
                }
                Interrupted => self.payload.Interrupted == other.payload.Interrupted,
                PermissionDenied => self.payload.PermissionDenied == other.payload.PermissionDenied,
                TimedOut => self.payload.TimedOut == other.payload.TimedOut,
                Unrecognized => self.payload.Unrecognized == other.payload.Unrecognized,
                Unsupported => self.payload.Unsupported == other.payload.Unsupported,
            }
        }
    }
}

impl Ord for ConnectErr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ConnectErr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use discriminant_ConnectErr::*;

        use std::cmp::Ordering::*;

        match self.discriminant.cmp(&other.discriminant) {
            Less => Option::Some(Less),
            Greater => Option::Some(Greater),
            Equal => unsafe {
                match self.discriminant {
                    AddrInUse => self.payload.AddrInUse.partial_cmp(&other.payload.AddrInUse),
                    AddrNotAvailable => self
                        .payload
                        .AddrNotAvailable
                        .partial_cmp(&other.payload.AddrNotAvailable),
                    ConnectionRefused => self
                        .payload
                        .ConnectionRefused
                        .partial_cmp(&other.payload.ConnectionRefused),
                    Interrupted => self
                        .payload
                        .Interrupted
                        .partial_cmp(&other.payload.Interrupted),
                    PermissionDenied => self
                        .payload
                        .PermissionDenied
                        .partial_cmp(&other.payload.PermissionDenied),
                    TimedOut => self.payload.TimedOut.partial_cmp(&other.payload.TimedOut),
                    Unrecognized => self
                        .payload
                        .Unrecognized
                        .partial_cmp(&other.payload.Unrecognized),
                    Unsupported => self
                        .payload
                        .Unsupported
                        .partial_cmp(&other.payload.Unsupported),
                }
            },
        }
    }
}

impl core::hash::Hash for ConnectErr {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use discriminant_ConnectErr::*;

        unsafe {
            match self.discriminant {
                AddrInUse => self.payload.AddrInUse.hash(state),
                AddrNotAvailable => self.payload.AddrNotAvailable.hash(state),
                ConnectionRefused => self.payload.ConnectionRefused.hash(state),
                Interrupted => self.payload.Interrupted.hash(state),
                PermissionDenied => self.payload.PermissionDenied.hash(state),
                TimedOut => self.payload.TimedOut.hash(state),
                Unrecognized => self.payload.Unrecognized.hash(state),
                Unsupported => self.payload.Unsupported.hash(state),
            }
        }
    }
}

impl ConnectErr {
    pub fn is_AddrInUse(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::AddrInUse)
    }

    pub fn is_AddrNotAvailable(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::AddrNotAvailable)
    }

    pub fn is_ConnectionRefused(&self) -> bool {
        matches!(
            self.discriminant,
            discriminant_ConnectErr::ConnectionRefused
        )
    }

    pub fn is_Interrupted(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::Interrupted)
    }

    pub fn is_PermissionDenied(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::PermissionDenied)
    }

    pub fn is_TimedOut(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::TimedOut)
    }

    pub fn unwrap_Unrecognized(mut self) -> ConnectErr_Unrecognized {
        debug_assert_eq!(self.discriminant, discriminant_ConnectErr::Unrecognized);
        unsafe { core::mem::ManuallyDrop::take(&mut self.payload.Unrecognized) }
    }

    pub fn is_Unrecognized(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::Unrecognized)
    }

    pub fn is_Unsupported(&self) -> bool {
        matches!(self.discriminant, discriminant_ConnectErr::Unsupported)
    }
}

impl ConnectErr {
    pub fn AddrInUse() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::AddrInUse,
            payload: union_ConnectErr { AddrInUse: () },
        }
    }

    pub fn AddrNotAvailable() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::AddrNotAvailable,
            payload: union_ConnectErr {
                AddrNotAvailable: (),
            },
        }
    }

    pub fn ConnectionRefused() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::ConnectionRefused,
            payload: union_ConnectErr {
                ConnectionRefused: (),
            },
        }
    }

    pub fn Interrupted() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::Interrupted,
            payload: union_ConnectErr { Interrupted: () },
        }
    }

    pub fn PermissionDenied() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::PermissionDenied,
            payload: union_ConnectErr {
                PermissionDenied: (),
            },
        }
    }

    pub fn TimedOut() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::TimedOut,
            payload: union_ConnectErr { TimedOut: () },
        }
    }

    pub fn Unrecognized(payload: ConnectErr_Unrecognized) -> Self {
        Self {
            discriminant: discriminant_ConnectErr::Unrecognized,
            payload: union_ConnectErr {
                Unrecognized: core::mem::ManuallyDrop::new(payload),
            },
        }
    }

    pub fn Unsupported() -> Self {
        Self {
            discriminant: discriminant_ConnectErr::Unsupported,
            payload: union_ConnectErr { Unsupported: () },
        }
    }
}

impl Drop for ConnectErr {
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            discriminant_ConnectErr::AddrInUse => {}
            discriminant_ConnectErr::AddrNotAvailable => {}
            discriminant_ConnectErr::ConnectionRefused => {}
            discriminant_ConnectErr::Interrupted => {}
            discriminant_ConnectErr::PermissionDenied => {}
            discriminant_ConnectErr::TimedOut => {}
            discriminant_ConnectErr::Unrecognized => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.Unrecognized)
            },
            discriminant_ConnectErr::Unsupported => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum discriminant_ReadErr {
    Interrupted = 0,
    InvalidFilename = 1,
    NotFound = 2,
    OutOfMemory = 3,
    PermissionDenied = 4,
    StaleNetworkFileHandle = 5,
    TimedOut = 6,
    TooManyHardlinks = 7,
    TooManySymlinks = 8,
    Unrecognized = 9,
    Unsupported = 10,
}

impl core::fmt::Debug for discriminant_ReadErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Interrupted => f.write_str("discriminant_ReadErr::Interrupted"),
            Self::InvalidFilename => f.write_str("discriminant_ReadErr::InvalidFilename"),
            Self::NotFound => f.write_str("discriminant_ReadErr::NotFound"),
            Self::OutOfMemory => f.write_str("discriminant_ReadErr::OutOfMemory"),
            Self::PermissionDenied => f.write_str("discriminant_ReadErr::PermissionDenied"),
            Self::StaleNetworkFileHandle => {
                f.write_str("discriminant_ReadErr::StaleNetworkFileHandle")
            }
            Self::TimedOut => f.write_str("discriminant_ReadErr::TimedOut"),
            Self::TooManyHardlinks => f.write_str("discriminant_ReadErr::TooManyHardlinks"),
            Self::TooManySymlinks => f.write_str("discriminant_ReadErr::TooManySymlinks"),
            Self::Unrecognized => f.write_str("discriminant_ReadErr::Unrecognized"),
            Self::Unsupported => f.write_str("discriminant_ReadErr::Unsupported"),
        }
    }
}

#[repr(C, align(8))]
pub union union_ReadErr {
    Interrupted: (),
    InvalidFilename: (),
    NotFound: (),
    OutOfMemory: (),
    PermissionDenied: (),
    StaleNetworkFileHandle: (),
    TimedOut: (),
    TooManyHardlinks: (),
    TooManySymlinks: (),
    Unrecognized: core::mem::ManuallyDrop<ConnectErr_Unrecognized>,
    Unsupported: (),
}

// const _SIZE_CHECK_union_ReadErr: () = assert!(core::mem::size_of::<union_ReadErr>() == 40);
const _ALIGN_CHECK_union_ReadErr: () = assert!(core::mem::align_of::<union_ReadErr>() == 8);

const _SIZE_CHECK_ReadErr: () = assert!(core::mem::size_of::<ReadErr>() == 40);
const _ALIGN_CHECK_ReadErr: () = assert!(core::mem::align_of::<ReadErr>() == 8);

impl ReadErr {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_ReadErr {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_ReadErr>(*bytes.as_ptr().add(32))
        }
    }

    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_ReadErr) {
        let discriminant_ptr: *mut discriminant_ReadErr = (self as *mut ReadErr).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

#[repr(C)]
pub struct ReadErr {
    payload: union_ReadErr,
    discriminant: discriminant_ReadErr,
}

impl Clone for ReadErr {
    fn clone(&self) -> Self {
        use discriminant_ReadErr::*;

        let payload = unsafe {
            match self.discriminant {
                Interrupted => union_ReadErr {
                    Interrupted: self.payload.Interrupted.clone(),
                },
                InvalidFilename => union_ReadErr {
                    InvalidFilename: self.payload.InvalidFilename.clone(),
                },
                NotFound => union_ReadErr {
                    NotFound: self.payload.NotFound.clone(),
                },
                OutOfMemory => union_ReadErr {
                    OutOfMemory: self.payload.OutOfMemory.clone(),
                },
                PermissionDenied => union_ReadErr {
                    PermissionDenied: self.payload.PermissionDenied.clone(),
                },
                StaleNetworkFileHandle => union_ReadErr {
                    StaleNetworkFileHandle: self.payload.StaleNetworkFileHandle.clone(),
                },
                TimedOut => union_ReadErr {
                    TimedOut: self.payload.TimedOut.clone(),
                },
                TooManyHardlinks => union_ReadErr {
                    TooManyHardlinks: self.payload.TooManyHardlinks.clone(),
                },
                TooManySymlinks => union_ReadErr {
                    TooManySymlinks: self.payload.TooManySymlinks.clone(),
                },
                Unrecognized => union_ReadErr {
                    Unrecognized: self.payload.Unrecognized.clone(),
                },
                Unsupported => union_ReadErr {
                    Unsupported: self.payload.Unsupported.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

impl core::fmt::Debug for ReadErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use discriminant_ReadErr::*;

        unsafe {
            match self.discriminant {
                Interrupted => {
                    let field: &() = &self.payload.Interrupted;
                    f.debug_tuple("ReadErr::Interrupted").field(field).finish()
                }
                InvalidFilename => {
                    let field: &() = &self.payload.InvalidFilename;
                    f.debug_tuple("ReadErr::InvalidFilename")
                        .field(field)
                        .finish()
                }
                NotFound => {
                    let field: &() = &self.payload.NotFound;
                    f.debug_tuple("ReadErr::NotFound").field(field).finish()
                }
                OutOfMemory => {
                    let field: &() = &self.payload.OutOfMemory;
                    f.debug_tuple("ReadErr::OutOfMemory").field(field).finish()
                }
                PermissionDenied => {
                    let field: &() = &self.payload.PermissionDenied;
                    f.debug_tuple("ReadErr::PermissionDenied")
                        .field(field)
                        .finish()
                }
                StaleNetworkFileHandle => {
                    let field: &() = &self.payload.StaleNetworkFileHandle;
                    f.debug_tuple("ReadErr::StaleNetworkFileHandle")
                        .field(field)
                        .finish()
                }
                TimedOut => {
                    let field: &() = &self.payload.TimedOut;
                    f.debug_tuple("ReadErr::TimedOut").field(field).finish()
                }
                TooManyHardlinks => {
                    let field: &() = &self.payload.TooManyHardlinks;
                    f.debug_tuple("ReadErr::TooManyHardlinks")
                        .field(field)
                        .finish()
                }
                TooManySymlinks => {
                    let field: &() = &self.payload.TooManySymlinks;
                    f.debug_tuple("ReadErr::TooManySymlinks")
                        .field(field)
                        .finish()
                }
                Unrecognized => {
                    let field: &ConnectErr_Unrecognized = &self.payload.Unrecognized;
                    f.debug_tuple("ReadErr::Unrecognized").field(field).finish()
                }
                Unsupported => {
                    let field: &() = &self.payload.Unsupported;
                    f.debug_tuple("ReadErr::Unsupported").field(field).finish()
                }
            }
        }
    }
}

impl Eq for ReadErr {}

impl PartialEq for ReadErr {
    fn eq(&self, other: &Self) -> bool {
        use discriminant_ReadErr::*;

        if self.discriminant != other.discriminant {
            return false;
        }

        unsafe {
            match self.discriminant {
                Interrupted => self.payload.Interrupted == other.payload.Interrupted,
                InvalidFilename => self.payload.InvalidFilename == other.payload.InvalidFilename,
                NotFound => self.payload.NotFound == other.payload.NotFound,
                OutOfMemory => self.payload.OutOfMemory == other.payload.OutOfMemory,
                PermissionDenied => self.payload.PermissionDenied == other.payload.PermissionDenied,
                StaleNetworkFileHandle => {
                    self.payload.StaleNetworkFileHandle == other.payload.StaleNetworkFileHandle
                }
                TimedOut => self.payload.TimedOut == other.payload.TimedOut,
                TooManyHardlinks => self.payload.TooManyHardlinks == other.payload.TooManyHardlinks,
                TooManySymlinks => self.payload.TooManySymlinks == other.payload.TooManySymlinks,
                Unrecognized => self.payload.Unrecognized == other.payload.Unrecognized,
                Unsupported => self.payload.Unsupported == other.payload.Unsupported,
            }
        }
    }
}

impl Ord for ReadErr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ReadErr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use discriminant_ReadErr::*;

        use std::cmp::Ordering::*;

        match self.discriminant.cmp(&other.discriminant) {
            Less => Option::Some(Less),
            Greater => Option::Some(Greater),
            Equal => unsafe {
                match self.discriminant {
                    Interrupted => self
                        .payload
                        .Interrupted
                        .partial_cmp(&other.payload.Interrupted),
                    InvalidFilename => self
                        .payload
                        .InvalidFilename
                        .partial_cmp(&other.payload.InvalidFilename),
                    NotFound => self.payload.NotFound.partial_cmp(&other.payload.NotFound),
                    OutOfMemory => self
                        .payload
                        .OutOfMemory
                        .partial_cmp(&other.payload.OutOfMemory),
                    PermissionDenied => self
                        .payload
                        .PermissionDenied
                        .partial_cmp(&other.payload.PermissionDenied),
                    StaleNetworkFileHandle => self
                        .payload
                        .StaleNetworkFileHandle
                        .partial_cmp(&other.payload.StaleNetworkFileHandle),
                    TimedOut => self.payload.TimedOut.partial_cmp(&other.payload.TimedOut),
                    TooManyHardlinks => self
                        .payload
                        .TooManyHardlinks
                        .partial_cmp(&other.payload.TooManyHardlinks),
                    TooManySymlinks => self
                        .payload
                        .TooManySymlinks
                        .partial_cmp(&other.payload.TooManySymlinks),
                    Unrecognized => self
                        .payload
                        .Unrecognized
                        .partial_cmp(&other.payload.Unrecognized),
                    Unsupported => self
                        .payload
                        .Unsupported
                        .partial_cmp(&other.payload.Unsupported),
                }
            },
        }
    }
}

impl core::hash::Hash for ReadErr {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use discriminant_ReadErr::*;

        unsafe {
            match self.discriminant {
                Interrupted => self.payload.Interrupted.hash(state),
                InvalidFilename => self.payload.InvalidFilename.hash(state),
                NotFound => self.payload.NotFound.hash(state),
                OutOfMemory => self.payload.OutOfMemory.hash(state),
                PermissionDenied => self.payload.PermissionDenied.hash(state),
                StaleNetworkFileHandle => self.payload.StaleNetworkFileHandle.hash(state),
                TimedOut => self.payload.TimedOut.hash(state),
                TooManyHardlinks => self.payload.TooManyHardlinks.hash(state),
                TooManySymlinks => self.payload.TooManySymlinks.hash(state),
                Unrecognized => self.payload.Unrecognized.hash(state),
                Unsupported => self.payload.Unsupported.hash(state),
            }
        }
    }
}

impl ReadErr {
    pub fn is_Interrupted(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::Interrupted)
    }

    pub fn is_InvalidFilename(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::InvalidFilename)
    }

    pub fn is_NotFound(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::NotFound)
    }

    pub fn is_OutOfMemory(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::OutOfMemory)
    }

    pub fn is_PermissionDenied(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::PermissionDenied)
    }

    pub fn is_StaleNetworkFileHandle(&self) -> bool {
        matches!(
            self.discriminant,
            discriminant_ReadErr::StaleNetworkFileHandle
        )
    }

    pub fn is_TimedOut(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::TimedOut)
    }

    pub fn is_TooManyHardlinks(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::TooManyHardlinks)
    }

    pub fn is_TooManySymlinks(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::TooManySymlinks)
    }

    pub fn unwrap_Unrecognized(mut self) -> ConnectErr_Unrecognized {
        debug_assert_eq!(self.discriminant, discriminant_ReadErr::Unrecognized);
        unsafe { core::mem::ManuallyDrop::take(&mut self.payload.Unrecognized) }
    }

    pub fn is_Unrecognized(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::Unrecognized)
    }

    pub fn is_Unsupported(&self) -> bool {
        matches!(self.discriminant, discriminant_ReadErr::Unsupported)
    }
}

impl ReadErr {
    pub fn Interrupted() -> Self {
        Self {
            discriminant: discriminant_ReadErr::Interrupted,
            payload: union_ReadErr { Interrupted: () },
        }
    }

    pub fn InvalidFilename() -> Self {
        Self {
            discriminant: discriminant_ReadErr::InvalidFilename,
            payload: union_ReadErr {
                InvalidFilename: (),
            },
        }
    }

    pub fn NotFound() -> Self {
        Self {
            discriminant: discriminant_ReadErr::NotFound,
            payload: union_ReadErr { NotFound: () },
        }
    }

    pub fn OutOfMemory() -> Self {
        Self {
            discriminant: discriminant_ReadErr::OutOfMemory,
            payload: union_ReadErr { OutOfMemory: () },
        }
    }

    pub fn PermissionDenied() -> Self {
        Self {
            discriminant: discriminant_ReadErr::PermissionDenied,
            payload: union_ReadErr {
                PermissionDenied: (),
            },
        }
    }

    pub fn StaleNetworkFileHandle() -> Self {
        Self {
            discriminant: discriminant_ReadErr::StaleNetworkFileHandle,
            payload: union_ReadErr {
                StaleNetworkFileHandle: (),
            },
        }
    }

    pub fn TimedOut() -> Self {
        Self {
            discriminant: discriminant_ReadErr::TimedOut,
            payload: union_ReadErr { TimedOut: () },
        }
    }

    pub fn TooManyHardlinks() -> Self {
        Self {
            discriminant: discriminant_ReadErr::TooManyHardlinks,
            payload: union_ReadErr {
                TooManyHardlinks: (),
            },
        }
    }

    pub fn TooManySymlinks() -> Self {
        Self {
            discriminant: discriminant_ReadErr::TooManySymlinks,
            payload: union_ReadErr {
                TooManySymlinks: (),
            },
        }
    }

    pub fn Unrecognized(payload: ConnectErr_Unrecognized) -> Self {
        Self {
            discriminant: discriminant_ReadErr::Unrecognized,
            payload: union_ReadErr {
                Unrecognized: core::mem::ManuallyDrop::new(payload),
            },
        }
    }

    pub fn Unsupported() -> Self {
        Self {
            discriminant: discriminant_ReadErr::Unsupported,
            payload: union_ReadErr { Unsupported: () },
        }
    }
}

impl Drop for ReadErr {
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            discriminant_ReadErr::Interrupted => {}
            discriminant_ReadErr::InvalidFilename => {}
            discriminant_ReadErr::NotFound => {}
            discriminant_ReadErr::OutOfMemory => {}
            discriminant_ReadErr::PermissionDenied => {}
            discriminant_ReadErr::StaleNetworkFileHandle => {}
            discriminant_ReadErr::TimedOut => {}
            discriminant_ReadErr::TooManyHardlinks => {}
            discriminant_ReadErr::TooManySymlinks => {}
            discriminant_ReadErr::Unrecognized => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.Unrecognized)
            },
            discriminant_ReadErr::Unsupported => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum discriminant_WriteErr {
    AlreadyExists = 0,
    ExecutableFileBusy = 1,
    FileTooLarge = 2,
    FilesystemQuotaExceeded = 3,
    Interrupted = 4,
    InvalidFilename = 5,
    NotFound = 6,
    OutOfMemory = 7,
    PermissionDenied = 8,
    ReadOnlyFilesystem = 9,
    ResourceBusy = 10,
    StaleNetworkFileHandle = 11,
    StorageFull = 12,
    TimedOut = 13,
    TooManyHardlinks = 14,
    TooManySymlinks = 15,
    Unrecognized = 16,
    Unsupported = 17,
    WasADirectory = 18,
    WriteZero = 19,
}

impl core::fmt::Debug for discriminant_WriteErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyExists => f.write_str("discriminant_WriteErr::AlreadyExists"),
            Self::ExecutableFileBusy => f.write_str("discriminant_WriteErr::ExecutableFileBusy"),
            Self::FileTooLarge => f.write_str("discriminant_WriteErr::FileTooLarge"),
            Self::FilesystemQuotaExceeded => {
                f.write_str("discriminant_WriteErr::FilesystemQuotaExceeded")
            }
            Self::Interrupted => f.write_str("discriminant_WriteErr::Interrupted"),
            Self::InvalidFilename => f.write_str("discriminant_WriteErr::InvalidFilename"),
            Self::NotFound => f.write_str("discriminant_WriteErr::NotFound"),
            Self::OutOfMemory => f.write_str("discriminant_WriteErr::OutOfMemory"),
            Self::PermissionDenied => f.write_str("discriminant_WriteErr::PermissionDenied"),
            Self::ReadOnlyFilesystem => f.write_str("discriminant_WriteErr::ReadOnlyFilesystem"),
            Self::ResourceBusy => f.write_str("discriminant_WriteErr::ResourceBusy"),
            Self::StaleNetworkFileHandle => {
                f.write_str("discriminant_WriteErr::StaleNetworkFileHandle")
            }
            Self::StorageFull => f.write_str("discriminant_WriteErr::StorageFull"),
            Self::TimedOut => f.write_str("discriminant_WriteErr::TimedOut"),
            Self::TooManyHardlinks => f.write_str("discriminant_WriteErr::TooManyHardlinks"),
            Self::TooManySymlinks => f.write_str("discriminant_WriteErr::TooManySymlinks"),
            Self::Unrecognized => f.write_str("discriminant_WriteErr::Unrecognized"),
            Self::Unsupported => f.write_str("discriminant_WriteErr::Unsupported"),
            Self::WasADirectory => f.write_str("discriminant_WriteErr::WasADirectory"),
            Self::WriteZero => f.write_str("discriminant_WriteErr::WriteZero"),
        }
    }
}

#[repr(C, align(8))]
pub union union_WriteErr {
    AlreadyExists: (),
    ExecutableFileBusy: (),
    FileTooLarge: (),
    FilesystemQuotaExceeded: (),
    Interrupted: (),
    InvalidFilename: (),
    NotFound: (),
    OutOfMemory: (),
    PermissionDenied: (),
    ReadOnlyFilesystem: (),
    ResourceBusy: (),
    StaleNetworkFileHandle: (),
    StorageFull: (),
    TimedOut: (),
    TooManyHardlinks: (),
    TooManySymlinks: (),
    Unrecognized: core::mem::ManuallyDrop<ConnectErr_Unrecognized>,
    Unsupported: (),
    WasADirectory: (),
    WriteZero: (),
}

// const _SIZE_CHECK_union_WriteErr: () = assert!(core::mem::size_of::<union_WriteErr>() == 40);
const _ALIGN_CHECK_union_WriteErr: () = assert!(core::mem::align_of::<union_WriteErr>() == 8);

const _SIZE_CHECK_WriteErr: () = assert!(core::mem::size_of::<WriteErr>() == 40);
const _ALIGN_CHECK_WriteErr: () = assert!(core::mem::align_of::<WriteErr>() == 8);

impl WriteErr {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_WriteErr {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_WriteErr>(*bytes.as_ptr().add(32))
        }
    }

    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_WriteErr) {
        let discriminant_ptr: *mut discriminant_WriteErr = (self as *mut WriteErr).cast();

        unsafe {
            *(discriminant_ptr.add(32)) = discriminant;
        }
    }
}

#[repr(C)]
pub struct WriteErr {
    payload: union_WriteErr,
    discriminant: discriminant_WriteErr,
}

impl Clone for WriteErr {
    fn clone(&self) -> Self {
        use discriminant_WriteErr::*;

        let payload = unsafe {
            match self.discriminant {
                AlreadyExists => union_WriteErr {
                    AlreadyExists: self.payload.AlreadyExists.clone(),
                },
                ExecutableFileBusy => union_WriteErr {
                    ExecutableFileBusy: self.payload.ExecutableFileBusy.clone(),
                },
                FileTooLarge => union_WriteErr {
                    FileTooLarge: self.payload.FileTooLarge.clone(),
                },
                FilesystemQuotaExceeded => union_WriteErr {
                    FilesystemQuotaExceeded: self.payload.FilesystemQuotaExceeded.clone(),
                },
                Interrupted => union_WriteErr {
                    Interrupted: self.payload.Interrupted.clone(),
                },
                InvalidFilename => union_WriteErr {
                    InvalidFilename: self.payload.InvalidFilename.clone(),
                },
                NotFound => union_WriteErr {
                    NotFound: self.payload.NotFound.clone(),
                },
                OutOfMemory => union_WriteErr {
                    OutOfMemory: self.payload.OutOfMemory.clone(),
                },
                PermissionDenied => union_WriteErr {
                    PermissionDenied: self.payload.PermissionDenied.clone(),
                },
                ReadOnlyFilesystem => union_WriteErr {
                    ReadOnlyFilesystem: self.payload.ReadOnlyFilesystem.clone(),
                },
                ResourceBusy => union_WriteErr {
                    ResourceBusy: self.payload.ResourceBusy.clone(),
                },
                StaleNetworkFileHandle => union_WriteErr {
                    StaleNetworkFileHandle: self.payload.StaleNetworkFileHandle.clone(),
                },
                StorageFull => union_WriteErr {
                    StorageFull: self.payload.StorageFull.clone(),
                },
                TimedOut => union_WriteErr {
                    TimedOut: self.payload.TimedOut.clone(),
                },
                TooManyHardlinks => union_WriteErr {
                    TooManyHardlinks: self.payload.TooManyHardlinks.clone(),
                },
                TooManySymlinks => union_WriteErr {
                    TooManySymlinks: self.payload.TooManySymlinks.clone(),
                },
                Unrecognized => union_WriteErr {
                    Unrecognized: self.payload.Unrecognized.clone(),
                },
                Unsupported => union_WriteErr {
                    Unsupported: self.payload.Unsupported.clone(),
                },
                WasADirectory => union_WriteErr {
                    WasADirectory: self.payload.WasADirectory.clone(),
                },
                WriteZero => union_WriteErr {
                    WriteZero: self.payload.WriteZero.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

impl core::fmt::Debug for WriteErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use discriminant_WriteErr::*;

        unsafe {
            match self.discriminant {
                AlreadyExists => {
                    let field: &() = &self.payload.AlreadyExists;
                    f.debug_tuple("WriteErr::AlreadyExists")
                        .field(field)
                        .finish()
                }
                ExecutableFileBusy => {
                    let field: &() = &self.payload.ExecutableFileBusy;
                    f.debug_tuple("WriteErr::ExecutableFileBusy")
                        .field(field)
                        .finish()
                }
                FileTooLarge => {
                    let field: &() = &self.payload.FileTooLarge;
                    f.debug_tuple("WriteErr::FileTooLarge")
                        .field(field)
                        .finish()
                }
                FilesystemQuotaExceeded => {
                    let field: &() = &self.payload.FilesystemQuotaExceeded;
                    f.debug_tuple("WriteErr::FilesystemQuotaExceeded")
                        .field(field)
                        .finish()
                }
                Interrupted => {
                    let field: &() = &self.payload.Interrupted;
                    f.debug_tuple("WriteErr::Interrupted").field(field).finish()
                }
                InvalidFilename => {
                    let field: &() = &self.payload.InvalidFilename;
                    f.debug_tuple("WriteErr::InvalidFilename")
                        .field(field)
                        .finish()
                }
                NotFound => {
                    let field: &() = &self.payload.NotFound;
                    f.debug_tuple("WriteErr::NotFound").field(field).finish()
                }
                OutOfMemory => {
                    let field: &() = &self.payload.OutOfMemory;
                    f.debug_tuple("WriteErr::OutOfMemory").field(field).finish()
                }
                PermissionDenied => {
                    let field: &() = &self.payload.PermissionDenied;
                    f.debug_tuple("WriteErr::PermissionDenied")
                        .field(field)
                        .finish()
                }
                ReadOnlyFilesystem => {
                    let field: &() = &self.payload.ReadOnlyFilesystem;
                    f.debug_tuple("WriteErr::ReadOnlyFilesystem")
                        .field(field)
                        .finish()
                }
                ResourceBusy => {
                    let field: &() = &self.payload.ResourceBusy;
                    f.debug_tuple("WriteErr::ResourceBusy")
                        .field(field)
                        .finish()
                }
                StaleNetworkFileHandle => {
                    let field: &() = &self.payload.StaleNetworkFileHandle;
                    f.debug_tuple("WriteErr::StaleNetworkFileHandle")
                        .field(field)
                        .finish()
                }
                StorageFull => {
                    let field: &() = &self.payload.StorageFull;
                    f.debug_tuple("WriteErr::StorageFull").field(field).finish()
                }
                TimedOut => {
                    let field: &() = &self.payload.TimedOut;
                    f.debug_tuple("WriteErr::TimedOut").field(field).finish()
                }
                TooManyHardlinks => {
                    let field: &() = &self.payload.TooManyHardlinks;
                    f.debug_tuple("WriteErr::TooManyHardlinks")
                        .field(field)
                        .finish()
                }
                TooManySymlinks => {
                    let field: &() = &self.payload.TooManySymlinks;
                    f.debug_tuple("WriteErr::TooManySymlinks")
                        .field(field)
                        .finish()
                }
                Unrecognized => {
                    let field: &ConnectErr_Unrecognized = &self.payload.Unrecognized;
                    f.debug_tuple("WriteErr::Unrecognized")
                        .field(field)
                        .finish()
                }
                Unsupported => {
                    let field: &() = &self.payload.Unsupported;
                    f.debug_tuple("WriteErr::Unsupported").field(field).finish()
                }
                WasADirectory => {
                    let field: &() = &self.payload.WasADirectory;
                    f.debug_tuple("WriteErr::WasADirectory")
                        .field(field)
                        .finish()
                }
                WriteZero => {
                    let field: &() = &self.payload.WriteZero;
                    f.debug_tuple("WriteErr::WriteZero").field(field).finish()
                }
            }
        }
    }
}

impl Eq for WriteErr {}

impl PartialEq for WriteErr {
    fn eq(&self, other: &Self) -> bool {
        use discriminant_WriteErr::*;

        if self.discriminant != other.discriminant {
            return false;
        }

        unsafe {
            match self.discriminant {
                AlreadyExists => self.payload.AlreadyExists == other.payload.AlreadyExists,
                ExecutableFileBusy => {
                    self.payload.ExecutableFileBusy == other.payload.ExecutableFileBusy
                }
                FileTooLarge => self.payload.FileTooLarge == other.payload.FileTooLarge,
                FilesystemQuotaExceeded => {
                    self.payload.FilesystemQuotaExceeded == other.payload.FilesystemQuotaExceeded
                }
                Interrupted => self.payload.Interrupted == other.payload.Interrupted,
                InvalidFilename => self.payload.InvalidFilename == other.payload.InvalidFilename,
                NotFound => self.payload.NotFound == other.payload.NotFound,
                OutOfMemory => self.payload.OutOfMemory == other.payload.OutOfMemory,
                PermissionDenied => self.payload.PermissionDenied == other.payload.PermissionDenied,
                ReadOnlyFilesystem => {
                    self.payload.ReadOnlyFilesystem == other.payload.ReadOnlyFilesystem
                }
                ResourceBusy => self.payload.ResourceBusy == other.payload.ResourceBusy,
                StaleNetworkFileHandle => {
                    self.payload.StaleNetworkFileHandle == other.payload.StaleNetworkFileHandle
                }
                StorageFull => self.payload.StorageFull == other.payload.StorageFull,
                TimedOut => self.payload.TimedOut == other.payload.TimedOut,
                TooManyHardlinks => self.payload.TooManyHardlinks == other.payload.TooManyHardlinks,
                TooManySymlinks => self.payload.TooManySymlinks == other.payload.TooManySymlinks,
                Unrecognized => self.payload.Unrecognized == other.payload.Unrecognized,
                Unsupported => self.payload.Unsupported == other.payload.Unsupported,
                WasADirectory => self.payload.WasADirectory == other.payload.WasADirectory,
                WriteZero => self.payload.WriteZero == other.payload.WriteZero,
            }
        }
    }
}

impl Ord for WriteErr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for WriteErr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use discriminant_WriteErr::*;

        use std::cmp::Ordering::*;

        match self.discriminant.cmp(&other.discriminant) {
            Less => Option::Some(Less),
            Greater => Option::Some(Greater),
            Equal => unsafe {
                match self.discriminant {
                    AlreadyExists => self
                        .payload
                        .AlreadyExists
                        .partial_cmp(&other.payload.AlreadyExists),
                    ExecutableFileBusy => self
                        .payload
                        .ExecutableFileBusy
                        .partial_cmp(&other.payload.ExecutableFileBusy),
                    FileTooLarge => self
                        .payload
                        .FileTooLarge
                        .partial_cmp(&other.payload.FileTooLarge),
                    FilesystemQuotaExceeded => self
                        .payload
                        .FilesystemQuotaExceeded
                        .partial_cmp(&other.payload.FilesystemQuotaExceeded),
                    Interrupted => self
                        .payload
                        .Interrupted
                        .partial_cmp(&other.payload.Interrupted),
                    InvalidFilename => self
                        .payload
                        .InvalidFilename
                        .partial_cmp(&other.payload.InvalidFilename),
                    NotFound => self.payload.NotFound.partial_cmp(&other.payload.NotFound),
                    OutOfMemory => self
                        .payload
                        .OutOfMemory
                        .partial_cmp(&other.payload.OutOfMemory),
                    PermissionDenied => self
                        .payload
                        .PermissionDenied
                        .partial_cmp(&other.payload.PermissionDenied),
                    ReadOnlyFilesystem => self
                        .payload
                        .ReadOnlyFilesystem
                        .partial_cmp(&other.payload.ReadOnlyFilesystem),
                    ResourceBusy => self
                        .payload
                        .ResourceBusy
                        .partial_cmp(&other.payload.ResourceBusy),
                    StaleNetworkFileHandle => self
                        .payload
                        .StaleNetworkFileHandle
                        .partial_cmp(&other.payload.StaleNetworkFileHandle),
                    StorageFull => self
                        .payload
                        .StorageFull
                        .partial_cmp(&other.payload.StorageFull),
                    TimedOut => self.payload.TimedOut.partial_cmp(&other.payload.TimedOut),
                    TooManyHardlinks => self
                        .payload
                        .TooManyHardlinks
                        .partial_cmp(&other.payload.TooManyHardlinks),
                    TooManySymlinks => self
                        .payload
                        .TooManySymlinks
                        .partial_cmp(&other.payload.TooManySymlinks),
                    Unrecognized => self
                        .payload
                        .Unrecognized
                        .partial_cmp(&other.payload.Unrecognized),
                    Unsupported => self
                        .payload
                        .Unsupported
                        .partial_cmp(&other.payload.Unsupported),
                    WasADirectory => self
                        .payload
                        .WasADirectory
                        .partial_cmp(&other.payload.WasADirectory),
                    WriteZero => self.payload.WriteZero.partial_cmp(&other.payload.WriteZero),
                }
            },
        }
    }
}

impl core::hash::Hash for WriteErr {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use discriminant_WriteErr::*;

        unsafe {
            match self.discriminant {
                AlreadyExists => self.payload.AlreadyExists.hash(state),
                ExecutableFileBusy => self.payload.ExecutableFileBusy.hash(state),
                FileTooLarge => self.payload.FileTooLarge.hash(state),
                FilesystemQuotaExceeded => self.payload.FilesystemQuotaExceeded.hash(state),
                Interrupted => self.payload.Interrupted.hash(state),
                InvalidFilename => self.payload.InvalidFilename.hash(state),
                NotFound => self.payload.NotFound.hash(state),
                OutOfMemory => self.payload.OutOfMemory.hash(state),
                PermissionDenied => self.payload.PermissionDenied.hash(state),
                ReadOnlyFilesystem => self.payload.ReadOnlyFilesystem.hash(state),
                ResourceBusy => self.payload.ResourceBusy.hash(state),
                StaleNetworkFileHandle => self.payload.StaleNetworkFileHandle.hash(state),
                StorageFull => self.payload.StorageFull.hash(state),
                TimedOut => self.payload.TimedOut.hash(state),
                TooManyHardlinks => self.payload.TooManyHardlinks.hash(state),
                TooManySymlinks => self.payload.TooManySymlinks.hash(state),
                Unrecognized => self.payload.Unrecognized.hash(state),
                Unsupported => self.payload.Unsupported.hash(state),
                WasADirectory => self.payload.WasADirectory.hash(state),
                WriteZero => self.payload.WriteZero.hash(state),
            }
        }
    }
}

impl WriteErr {
    pub fn is_AlreadyExists(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::AlreadyExists)
    }

    pub fn is_ExecutableFileBusy(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::ExecutableFileBusy)
    }

    pub fn is_FileTooLarge(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::FileTooLarge)
    }

    pub fn is_FilesystemQuotaExceeded(&self) -> bool {
        matches!(
            self.discriminant,
            discriminant_WriteErr::FilesystemQuotaExceeded
        )
    }

    pub fn is_Interrupted(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::Interrupted)
    }

    pub fn is_InvalidFilename(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::InvalidFilename)
    }

    pub fn is_NotFound(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::NotFound)
    }

    pub fn is_OutOfMemory(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::OutOfMemory)
    }

    pub fn is_PermissionDenied(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::PermissionDenied)
    }

    pub fn is_ReadOnlyFilesystem(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::ReadOnlyFilesystem)
    }

    pub fn is_ResourceBusy(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::ResourceBusy)
    }

    pub fn is_StaleNetworkFileHandle(&self) -> bool {
        matches!(
            self.discriminant,
            discriminant_WriteErr::StaleNetworkFileHandle
        )
    }

    pub fn is_StorageFull(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::StorageFull)
    }

    pub fn is_TimedOut(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::TimedOut)
    }

    pub fn is_TooManyHardlinks(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::TooManyHardlinks)
    }

    pub fn is_TooManySymlinks(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::TooManySymlinks)
    }

    pub fn unwrap_Unrecognized(mut self) -> ConnectErr_Unrecognized {
        debug_assert_eq!(self.discriminant, discriminant_WriteErr::Unrecognized);
        unsafe { core::mem::ManuallyDrop::take(&mut self.payload.Unrecognized) }
    }

    pub fn is_Unrecognized(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::Unrecognized)
    }

    pub fn is_Unsupported(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::Unsupported)
    }

    pub fn is_WasADirectory(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::WasADirectory)
    }

    pub fn is_WriteZero(&self) -> bool {
        matches!(self.discriminant, discriminant_WriteErr::WriteZero)
    }
}

impl WriteErr {
    pub fn AlreadyExists() -> Self {
        Self {
            discriminant: discriminant_WriteErr::AlreadyExists,
            payload: union_WriteErr { AlreadyExists: () },
        }
    }

    pub fn ExecutableFileBusy() -> Self {
        Self {
            discriminant: discriminant_WriteErr::ExecutableFileBusy,
            payload: union_WriteErr {
                ExecutableFileBusy: (),
            },
        }
    }

    pub fn FileTooLarge() -> Self {
        Self {
            discriminant: discriminant_WriteErr::FileTooLarge,
            payload: union_WriteErr { FileTooLarge: () },
        }
    }

    pub fn FilesystemQuotaExceeded() -> Self {
        Self {
            discriminant: discriminant_WriteErr::FilesystemQuotaExceeded,
            payload: union_WriteErr {
                FilesystemQuotaExceeded: (),
            },
        }
    }

    pub fn Interrupted() -> Self {
        Self {
            discriminant: discriminant_WriteErr::Interrupted,
            payload: union_WriteErr { Interrupted: () },
        }
    }

    pub fn InvalidFilename() -> Self {
        Self {
            discriminant: discriminant_WriteErr::InvalidFilename,
            payload: union_WriteErr {
                InvalidFilename: (),
            },
        }
    }

    pub fn NotFound() -> Self {
        Self {
            discriminant: discriminant_WriteErr::NotFound,
            payload: union_WriteErr { NotFound: () },
        }
    }

    pub fn OutOfMemory() -> Self {
        Self {
            discriminant: discriminant_WriteErr::OutOfMemory,
            payload: union_WriteErr { OutOfMemory: () },
        }
    }

    pub fn PermissionDenied() -> Self {
        Self {
            discriminant: discriminant_WriteErr::PermissionDenied,
            payload: union_WriteErr {
                PermissionDenied: (),
            },
        }
    }

    pub fn ReadOnlyFilesystem() -> Self {
        Self {
            discriminant: discriminant_WriteErr::ReadOnlyFilesystem,
            payload: union_WriteErr {
                ReadOnlyFilesystem: (),
            },
        }
    }

    pub fn ResourceBusy() -> Self {
        Self {
            discriminant: discriminant_WriteErr::ResourceBusy,
            payload: union_WriteErr { ResourceBusy: () },
        }
    }

    pub fn StaleNetworkFileHandle() -> Self {
        Self {
            discriminant: discriminant_WriteErr::StaleNetworkFileHandle,
            payload: union_WriteErr {
                StaleNetworkFileHandle: (),
            },
        }
    }

    pub fn StorageFull() -> Self {
        Self {
            discriminant: discriminant_WriteErr::StorageFull,
            payload: union_WriteErr { StorageFull: () },
        }
    }

    pub fn TimedOut() -> Self {
        Self {
            discriminant: discriminant_WriteErr::TimedOut,
            payload: union_WriteErr { TimedOut: () },
        }
    }

    pub fn TooManyHardlinks() -> Self {
        Self {
            discriminant: discriminant_WriteErr::TooManyHardlinks,
            payload: union_WriteErr {
                TooManyHardlinks: (),
            },
        }
    }

    pub fn TooManySymlinks() -> Self {
        Self {
            discriminant: discriminant_WriteErr::TooManySymlinks,
            payload: union_WriteErr {
                TooManySymlinks: (),
            },
        }
    }

    pub fn Unrecognized(payload: ConnectErr_Unrecognized) -> Self {
        Self {
            discriminant: discriminant_WriteErr::Unrecognized,
            payload: union_WriteErr {
                Unrecognized: core::mem::ManuallyDrop::new(payload),
            },
        }
    }

    pub fn Unsupported() -> Self {
        Self {
            discriminant: discriminant_WriteErr::Unsupported,
            payload: union_WriteErr { Unsupported: () },
        }
    }

    pub fn WasADirectory() -> Self {
        Self {
            discriminant: discriminant_WriteErr::WasADirectory,
            payload: union_WriteErr { WasADirectory: () },
        }
    }

    pub fn WriteZero() -> Self {
        Self {
            discriminant: discriminant_WriteErr::WriteZero,
            payload: union_WriteErr { WriteZero: () },
        }
    }
}

impl Drop for WriteErr {
    fn drop(&mut self) {
        // Drop the payloads
        match self.discriminant() {
            discriminant_WriteErr::AlreadyExists => {}
            discriminant_WriteErr::ExecutableFileBusy => {}
            discriminant_WriteErr::FileTooLarge => {}
            discriminant_WriteErr::FilesystemQuotaExceeded => {}
            discriminant_WriteErr::Interrupted => {}
            discriminant_WriteErr::InvalidFilename => {}
            discriminant_WriteErr::NotFound => {}
            discriminant_WriteErr::OutOfMemory => {}
            discriminant_WriteErr::PermissionDenied => {}
            discriminant_WriteErr::ReadOnlyFilesystem => {}
            discriminant_WriteErr::ResourceBusy => {}
            discriminant_WriteErr::StaleNetworkFileHandle => {}
            discriminant_WriteErr::StorageFull => {}
            discriminant_WriteErr::TimedOut => {}
            discriminant_WriteErr::TooManyHardlinks => {}
            discriminant_WriteErr::TooManySymlinks => {}
            discriminant_WriteErr::Unrecognized => unsafe {
                core::mem::ManuallyDrop::drop(&mut self.payload.Unrecognized)
            },
            discriminant_WriteErr::Unsupported => {}
            discriminant_WriteErr::WasADirectory => {}
            discriminant_WriteErr::WriteZero => {}
        }
    }
}
