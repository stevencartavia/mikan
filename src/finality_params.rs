use crate::{error::BlockError, vote::Vote};
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
#[derive(Debug)]
pub struct FinalityParams {
    pub height: usize,
    /// list of validators that voted on this block.
    pub votes: Vec<Vote>,
}
#[allow(dead_code)]
impl FinalityParams {
    pub fn new(height: usize, votes: Vec<Vote>) -> Self {
        Self { height, votes }
    }

    /// Calculates the Merkle Tree Root of the `votes.signature` .
    /// the leaves are SHA256 hashes of the signatures in `FinalityParams.votes`.
    pub fn tree_root(&self) -> eyre::Result<Vec<u8>> {
        let leaves: Vec<[u8; 32]> = self
            .votes
            .iter()
            .map(|x| Sha256::hash(&x.signature))
            .collect();

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

        Ok(merkle_tree
            .root()
            .map(Vec::from)
            .ok_or(BlockError::MerkleTreeError)?)
    }

    pub fn get_tree(&self) -> eyre::Result<MerkleTree<Sha256>> {
        let leaves: Vec<[u8; 32]> = self
            .votes
            .iter()
            .map(|x| Sha256::hash(&x.signature))
            .collect();

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

        Ok(merkle_tree)
    }

    pub fn basic_validation(&self) -> eyre::Result<()> {
        if self.height == 0 {
            return Err(BlockError::InvalidBlockNumber(self.height).into());
        }

        for _vote in &self.votes {
            // TODO:Validate each signature belongs to the respective validator address i.e. ecdsa signature verification

            todo!()
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{block::mock_make_validator, vote::Vote};

    use super::*;

    #[test]
    fn signature_merkle_tree_verification() {
        let vote_1 = Vote::new(mock_make_validator(), Vec::from("1234"), 2);
        let vote_2 = Vote::new(mock_make_validator(), Vec::from("5678"), 2);
        let vote_3 = Vote::new(mock_make_validator(), Vec::from("9012"), 2);
        let vote_4 = Vote::new(mock_make_validator(), Vec::from("3456"), 2);

        let finality_param =
            FinalityParams::new(2, [vote_1, vote_2.clone(), vote_3, vote_4].to_vec());
        let tree = finality_param.get_tree().unwrap();

        let index = vec![1];
        let leaf_value_to_prove = vec![Sha256::hash(&vote_2.signature)];

        let merkle_proof = tree.proof(&index);
        let merkle_root = tree.root().unwrap();

        assert!(merkle_proof.verify(merkle_root, &index, &leaf_value_to_prove, 4));
    }
}
