use poc::wit::types::*;

wit_bindgen::generate!({
    world: "authorizer",
    path: "../../wit",
});

struct Authorizer;

impl Guest for Authorizer {
    fn read_approval(signals: EventSet, receptors: AllowSet) -> bool {
        Self::write_approval(signals, receptors)
    }

    /// Default implementation for permission validation.
    fn write_approval(signals: EventSet, receptors: AllowSet) -> bool {
        let mut signals = signals;
        signals.inner.retain(|signal| {
            let NodeValueEvent::AccountAsset(AccountAssetE { status_bit }) = signal.value;
            let receptors = receptors
                .inner
                .iter()
                .filter(|receptor| receptor.key.captures(&signal.key));
            let Some(bit_mask_union) = receptors
                .map(|entry| {
                    let NodeValueAllow::AccountAsset(AccountAssetA { bit_mask }) = entry.value;
                    bit_mask
                })
                .reduce(|acc, bit| acc | bit)
            else {
                return false;
            };

            let approved = status_bit & !bit_mask_union == 0;
            !approved
        });

        signals.inner.is_empty()
    }
}

export!(Authorizer);

// TODO: move common types to separate crate from host
pub trait Capture {
    type Captured;
    fn captures(&self, candidate: &Self::Captured) -> bool;
}

impl Capture for FuzzyNodeKey {
    type Captured = NodeKey;
    fn captures(&self, key: &Self::Captured) -> bool {
        let (
            FuzzyNodeKey::AccountAsset(FuzzyCompositeKey { e0: z0, e1: z1 }),
            NodeKey::AccountAsset(CompositeKey { e0, e1 }),
        ) = (self, key);
        z0.as_ref().is_none_or(|z0| z0 == e0) && z1.as_ref().is_none_or(|z1| z1 == e1)
    }
}
