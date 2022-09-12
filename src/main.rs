extern crate mpi;
use hypre_rs::{HYPRE_IJMatrix, HYPRE_IJMatrixCreate};
use mpi::traits::*;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let start = rank * 5;
    let end = start + 4;

    let mut v = std::mem::MaybeUninit::uninit();
    unsafe {
        let pt: *mut HYPRE_IJMatrix = v.as_mut_ptr();
        HYPRE_IJMatrixCreate(world.as_raw(), start, end, start, end, pt);
    }
}
