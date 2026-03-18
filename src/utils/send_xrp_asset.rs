// src/utils/send_xrp_asset.rs
use crate::context::{EuroContext, RlusdContext, SgdContext, XrpContext};
use dioxus_native::prelude::ReadableExt;

#[derive(Clone, PartialEq, Debug)]
pub enum SendAsset {
    Xrp,
    Rlusd,
    Euro,
    Sgd,
    // ── ADD NEW TOKENS HERE ──
}

impl SendAsset {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "RLUSD" => Self::Rlusd,
            "EUROP" => Self::Euro,
            "XSGD" => Self::Sgd,

            _ => Self::Xrp,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Xrp => "XRP",
            Self::Rlusd => "RLUSD",
            Self::Euro => "EUROP",
            Self::Sgd => "XSGD",
        }
    }

    pub fn has_usd_equivalent(&self) -> bool {
        matches!(self, Self::Xrp)
    }

    pub fn fiat_rate_key(&self) -> Option<&'static str> {
        match self {
            Self::Xrp => Some("XRP/USD"),
            _ => None,
        }
    }

    pub fn balance(
        &self,
        xrp: &XrpContext,
        rlusd: &RlusdContext,
        euro: &EuroContext,
        sgd: &SgdContext,
    ) -> f64 {
        match self {
            Self::Xrp => xrp.wallet_balance.read().0,
            Self::Rlusd => rlusd.rlusd.read().0,
            Self::Euro => euro.euro.read().0,
            Self::Sgd => sgd.sgd.read().0,
        }
    }
}
