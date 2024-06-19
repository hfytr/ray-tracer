#[cfg(test)]
mod tests {
    use ray_tracer::{renderer::Renderer, vector::Vector3};

    #[test]
    fn test1() {
        let mut renderer = Renderer::new(
            Vector3::<f64>::default(),
            Vector3::<f64>::new(-80.0, 100.0, 45.0),
            160,
            (16, 9),
        );
        renderer.add_mesh_from_file("assets/triangle.obj").unwrap();
        println!("{:?}", renderer);
        let image = renderer.render();
        image.write_to_png("img.png");
    }
}
