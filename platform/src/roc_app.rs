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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum InternalMethod {
    Connect = 0,
    Delete = 1,
    Get = 2,
    Head = 3,
    Options = 4,
    Patch = 5,
    Post = 6,
    Put = 7,
    Trace = 8,
}

impl core::fmt::Debug for InternalMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Connect => f.write_str("InternalMethod::Connect"),
            Self::Delete => f.write_str("InternalMethod::Delete"),
            Self::Get => f.write_str("InternalMethod::Get"),
            Self::Head => f.write_str("InternalMethod::Head"),
            Self::Options => f.write_str("InternalMethod::Options"),
            Self::Patch => f.write_str("InternalMethod::Patch"),
            Self::Post => f.write_str("InternalMethod::Post"),
            Self::Put => f.write_str("InternalMethod::Put"),
            Self::Trace => f.write_str("InternalMethod::Trace"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct InternalRequest {
    pub body: roc_std::RocList<u8>,
    pub headers: roc_std::RocList<InternalHeader>,
    pub mimeType: roc_std::RocStr,
    pub timeout: InternalTimeoutConfig,
    pub url: roc_std::RocStr,
    pub method: InternalMethod,
}

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct InternalResponse {
    pub body: roc_std::RocList<u8>,
    pub headers: roc_std::RocList<InternalHeader>,
    pub status: u16,
}

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub struct InternalHeader {
    pub name: roc_std::RocStr,
    pub value: roc_std::RocList<u8>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum discriminant_InternalTimeoutConfig {
    NoTimeout = 0,
    TimeoutMilliseconds = 1,
}

impl core::fmt::Debug for discriminant_InternalTimeoutConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoTimeout => f.write_str("discriminant_InternalTimeoutConfig::NoTimeout"),
            Self::TimeoutMilliseconds => {
                f.write_str("discriminant_InternalTimeoutConfig::TimeoutMilliseconds")
            }
        }
    }
}

#[repr(C, align(8))]
pub union union_InternalTimeoutConfig {
    NoTimeout: (),
    TimeoutMilliseconds: u64,
}

const _SIZE_CHECK_union_InternalTimeoutConfig: () =
    assert!(core::mem::size_of::<union_InternalTimeoutConfig>() == 8);
const _ALIGN_CHECK_union_InternalTimeoutConfig: () =
    assert!(core::mem::align_of::<union_InternalTimeoutConfig>() == 8);

const _SIZE_CHECK_InternalTimeoutConfig: () =
    assert!(core::mem::size_of::<InternalTimeoutConfig>() == 16);
const _ALIGN_CHECK_InternalTimeoutConfig: () =
    assert!(core::mem::align_of::<InternalTimeoutConfig>() == 8);

impl InternalTimeoutConfig {
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_InternalTimeoutConfig {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_InternalTimeoutConfig>(*bytes.as_ptr().add(8))
        }
    }

    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_InternalTimeoutConfig) {
        let discriminant_ptr: *mut discriminant_InternalTimeoutConfig =
            (self as *mut InternalTimeoutConfig).cast();

        unsafe {
            *(discriminant_ptr.add(8)) = discriminant;
        }
    }
}

#[repr(C)]
pub struct InternalTimeoutConfig {
    payload: union_InternalTimeoutConfig,
    discriminant: discriminant_InternalTimeoutConfig,
}

impl Clone for InternalTimeoutConfig {
    fn clone(&self) -> Self {
        use discriminant_InternalTimeoutConfig::*;

        let payload = unsafe {
            match self.discriminant {
                NoTimeout => union_InternalTimeoutConfig {
                    NoTimeout: self.payload.NoTimeout.clone(),
                },
                TimeoutMilliseconds => union_InternalTimeoutConfig {
                    TimeoutMilliseconds: self.payload.TimeoutMilliseconds.clone(),
                },
            }
        };

        Self {
            discriminant: self.discriminant,
            payload,
        }
    }
}

impl core::fmt::Debug for InternalTimeoutConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use discriminant_InternalTimeoutConfig::*;

        unsafe {
            match self.discriminant {
                NoTimeout => {
                    let field: &() = &self.payload.NoTimeout;
                    f.debug_tuple("InternalTimeoutConfig::NoTimeout")
                        .field(field)
                        .finish()
                }
                TimeoutMilliseconds => {
                    let field: &u64 = &self.payload.TimeoutMilliseconds;
                    f.debug_tuple("InternalTimeoutConfig::TimeoutMilliseconds")
                        .field(field)
                        .finish()
                }
            }
        }
    }
}

impl Eq for InternalTimeoutConfig {}

impl PartialEq for InternalTimeoutConfig {
    fn eq(&self, other: &Self) -> bool {
        use discriminant_InternalTimeoutConfig::*;

        if self.discriminant != other.discriminant {
            return false;
        }

        unsafe {
            match self.discriminant {
                NoTimeout => self.payload.NoTimeout == other.payload.NoTimeout,
                TimeoutMilliseconds => {
                    self.payload.TimeoutMilliseconds == other.payload.TimeoutMilliseconds
                }
            }
        }
    }
}

impl Ord for InternalTimeoutConfig {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for InternalTimeoutConfig {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use discriminant_InternalTimeoutConfig::*;

        use std::cmp::Ordering::*;

        match self.discriminant.cmp(&other.discriminant) {
            Less => Option::Some(Less),
            Greater => Option::Some(Greater),
            Equal => unsafe {
                match self.discriminant {
                    NoTimeout => self.payload.NoTimeout.partial_cmp(&other.payload.NoTimeout),
                    TimeoutMilliseconds => self
                        .payload
                        .TimeoutMilliseconds
                        .partial_cmp(&other.payload.TimeoutMilliseconds),
                }
            },
        }
    }
}

impl core::hash::Hash for InternalTimeoutConfig {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use discriminant_InternalTimeoutConfig::*;

        unsafe {
            match self.discriminant {
                NoTimeout => self.payload.NoTimeout.hash(state),
                TimeoutMilliseconds => self.payload.TimeoutMilliseconds.hash(state),
            }
        }
    }
}

impl InternalTimeoutConfig {
    pub fn is_NoTimeout(&self) -> bool {
        matches!(
            self.discriminant,
            discriminant_InternalTimeoutConfig::NoTimeout
        )
    }

    pub fn unwrap_TimeoutMilliseconds(mut self) -> u64 {
        debug_assert_eq!(
            self.discriminant,
            discriminant_InternalTimeoutConfig::TimeoutMilliseconds
        );
        unsafe { self.payload.TimeoutMilliseconds }
    }

    pub fn is_TimeoutMilliseconds(&self) -> bool {
        matches!(
            self.discriminant,
            discriminant_InternalTimeoutConfig::TimeoutMilliseconds
        )
    }
}

impl InternalTimeoutConfig {
    pub fn NoTimeout() -> Self {
        Self {
            discriminant: discriminant_InternalTimeoutConfig::NoTimeout,
            payload: union_InternalTimeoutConfig { NoTimeout: () },
        }
    }

    pub fn TimeoutMilliseconds(payload: u64) -> Self {
        Self {
            discriminant: discriminant_InternalTimeoutConfig::TimeoutMilliseconds,
            payload: union_InternalTimeoutConfig {
                TimeoutMilliseconds: payload,
            },
        }
    }
}
