use blake2s_simd::Params;
use rand::{CryptoRng, Rng, SeedableRng};
use rand_chacha::ChaChaRng;

pub const SEED_PERSONALIZATION: &[u8] = b"ALEOSEED";

pub fn derive_rng_from_seed(seed: &[u8]) -> impl Rng + CryptoRng {
    let seed_hash = Params::new()
        .personal(SEED_PERSONALIZATION)
        .to_state()
        .update(seed)
        .finalize();
    ChaChaRng::from_seed(*seed_hash.as_array())
}
