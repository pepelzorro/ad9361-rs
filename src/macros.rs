//! Macros

macro_rules! ad9361_method {
    // Call a method with pass-by-value arguments
    (SET: $name:ident;  $( $arg:ident : $t:ty $(=> $rust:ty)* ),* $(; $doc:expr)* ) => {

        paste! {
            $( #[doc=$doc] )*
            pub fn $name(&mut self, $( $arg:$t ),*) -> Result<(), i32> {
                assert!(!self.inner.is_null(), "Must call init() method before accessing ad9361");
                let inner_ptr = self.inner;
                $(
                    $( let $arg: $rust = $arg.into(); )*
                )*

                let status = unsafe {
                    bindings::[< ad9361_ $name >](inner_ptr, $( $arg ),*)
                };

                if status == 0 {
                    Ok(())
                } else {
                    Err(status)
                }
            }
        }
    };
    // Call a method, optionally with pass-by-value arguments, followed by a
    // pass-by-reference return value
    (GET: $name:ident $(, $aux:ident : $t:ty )*;  $rust:ty $(> $intermediate:expr)* => $ret:ty
     $(; $doc:expr)* ) => {

        paste! {
            $( #[doc=$doc] )*
            pub fn $name(&self, $( $aux:$t ),*) -> Result<$ret, i32> {
                assert!(!self.inner.is_null(), "Must call init() method before accessing ad9361");
                let inner_ptr = self.inner;
                let mut result: $rust = Default::default();
                let result_ptr = &mut result;

                let status = unsafe {
                    bindings::[< ad9361_ $name >](inner_ptr, $( $aux, )* result_ptr)
                };

                if status == 0 {
                    $( let result = $intermediate::from(result); )*
                    Ok(result.into())
                } else {
                    Err(status)
                }
            }
        }
    };
    // Call an infalliable method, optionally with pass-by-value arguments,
    // followed by a pass-by-reference return value
    (GET_INFALLIBLE: $name:ident $(, $aux:ident : $t:ty )*;  $rust:ty $(> $intermediate:expr)* => $ret:ty
     $(; $doc:expr)* ) => {

        paste! {
            $( #[doc=$doc] )*
            pub fn $name(&self, $( $aux:$t ),*) -> $ret {
                assert!(!self.inner.is_null(), "Must call init() method before accessing ad9361");
                let inner_ptr = self.inner;
                let mut result: $rust = Default::default();
                let result_ptr = &mut result;

                let _: () = unsafe {
                    bindings::[< ad9361_ $name >](inner_ptr, $( $aux, )* result_ptr)
                };

                $( let result = $intermediate::from(result); )*
                result.into()
            }
        }
    };
    // Call an infalliable method, optionally with pass-by-value arguments
    (GET_INFALLIBLE_VAL: $name:ident $(, $aux:ident : $t:ty )*;  $rust:ty $(> $intermediate:expr)* => $ret:ty
     $(; $doc:expr)* ) => {

        paste! {
            $( #[doc=$doc] )*
            pub fn $name(&self, $( $aux:$t ),*) -> $ret {
                assert!(!self.inner.is_null(), "Must call init() method before accessing ad9361");
                let inner_ptr = self.inner;
                let retval = unsafe {
                    bindings::[< ad9361_ $name >](inner_ptr, $( $aux, )*)
                };

                $( let retval = $intermediate::from(retval); )*
                retval.into()
            }
        }
    };
    (GET_SET: $name:ident $(, $aux:ident:$t:ty )*; $arg:ty $(> $intermediate:expr)* => $rust:ty
        $(; $doc:expr)* ) => {

        paste! {
            ad9361_method!(SET: [< set_ $name >]; $( $aux:$t, )* arg: $arg => $rust
                           $(; concat!("Set the ", $doc) )*);
            ad9361_method!(GET: [< get_ $name >] $(, $aux:$t )*; $rust $(> $intermediate)* => $arg
                           $(; concat!("Get the ", $doc) )*);

        }
    };
    (GET_SET2: $name:ident $(, $aux:ident:$t:ty )*; $arg:ty $(> $intermediate:expr)* => $rust:ty
        $(; $doc:expr)* ) => {

        paste! {
            ad9361_method!(SET: [< $name >]; $( $aux:$t, )* arg: $arg => $rust
                           $(; concat!("Set the ", $doc) )*);
            ad9361_method!(GET_INFALLIBLE: [< get_ $name >] $(, $aux:$t )*; $rust $(> $intermediate)* => $arg
                           $(; concat!("Get the ", $doc) )*);

        }
    };
}
