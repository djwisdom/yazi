use yazi_shared::event::{CmdCow, Data};

pub struct SpotOpt {
	pub skip: Option<usize>,
}

impl From<CmdCow> for SpotOpt {
	fn from(c: CmdCow) -> Self { Self { skip: c.get("skip").and_then(Data::as_usize) } }
}
