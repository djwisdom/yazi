use yazi_adapter::Dimension;
use yazi_macro::render;
use yazi_shared::event::CmdCow;
use yazi_widgets::{Scrollable, Step};

use crate::help::{HELP_MARGIN, Help};

struct Opt {
	step: Step,
}

impl From<CmdCow> for Opt {
	fn from(c: CmdCow) -> Self {
		Self { step: c.first().and_then(|d| d.try_into().ok()).unwrap_or_default() }
	}
}

impl From<isize> for Opt {
	fn from(n: isize) -> Self { Self { step: n.into() } }
}

impl Help {
	#[yazi_codegen::command]
	pub fn arrow(&mut self, opt: Opt) {
		render!(self.scroll(opt.step));
	}
}

impl Scrollable for Help {
	#[inline]
	fn total(&self) -> usize { self.bindings.len() }

	#[inline]
	fn limit(&self) -> usize { Dimension::available().rows.saturating_sub(HELP_MARGIN) as usize }

	#[inline]
	fn cursor_mut(&mut self) -> &mut usize { &mut self.cursor }

	#[inline]
	fn offset_mut(&mut self) -> &mut usize { &mut self.offset }
}
