use serde_yaml::{ Value, Mapping };
use crate::{ Merge, MergeError, MergeResult };
use std:: { mem::replace, mem::discriminant as discri };


impl Merge for Value
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		// You can always replace a null value
		//
		if self == &Value::Null
		{
			replace( self, other );
			return Ok(())
		}

		// We do not allow overriding values of a different type. This allows the code
		// to count on type safety.
		//
		if discri( self ) != discri( &other )
		{
			return Err( MergeError::MergeWrongType( self.clone(), other ).into() );
		}


		match other
		{
			// We do not allow unsetting configuration values. This allows setting a default
			// value and knowing that a value of the correct type will always exist, whatever
			// the user overrides.
			//
			Value::Null         => {},
			Value::Bool     (_) => { replace( self, other ); } ,
			Value::Number   (_) => { replace( self, other ); } ,
			Value::String   (_) => { replace( self, other ); } ,
			Value::Sequence (_) => { replace( self, other ); } ,

			Value::Mapping  (y) =>
			{
				if let Value::Mapping( x ) = self
				{
					x.merge( y )?;
				}
			},
		};

		Ok(())
	}
}



impl Merge for Mapping
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		for (k, v) in other.into_iter()
		{
			match self.get_mut( &k )
			{
				Some( val ) => { val .merge (    v )?; },
				None        => { self.insert( k, v ) ; },

			}
		}

		Ok(())
	}
}





#[ cfg( test ) ]
//
mod tests
{
	use crate::*;

	use serde_yaml::{ from_str, Value };


	// Takes away some boilerplate
	//
	fn cmp( a: &str, b: &str, expect: &str )
	{
		let mut a : Value = from_str( a      ).unwrap();
		let b     : Value = from_str( b      ).unwrap();
		let c     : Value = from_str( expect ).unwrap();

		a.merge( b ).unwrap();

		assert_eq!( a, c );
	}


	#[test]
	//
	fn merge_into_null()
	{
		cmp( "~", "{ a: -2 }", "{ a: -2 }" );
	}


	#[test] fn basic_array      () { cmp( "[ 1 ]"   , "[ 2 ]"      , "[ 2 ]"       ); }
	#[test] fn empty_array      () { cmp( "[]"      , "[ 1, 2 ]"   , "[ 1, 2 ]"    ); }
	#[test] fn merge_empty_array() { cmp( "[ 1, 2 ]", "[]"         , "[]"          ); }
	#[test] fn complex_array    () { cmp( "[ 1, 3 ]", "[ 2, 4 ]"   , "[ 2, 4 ]"    ); }
	#[test] fn overlap_array    () { cmp( "[ 1, 3 ]", "[ 1, 2, 4 ]", "[ 1, 2, 4 ]" ); }


	#[ test         ]
	#[ should_panic ]
	//
	fn basic_object_null()
	{
		let mut a = Value::String( "bli".to_string() );
		let     b = Value::Null;

		a.merge( b ).unwrap();
	}


	#[test] fn basic_object_bool       () { cmp( "a: true"      , "a: false"     , "a: false"      ); }
	#[test] fn basic_object_empty      () { cmp( "{}"           , "{ a: true\n }", "{ a: true\n }" ); }
	#[test] fn basic_object_merge_empty() { cmp( "{ a: true\n }", "{}"           , "{ a: true\n }" ); }


	#[ test         ]
	#[ should_panic ]
	//
	fn basic_object_string_bool()
	{
		let mut a = Value::String( "bli".to_string() );
		let     b = Value::Bool( true );

		a.merge( b ).unwrap();
	}


	#[test] fn basic_object_u64   () { cmp( "{ a: 1 }"    , "{ a: 2 }"    , "{ a: 2 }"     ); }
	#[test] fn basic_object_i64   () { cmp( "{ a: -1 }"   , "{ a: -2 }"   , "{ a: -2 }"    ); }
	#[test] fn basic_object_f64   () { cmp( "{ a: -1.3 }" , "{ a: -2.5 }" , "{ a: -2.5 }"  ); }
	#[test] fn basic_object_string() { cmp( "{ a: bli\n }", "{ a: bla\n }", "{ a: bla\n }" ); }


	#[test]
	//
	fn nested_object()
	{
		let a =
"
a: 1
obj:
  blo: bling
  bli: bli
";

		let b =
"
obj:
  bli: bla
";

		let expect =
"
a: 1
obj:
  blo: bling
  bli: bla
";

		cmp( a, b, expect );
	}


	#[test]
	//
	fn u64_nested_object()
	{
		let a =
"
a: 1
obj:
  blo: bling
  bli: bli
";

		let b =
"
a: 2
obj:
  bli: bla
";

		let expect =
"
a: 2
obj:
  blo: bling
  bli: bla
";

		cmp( a, b, expect );
	}


	#[test]
	//
	fn u64_nested_array()
	{
		let a =
"
a: 1
obj:
  bli: [ 1 ]
";

		let b =
"
a: 2
obj:
  bli: [ 2, 4 ]
";

		let expect =
"
a: 2
obj:
  bli: [ 2, 4 ]
";

		cmp( a, b, expect );
	}


}
