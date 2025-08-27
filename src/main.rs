use image::ImageReader;
use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use show_image::{event, Image, create_window};


// This is the length of the rotating strip fixed into the wheels radius
const LED_STRIP_LENGTH: u8 = 14; // This are the image dimensions to be displayed on the wheel
const IMAGE_WIDTH: u8 = LED_STRIP_LENGTH*2;
const IMAGE_HEIGHT: u8 = IMAGE_WIDTH;

/*
 * This reads the image into a DynamicImage.
 * It also resize it to fit into the LED_STRIP_LENGTH
 */
fn read_image() -> DynamicImage {
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
fn convert_to_rgb8(image: &DynamicImage) -> RgbImage {
    image.to_rgb8()
}

// Function to display the image for test purposes
fn display_image(image: RgbImage) -> Result<(), Box<dyn std::error::Error>> {
    let image = Image::from(image);
    let window = create_window("image", Default::default()).expect("Problem creating window");
    window.set_image("image-001", image).expect("set_image has failed");
    // Print keyboard events until Escape is pressed, then exit.
    // If the user closes the window, the channel is closed and the loop also exits.
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
fn polar_to_cartesian(image: &RgbImage, data: &ImageData, theta: f32) {
    // Use both the image and the processed center data etc to start getting pixels
}

struct ImageData {
    middle_x: f32,
    middle_y: f32,
    r_max: f32,
    radius_vector: Vec<f32>,
}
impl ImageData {
    pub fn new() -> ImageData {
        // Create a virtual center in case the strip dimensions are Even  number (there is no real
        // median or middle point)
        let (middle_x, middle_y, r_max) = (
            (f32::from(IMAGE_WIDTH) - 1f32) / 2.0,
            (f32::from(IMAGE_HEIGHT) - 1f32) / 2.0,
            (f32::from(LED_STRIP_LENGTH) * 2.0 - 1f32) / 2.0
        );
        println!("Create new ImageDatan, center of the picture: x={middle_x}, y={middle_y}");
        let mut r: Vec<f32> = vec![0.0;LED_STRIP_LENGTH as usize];
        for i in 0..LED_STRIP_LENGTH {
            r[i as usize] = f32::from(i) * r_max / (f32::from(LED_STRIP_LENGTH)-1f32);
        }
        println!("Radius values are: {:?}", r);
        ImageData {
            middle_x: middle_x,
            middle_y: middle_y,
            r_max: r_max,
            radius_vector: r,
        }
    }
}

// #[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_content = read_image();
    let rgb_image = convert_to_rgb8(&img_content);
    // Check the dimensions of the output pic
    println!("Dimensions of the scaled pic are: Width: {}, Height: {}", rgb_image.width(), rgb_image.height());
    // display_image(rgb_image.clone())
    let image_data = ImageData::new();
    polar_to_cartesian(&rgb_image, &image_data, 1.2);
    Ok(())
}
