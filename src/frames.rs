use std::collections::HashMap;

pub struct Frame {
    pub device: &'static str,
    pub variant: &'static str,
    pub dimensions: (u32, u32),
    pub corner_radius: u32,
    pub path: &'static str,
}

pub fn all_frames() -> Vec<Frame> {
    vec![
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Black Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Black Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Black Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Black Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Desert Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Desert Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Desert Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Desert Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Natural Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Natural Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Natural Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Natural Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "White Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - White Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "White Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - White Titanium - Portrait.png",
        },






        Frame {
            device: "iPhone 16 Pro",
            variant: "Black Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - Black Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Black Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - Black Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Desert Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - Desert Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Desert Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - Desert Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Natural Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - Natural Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Natural Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - Natural Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "White Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - White Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "White Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - White Titanium - Portrait.png",
        },
    ]
    //
    // const I_PHONE_16_PRO_LANDSCAPE: Device = Device {
    //     dimensions: (2622, 1206),
    //     style: "frames/iPhone 16 Pro/iPhone 16 Pro - Black Titanium - Landscape.png",
    //     corner_radius: 100,
    // };
    //
    // const I_PHONE_16_PRO_PORTRAIT: Device = Device {
    //     dimensions: (1206, 2622),
    //     style: "frames/iPhone 16 Pro/iPhone 16 Pro - Black Titanium - Portrait.png",
    //     corner_radius: 100,
    // };
    //
    // const I_PHONE_16_PLUS_LANDSCAPE: Device = Device {
    //     dimensions: (2796, 1290),
    //     style: "frames/iPhone 16 Plus/iPhone 16 Plus - Black - Landscape.png",
    //     corner_radius: 100,
    // };
    //
    // const I_PHONE_16_PLUS_PORTRAIT: Device = Device {
    //     dimensions: (1290, 2796),
    //     style: "frames/iPhone 16 Plus/iPhone 16 Plus - Black - Portrait.png",
    //     corner_radius: 100,
    // };
    //
    // const I_PHONE_16_LANDSCAPE: Device = Device {
    //     dimensions: (2556, 1179),
    //     style: "frames/iPhone 16/iPhone 16 - Black - Landscape.png",
    //     corner_radius: 100,
    // };
    //
    // const I_PHONE_16_PORTRAIT: Device = Device {
    //     dimensions: (1179, 2556),
    //     style: "frames/iPhone 16/iPhone 16 - Black - Portrait.png",
    //     corner_radius: 100,
    // };
}
