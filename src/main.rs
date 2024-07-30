fn main() {
    let dev = cudarc::driver::CudaDevice::new(0)?;

    // allocate buffers
    let inp = dev.htod_copy(vec![1.0f32; 100])?;
    let mut out = dev.alloc_zeros::<f32>(100)?;
    println!("Hello, world!");
}
