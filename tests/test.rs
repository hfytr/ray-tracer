#[cfg(test)]
mod tests {
    use ray_tracer::{renderer::Renderer, vector::Vector3};

    #[test]
    fn test1() {
        let mut renderer = Renderer::new(
            Vector3::<f64>::new(0.0, -1500.0, 160.0),
            Vector3::<f64>::new(-80.0, -1400.0, 200.0),
            160,
            (16, 9),
            0.001,
            4,
            100,
        );
        renderer.load_obj("assets/lightknight.obj").unwrap();
        let image = renderer.render();
        image.write_to_png("img.png").unwrap();
    }
}
