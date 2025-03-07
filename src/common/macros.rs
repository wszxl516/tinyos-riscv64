#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
#[macro_export]
macro_rules! count_fields {
    ($($tts:tt)*) => {0 $(+ crate::replace_expr!($tts 1))*};
}
#[macro_export]
macro_rules! impl_field_name_and_value {
    {
    $field_type: tt, $(#[$inner:meta])* $vis:vis struct $name:ident {$($field_vis:vis $field_name: ident), + $(,)?}
    }  => {
        $(#[$inner])*
        $vis struct $name{
            $($field_vis $field_name: $field_type,)*
        }
        impl $name{
            pub const FIELD_NAMES: [&str; crate::count_fields!($($field_name) *)] = [
                $( stringify!($field_name) ),*
            ];
            pub const fn field_names(&self) -> &'static [&'static str]{
                &Self::FIELD_NAMES
            }
            pub const fn field_values(&self) -> [$field_type; crate::count_fields!($($field_name) *)]{
                [$( self.$field_name ),*]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_numeric_enum {
    {
    $valtype:ident, $(#[$inner:meta])* $vis:vis $name:ident [$($field_name: tt = $field_value: expr), + $(,)?]
    }  => {
        #[repr($valtype)]
        $(#[$inner])*
        $vis enum $name{
            $($field_name = $field_value,)*
            Unknown($valtype)
        }
        impl $name{
            pub const fn from_value(bits: $valtype) -> Self {
                match bits {
                    $($field_value => Self::$field_name,)*
                    _ => Self::Unknown(bits)
                }
            }
            pub const fn into_value(self) -> $valtype {
                match self {
                    $(Self::$field_name => $field_value,)*
                    Self::Unknown(_) => $valtype::MAX
                }
            }
        }
    };
}
