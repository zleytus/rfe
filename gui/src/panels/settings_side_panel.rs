use egui::{Align, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder, TableRow};
use rfe::Frequency;

use crate::settings::FrequencyUnits;

#[derive(Debug, Default, Clone)]
pub struct InfoItem<'a> {
    title: &'a str,
    value: String,
    units: Option<FrequencyUnits>,
}

impl<'a> InfoItem<'a> {
    pub fn new(title: &'a str, value: String) -> Self {
        Self {
            title,
            value,
            units: None,
        }
    }

    pub fn new_freq(title: &'a str, freq: Frequency, units: FrequencyUnits) -> Self {
        let value = match units {
            FrequencyUnits::Hz => freq.as_hz().to_string(),
            FrequencyUnits::Khz => freq.as_khz_f64().to_string(),
            FrequencyUnits::Mhz => freq.as_mhz_f64().to_string(),
            FrequencyUnits::Ghz => freq.as_ghz_f64().to_string(),
        };
        Self {
            title,
            value,
            units: Some(units),
        }
    }

    pub fn add_to_row(&self, mut row: TableRow<'_, '_>) {
        row.col(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label(self.title);
            });
        });
        row.col(|ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if let Some(units) = self.units {
                    ui.label(units.to_string());
                }
                ui.label(&self.value);
            });
        });
    }
}

pub struct InfoCategory<'a> {
    title: &'a str,
}

impl<'a> InfoCategory<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }

    pub fn show(self, ui: &mut Ui, info_items: &[InfoItem<'a>]) {
        ui.label(RichText::new(self.title).size(16.0).strong());
        ui.add_space(5.0);
        TableBuilder::new(ui)
            .id_salt(self.title)
            .striped(true)
            .column(Column::remainder())
            .column(Column::auto())
            .body(|body| {
                body.rows(30.0, info_items.len(), |row| {
                    if let Some(info_item) = info_items.get(row.index()) {
                        info_item.add_to_row(row);
                    }
                });
            });
    }
}

#[derive(Clone)]
pub struct Setting<'a, F: FnOnce(&mut Ui)> {
    title: &'a str,
    content: F,
}

impl<'a, F: FnOnce(&mut Ui)> Setting<'a, F> {
    pub fn new(title: &'a str, content: F) -> Self {
        Setting { title, content }
    }

    pub fn add_to_row(self, mut row: TableRow<'_, '_>) {
        row.col(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label(self.title);
            });
        });
        row.col(|ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), self.content);
        });
    }
}

pub struct SettingsCategory<'a> {
    title: &'a str,
}

impl<'a> SettingsCategory<'a> {
    pub fn new(title: &'a str) -> Self {
        SettingsCategory { title }
    }

    fn show_internal(
        self,
        ui: &mut Ui,
        rows: usize,
        add_row_content: impl FnMut(TableRow<'_, '_>),
        add_bottom_content: Option<impl FnOnce(&mut Ui)>,
    ) {
        ui.label(RichText::new(self.title).size(16.0).strong());
        ui.add_space(5.0);
        ui.push_id(self.title, |ui| {
            TableBuilder::new(ui)
                .id_salt(self.title)
                .striped(true)
                .column(Column::remainder())
                .column(Column::auto())
                .body(|body| {
                    body.rows(30.0, rows, add_row_content);
                });
        });
        if let Some(add_bottom_content) = add_bottom_content {
            add_bottom_content(ui);
        }
    }

    pub fn show(self, ui: &mut Ui, rows: usize, add_row_content: impl FnMut(TableRow<'_, '_>)) {
        self.show_internal(ui, rows, add_row_content, None::<fn(&mut Ui)>);
    }
}
