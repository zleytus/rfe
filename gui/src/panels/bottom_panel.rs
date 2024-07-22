use egui::{Align, Context, InnerResponse, Layout, TopBottomPanel};

use crate::{
    settings::RfeSettings,
    widgets::{
        PauseScanningButton, PlotSettingsToggleButton, ResumeScanningButton,
        RfeSettingsToggleButton, UnitsComboBox,
    },
    App,
};

pub struct BottomPanel {
    panel: TopBottomPanel,
}

impl BottomPanel {
    pub fn new() -> Self {
        Self {
            panel: TopBottomPanel::bottom("bottom-panel").default_height(25.0),
        }
    }

    pub fn show(self, ctx: &Context, app: &mut App) -> InnerResponse<()> {
        self.panel.show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if ui
                        .add(RfeSettingsToggleButton::new(app.show_rfe_settings))
                        .clicked()
                    {
                        app.show_rfe_settings = !app.show_rfe_settings;
                    }
                    ui.add_enabled_ui(app.rfe.is_some(), |ui| {
                        if app.paused {
                            if ui.add(ResumeScanningButton::default()).clicked() {
                                app.paused = false;
                            }
                        } else {
                            if ui.add(PauseScanningButton::default()).clicked() {
                                app.paused = true;
                            }
                        }
                    });
                });
                columns[1].with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add(PlotSettingsToggleButton::new(app.show_plot_settings))
                        .clicked()
                    {
                        app.show_plot_settings = !app.show_plot_settings;
                    }
                    UnitsComboBox::default().show_ui(ui, &mut app.units, |unit| {
                        app.rfe_settings = RfeSettings::new(app.rfe.as_ref(), unit);
                    });
                });
            });
        })
    }
}
