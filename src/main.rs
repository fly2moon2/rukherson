use nokhwa::{ Camera, CameraFormat,FrameFormat};
/// opencv::Result vs anyhow::Result
/// https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff
/// use anyhow::Result; // Automatically handle the error types
use opencv::{
    prelude::*,
    videoio,
    highgui,
    Result
}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.

fn main() -> Result<()> { // Note, this is anyhow::Result
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
       
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    Ok(())
}  

/* 
fn main() {

    // https://doc.rust-lang.org/rust-by-example/std/str.html
// (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    //let pangram = String::from("the quick brown fox jumps over the lazy dog");
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

        // You can use escapes to write bytes by their hexadecimal values...
        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
    
        // ...or Unicode code points.
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    
        println!("Unicode character {} (U+211D) is called {}",
                    unicode_codepoint, character_name );
    
    
        let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
        println!("{}", long_string);

/* test nokhwa */
/*     // set up the Camera
let mut camera = Camera::new(
    0, // index
    Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
)
.unwrap();
// open stream
camera.open_stream().unwrap();
loop {
    //let frame = camera.get_frame().unwrap();
/*     let frame = camera.frame().unwrap();
    
    println!("{}, {}", frame.width(), frame.height()); */
/*     let info1=camera.info().description();
    println!("result {}", info1); */
/*     let framer = camera.frame_rate();
    
    println!("{}", framer); */
} */
}
 */