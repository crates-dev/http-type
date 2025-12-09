use crate::*;

/// Implements the `Default` trait for `Method`.
impl Default for Method {
    /// Returns the default `Method` variant, which is `UNKNOWN` with an empty string.
    ///
    /// # Returns
    ///
    /// The default `Method` variant.
    #[inline(always)]
    fn default() -> Self {
        Self::UNKNOWN(String::new())
    }
}

/// Implements the `Display` trait for `Method`, allowing it to be formatted as a string.
impl Display for Method {
    /// Formats the `Method` variant into its string representation.
    ///
    /// # Arguments
    ///
    /// - `f` - The formatter to write the string into.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::GET => GET,
            Self::POST => POST,
            Self::CONNECT => CONNECT,
            Self::DELETE => DELETE,
            Self::HEAD => HEAD,
            Self::PATCH => PATCH,
            Self::TRACE => TRACE,
            Self::PUT => PUT,
            Self::OPTIONS => OPTIONS,
            Self::UNKNOWN(methods) => methods,
        };
        write!(f, "{res}")
    }
}

/// Implements the `FromStr` trait for `Method`, allowing conversion from a string slice.
impl FromStr for Method {
    /// The error type returned when conversion fails.
    type Err = ();

    /// Converts a string slice into a `Method` variant.
    ///
    /// This method attempts to parse the input string into a known `Method` variant.
    /// If the string does not match any known method, it returns an `UNKNOWN` variant
    /// containing the original string.
    ///
    /// # Arguments
    ///
    /// - `methods_str` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Method` variant if successful, or `Self::Err` on failure.
    #[inline(always)]
    fn from_str(methods_str: &str) -> Result<Self, Self::Err> {
        match methods_str {
            GET => Ok(Self::GET),
            POST => Ok(Self::POST),
            PUT => Ok(Self::PUT),
            DELETE => Ok(Self::DELETE),
            PATCH => Ok(Self::PATCH),
            HEAD => Ok(Self::HEAD),
            OPTIONS => Ok(Self::OPTIONS),
            CONNECT => Ok(Self::CONNECT),
            TRACE => Ok(Self::TRACE),
            _ => Ok(Self::UNKNOWN(methods_str.to_string())),
        }
    }
}

impl Method {
    /// Creates a new instance of `Method` with the default value of `Self::GET`.
    ///
    /// This is a shorthand for using the `default` method.
    ///
    /// # Returns
    ///
    /// A new `Method` instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the current method is `GET`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `GET`, `false` otherwise.
    #[inline(always)]
    pub fn is_get(&self) -> bool {
        matches!(self, Self::GET)
    }

    /// Checks if the current method is `POST`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `POST`, `false` otherwise.
    #[inline(always)]
    pub fn is_post(&self) -> bool {
        matches!(self, Self::POST)
    }

    /// Checks if the current method is `PUT`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `PUT`, `false` otherwise.
    #[inline(always)]
    pub fn is_put(&self) -> bool {
        matches!(self, Self::PUT)
    }

    /// Checks if the current method is `DELETE`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `DELETE`, `false` otherwise.
    #[inline(always)]
    pub fn is_delete(&self) -> bool {
        matches!(self, Self::DELETE)
    }

    /// Checks if the current method is `PATCH`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `PATCH`, `false` otherwise.
    #[inline(always)]
    pub fn is_patch(&self) -> bool {
        matches!(self, Self::PATCH)
    }

    /// Checks if the current method is `HEAD`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `HEAD`, `false` otherwise.
    #[inline(always)]
    pub fn is_head(&self) -> bool {
        matches!(self, Self::HEAD)
    }

    /// Checks if the current method is `OPTIONS`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `OPTIONS`, `false` otherwise.
    #[inline(always)]
    pub fn is_options(&self) -> bool {
        matches!(self, Self::OPTIONS)
    }

    /// Checks if the current method is `CONNECT`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `CONNECT`, `false` otherwise.
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        matches!(self, Self::CONNECT)
    }

    /// Checks if the current method is `TRACE`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `TRACE`, `false` otherwise.
    #[inline(always)]
    pub fn is_trace(&self) -> bool {
        matches!(self, Self::TRACE)
    }

    /// Checks if the current method is `UNKNOWN`.
    ///
    /// # Returns
    ///
    /// `true` if the method is `UNKNOWN`, `false` otherwise.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::UNKNOWN(_))
    }
}
