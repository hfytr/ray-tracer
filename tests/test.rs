#[cfg(test)]
mod tests {
    use ray_tracer::{renderer::Renderer, vector::Vector3};

    #[test]
    fn test1() {
        let mut renderer = Renderer::new(
            Vector3::<f64>::new(0.0, -1100.0, 0.0),
            Vector3::<f64>::new(-80.0, -900.0, 45.0),
            160,
            (16, 9),
        );
        renderer.load_obj("assets/knight.obj").unwrap();
        let image = renderer.render();
        image.write_to_png("img.png").unwrap();
    }
}
