use specs::{Component, VecStorage};

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct Material {
    pub opaque: bool,
    pub visible: bool,
    pub solid: bool,
}

pub fn smoke() -> Material {
    Material {
        opaque: true,
        visible: true,
        solid: false,
    }
}

pub fn stone() -> Material {
    Material {
        opaque: true,
        visible: true,
        solid: true,
    }
}

pub fn flesh() -> Material {
    Material {
        opaque: true,
        visible: true,
        solid: true,
    }
}
