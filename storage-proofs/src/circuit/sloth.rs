use bellperson::{ConstraintSystem, SynthesisError};
use ff::Field;
use fil_sapling_crypto::circuit::num;
use paired::Engine;

use crate::circuit::constraint;

/// Circuit version of sloth decoding.
pub fn decode<E, CS>(
    mut cs: CS,
    key: &num::AllocatedNum<E>,
    ciphertext: Option<E::Fr>,
) -> Result<num::AllocatedNum<E>, SynthesisError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let plaintext = num::AllocatedNum::alloc(cs.namespace(|| "decoded"), || {
        Ok(ciphertext.ok_or_else(|| SynthesisError::AssignmentMissing)?)
    })?;

    decode_no_alloc(cs.namespace(|| "plaintext"), key, &plaintext)
}

pub fn decode_no_alloc<E, CS>(
    mut cs: CS,
    key: &num::AllocatedNum<E>,
    ciphertext: &num::AllocatedNum<E>,
) -> Result<num::AllocatedNum<E>, SynthesisError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    sub(cs.namespace(|| "decode-sub"), &ciphertext, key)
}

fn sub<E: Engine, CS: ConstraintSystem<E>>(
    mut cs: CS,
    a: &num::AllocatedNum<E>,
    b: &num::AllocatedNum<E>,
) -> Result<num::AllocatedNum<E>, SynthesisError> {
    let res = num::AllocatedNum::alloc(cs.namespace(|| "sub num"), || {
        let mut tmp = a
            .get_value()
            .ok_or_else(|| SynthesisError::AssignmentMissing)?;
        tmp.sub_assign(
            &b.get_value()
                .ok_or_else(|| SynthesisError::AssignmentMissing)?,
        );

        Ok(tmp)
    })?;

    // a - b = res
    constraint::difference(&mut cs, || "subtraction constraint", &a, &b, &res);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::test::TestConstraintSystem;
    use crate::crypto::sloth;
    use paired::bls12_381::{Bls12, Fr};
    use rand::{Rng, SeedableRng, XorShiftRng};

    #[test]
    fn sloth_snark_decode() {
        let rng = &mut XorShiftRng::from_seed([0x3dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        for _ in 0..10 {
            let key: Fr = rng.gen();
            let plaintext: Fr = rng.gen();
            let ciphertext = sloth::encode::<Bls12>(&key, &plaintext);

            // Vanilla
            let decrypted = sloth::decode::<Bls12>(&key, &ciphertext);

            assert_eq!(plaintext, decrypted, "vanilla failed");

            let mut cs = TestConstraintSystem::<Bls12>::new();

            let key_num = num::AllocatedNum::alloc(cs.namespace(|| "key"), || Ok(key)).unwrap();
            let out = decode(cs.namespace(|| "sloth"), &key_num, Some(ciphertext)).unwrap();

            assert!(cs.is_satisfied());
            assert_eq!(out.get_value().unwrap(), decrypted, "no interop");
        }
    }

    #[test]
    fn sloth_snark_decode_bad() {
        let rng = &mut XorShiftRng::from_seed([0x3dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        for _ in 0..10 {
            let key: Fr = rng.gen();
            let key_bad: Fr = rng.gen();
            let plaintext: Fr = rng.gen();

            let ciphertext = sloth::encode::<Bls12>(&key, &plaintext);

            let decrypted = sloth::decode::<Bls12>(&key, &ciphertext);
            let mut cs = TestConstraintSystem::<Bls12>::new();
            let key_bad_num =
                num::AllocatedNum::alloc(cs.namespace(|| "key bad"), || Ok(key_bad)).unwrap();

            let out = decode(cs.namespace(|| "sloth"), &key_bad_num, Some(ciphertext)).unwrap();

            assert!(cs.is_satisfied());
            assert_ne!(out.get_value().unwrap(), decrypted);
        }
    }

    #[test]
    fn sloth_snark_decode_different_iterations() {
        let rng = &mut XorShiftRng::from_seed([0x3dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        for _ in 0..10 {
            let mut key: Fr = rng.gen();
            let plaintext: Fr = rng.gen();

            let ciphertext = sloth::encode::<Bls12>(&key, &plaintext);
            let decrypted = sloth::decode::<Bls12>(&key, &ciphertext);

            {
                let mut cs = TestConstraintSystem::<Bls12>::new();
                key.add_assign(&Fr::one());
                let key_num = num::AllocatedNum::alloc(cs.namespace(|| "key"), || Ok(key)).unwrap();

                let out_other =
                    decode(cs.namespace(|| "sloth other"), &key_num, Some(ciphertext)).unwrap();

                assert!(cs.is_satisfied());
                assert_ne!(out_other.get_value().unwrap(), decrypted);
            }
        }
    }

    #[test]
    fn sub_constraint() {
        let rng = &mut XorShiftRng::from_seed([0x3dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

        for _ in 0..100 {
            let mut cs = TestConstraintSystem::<Bls12>::new();

            let a = num::AllocatedNum::alloc(cs.namespace(|| "a"), || Ok(rng.gen())).unwrap();
            let b = num::AllocatedNum::alloc(cs.namespace(|| "b"), || Ok(rng.gen())).unwrap();

            let res = sub(cs.namespace(|| "a-b"), &a, &b).expect("subtraction failed");

            let mut tmp = a.get_value().unwrap().clone();
            tmp.sub_assign(&b.get_value().unwrap());

            assert_eq!(res.get_value().unwrap(), tmp);
            assert!(cs.is_satisfied());
        }
    }
}
