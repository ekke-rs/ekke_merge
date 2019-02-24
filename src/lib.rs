mod merge;
mod error;
mod std_impl;

pub use merge::
{
	Merge ,
};


pub use error::
{
	MergeResult ,
};


#[ cfg( feature = "serdeyaml" ) ]
//
pub use error::
{
	MergeError  ,
};


#[ cfg( feature = "serdeyaml" ) ]
//
mod serde_yaml_impl;
