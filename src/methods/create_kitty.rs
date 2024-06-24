use crate::{Kitty, KittyColor};

pub fn spawn_kitty(color: String) -> Kitty {
    let color = pick_color(&color);
    let my_kitty = Kitty::new(color);
    dbg!(&my_kitty); // This is how to see what your kitty looks like!
    assert!(my_kitty.is_hungry()); // This is how to prove your kitty is hungry!
    println!("your kitty's personality: {}", my_kitty.personality());
    my_kitty
}

pub fn pick_color(color: &str) -> KittyColor {
    match color {
        "Black" => KittyColor::Black,
        "Orange" => KittyColor::Orange,
        "Cream" => KittyColor::Cream,
        "Calico" => KittyColor::Calico,
        &_ => todo!(),
    }
}
