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
