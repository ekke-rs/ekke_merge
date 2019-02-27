use ekke_merge::{ Merge, MergeResult };
use ekke_merge_derive::{ Merge };


// PartialEq and Debug are required by assert, not by Merge
//
#[ derive( PartialEq, Debug, Merge ) ]
//
struct Settings
{
	name  : String,
	amount: usize ,
}


fn main() -> MergeResult<()>
{
	let mut a = Settings{ name: "a".to_string(), amount: 5 };
	let     b = Settings{ name: "b".to_string(), amount: 8 };

	a.merge( b )?;

	assert_eq!( a, Settings{ name: "b".to_string(), amount: 8 } );

	dbg!( a );

	Ok(())
}
