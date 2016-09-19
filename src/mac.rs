// Rust code generation
#[macro_export]
macro_rules! cpp {
    // Finished
    () => {};

    // Closures
    (($($id:ident as $cty:tt),*) -> $rty:ty as $crty:tt $body:tt) => {
        {
            #[derive(rust_cpp_internal)]
            struct Dummy(__!(
                ($($id as $cty),*) -> $rty as $crty $body
            ));
            Dummy::call($(& $id as *const _ as *const u8),*)
        }
    };

    // Parse toplevel #include macros
    (#include < $i:ident > $($rest:tt)*) => {cpp!{$($rest)*}};
    (#include $l:tt $($rest:tt)*) => {cpp!{$($rest)*}};

    // Parse toplevel raw macros
    (raw $body:tt $($rest:tt)*) => {cpp!{$($rest)*}};

    // Parse parameters
    (CPP_PARAM $name:ident : $t:ty as $cppt:tt) => {
        $name: $t
    };
    (CPP_PARAM $name:ident : $t:ty as $cppt:tt , $($rest:tt)*) => {
        $name: $t ,
        cpp!{CPP_PARAM $($rest)*}
    };

    // Parse function declarations
    ($(#[$m:meta])*
     fn $id:ident ( $($name:ident : $t:ty as $cppt:tt),* ) -> $rt:ty as $rcppt:tt $body:tt $($rest:tt)*) => {
        extern "C" {
            $(#[$m])*
            fn $id ( $($name : $t),* ) -> $rt ;
        }
        cpp!{$($rest)*}
    };
    ($(#[$m:meta])*
     fn $id:ident ( $($name:ident : $t:ty as $cppt:tt),* ) $body:tt $($rest:tt)*) => {
        extern "C" {
            $(#[$m])*
            fn $id ( $($name : $t),* ) ;
        }
        cpp!{$($rest)*}
    };

    // Parse struct definiton
    ($(#[$m:meta])*
     struct $id:ident { $($i:ident : $t:ty as $cpp:tt ,)* } $($rest:tt)*) => {
        $(#[$m])*
        #[repr(C)]
        struct $id {
            $($i : $t ,)*
        }
        cpp!{$($rest)*}
    };

    // Parse enum definition
    ($(#[$m:meta])*
     enum $id:ident { $($i:ident ,)* } $($rest:tt)*) => {
        $(#[$m])*
        #[repr(C)]
        enum $id {
            $($i ,)*
        }
        cpp!{$($rest)*}
    };

    // Parse enum class definition
    ($(#[$m:meta])*
     enum class $id:ident { $($i:ident ,)* } $($rest:tt)*) => {
        $(#[$m])*
        #[repr(C)]
        enum $id {
            $($i ,)*
        }
        cpp!{$($rest)*}
    };

    // Parse prefixed enum definition
    ($(#[$m:meta])*
     enum prefix $id:ident { $($i:ident ,)* } $($rest:tt)*) => {
        $(#[$m])*
        #[repr(C)]
        enum $id {
            $($i ,)*
        }
        cpp!{$($rest)*}
    };
}
