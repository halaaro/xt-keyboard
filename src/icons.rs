pub struct Icon {
    pub name: &'static str,
    pub author: &'static str,
    pub pixels: &'static [&'static str],
}
impl Icon {
    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }
    pub fn height(&self) -> usize {
        self.pixels.len()
    }
}

pub const CROSS: Icon = Icon {
    name: "cross",
    author: "emma",
    pixels: &[
        "  X  ", "  X  ", "  X  ", "XXXXX", "  X  ", "  X  ", "  X  ", "  X  ",
    ],
};

pub const SM_HEART: Icon = Icon {
    name: "small heart",
    author: "aaron",
    pixels: &[
        " XXX   XXX ",
        "XXXXX XXXXX",
        " XXXXXXXXX ",
        "   XXXXX   ",
        "     X     ",
    ],
};

pub const BREAD: Icon = Icon {
    name: "bread",
    author: "anna",
    pixels: &[
        "   XXXXXX ",
        "  X      X",
        "   xxxxx X",
        " X xxxxx X",
        "   xxxxx X",
        "X  xxxxx X",
        "         X",
        " XXXXXXXX ",
    ],
};

pub const _2HEART: Icon = Icon {
    name: "2heart",
    author: "emma",
    pixels: &[
        " XX XX ", "X  X  X", "X X X X", "X XXX X", " X X X ", "  X X  ", "   X   ",
    ],
};

pub const CHECKER: Icon = Icon {
    name: "checker",
    author: "emma",
    pixels: &[
        "X X X", " X X ", "X X X", " X X ", "X X X", " X X ", "X X X", " X X ", "X X X",
    ],
};

pub const CANDY: Icon = Icon {
    name: "candy",
    author: "emma",
    pixels: &[
        "X      X", "XX XX XX", "XXXXXXXX", "XXXXXXXX", "XX XX XX", "X      X",
    ],
};

pub const LOLLIPOP: Icon = Icon {
    name: "lollipop",
    author: "anna",
    pixels: &[
        " XXX ", "XXXXX", "XXXXX", "XXXXX", " XXX ", "  X  ", "  X  ", "  X  ",
    ],
};

pub const DAISY: Icon = Icon {
    name: "daisy",
    author: "anna",
    pixels: &[" XXXX ", "X    X", "X xx X", "X    X", " XXXX "],
};

pub const ICONS: &[&Icon] = &[
    &BREAD, &CANDY, &CHECKER, &CROSS, &DAISY, &LOLLIPOP, &SM_HEART, &_2HEART,
];
