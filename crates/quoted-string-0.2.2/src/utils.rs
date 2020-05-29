#[doc(hidden)]
#[macro_export]
macro_rules! assert_ok {
    ($val:expr) => ({
        match $val {
            Ok( res ) => res,
            Err( err ) => panic!( "expected Ok(..) got Err({:?})", err)
        }
    });
    ($val:expr, $ctx:expr) => ({
        match $val {
            Ok( res ) => res,
            Err( err ) => panic!( "expected Ok(..) got Err({:?}) [ctx: {:?}]", err, $ctx)
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! assert_err {
    ($val:expr) => ({
        match $val {
            Ok( val ) => panic!( "expected Err(..) got Ok({:?})", val),
            Err( err ) => err,
        }
    });
    ($val:expr, $ctx:expr) => ({
        match $val {
            Ok( val ) => panic!( "expected Err(..) got Ok({:?}) [ctx: {:?}]", val, $ctx),
            Err( err ) => err,
        }
    });
}
