#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
  
    let n = sp1_zkvm::io::read::<u32>();

    sp1_zkvm::io::commit(&n);

    let mut a = n*10;

    sp1_zkvm::io::commit(&a);

}