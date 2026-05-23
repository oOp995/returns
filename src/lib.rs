#[macro_export]
#[cfg(debug_assertions)]
macro_rules! return_shit {
    () => {

        core::panic!("debug-mode::return_shit! reached at [{}:{}:{}]\nyou may want to return valid value before going to release mode"
            ,file!()
            ,line!()
            ,column!()
        );
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! return_shit {
    () => {

        compile_error!("garbage return type left in release mode\nrelease-mode::return_shit!\nyou may want to return valid value before going to production")
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! return_default {
    () => {
        return Default::default();
    };
    (force) => {{ return Default::default(); }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! return_default {
    () => {
        compile_error!("default-value return type left in release mode\nrelease-mode::return_default!\nyou may want to return valid value before going to production\nif you insist to use defaults in release mode use:\n`returns::return_default!(force)`")
    };
    (force)=>{
        return Default::default();
    }
}
