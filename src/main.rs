use std::{fs::File, io::BufWriter};

const WIDTH: i32 = 780;
const HEIGHT: i32 = 1280;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loading from a file

    let handle = rsvg::Loader::new().read_path("input.svg").unwrap();

    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, WIDTH, HEIGHT).unwrap();
    let cr = cairo::Context::new(&surface).expect("Failed to create a cairo context");

    let renderer = rsvg::CairoRenderer::new(&handle);
    renderer
        .render_document(
            &cr,
            &cairo::Rectangle::new(0.0, 0.0, f64::from(WIDTH), f64::from(HEIGHT)),
        )
        .unwrap();

    let output_file = File::create("output.png")?; // Create a file
    let mut writer = BufWriter::new(output_file);
    surface.write_to_png(&mut writer)?;
    Ok(())
}
