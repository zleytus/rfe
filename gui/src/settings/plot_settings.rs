use egui::Color32;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlotSettings {
    pub autoscale_y_axis: bool,
    pub y_axis_max: i32,
    pub y_axis_min: i32,
    pub amp_offset: i32,
    pub show_current_sweep: bool,
    pub current_sweep_color: Color32,
    pub show_average_sweep: bool,
    pub average_sweep_color: Color32,
    pub show_max_sweep: bool,
    pub max_sweep_color: Color32,
    pub show_threshold_line: bool,
    pub threshold_line_value_dbm: i32,
    pub threshold_line_color: Color32,
}

impl Default for PlotSettings {
    fn default() -> Self {
        Self {
            autoscale_y_axis: false,
            y_axis_max: -40,
            y_axis_min: -120,
            amp_offset: 0,
            show_current_sweep: true,
            current_sweep_color: Color32::from_rgb(46, 204, 64),
            show_average_sweep: false,
            average_sweep_color: Color32::from_rgb(0, 116, 217),
            show_max_sweep: true,
            max_sweep_color: Color32::from_rgb(255, 65, 54),
            show_threshold_line: false,
            threshold_line_value_dbm: -80,
            threshold_line_color: Color32::from_rgb(255, 220, 0),
        }
    }
}
