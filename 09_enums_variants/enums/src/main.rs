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

#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Tuscany,
}

struct Wine {
    name: String,
    region: WineRegions,
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Tuscany => println!("Wine region {:?} is supportted", w),
        _ => println!("Wine region {:?} not supported", w),
    };
}

fn main() {
    let disk_type = DiskType::SSD;
    match disk_type {
        DiskType::HDD => println!("Hard disk"),
        DiskType::SSD => println!("Solid state"),
    };

    let disk_size = DiskSize::GB(128);
    println!("Disk size {:?}", disk_size);

    let wine = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };
    println!("Wine {} form {:?}", wine.name, wine.region);
    supported_regions(WineRegions::Bordeaux);
    supported_regions(wine.region);
}
