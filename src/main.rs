mod image_processor;
use image_processor::{ImageData, N_SLICES};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_content = image_processor::read_image();
    let rgb_image = image_processor::convert_to_rgb8(&img_content);
    // Check the dimensions of the output pic
    // println!("Dimensions of the scaled pic are: Width: {}, Height: {}", rgb_image.width(), rgb_image.height());
    let image_data = ImageData::new();
    let colors_vector = image_processor::prepare_color_vector(&rgb_image, &image_data);
    println!("The vector of colors for an angle step: {} is {:#?}", N_SLICES, colors_vector);
    // Ok(())
    image_processor::display_image(rgb_image.clone())
}
