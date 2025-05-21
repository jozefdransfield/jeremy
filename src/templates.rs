use std::path::PathBuf;
use resvg::{tiny_skia, usvg};

pub fn render(template: PathBuf, screenshot: PathBuf, output: PathBuf) {
    // load the template in xml,

    // replace screenshot with base64 encoded image
    
    let opt = usvg::Options {
        resources_dir: Some(
            std::path::PathBuf::from(&template)
                .parent()
                .unwrap()
                .to_owned(),
        ),
        ..usvg::Options::default()
    };

    let tree = {
        let svg_data = std::fs::read(&template).unwrap();
        usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    let size = tree
        .size()
        .to_int_size();

    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    let render_ts = tiny_skia::Transform::from_scale(
        size.width() as f32 / tree.size().width() as f32,
        size.height() as f32 / tree.size().height() as f32,
    );
    resvg::render(&tree, render_ts, &mut pixmap.as_mut());

    pixmap.save_png(output).expect("Failed to save PNG");
}