use failure    :: { Error } ;

#[ cfg( feature = "serdeyaml" ) ] use failure    :: { Fail  } ;
#[ cfg( feature = "serdeyaml" ) ] use serde_yaml :: { Value } ;


/// Custom result type, Allows to omit error type since it's always
/// [`failure::Error`](https://docs.rs/failure/0.1.5/failure/struct.Error.html).
///
pub type MergeResult<T> = Result< T, Error >;


/// The specific errors ekke_io can return.
///
#[ derive( Debug, Fail ) ]
//
#[ cfg( feature = "serdeyaml" ) ]
//
pub enum MergeError
{
	#[ fail( display = "Cannot merge two config values of different types: {:#?} and {:#?}", _0, _1 ) ]
	//
	MergeWrongType( Value, Value ),
}
