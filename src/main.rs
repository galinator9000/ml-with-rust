
use candle_core::{Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ma look at me I'm in Rust, multiplying tensors!");

    let device = Device::Cpu;

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    println!("{a}");

    let b = Tensor::randn(0f32, 1., (3, 4), &device)?;
    println!("{b}");

    let c = a.matmul(&b)?;
    println!("{c}");

    Ok(())
}
