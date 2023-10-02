use {
    glassbench::*, solana_merkle_tree::MerkleTree as SolanaMerkleTree,
    solana_optimised_merkle::MerkleTree as OptimisedMerkleTree, solana_sdk::signature::Signature,
};

fn benchmark_merkle_tree(b: &mut Bench) {
    let mut leaves = vec![];
    let leaf_count: usize = 1000000;
    for _ in 0..leaf_count {
        leaves.push(Signature::new_unique().to_string().as_bytes().to_owned());
    }

    b.task(
        format!("solana-merkle-tree | {} leaves | Default", leaf_count),
        |task| {
            task.iter(|| {
                let solana_merkle = SolanaMerkleTree::new(leaves.as_slice());
                let _root = solana_merkle.get_root();
            });
        },
    );

    b.task(
        format!(
            "solana-merkle-tree | {} leaves | Hash Par Insert Once",
            leaf_count
        ),
        |task| {
            task.iter(|| {
                let solana_merkle = OptimisedMerkleTree::new(leaves.as_slice());
                let _root = solana_merkle.get_root();
            });
        },
    );
}

glassbench!("My Merkle Tree v/s Solana Labs Tree", benchmark_merkle_tree,);
