use wgpu_playground::run;
use pollster;

fn main() {
    pollster::block_on(run());
}
