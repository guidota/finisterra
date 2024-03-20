use once_cell::sync::Lazy;

pub static Z: Lazy<[[[f32; 100]; 100]; 4]> = Lazy::new(|| {
    [
        build_layer(0),
        build_layer(1),
        build_layer(2),
        build_layer(3),
    ]
});

fn build_layer(layer: usize) -> [[f32; 100]; 100] {
    let array = [0.; 100];
    let mut matrix = [array; 100];
    for (x, item) in matrix.iter_mut().enumerate() {
        for (y, item) in item.iter_mut().enumerate() {
            *item = calculate_z(layer, x, y);
        }
    }

    matrix
}

fn calculate_z(layer: usize, x: usize, y: usize) -> f32 {
    match layer {
        0 => 0.,
        3 => 0.99,
        i => (i as f32 * 1000. + (100. - y as f32) * 10. - (100. - x as f32)) / 4000.,
    }
}
