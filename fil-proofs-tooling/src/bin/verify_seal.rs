#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate filecoin_proofs;
extern crate storage_proofs;


use filecoin_proofs::verify_seal;
use filecoin_proofs::PoRepConfig;
use filecoin_proofs::SealPreCommitOutput;
use filecoin_proofs::ProverId;
use filecoin_proofs::Ticket;


// verify_seal args: [0, 4, 0, 0, 0, 0, 0, 0, 1, 239, 51, 41, 62, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 188, 173, 224, 31, 107, 17, 69, 8, 14, 138, 188, 4, 103, 127, 136, 172, 141, 24
// , 231, 172, 65, 3, 141, 114, 184, 181, 110, 234, 205, 235, 110, 221, 231, 195, 145, 197, 238, 59, 178, 40, 131, 234, 207, 101, 127, 35, 240, 135, 187, 195, 237, 225, 41, 50, 171, 31
// , 227, 210, 135, 189, 196, 42, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// [160, 4, 82, 102, 74, 50, 19, 106, 112, 3, 49, 161, 54, 111, 194, 73, 211, 1, 198, 76, 111, 178, 197, 125, 215, 26, 117, 29, 21, 97, 107, 17, 211, 245, 151, 187, 14, 203, 6, 240, 60
// , 107, 244, 115, 74, 225, 179, 46, 134, 132, 115, 242, 240, 124, 90, 228, 132, 248, 38, 214, 144, 154, 131, 97, 225, 30, 156, 219, 144, 60, 239, 161, 117, 51, 45, 68, 227, 167, 65,
// 123, 26, 151, 18, 218, 168, 222, 25, 11, 142, 126, 190, 56, 174, 42, 131, 61, 4, 8, 227, 147, 86, 163, 88, 52, 177, 112, 211, 241, 188, 30, 163, 185, 223, 153, 207, 238, 40, 101, 22
// 0, 55, 199, 0, 69, 92, 190, 236, 73, 76, 146, 62, 108, 170, 152, 25, 177, 224, 80, 249, 140, 61, 225, 77, 198, 232, 131, 137, 162, 16, 163, 19, 253, 200, 204, 189, 124, 67, 150, 149
// , 200, 236, 242, 114, 187, 168, 16, 148, 177, 30, 128, 218, 209, 234, 202, 72, 244, 159, 68, 195, 153, 169, 153, 132, 75, 55, 67, 97, 214, 84, 12, 22, 153, 37]

const CPU_PARAMS_HDR: [u8; 184] = [
    0, 4, 0, 0, 0, 0, 0, 0, 1, 239, 51, 41, 62, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    172, 188, 173, 224, 31, 107, 17, 69, 8, 14, 138, 188, 4, 103, 127, 136, 172, 
    141, 24, 231, 172, 65, 3, 141, 114, 184, 181, 110, 234, 205, 235, 110, 221, 231, 
    195, 145, 197, 238, 59, 178, 40, 131, 234, 207, 101, 127, 35, 240, 135, 187, 195, 
    237, 225, 41, 50, 171, 31, 227, 210, 135, 189, 196, 42, 3, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0
];

const CPU_PARAMS_PAYLOAD: [u8; 192] = [
    160, 4, 82, 102, 74, 50, 19, 106, 112, 3, 49, 161, 54, 111, 194, 73, 211, 1, 198, 
    76, 111, 178, 197, 125, 215, 26, 117, 29, 21, 97, 107, 17, 211, 245, 151, 187, 14, 
    203, 6, 240, 60, 107, 244, 115, 74, 225, 179, 46, 134, 132, 115, 242, 240, 124, 90, 
    228, 132, 248, 38, 214, 144, 154, 131, 97, 225, 30, 156, 219, 144, 60, 239, 161, 117, 
    51, 45, 68, 227, 167, 65, 123, 26, 151, 18, 218, 168, 222, 25, 11, 142, 126, 190, 56, 
    174, 42, 131, 61, 4, 8, 227, 147, 86, 163, 88, 52, 177, 112, 211, 241, 188, 30, 163, 
    185, 223, 153, 207, 238, 40, 101, 220, 55, 199, 0, 69, 92, 190, 236, 73, 76, 146, 62, 
    108, 170, 152, 25, 177, 224, 80, 249, 140, 61, 225, 77, 198, 232, 131, 137, 162, 16, 
    163, 19, 253, 200, 204, 189, 124, 67, 150, 149, 200, 236, 242, 114, 187, 168, 16, 148, 
    177, 30, 128, 218, 209, 234, 202, 72, 244, 159, 68, 195, 153, 169, 153, 132, 75, 55, 
    67, 97, 214, 84, 12, 22, 153, 37
];



use storage_proofs::sector::SectorId;


// pub type SectorSize          : u64
// pub type PoRepProofPartitions: u8
#[derive(Debug)]
pub struct SealParamsHeader {
    porep_config: PoRepConfig,       // (SectorSize, PoRepProofPartitions)
    pre_commit: SealPreCommitOutput, // { a: [u8; 32], b: [u8; 32] }
    prover_id: ProverId, // [u8; 32]
    sector_id: SectorId, // u64
    ticket: Ticket,      // [u8; 32]
    seed: Ticket,        // [u8; 32]
}

#[derive(Debug)]
pub struct SealParams {
    header: SealParamsHeader,
    buf: Vec<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    
    pretty_env_logger::init_timed();

    unsafe {
        let len = std::mem::size_of::<SealParamsHeader>();
        assert_eq!(len, CPU_PARAMS_HDR.len());

        let ptr = CPU_PARAMS_HDR.as_ptr() as *const SealParamsHeader;
        let hdr: &SealParamsHeader = &*ptr;

        let porep_config = hdr.porep_config;
        let comm_r = hdr.pre_commit.comm_r;
        let comm_d = hdr.pre_commit.comm_d;
        let prover_id = hdr.prover_id;
        let sector_id = hdr.sector_id;
        let ticket = hdr.ticket;
        let seed = hdr.seed;

        let buf = CPU_PARAMS_PAYLOAD.to_vec();

        // std::mem::transmute::<*const u8, >()
        // Verification is cheap when parameters are cached,
        // and it is never correct to return a proof which does not verify.
        assert!(
            verify_seal(
                porep_config,
                comm_r,
                comm_d,
                prover_id,
                sector_id,
                ticket,
                seed,
                &buf,
            )
            .expect("post-seal verification sanity check failed"),
            "invalid seal generated, bad things have happened"
        );

        info!("seal_commit:end");

        // Ok(SealCommitOutput { proof: buf })
    }

    Ok(())
}