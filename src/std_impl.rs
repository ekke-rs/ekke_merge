use crate::{ Merge, MergeResult };
use std::collections::{ BTreeMap, btree_map::Entry as BtEntry, HashMap, hash_map::Entry as HsEntry };
use std::{ vec::Vec, mem::replace };


// Primitives ---------------------------------

macro_rules! simple_merge
{
	( $type:ty ) =>

	(
		impl Merge for $type
		{
			fn merge( &mut self, other: Self ) -> MergeResult<()>
			{
				replace( self, other );

				Ok(())
			}
		}
	)
}


simple_merge!( bool         );
simple_merge!( char         );

simple_merge!( u8           );
simple_merge!( u16          );
simple_merge!( u32          );
simple_merge!( u64          );
simple_merge!( u128         );
simple_merge!( usize        );

simple_merge!( i8           );
simple_merge!( i16          );
simple_merge!( i32          );
simple_merge!( i64          );
simple_merge!( i128         );
simple_merge!( isize        );

simple_merge!( f32          );
simple_merge!( f64          );

simple_merge!( String       );


// Collections ---------------------------------

impl<K, V> Merge for BTreeMap< K, V >

where K: std::cmp::Ord,
      V: Merge,
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		for (k, v) in other.into_iter()
		{
			match self.entry( k )
			{
				BtEntry::Occupied( mut e ) => { e.get_mut().merge( v )?; },
				BtEntry::Vacant  (     e ) => { e.insert( v )          ; },
			};
		}

		Ok(())
	}
}


impl<K, V> Merge for HashMap< K, V >

where K: std::cmp::Ord + std::hash::Hash,
      V: Merge,
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		for (k, v) in other.into_iter()
		{
			match self.entry( k )
			{
				HsEntry::Occupied( mut e ) => { e.get_mut().merge( v )?; },
				HsEntry::Vacant  (     e ) => { e.insert( v )          ; },
			};
		}

		Ok(())
	}
}



// We switched to not merging arrays, because it means you cannot unset values once they are present.
//
/*impl Merge for Vec< Value >
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		for v in other.into_iter()
		{
			if !self.contains( &v )
			{
				self.push( v );
			}
		}

		Ok(())
	}
}*/


impl<T> Merge for Vec< T > where T: Merge
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		replace( self, other );

		Ok(())
	}
}



#[ cfg( test ) ]
//
mod tests
{
	use crate::*;
	use std::collections::BTreeMap;
	use std::collections::HashMap;

	fn cmp<T>( mut a: T, b: T, expect: T )
	where T: Merge + std::cmp::PartialEq + std::fmt::Debug
	{
		a.merge( b ).unwrap();
		assert_eq!( a, expect );

	}


	#[ test ] fn merge_u8   () { cmp( 5 as u8   , 6 as u8   , 6 as u8    ) }
	#[ test ] fn merge_u16  () { cmp( 5 as u16  , 6 as u16  , 6 as u16   ) }
	#[ test ] fn merge_u32  () { cmp( 5 as u32  , 6 as u32  , 6 as u32   ) }
	#[ test ] fn merge_u64  () { cmp( 5 as u64  , 6 as u64  , 6 as u64   ) }
	#[ test ] fn merge_u128 () { cmp( 5 as u128 , 6 as u128 , 6 as u128  ) }
	#[ test ] fn merge_usize() { cmp( 5 as usize, 6 as usize, 6 as usize ) }

	#[ test ] fn merge_i8   () { cmp( 5 as i8   , -6 as i8   , -6 as i8    ) }
	#[ test ] fn merge_i16  () { cmp( 5 as i16  , -6 as i16  , -6 as i16   ) }
	#[ test ] fn merge_i32  () { cmp( 5 as i32  , -6 as i32  , -6 as i32   ) }
	#[ test ] fn merge_i64  () { cmp( 5 as i64  , -6 as i64  , -6 as i64   ) }
	#[ test ] fn merge_i128 () { cmp( 5 as i128 , -6 as i128 , -6 as i128  ) }
	#[ test ] fn merge_isize() { cmp( 5 as isize, -6 as isize, -6 as isize ) }

	#[ test ] fn merge_bool()  { cmp( true , false, false ) }
	#[ test ] fn merge_bool2() { cmp( false,  true, true  ) }

	#[ test ] fn merge_char()  { cmp( 'a', 'b', 'b' ) }

	// Collections
	//
	#[ test ] fn merge_vec()  { cmp( vec![ 1, 2 ], vec![ 3, 4 ], vec![ 3, 4 ] ) }


	#[ test ] fn merge_btmap()
	{
		let mut a = BTreeMap::new();
		let mut b = BTreeMap::new();
		let mut c = BTreeMap::new();

		a.insert( "a", 5 );
		a.insert( "b", 7 );

		b.insert( "b", 1 );

		c.insert( "a", 5 );
		c.insert( "b", 1 );

		cmp( a, b, c )
	}


	#[ test ] fn merge_nested_btmap()
	{
		let mut a = BTreeMap::new();
		let mut b = BTreeMap::new();
		let mut c = BTreeMap::new();
		let mut m = BTreeMap::new();
		let mut n = BTreeMap::new();
		let mut o = BTreeMap::new();

		m.insert( "m", 1 );
		m.insert( "n", 2 );
		n.insert( "n", 3 );

		a.insert( "a", m );
		b.insert( "a", n );

		o.insert( "m", 1 );
		o.insert( "n", 3 );

		c.insert( "a", o );

		cmp( a, b, c )
	}


	#[ test ] fn merge_hashmap()
	{
		let mut a = HashMap::new();
		let mut b = HashMap::new();
		let mut c = HashMap::new();

		a.insert( "a", 5 );
		a.insert( "b", 7 );

		b.insert( "b", 1 );

		c.insert( "a", 5 );
		c.insert( "b", 1 );

		cmp( a, b, c )
	}


	#[ test ] fn merge_nested_hashmap()
	{
		let mut a = HashMap::new();
		let mut b = HashMap::new();
		let mut c = HashMap::new();
		let mut m = HashMap::new();
		let mut n = HashMap::new();
		let mut o = HashMap::new();

		m.insert( "m", 1 );
		m.insert( "n", 2 );
		n.insert( "n", 3 );

		a.insert( "a", m );
		b.insert( "a", n );

		o.insert( "m", 1 );
		o.insert( "n", 3 );

		c.insert( "a", o );

		cmp( a, b, c )
	}
}
