use egui::Color32;

/// The settings of the sweep plot's appearance.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TraceSettings {
    pub autoscale_y_axis: bool,
    pub y_axis_max: i32,
    pub y_axis_min: i32,
    pub amp_offset: i32,
    pub current_trace_color: Color32,
    pub average_trace_color: Color32,
    pub max_trace_color: Color32,
    pub average_iterations: u8,
    pub hide_trace: bool,
}

impl Default for TraceSettings {
    fn default() -> Self {
        Self {
            autoscale_y_axis: false,
            y_axis_max: -40,
            y_axis_min: -120,
            amp_offset: 0,
            current_trace_color: Color32::from_rgb(46, 204, 64),
            average_trace_color: Color32::from_rgb(0, 116, 217),
            average_iterations: 5,
            max_trace_color: Color32::from_rgb(255, 65, 54),
            hide_trace: false,
        }
    }
}
