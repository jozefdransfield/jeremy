#[cfg(test)]
mod tests {
    use crate::frame;
    use std::path::PathBuf; // import functions from parent module

    #[test]
    fn test_iphone_16_pro_max_frames() {
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Portrait.png"),
            None,
            PathBuf::from("target/iPhone 16 Pro Max - Default - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Landscape.png"),
            None,
            PathBuf::from("target/iPhone 16 Pro Max - Default - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Portrait.png"),
            Some(String::from("Black Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - Black Titanium - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Landscape.png"),
            Some(String::from("Black Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - Black Titanium - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Portrait.png"),
            Some(String::from("Desert Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - Desert Titanium - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Landscape.png"),
            Some(String::from("Desert Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - Desert Titanium - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Portrait.png"),
            Some(String::from("Natural Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - Natural Titanium - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Landscape.png"),
            Some(String::from("Natural Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - Natural Titanium - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Portrait.png"),
            Some(String::from("White Titanium")),
            PathBuf::from("target/iPhone 6.9 - Placeholder.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Landscape.png"),
            Some(String::from("White Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - White Titanium - Landscape.png"),
        );
    }

    #[test]
    fn test_iphone_16_pro_frames() {
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Portrait.png"),
            None,
            PathBuf::from("target/iPhone 16 Pro - Default - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Landscape.png"),
            None,
            PathBuf::from("target/iPhone 16 Pro - Default - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Portrait.png"),
            Some(String::from("Black Titanium")),
            PathBuf::from("target/iPhone 16 Pro - Black Titanium - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Landscape.png"),
            Some(String::from("Black Titanium")),
            PathBuf::from("target/iPhone 16 Pro - Black Titanium - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Portrait.png"),
            Some(String::from("Desert Titanium")),
            PathBuf::from("target/iPhone 16 Pro - Desert Titanium - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Landscape.png"),
            Some(String::from("Desert Titanium")),
            PathBuf::from("target/iPhone 16 Pro - Desert Titanium - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Portrait.png"),
            Some(String::from("Natural Titanium")),
            PathBuf::from("target/iPhone 16 Pro - Natural Titanium - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro - Landscape.png"),
            Some(String::from("Natural Titanium")),
            PathBuf::from("target/iPhone 16 Pro - Natural Titanium - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Portrait.png"),
            Some(String::from("White Titanium")),
            PathBuf::from("target/iPhone 6.9 - Placeholder.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Pro Max - Landscape.png"),
            Some(String::from("White Titanium")),
            PathBuf::from("target/iPhone 16 Pro Max - White Titanium - Landscape.png"),
        );
    }

    #[test]
    fn test_iphone_16_plus_frames() {
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Portrait.png"),
            None,
            PathBuf::from("target/iPhone 16 Plus - Default - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Landscape.png"),
            None,
            PathBuf::from("target/iPhone 16 Plus - Default - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Portrait.png"),
            Some(String::from("Black")),
            PathBuf::from("target/iPhone 16 Plus - Black - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Landscape.png"),
            Some(String::from("Black")),
            PathBuf::from("target/iPhone 16 Plus - Black - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Portrait.png"),
            Some(String::from("Pink")),
            PathBuf::from("target/iPhone 16 Plus - Pink - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Landscape.png"),
            Some(String::from("Pink")),
            PathBuf::from("target/iPhone 16 Plus - Pink - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Portrait.png"),
            Some(String::from("Teal")),
            PathBuf::from("target/iPhone 16 Plus - Teal - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Landscape.png"),
            Some(String::from("Teal")),
            PathBuf::from("target/iPhone 16 Plus - Teal - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Portrait.png"),
            Some(String::from("Ultramarine")),
            PathBuf::from("target/iPhone 16 Plus - Ultramarine - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Landscape.png"),
            Some(String::from("Ultramarine")),
            PathBuf::from("target/iPhone 16 Plus - Ultramarine - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Portrait.png"),
            Some(String::from("White")),
            PathBuf::from("target/iPhone 16 Plus - White - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 Plus - Landscape.png"),
            Some(String::from("White")),
            PathBuf::from("target/iPhone 16 Plus - White - Landscape.png"),
        );
    }

    #[test]
    fn test_iphone_16_frames() {
        frame(
            PathBuf::from("screenshots/iPhone 16 - Portrait.png"),
            None,
            PathBuf::from("target/iPhone 16 - Default - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 - Landscape.png"),
            None,
            PathBuf::from("target/iPhone 16 - Default - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 - Portrait.png"),
            Some(String::from("Black")),
            PathBuf::from("target/iPhone 16 - Black - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 - Landscape.png"),
            Some(String::from("Black")),
            PathBuf::from("target/iPhone 16 - Black - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 - Portrait.png"),
            Some(String::from("Pink")),
            PathBuf::from("target/iPhone 16 - Pink - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 - Landscape.png"),
            Some(String::from("Pink")),
            PathBuf::from("target/iPhone 16 - Pink - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 - Portrait.png"),
            Some(String::from("Teal")),
            PathBuf::from("target/iPhone 16 - Teal - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 - Landscape.png"),
            Some(String::from("Teal")),
            PathBuf::from("target/iPhone 16 - Teal - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 - Portrait.png"),
            Some(String::from("Ultramarine")),
            PathBuf::from("target/iPhone 16 - Ultramarine - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 - Landscape.png"),
            Some(String::from("Ultramarine")),
            PathBuf::from("target/iPhone 16 - Ultramarine - Landscape.png"),
        );

        frame(
            PathBuf::from("screenshots/iPhone 16 - Portrait.png"),
            Some(String::from("White")),
            PathBuf::from("target/iPhone 16 - White - Portrait.png"),
        );
        frame(
            PathBuf::from("screenshots/iPhone 16 - Landscape.png"),
            Some(String::from("White")),
            PathBuf::from("target/iPhone 16 - White - Landscape.png"),
        );
    }
}
