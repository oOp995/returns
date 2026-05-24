/// Works like a compiler silencer, returns (never) value `!` to the compiler which coerces into any
/// type `T` in favor of making the compiler silent while prototyping in situations where you need to
///  continue the architecture and you do not care about the returned value yet
/// 
/// # Behaviour
/// - in **debug mode** it will panic with detailed msg
/// - in **release mode** it will trigger compile error 
/// so it can not reach **release mode** by accident
/// 
/// # Panics
/// - this macro will panic in **debug mode** under any circumstances, it is just a place holder, compiler silencer
/// 
/// # Problem 
/// this example will not compile
/// ```compile_fail
/// pub fn prototype<T>(opt:Option<T>)->T{
///    match opt {
///        Some(val)=>val,
///        None=>{
///            //i do not know what to make here yet
///            //but compiler is yelling at me, he wants
///            //some returned value to eat.
///            
///        }
///    }
///}
/// ```
/// # Solution (temporary) 
/// - this example will compile.
/// - it will panic in **debug mode**
/// - it will trigger compile error in **release mode**
/// - it is just a place holder
/// ```should_panic
/// pub fn prototype<T>(opt:Option<T>)->T{
///    match opt {
///        Some(val)=>val,
///        None=>{
///            //i do not know what to make here yet
///            //but compiler is yelling at me, he wants
///            //some returned value to eat.
/// 
///            //add the next line and compiler will
///            //sit tight for a while.
///            returns::return_shit!()
///        }
///    }
///}
/// fn main(){
///     prototype::<i32>(None);
/// }
/// ```
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
/// Returns the default value of `T` that means it returns
/// `<T as Default>::default()` in situations where default values sound more 
/// propper than panics in debug mode or situations that default values can be actually
/// usefull 
/// 
/// 
/// # Behaviour
/// - in **debug mode** it will compile normally and returns `<T as Default>::default()` to the calling code
/// - in **release mode** it will trigger compile error
/// so it can not reach **release mode** by accident
/// - in **release mode** if you explicitly opt-in to use the default value thru using 
/// `return_default!(force)`, it will compile normally and returns `<T as Default>::default()` to the calling code
/// 
/// # Panics
/// - no panics
/// 
/// # Problem 
/// this example will not compile
/// ```compile_fail
/// pub fn diagnose<T>() -> T 
/// where 
///     T:Default
/// {
///     // compiler is yelling at me 
///     // to return some value
///     
/// }
///```
/// # Solution 
/// - this example will compile and run normally in **debug mode**.
/// - it will trigger compile error in **release mode**
/// ```rust
/// pub fn diagnose<T>() -> T 
/// where 
///     T:Default
/// {
///     // compiler is yelling at me 
///     // to return some value
/// 
///     //add this line, and compiler
///     //will go for vacancy
///     returns::return_default!()
/// }
/// fn main (){
///     diagnose::<i32>();
/// }
/// ```
/// 
/// - foce the compiler to use the default value in **release mode**
/// ```rust
/// pub fn diagnose<T>() -> T 
/// where 
///     T:Default
/// {
///     // compiler is yelling at me 
///     // to return some value
/// 
///     //add this line, and compiler
///     //will go for vacancy
///     returns::return_default!(force)
/// }
/// fn main (){
///     diagnose::<i32>();
/// }
/// ```
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
