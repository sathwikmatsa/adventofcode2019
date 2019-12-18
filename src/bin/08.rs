use std::fs;

#[derive(Default)]
struct Layer {
    data: [[u8; 25]; 6],
    num_zeroes: u8,
    num_ones: u8,
    num_twos: u8,
}

impl Layer {
    fn transparent() -> Self {
        Self {
            data: [[2; 25]; 6],
            num_zeroes: 0,
            num_ones: 0,
            num_twos: 150,
        }
    }
}

#[derive(Default)]
struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn load_data(&mut self, data: String) {
        let mut digits = data
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .peekable();
        loop {
            let mut layer = Layer::default();
            let mut n_zeroes = 0;
            let mut n_ones = 0;
            let mut n_twos = 0;

            for i in 0..6 {
                for j in 0..25 {
                    let digit = digits.next();
                    match digit {
                        Some(0) => n_zeroes += 1,
                        Some(1) => n_ones += 1,
                        Some(2) => n_twos += 1,
                        _ => (),
                    }
                    layer.data[i][j] = digit.unwrap();
                }
            }

            layer.num_zeroes = n_zeroes;
            layer.num_ones = n_ones;
            layer.num_twos = n_twos;

            self.layers.push(layer);
            if digits.peek().is_none() {
                break;
            }
        }
    }
    fn render(&self) {
        let mut image = Layer::transparent();
        for layer in self.layers.iter() {
            for i in 0..6 {
                for j in 0..25 {
                    if image.data[i][j] == 2 {
                        image.data[i][j] = layer.data[i][j];
                    }
                }
            }
        }

        for i in 0..6 {
            for j in 0..25 {
                match image.data[i][j] {
                    1 => print!("$$"),
                    _ => print!("  "),
                }
            }
            println!("");
        }
    }
}

fn main() {
    // read input
    let input: String = fs::read_to_string("input/08.txt")
        .expect("Failed to read input from input/08.txt")
        .trim()
        .to_string();

    let mut image = Image::default();
    image.load_data(input);

    let layer = image.layers.iter().min_by_key(|x| x.num_zeroes).unwrap();
    println!("{}", layer.num_ones as u32 * layer.num_twos as u32);

    image.render();
}
