pub fn main() {
    let cor = Color::CymkColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "azul",
            Color::RgbColor(0, 0, 0)
            | Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "preta",
            Color::RgbColor(_, _, _) => "RGB desconhecido",
            Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CYMK desconhecido",
        }
    );
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}
