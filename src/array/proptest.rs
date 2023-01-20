use crate::{ByteArray, Format};
use proptest::arbitrary::StrategyFor;
use proptest::prelude::*;
use proptest::strategy::Map;

impl<F: Format, const N: usize> Arbitrary for ByteArray<F, N> {
    type Parameters = ();
    type Strategy = Map<StrategyFor<[u8; N]>, fn([u8; N]) -> Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<[u8; N]>().prop_map(Self::from)
    }
}
