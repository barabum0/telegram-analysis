use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use chrono::{Datelike, DateTime, NaiveDate};
use egui_notify::Toasts;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};
use futures::StreamExt;
use log::info;

#[cfg(target_arch = "wasm32")]
use crate::file_reader::open_file_picker;
use crate::graph::{count_by_days, split_by_people};
use crate::tg_result::{Actor, TgExportMessage, TgExportResult};


struct Graphs {
    pub messages_by_actor: HashMap<Actor, Vec<TgExportMessage>>,
    pub count_by_days: HashMap<Actor, Vec<[f64; 2]>>,
}

impl Clone for Graphs {
    fn clone(&self) -> Self {
        Self {
            messages_by_actor: self.messages_by_actor.clone(),
            count_by_days: self.count_by_days.clone(),
        }
    }
}

pub(crate) struct MyApp {
    file_content: Rc<RefCell<Option<String>>>,
    toasts: Toasts,
    export_result: Option<TgExportResult>,
    graphs: Option<Graphs>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            file_content: Rc::new(RefCell::new(None)),
            toasts: Toasts::default(),
            export_result: None,
            graphs: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.toasts.show(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            let file_content = self.file_content.clone();

            ui.heading("Telegram Chat Message Analyzer");

            let content = &*file_content.borrow();

            if content.is_none() {
                if ui.button("Open file").clicked() {
                    #[cfg(target_arch = "wasm32")]
                    {
                        open_file_picker(file_content.clone());
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        self.toasts.error("Открытие файлов доступно только в веб-версии.");
                    }
                }
            } else {
                if ui.button("Close file").clicked() {
                    self.file_content = Rc::new(RefCell::new(None));
                    self.export_result = None;
                    self.graphs = None;
                }
                ui.separator();

                ui.label("Messages by days");

                if content.is_some() {
                    let mut parsed: TgExportResult = serde_json::from_str(&content.clone().unwrap()).unwrap();
                    parsed.messages.sort_by_key(|m| { m.get_actor_id() });
                    parsed.messages.reverse();

                    if self.graphs.is_none() {
                        let messages_by_actor = split_by_people(&parsed.messages);
                        self.graphs = Some(
                            Graphs {
                                count_by_days: count_by_days(&messages_by_actor),
                                messages_by_actor,
                            }
                        );
                    }
                    self.export_result = Some(parsed);
                }

                if self.graphs.is_some() {
                    let plot = Plot::new("message_by_days")
                        .legend(Legend::default())
                        .min_size(ui.available_size())
                        .show_axes(true)
                        .show_grid(true)
                        .x_axis_label("День")
                        .y_axis_label("Количество сообщений")
                        .x_axis_formatter(|m, r| { NaiveDate::from_num_days_from_ce_opt(m.value.round() as i32).unwrap().format("%Y-%m-%d").to_string() });

                    plot.show(ui, |plot_ui| {
                        for (actor, messages) in self.graphs.clone().unwrap().count_by_days {
                            plot_ui.line(Line::new(PlotPoints::new(messages)).name(actor.name))
                        }
                    });
                } else {
                    ui.label("File is not opened.");
                }
            }
        });
    }
}