pub mod app;
pub mod components;
pub mod pages;
pub mod sections;

pub mod utils {
    pub fn map_y_to_value(y: f64, y_visible_coord: f64) -> f64 {
        let start_y = y_visible_coord;
        let end_y = y_visible_coord + 500.0;
        let start_value = 120.0;
        let end_value = 0.0;

        if y < start_y {
            return start_value;
        }
        if y > end_y {
            return end_value;
        }

        let scale = (y - start_y) / (end_y - start_y);
        start_value + scale * (end_value - start_value)
    }
}
