use super::*;

#[test]
fn read_image() {
    read_image_bytes("test_images/1.png").unwrap();
    assert!(true);
}

#[test]
fn ihdr_len() {
    let bytes = read_image_bytes("test_images/1.png").unwrap();
    png::read_png(&bytes).unwrap();
}
