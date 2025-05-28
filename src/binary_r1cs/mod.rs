use nimue::{Arthur, IOPattern, Merlin, ProofResult};

use lattirust_arithmetic::challenge_set::labrador_challenge_set::LabradorChallengeSet;
use lattirust_arithmetic::challenge_set::weighted_ternary::WeightedTernaryChallengeSet;
use lattirust_arithmetic::nimue::iopattern::{SerIOPattern, SqueezeFromRandomBytes};
use lattirust_arithmetic::ring::{PolyRing, Z2};
use lattirust_arithmetic::ring::representatives::WithSignedRepresentative;
use lattirust_arithmetic::traits::FromRandomBytes;
use relations::principal_relation::PrincipalRelation;
use relations::r1cs::R1CS;
use relations::reduction::Reduction;

use crate::binary_r1cs::prover::prove_reduction_binaryr1cs_labradorpr;
use crate::binary_r1cs::util::BinaryR1CSCRS;
use crate::binary_r1cs::verifier::verify_reduction_binaryr1cs_labradorpr;
use crate::{nvtx_timed, nvtx_timed_pop};

pub mod prover;
#[cfg(test)]
pub mod test;
pub mod util;
pub mod verifier;

pub type BinaryR1CS = R1CS<Z2>;

pub struct ReductionBinaryR1CSPrincipalRelation<R: PolyRing> {
    _marker: std::marker::PhantomData<R>,
}

impl<R: PolyRing> Reduction<BinaryR1CS, PrincipalRelation<R>, BinaryR1CSCRS<R>>
    for ReductionBinaryR1CSPrincipalRelation<R>
where
    LabradorChallengeSet<R>: FromRandomBytes<R>,
    WeightedTernaryChallengeSet<R>: FromRandomBytes<R>,
    <R as PolyRing>::BaseRing: WithSignedRepresentative,
{
    fn iopattern(
        pp: &BinaryR1CSCRS<R>,
        _index_in_: &Self::IndexIn,
        _instance_in_: &Self::InstanceIn,
    ) -> IOPattern {
        nvtx_timed!("iopattern");
        let k = pp.num_constraints;
        let secparam = pp.security_parameter;
        let pattern = IOPattern::new("reduction_binaryr1cs_principalrelation")
            .absorb_vector::<R>(pp.A.nrows(), "prover message 1 (t)")
            .squeeze_binary_matrix(secparam, k, "verifier message 1 (alpha)")
            .squeeze_binary_matrix(secparam, k, "verifier message 1 (beta)")
            .squeeze_binary_matrix(secparam, k, "verifier message 1 (gamma)")
            .absorb_vector_canonical::<R::BaseRing>(secparam, "prover message 2 (g)");
        nvtx_timed_pop!();
        pattern
    }

    fn prove(
        pp: &BinaryR1CSCRS<R>,
        index: &Self::IndexIn,
        instance: &Self::InstanceIn,
        witness: &Self::WitnessIn,
        merlin: &mut Merlin,
    ) -> ProofResult<(Self::IndexOut, Self::InstanceOut, Self::WitnessOut)> {
        nvtx_timed!("prove");
        let result = Ok(prove_reduction_binaryr1cs_labradorpr(
            pp, merlin, index, instance, witness,
        ));
        nvtx_timed_pop!();
        result
    }

    fn verify(
        pp: &BinaryR1CSCRS<R>,
        index_in: &Self::IndexIn,
        instance_in: &Self::InstanceIn,
        arthur: &mut Arthur,
    ) -> ProofResult<(Self::IndexOut, Self::InstanceOut)>
    {
        //stop_tracking_backtrace();
        nvtx_timed!("verify");
        let result = verify_reduction_binaryr1cs_labradorpr(arthur, pp, index_in, instance_in);
        nvtx_timed_pop!();
        result
    }
}
