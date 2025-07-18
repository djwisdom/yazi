use anyhow::{Context, Result};
use serde::Deserialize;
use yazi_codegen::DeserializeOver1;
use yazi_fs::{Xdg, ok_or_not_found};

use crate::{mgr, open, opener, plugin, popup, preview, tasks, which};

#[derive(Deserialize, DeserializeOver1)]
pub struct Yazi {
	pub mgr:     mgr::Mgr,
	pub preview: preview::Preview,
	pub opener:  opener::Opener,
	pub open:    open::Open,
	pub tasks:   tasks::Tasks,
	pub plugin:  plugin::Plugin,
	pub input:   popup::Input,
	pub confirm: popup::Confirm,
	pub pick:    popup::Pick,
	pub which:   which::Which,
}

impl Yazi {
	pub(super) fn read() -> Result<String> {
		let p = Xdg::config_dir().join("yazi.toml");
		ok_or_not_found(std::fs::read_to_string(&p))
			.with_context(|| format!("Failed to read config {p:?}"))
	}

	pub(super) fn reshape(self) -> Result<Self> {
		Ok(Self {
			mgr:     self.mgr.reshape()?,
			preview: self.preview.reshape()?,
			opener:  self.opener.reshape()?,
			open:    self.open.reshape()?,
			tasks:   self.tasks.reshape()?,
			plugin:  self.plugin.reshape()?,
			input:   self.input,
			confirm: self.confirm,
			pick:    self.pick,
			which:   self.which,
		})
	}
}
