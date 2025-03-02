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
macro_rules! display_with_field_name {
    {
    $fmt:literal,$(#[$inner:meta])* $vis:vis struct $name:ident {$($field_vis:vis $field_name: ident: $field_type: tt), + $(,)?}
    }  => {
        $(#[$inner])*
        $vis struct $name{
            $($field_vis $field_name: $field_type,)*
        }
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let arr = self.array();
            paste::paste!{
                #[allow(non_upper_case_globals)]
                const [<$name _ FIELD_NAMES>]: [&str; crate::count_fields!($($field_name) *)] = [
                    $( stringify!($field_name) ),*
                ];
            }
            let field_names = &paste::paste! { [<$name _ FIELD_NAMES>] };
            let m = field_names.len() % 3;
            for index in (0..field_names.len() - m).step_by(3){
                let _ = f.write_fmt(format_args!(
                    concat!($fmt," ",$fmt, " ",$fmt,"\n"),
                    field_names[index],
                    arr[index],
                    field_names[index + 1],
                    arr[index + 1],
                    field_names[index + 2],
                    arr[index + 2],
                ));
            }
            match m{
                1 => {let _ = f.write_fmt(format_args!(
                    concat!($fmt,"\n"),
                    field_names[field_names.len() -1],
                    arr[field_names.len() -1]));},
                2 => {let _ = f.write_fmt(format_args!(
                    concat!($fmt, " ",$fmt,"\n"),
                    field_names[field_names.len() -2],
                    arr[field_names.len() -2],
                    field_names[field_names.len() -1],
                    arr[field_names.len() -1]));},
                _ => unreachable!()
            }
            Ok(())
        }
        }
    };
}
