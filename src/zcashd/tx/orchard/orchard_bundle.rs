use anyhow::Result;

use crate::{parse, Blob, Blob32, Blob64, Parse, Parser};

use crate::ZatBalance;

use super::{OrchardAction, OrchardAnchor, OrchardAuthorized, OrchardFlags};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardBundle {
    actions: Vec<OrchardAction>,
    flags: OrchardFlags,
    balance: ZatBalance,
    anchor: OrchardAnchor,
    authorized: OrchardAuthorized,
}


impl OrchardBundle {
    pub fn actions(&self) -> &[OrchardAction] {
        &self.actions
    }

    pub fn flags(&self) -> &OrchardFlags {
        &self.flags
    }

    pub fn balance(&self) -> &ZatBalance {
        &self.balance
    }

    pub fn anchor(&self) -> &OrchardAnchor {
        &self.anchor
    }

    pub fn authorized(&self) -> &OrchardAuthorized {
        &self.authorized
    }
}

impl Parse for OrchardBundle {
    fn parse(p: &mut Parser) -> Result<Self> {
        let actions_without_auth: Vec<OrchardAction> = parse!(p, "orchard bundle actions")?;
        // let actions = parse!(p, "orchard bundle actions")?;
        let flags = parse!(p, "orchard bundle flags")?;
        let balance = parse!(p, "orchard bundle balance")?;
        let anchor = parse!(p, "orchard bundle anchor")?;
        let authorized = parse!(p, "orchard bundle authorized")?;
        Ok(Self {
            actions,
            flags,
            balance,
            anchor,
            authorized,
        })
    }
}
