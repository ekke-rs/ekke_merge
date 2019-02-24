use std:: { mem::replace };
use crate:: { MergeResult };



pub trait Merge
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>;
}




// TODO: Make generic for all tuple lengths, and verify it works with repeating types, eg. (T,T)
//
impl<T, U> Merge for (T, U) where T: Merge, U: Merge
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>
	{
		replace( &mut self.0, other.0 );
		replace( &mut self.1, other.1 );

		Ok(())
	}
}



#[ cfg( test ) ]
//
mod tests
{
	// use crate::*;

}

