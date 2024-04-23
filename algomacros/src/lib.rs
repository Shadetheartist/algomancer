macro_rules! impl_key_wrapper {
    ($t:ident<$inner:ty>) => {
        impl std::fmt::Display for $t<$inner> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($t), self.0)
            }
        }

        impl std::fmt::Debug for $t<$inner> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($t), self.0)
            }
        }

        impl From<$inner> for $t<$inner> {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }
    };
}