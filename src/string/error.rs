//! Contains all of this crate errors

use core::fmt::{self, Debug, Display, Formatter};
use core::{char::DecodeUtf16Error, str::Utf8Error};

/// Every error possible when using [`StaticString`]
///
/// [`StaticString`]: ../struct.StaticString.html
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
  /// Conversion between available byte slice and UTF-8 failed
  Utf8(Utf8Error),
  /// Conversion between available `u16` slice and string failed
  Utf16(DecodeUtf16Error),
  /// Accessed invalid Utf8 character index
  NotCharBoundary,
  /// Out of boundaries access
  OutOfBounds,
}

impl Error {
  #[inline]
  pub fn is_utf8(&self) -> bool {
    match self {
      Self::Utf8(_) => true,
      _ => false,
    }
  }

  #[inline]
  pub fn is_utf16(&self) -> bool {
    match self {
      Self::Utf16(_) => true,
      _ => false,
    }
  }

  #[inline]
  pub fn is_out_of_bounds(&self) -> bool {
    match self {
      Self::OutOfBounds => true,
      _ => false,
    }
  }

  #[inline]
  pub fn isnt_char_boundary(&self) -> bool {
    match self {
      Self::NotCharBoundary => true,
      _ => false,
    }
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Utf8(err) => write!(f, "{}", err),
      Self::Utf16(err) => write!(f, "{}", err),
      Self::OutOfBounds => write!(f, "Out Of Bounds"),
      Self::NotCharBoundary => write!(f, "Not Char Boundary"),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<DecodeUtf16Error> for Error {
  #[inline]
  fn from(err: DecodeUtf16Error) -> Self {
    Self::Utf16(err)
  }
}

impl From<Utf8Error> for Error {
  #[inline]
  fn from(err: Utf8Error) -> Self {
    Self::Utf8(err)
  }
}
