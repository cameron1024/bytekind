use proptest::arbitrary::StrategyFor;
use proptest::prelude::*;
use proptest::strategy::Map;

use crate::{Bytes, Format};

impl<F: Format> Arbitrary for Bytes<F> {
    type Parameters = ();
    type Strategy = Map<StrategyFor<Vec<u8>>, fn(Vec<u8>) -> Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<Vec<u8>>().prop_map(Self::from)
    }
}
