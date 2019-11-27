#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate ff;
extern crate paired;
extern crate bellperson;

extern crate filecoin_proofs;
extern crate storage_proofs;


use bellperson::domain::EvaluationDomain;
use bellperson::domain::Group;
use bellperson::domain::Scalar;
use bellperson::domain::gpu_fft_supported;
use bellperson::multicore::Worker;

use ff::Field;
use ff::PrimeField;
use ff::ScalarEngine;
use paired::Engine;
use paired::bls12_381::{Bls12, Fr};

use storage_proofs::sector::SectorId;
use filecoin_proofs::verify_seal;
use filecoin_proofs::PoRepConfig;
use filecoin_proofs::SealPreCommitOutput;
use filecoin_proofs::ProverId;
use filecoin_proofs::Ticket;
use filecoin_proofs::parameters::setup_params;
use filecoin_proofs::types::{
    Commitment, PaddedBytesAmount, PieceInfo, PoRepProofPartitions,
    SealCommitOutput,
};

// FN seal_commit
//     let compound_setup_params = compound_proof::SetupParams {
//         vanilla_params: setup_params(
//             PaddedBytesAmount::from(porep_config),
//             usize::from(PoRepProofPartitions::from(porep_config)),
//         ),
//         partitions: Some(usize::from(PoRepProofPartitions::from(porep_config))),
//     };
// 
//     let compound_public_params = StackedCompound::setup(&compound_setup_params)?;
// 
//     let proof = StackedCompound::prove(
//         &compound_public_params,
//         &public_inputs,
//         &private_inputs,
//         &groth_params,
//     )?;
// 
//     // Delete cached MTs that are no longer needed.
//     TemporaryAux::<DefaultTreeHasher, DefaultPieceHasher>::delete(t_aux)?;
// 
//     let mut buf = Vec::with_capacity(
//         SINGLE_PARTITION_PROOF_LEN * usize::from(PoRepProofPartitions::from(porep_config)),
//     );
// 
//     proof.write(&mut buf)?;
// 

// FN verify_seal
    // let compound_setup_params = compound_proof::SetupParams {
    //     vanilla_params: setup_params(
    //         PaddedBytesAmount::from(porep_config),
    //         usize::from(PoRepProofPartitions::from(porep_config)),
    //     ),
    //     partitions: Some(usize::from(PoRepProofPartitions::from(porep_config))),
    // };

    // let compound_public_params: compound_proof::PublicParams<
    //     '_,
    //     StackedDrg<'_, DefaultTreeHasher, DefaultPieceHasher>,
    // > = StackedCompound::setup(&compound_setup_params)?;

    // let public_inputs = stacked::PublicInputs::<
    //     <DefaultTreeHasher as Hasher>::Domain,
    //     <DefaultPieceHasher as Hasher>::Domain,
    // > {
    //     replica_id,
    //     tau: Some(Tau { comm_r, comm_d }),
    //     seed,
    //     k: None,
    // };

    // let verifying_key = get_stacked_verifying_key(porep_config)?;

// use storage_proofs::stacked::SetupParams;
// use storage_proofs::compound_proof::SetupParams;
// use bellperson::groth16::Parameters;

fn new<E: Engine, G: Group<E>>() -> Vec<G> {
    vec![G::group_zero(); 10]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "full");
    
    pretty_env_logger::init_timed();
    
    // Vec<Scalar<Fr>>
    let mut data: Vec<_> = new::<Bls12, Scalar<Bls12>>();
    println!("{:?}", data);
    let n = data.len();

    let mut a: EvaluationDomain<Bls12, Scalar<Bls12>> = EvaluationDomain::from_coeffs(data)?;

    let mut log_d = 0u32;
    while (1 << log_d) < n {
        log_d += 1;
    }

    let mut fft_kern = match gpu_fft_supported::<Bls12>(log_d) {
        Ok(fft_kern) => Some(fft_kern),
        Err(e) => {
            error!("{:?}", e);
            None
        }
    };

    {
        let coeffs: &[Scalar<Bls12>] = a.as_ref();
        println!("{:?}", coeffs);
    }
    

    let worker = Worker::new();

    a.ifft(&worker, &mut fft_kern);
    {
        let coeffs: &[Scalar<Bls12>] = a.as_ref();
        println!("{:?}", coeffs);
    }

    Ok(())
}

