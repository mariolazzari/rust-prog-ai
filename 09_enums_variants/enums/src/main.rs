enum DiskType {
    SSD,
    HDD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    let disk_type = DiskType::SSD;
    match disk_type {
        DiskType::HDD => println!("Hard disk"),
        DiskType::SSD => println!("Solid state"),
    };

    let disk_size = DiskSize::GB(128);
    println!("Disk size {:?}", disk_size);
}
