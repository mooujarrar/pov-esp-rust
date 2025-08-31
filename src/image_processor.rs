use image::{ImageReader, DynamicImage, RgbImage};
use image::imageops::FilterType;
use show_image::{event, Image, create_window};
use lerp::Lerp;
use std::f32::consts;

// This is the length of the rotating strip fixed into the wheels radius
const LED_STRIP_LENGTH: u8 = 14; // This are the image dimensions to be displayed on the wheel
const IMAGE_WIDTH: u8 = LED_STRIP_LENGTH*2;
const IMAGE_HEIGHT: u8 = IMAGE_WIDTH;
pub const N_SLICES: u8 = 180;

pub struct ImageData {
    center_x: f32,
    center_y: f32,
    radius_vector: Vec<f32>,
}
impl ImageData {
    pub fn new() -> ImageData {
        // Create a virtual center in case the strip dimensions are Even number (there is no real
        // median or middle point)
        let (middle_x, middle_y, r_max) = (
            (f32::from(IMAGE_WIDTH) - 1f32) / 2.0,
            (f32::from(IMAGE_HEIGHT) - 1f32) / 2.0,
            (f32::from(LED_STRIP_LENGTH) * 2.0 - 1f32) / 2.0
        );
        // println!("Create new ImageData, center of the pucture: x={middle_x}, y={middle_y}");
        let mut r: Vec<f32> = vec![0.0;LED_STRIP_LENGTH as usize];
        for i in 0..LED_STRIP_LENGTH {
            r[i as usize] = f32::from(i) * r_max / (f32::from(LED_STRIP_LENGTH)-1f32);
        }
        // println!("Radius values are: {:?}", r);
        ImageData {
            center_x: middle_x,
            center_y: middle_y,
            radius_vector: r,
        }
    }
}

/*
 * This reads the image into a DynamicImage.
 * It also resize it to fit into the LED_STRIP_LENGTH
 */
pub fn read_image() -> DynamicImage {
    let img = ImageReader::open("pinguin.png");
    let decoded_content = match img {
        Ok(content) => match content.decode() {
            Ok(decoded_content) => decoded_content,
            Err(error) => panic!("Image could not be decoded, reason {error}"),
        },
        Err(error) => panic!("Problem opening file, reason {error}"),
    };
    // If everything is fine and the image is read, we try to resize it
    decoded_content.resize_exact(u32::from(IMAGE_WIDTH), u32::from(IMAGE_HEIGHT), FilterType::Triangle)
}

// Take a DynamicImage and convert it to RgbImage (Clone and moving ownership)
pub fn convert_to_rgb8(image: &DynamicImage) -> RgbImage {
    image.to_rgb8()
}

// Function to display the image for test purposes
pub fn display_image(image: RgbImage) -> Result<(), Box<dyn std::error::Error>> {
    let image = Image::from(image);
    let window = create_window("image", Default::default()).expect("Problem creating window");
    window.set_image("image-001", image).expect("set_image has failed");
    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exists.
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
    Ok(())
}

/*
 * This function computes for a given angle of the strip the full RGB strip values to show
 */
pub fn polar_to_cartesian(image: &RgbImage, data: &ImageData, theta: f32) -> Vec<Vec<f32>> {
    // Use both the image and the processed center data etc to start getting pixels
    let radius_vector = &data.radius_vector;
    let vector_iter = radius_vector.iter();
    let mut index: u8 = 0;
    let mut led_colors: Vec<Vec<f32>> = vec![vec![0.0, 0.0, 0.0];LED_STRIP_LENGTH as usize];
    for val in vector_iter {
        let xf = data.center_x + val * f32::cos(theta);
        let yf = data.center_y + val * f32::sin(theta);
        // println!("Iteration {}, xf {}, yf {}", index, xf, yf);
        
        // Bilinear interpolation
        let mut x_0 = f32::floor(xf);
        let mut y_0 = f32::floor(yf);

        let dx = xf - x_0;
        let dy = yf - y_0;

        x_0 = 0.0f32.max(x_0.min(f32::from(IMAGE_WIDTH-2)));
        y_0 = 0.0f32.max(y_0.min(f32::from(IMAGE_HEIGHT-2)));

        // Get the surrounding centers pixel values to lerp them after
        let c_00 = image.get_pixel(x_0 as u32, y_0 as u32); // Default Pixel = Rgb<u8> and Rgb
                                                            // struct is equivalent to: [u8;3]
        let c_10 = image.get_pixel(x_0 as u32 + 1, y_0 as u32);
        let c_01 = image.get_pixel(x_0 as u32, y_0 as u32 + 1);
        let c_11 = image.get_pixel(x_0 as u32 + 1, y_0 as u32 + 1);
        // println!("Pixel: Red: {}, Green: {}, Blue: {}", c_00[0], c_00[1], c_00[2]);

        // Now user lerp to blend the surrounding and come up with a better colour
        let vec_c_00 = Vec::from(c_00.0);
        let vec_c_10 = Vec::from(c_10.0);
        let vec_c_01 = Vec::from(c_01.0);
        let vec_c_11 = Vec::from(c_11.0);

        let c0: Vec<f32> = vec_c_00.iter().zip(vec_c_10.iter()).map(|(&p_00, &p_10)| f32::from(p_00).lerp(f32::from(p_10), dx)).collect();
        let c1: Vec<f32> = vec_c_01.iter().zip(vec_c_11.iter()).map(|(&p_01, &p_11)| f32::from(p_01).lerp(f32::from(p_11), dx)).collect();
        let c: Vec<f32> = c0.iter().zip(c1.iter()).map(|(&p_0, &p_1)| f32::from(p_0).lerp(f32::from(p_1), dy)).collect();

        // println!("Vector o be added now: {:?}", c);
        led_colors[index as usize] = c;
        index += 1;
    }
    // println!(" For angle theta: {}, the led strip should display {:#?}", theta, led_colors);
    led_colors
}

pub fn prepare_color_vector(image: &RgbImage, data: &ImageData) -> Vec<Vec<Vec<f32>>> {
    let mut result_vector: Vec<Vec<Vec<f32>>> = Vec::new();
    let two_pi = 2.0 * consts::PI;
    let angle_step: f32 = two_pi / f32::from(N_SLICES);
    let mut current_theta: f32 = 0.0;
    loop {
        let colors = polar_to_cartesian(&image, &data, current_theta);
        result_vector.push(colors);
        current_theta = current_theta + angle_step;
        if current_theta > two_pi {
            break;
        }
    }
    result_vector
}
