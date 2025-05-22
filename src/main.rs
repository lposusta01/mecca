use std::fs::OpenOptions;
use std::io::Write as _;

const SCRIPT: &str = include_str!("./mecca.txt");
const COL_BOUNDS: [u32; 2] = [5, 78];
const ROW_BOUNDS: [u32; 2] = [16, 38];

//antialiasing characters:
//░▒▓█

fn antialias(r: u8) -> char {
    if r >= 204 {
        return '█';
    } else if r >= 153 {
        return '▓';
    } else if r >= 102 {
        return '▒';
    } else if r >= 51 {
        return '░';
    } else {
        return ' ';
    }
}

fn main() {
    let wrapped_script = textwrap::wrap(SCRIPT, (COL_BOUNDS[1] - COL_BOUNDS[0]) as usize);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("out.txt")
        .unwrap();

    let decoded = image::open("./src/mecca.png").unwrap();
    let resized = decoded.resize_exact(200, 100, image::imageops::FilterType::Nearest);
    let buffer = resized.into_rgb8();

    let width = buffer.width();

    'draw: for (index, pixel) in buffer.pixels().enumerate() {
        let row = index as u32 / width;
        let col = index as u32 % width;

        let inside_text_bounds = (row >= ROW_BOUNDS[0] && row < ROW_BOUNDS[1])
            && (col >= COL_BOUNDS[0] && col < COL_BOUNDS[1]);
        'text_print: {
            if inside_text_bounds {
                let line_index = row - ROW_BOUNDS[0];
                let col_index = col - COL_BOUNDS[0];

                let line = wrapped_script.get(line_index as usize);
                let Some(line) = line else { break 'text_print };

                let character = line.chars().nth(col_index as usize);
                let Some(character) = character else { break 'text_print };

                write!(&mut file, "{character}").unwrap();
                continue 'draw;

            }
        }

        let [r, g, b] = pixel.0;
        
        write!(&mut file, "{}", antialias(r));

        if col == width - 1 {
            write!(&mut file, "\n").unwrap();
        }
    }
}