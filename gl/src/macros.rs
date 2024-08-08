#[macro_export]
macro_rules! raw_error {
    (
        $qual:vis enum $enum:ident {
            $(
                #[doc = $desc:expr]
                $val:ident = $const:ident,
            )*
        }
    ) => {
        #[derive(Debug)]
        $qual enum $enum {
            $(
                #[doc=$desc]
                $val,
            )*
        }

        impl Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $enum::*;
                match self {
                    $($val => $desc,)*
                }.fmt(f)
            }
        }

        impl Error for $enum {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                None
            }
        }

        impl From<$enum> for i32 {
            fn from(value: $enum) -> Self {
                use $enum::*;
                match value {
                    $($val => $const,)*
                }
            }
        }

        impl TryFrom<i32> for $enum {
            type Error = i32;
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                use $enum::*;
                match value {
                    $($const => Ok($val),)*
                    e => Err(e)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! attr_list {
    (
        $qual:vis struct $struct:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $func:ident: $type:ident = $const:ident,
            )*
        }
    ) => {
        $qual struct $struct {
            list: Vec<EGLint>,
        }

        impl $struct {
            pub fn new() -> Self {
                Self { list: vec!(EGL_NONE) }
            }

            pub fn as_ptr(&self) -> *const EGLint {
                self.list.as_ptr()
            }

            $(
                $(#[$inner $($args)*])*
                pub fn $func(mut self, value: $type) -> Self {
                    self.list.insert(0, value.into());
                    self.list.insert(0, $const);
                    self
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! attr_bitstruct {
    (
        $qual:vis bitstruct $struct:ident($default:expr) {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $func:ident = $const:ident,
            )*
        }
    ) => {
        $qual struct $struct(i32);

        impl $struct {
            pub fn new() -> Self {
                Self($default)
            }
            $(
                $(#[$inner $($args)*])*
                pub fn $func(mut self) -> Self {
                    self.0 |= $const;
                    self
                }
            )*
        }

        impl From<$struct> for i32 {
            fn from(value: $struct) -> i32 {
                value.0
            }
        }
    };
}

#[macro_export]
macro_rules! attr_enum {
    (
        $qual:vis enum $enum:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $val:ident = $const:ident,
            )*
        }
    ) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(i32)]
        $qual enum $enum {
            $(
                $(#[$inner $($args)*])*
                $val = $const,
            )*
        }

        impl From<$enum> for i32 {
            fn from(value: $enum) -> i32 {
                value as i32
            }
        }
    };
}
