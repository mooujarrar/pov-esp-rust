use image::ImageReader;
use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use show_image::{event, Image, create_window};


// This is the length of the rotating strip fixed into the wheels radius
const LED_STRIP_LENGTH: u32 = 14;

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
    decoded_content.resize_exact(LED_STRIP_LENGTH*2, LED_STRIP_LENGTH*2, FilterType::Triangle)
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

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img_content = read_image();
    let rgb_image = convert_to_rgb8(&img_content);
    // Check the dimensions of the output pic
    println!("Dimensions of the scaled pic are: Width: {}, Height: {}", rgb_image.width(), rgb_image.height());
    display_image(rgb_image.clone())
}
