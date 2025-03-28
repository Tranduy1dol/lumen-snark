pub mod freivalds;
pub mod reed_solomon;

fn main() {
    reed_solomon::reed_solomon_simulation();
    freivalds::freivalds_simulations();
}
