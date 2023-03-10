use ed25519::signature::{Signer, Verifier};
use ring_compat::signature::ed25519::{SigningKey, VerifyingKey};

pub struct SignerUtil<S>
where
    S: Signer<ed25519::Signature>,
{
    pub signing_key: S,
}

impl<S> SignerUtil<S>
where
    S: Signer<ed25519::Signature>,
{
    pub fn sign(&self, person: &str) -> ed25519::Signature {
        // NOTE: use `try_sign` if you'd like to be able to handle
        // errors from external signing services/devices (e.g. HSM/KMS)
        // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
        self.signing_key.sign(format_message(person).as_bytes())
    }
}

pub struct VerifierUtil<V> {
    pub verifying_key: V,
}

impl<V> VerifierUtil<V>
where
    V: Verifier<ed25519::Signature>,
{
    pub fn verify(
        &self,
        person: &str,
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verifying_key
            .verify(format_message(person).as_bytes(), signature)
    }
}

pub type RingSigner = SignerUtil<SigningKey>;
pub type RingVerifier = VerifierUtil<VerifyingKey>;

fn format_message(person: &str) -> String {
    format!("Hello, {}!", person)
}
