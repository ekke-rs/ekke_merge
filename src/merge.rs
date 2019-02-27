use crate:: { MergeResult };

pub trait Merge
{
	fn merge( &mut self, other: Self ) -> MergeResult<()>;
}


