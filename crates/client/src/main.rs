use client::Finisterra;
use roma::block_on;
use roma::run;

mod settings;

fn main() {
    block_on(run(Finisterra::default()));
}
