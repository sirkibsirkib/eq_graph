use crate::EqGraph;

#[test]
fn test() {
    use tinyrand::{Rand, Seeded, StdRand};
    let mut rng = StdRand::seed(0);
    let vert_domain_size = [1u16, 4, 16, 64, 256];
    let relations = [0u16, 1, 4, 16, 64, 256];
    for v in vert_domain_size {
        for r in relations {
            let mut eg = EqGraph::default();
            for _ in 0..r {
                eg.relate(rng.next_lim_u16(v), rng.next_lim_u16(v));
            }
            assert!(eg.decreasing());
            eg.flatten();
            assert!(eg.decreasing());
        }
    }
}
