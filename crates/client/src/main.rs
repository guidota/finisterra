use client::Finisterra;
use roma::block_on;
use roma::run;

mod settings;

fn main() {
    let base_path = "./assets/99z/graphics/".to_string();
    block_on(run(base_path, Finisterra::default()));
}
