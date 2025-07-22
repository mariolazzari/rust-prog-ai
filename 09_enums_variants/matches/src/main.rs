enum WineGrapes {
    Wine1,
    Wine2,
    Wine3,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::Wine1 => println!("Wine 1"),
        // WineGrapes::Wine2 => println!("Wine 2"),
        // WineGrapes::Wine3 => println!("Wine 3"),
        _ => print!("Wine"),
    }
}

fn main() {
    taste_wine(WineGrapes::Wine1);
    taste_wine(WineGrapes::Wine2);
    taste_wine(WineGrapes::Wine3);
}
