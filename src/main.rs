use ark_ff::One;
use ark_relations::r1cs::{self, ConstraintSystem, LinearCombination, Variable};
use ark_test_curves::bls12_381::Fr;

// R1CS
fn main() -> anyhow::Result<()> {
    zeroknowledge()?;
    print!("OK");
    Ok(())
}

fn zeroknowledge() -> anyhow::Result<()> {
    // y = (x+z)^2+z+1
    // y = x^2 + z^2 + 2xz + z + 1
    // in SNARK language: I know x and z such that y=11
    // x=1 and z=2 => y=12

    let cs = ConstraintSystem::<Fr>::new_ref();

    // 6 variables in the system

    // private witness
    let x = cs.new_witness_variable(|| Ok(Fr::from(1)))?;
    let z = cs.new_witness_variable(|| Ok(Fr::from(2)))?;
    let t1 = cs.new_witness_variable(|| Ok(Fr::from(1) * Fr::from(1)))?;
    let t2 = cs.new_witness_variable(|| Ok(Fr::from(2) * Fr::from(2)))?;
    let t3 = cs.new_witness_variable(|| Ok(Fr::from(2) * Fr::from(1) * Fr::from(2)))?;

    // public input
    let y = cs.new_input_variable(|| {
        Ok(Fr::from(1) * Fr::from(1)
            + Fr::from(2) * Fr::from(1) * Fr::from(2)
            + Fr::from(2) * Fr::from(2)
            + Fr::from(2)
            + Fr::from(1))
    })?;

    let zero: LinearCombination<Fr> = r1cs::LinearCombination::zero();
    let one = Variable::One;

    // 4 constraints
    //                    w.a     * w.b     = w.c
    cs.enforce_constraint(x.into(), x.into(), t1.into())?; // T1 = x * x
    cs.enforce_constraint(z.into(), z.into(), t2.into())?; // T2 = z * z
    cs.enforce_constraint(zero.clone() + x + x, z.into(), t3.into())?; // T3 = 2 * x * z
    cs.enforce_constraint(zero + t1 + t2 + t3 + z + one, one.into(), y.into())?; // y = T1 + T2 + T3 + z + 1

    cs.finalize();
    assert!(cs.is_satisfied().unwrap(), "CS is not satisfied");

    let matrices = cs.to_matrices().unwrap();

    // println!("{:#?}", matrices.a);
    // println!();
    // println!("{:#?}", matrices.b);
    // println!();
    // println!("{:#?}", matrices.c);

    Ok(())
}
