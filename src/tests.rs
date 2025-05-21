// Unit tests
#[cfg(test)]
mod tests {
    use crate::frame;
    use super::*; // import functions from parent module

    #[test]
    fn test_add() {
        frame(
            "screenshots/iPhone 16 Pro Max - Portrait.png",
            None,
            "target/iPhone 16 Pro Max - Default - Portrait.png",
        );
        frame(
            "screenshots/iPhone 16 Pro Max - Landscape.png",
            None,
            "target/iPhone 16 Pro Max - Default - Landscape.png",
        );

        frame(
            "screenshots/iPhone 16 Pro Max - Portrait.png",
            Some("Black Titanium"),
            "target/iPhone 16 Pro Max - Black Titanium - Portrait.png",
        );
        frame(
            "screenshots/iPhone 16 Pro Max - Landscape.png",
            Some("Black Titanium"),
            "target/iPhone 16 Pro Max - Black Titanium - Landscape.png",
        );

        frame(
            "screenshots/iPhone 16 Pro Max - Portrait.png",
            Some("Desert Titanium"),
            "target/iPhone 16 Pro Max - Desert Titanium - Portrait.png",
        );
        frame(
            "screenshots/iPhone 16 Pro Max - Landscape.png",
            Some("Desert Titanium"),
            "target/iPhone 16 Pro Max - Desert Titanium - Landscape.png",
        );

        frame(
            "screenshots/iPhone 16 Pro Max - Portrait.png",
            Some("Natural Titanium"),
            "target/iPhone 16 Pro Max - Natural Titanium - Portrait.png",
        );
        frame(
            "screenshots/iPhone 16 Pro Max - Landscape.png",
            Some("Natural Titanium"),
            "target/iPhone 16 Pro Max - Natural Titanium - Landscape.png",
        );

        frame(
            "screenshots/iPhone 16 Pro Max - Portrait.png",
            Some("White Titanium"),
            "target/iPhone 16 Pro Max - White Titanium - Portrait.png",
        );
        frame(
            "screenshots/iPhone 16 Pro Max - Landscape.png",
            Some("White Titanium"),
            "target/iPhone 16 Pro Max - White Titanium - Landscape.png",
        );
        // frame(
        //     "screenshots/iPhone 16 Pro - Portrait.png",
        //     "target/iPhone 16 Pro - Portrait.png",
        // );
        // frame(
        //     "screenshots/iPhone 16 Pro - Landscape.png",
        //     "target/iPhone 16 Pro - Landscape.png",
        // );
        //
        // frame(
        //     "screenshots/iPhone 16 Plus - Portrait.png",
        //     "target/iPhone 16 Plus - Portrait.png",
        // );
        // frame(
        //     "screenshots/iPhone 16 Plus - Landscape.png",
        //     "target/iPhone 16 Plus - Landscape.png",
        // );
        //
        // frame(
        //     "screenshots/iPhone 16 - Portrait.png",
        //     "target/iPhone 16 - Portrait.png",
        // );
        // frame(
        //     "screenshots/iPhone 16 - Landscape.png",
        //     "target/iPhone 16 - Landscape.png",
        // );
    }
}