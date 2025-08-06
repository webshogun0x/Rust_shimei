use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(c: &str, seperator: char) -> Option<(T, T)> {
    match c.find(seperator) {
        None => None,
        Some(index) => match (T::from_str(&c[..index]), T::from_str(&c[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn parse_complex(c: &str) -> Option<Complex<f64>> {
    parse_pair::<f64>(c, ',');
    match parse_pair(c, ',') {
        Some((re, im))=> Some(Complex { re, im }),
        None => None,
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
)  -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

fn write_file(filename:&str, pixels: & [u8], bounds: &(usize, usize)) -> Result<(), std::io::Error> {
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(e) => return Err(e) 
    };

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<usize>("y* 20", ','), None);
    assert_eq!(parse_pair::<f64>("10.6 x 25.5", 'x'), Some((10.6, 25.5)));
    assert_eq!(parse_pair::<i32>("56 - 78", '-'), Some((56, 78)));

    assert_eq!(parse_complex("10.6, 25.5"), Some(Complex { re: 10.6, im: 25.5 }));
    assert_eq!(parse_complex("525.5"), None);
}

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("USAGE: {} FILE PIXELS UPPERLEFT LOWRRIGHT", args[0]);
        eprintln!("Example: {} Mendel.png 1080x720 -1.20, 0.32 -1,0.2", args[0]);
        std::process::exit(1)
    }

    let bounds:(usize,usize) = parse_pair(&args[2], 'x').expect("error passing image dimension");
    let upper_left = parse_complex(&args[3]).expect("erro passing upper left point");
    let lower_right = parse_complex(&args[4]).expect("error passing lower right point");
    let mut pixel  = vec![0; bounds.0 * bounds.1];

    render(&mut pixel, bounds, upper_left, lower_right);

    write_file(&args[1], &pixel, &bounds).expect("error writing PNG file")
}
