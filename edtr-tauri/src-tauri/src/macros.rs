#[macro_export]
macro_rules! to_setup {
    ($window:ident | $($n:ident),*) => {
        to_setup!($window | # $($n);* # 0.0 ; $($n)*);
    };

    ($window:ident | # $($spare:ident);* # $count:expr ; $first:ident $($rest:ident)*) => {
        to_setup!($window | # $($spare);* # $count + 1.0 ; $($rest)* );
    };

    ($window:ident | # $($spare:ident);* # $count:expr ;) => {
        static mut COUNTS: f32 = 0.0;

        tokio::join!($(
            {
                let w = $window.clone();

                async move {
                    $spare().await;
                    w.emit("splash:progress", ((unsafe {COUNTS += 1.0; COUNTS}) / $count) * 100.0).unwrap();
                    if unsafe { COUNTS } == $count {
                        w.emit("splash:loaded", 0).unwrap();
                    }
                }
            }
        ),*);
    };
}

#[macro_export]
/// Exports the fields of a given struct to an array in a JSON file.
macro_rules! export_keys {
    ($strt:ty, $path:literal) => {
        export_keys!(@ $strt, $path);
    };

    ($strt:ty) => {
        export_keys!(@ $strt, "./fields.json");
    };

    (@ $strt:ty, $path:literal) => {
        #[cfg(test)]
        mod export_keys {
            use serde::de::{self, Deserialize, Deserializer, Visitor};
            use serde::{forward_to_deserialize_any};

            // https://github.com/serde-rs/serde/issues/1110
            fn struct_fields<'de, T>() -> &'static [&'static str]
            where
                T: Deserialize<'de>,
            {
                struct StructFieldsDeserializer<'a> {
                    fields: &'a mut Option<&'static [&'static str]>,
                }

                impl<'de, 'a> Deserializer<'de> for StructFieldsDeserializer<'a> {
                    type Error = serde::de::value::Error;

                    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
                    where
                        V: Visitor<'de>,
                    {
                        Err(de::Error::custom(""))
                    }

                    fn deserialize_struct<V>(
                        self,
                        _name: &'static str,
                        fields: &'static [&'static str],
                        visitor: V,
                    ) -> Result<V::Value, Self::Error>
                    where
                        V: Visitor<'de>,
                    {
                        *self.fields = Some(fields);
                        self.deserialize_any(visitor)
                    }

                    forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
                        byte_buf option unit unit_struct newtype_struct seq tuple
                        tuple_struct map enum identifier ignored_any
                    }
                }

                let mut fields = None;
                let _ = T::deserialize(StructFieldsDeserializer {
                    fields: &mut fields,
                });
                fields.unwrap()
            }

            #[test]
            fn export_keys() {
                let fields = struct_fields::<$strt>();
                std::fs::write($path, serde_json::to_string(&fields).expect("invalid field names")).expect("failed to write field names");
            }
        }
    };
}

/// Generates a handler that returns the instance of the given struct so that it can be managed from
/// JS. Struct must also be added to the [`gen_handlers`](gen_handlers) macro
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! js_manage {
    ( $($path:ident)::* ) => {
        impl $($path)::* where Self: TS {}

        tt_call::tt_call! {
            macro = [{ $crate::__last_of_path }]
            input = [{ $($path)::* }]
            ~~> crate::__inner_manage_gen
        }
    };
}

// Generates the handlers for tauri
#[macro_export]
macro_rules! __inner_manage_gen {
    (
        data = [{ $name:ident, $($path:ident)::* }]
    ) => {
        paste::paste! {
            #[macro_export]
            #[allow(clippy::crate_in_macro_def)]
            macro_rules! [<__import_ $name:lower>] {
                () => {
                    #[tauri::command]
                    fn [<get_ $name:lower>](state: tauri::State< $($path)::* >) -> &$($path)::* {
                        state.inner()
                    }
                }
            }
        }
    };
}

/// Gets the last ident in a path
#[macro_export]
macro_rules! __last_of_path {
    (
        $caller:tt
        input = [{ $($path:ident)::* }]
    ) => {
        $crate::__last_of_path!(
            # $caller # $($path)::* | $($path)::*
        );
    };

    (# $caller:tt # $($path:ident)::* | $_:ident :: $($rest:tt)+) => {
        $crate::__last_of_path!(# $caller # $($path)::* | $($rest)+);
    };

    (# $caller:tt # $($path:ident)::* | ::$($rest:tt)+) => {
        $crate::__last_of_path!(# $caller # $($path)::* | $($rest)+)
    };

    (# $caller:tt # $($path:ident)::* | $id:ident) => {
        tt_call::tt_return! {
            $caller
            data = [{ $id, $($path)::* }]
        }
    };
}

#[macro_export]
macro_rules! gen_handers {
    ($($structs:ident),*) => {
        paste::paste! {
            $(
                [<__import_ $structs:lower>]! {}
            )*

            struct Handlers;

            impl Handlers {
                fn all<R: tauri::Runtime>(invoker: tauri::Invoke<R>)
                {
                    let _tauri_invoker: &'static dyn Fn(tauri::Invoke<R>) = &tauri::generate_handler![
                        $(
                            [<get_ $structs:lower>]
                        ),*
                    ];

                    _tauri_invoker(invoker);
                }
            }
        }
    }
}
