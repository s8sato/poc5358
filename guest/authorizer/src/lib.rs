use poc::wit::types::*;

wit_bindgen::generate!({
    world: "universe",
    path: "../../wit",
});

struct Authorizer;

/// Default implementation for permission validation.
impl Guest for Authorizer {
    fn read_request(_args: String) -> ReadSet {
        unimplemented!("boilerplate");
    }

    fn read_approval(signals: ReadSet, receptors: AllowSet) -> bool {
        let mut signals = signals;
        signals.inner.retain(|signal| {
            let approved = receptors.inner.iter().any(|receptor| {
                let captures = receptor.key.captures(&signal.key);
                let NodeValueAllow::AccountAsset(AccountAssetA { bit_mask }) = receptor.value;
                let passes = 0b0000_0001 & !bit_mask == 0;
                captures && passes
            });

            !approved
        });

        signals.inner.is_empty()
    }

    fn write_request(_view: ViewSet, _args: String) -> WriteSet {
        unimplemented!("boilerplate");
    }

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
pub trait Capture<T> {
    fn captures(&self, candidate: &T) -> bool;
}

impl Capture<NodeKey> for FuzzyNodeKey {
    fn captures(&self, key: &NodeKey) -> bool {
        let (
            FuzzyNodeKey::AccountAsset(FuzzyCompositeKey { e0: z0, e1: z1 }),
            NodeKey::AccountAsset(CompositeKey { e0, e1 }),
        ) = (self, key);
        z0.as_ref().is_none_or(|z0| z0 == e0) && z1.as_ref().is_none_or(|z1| z1 == e1)
    }
}

impl Capture<FuzzyNodeKey> for FuzzyNodeKey {
    fn captures(&self, candidate: &FuzzyNodeKey) -> bool {
        let (
            FuzzyNodeKey::AccountAsset(FuzzyCompositeKey { e0: z0, e1: z1 }),
            FuzzyNodeKey::AccountAsset(FuzzyCompositeKey { e0, e1 }),
        ) = (self, candidate);
        z0.as_ref()
            .is_none_or(|z0| e0.as_ref().is_some_and(|e0| z0 == e0))
            && z1
                .as_ref()
                .is_none_or(|z1| e1.as_ref().is_some_and(|e1| z1 == e1))
    }
}
