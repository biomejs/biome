use crate::{AnyGritPattern, GritRoot};
use biome_rowan::AstNode;

pub trait GritRootExt {
    fn pattern(&self) -> Option<AnyGritPattern>;
}

impl GritRootExt for GritRoot {
    fn pattern(&self) -> Option<AnyGritPattern> {
        self.definitions()
            .into_iter()
            .find_map(|definition| AnyGritPattern::cast(definition.ok()?.into_syntax()))
    }
}
