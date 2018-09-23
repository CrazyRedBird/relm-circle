use {
    self::Msg::{Calculate, Quit},
    gtk::prelude::*,
    relm::{connect, connect_stream, Widget},
    relm_attributes::widget,
    relm_derive::Msg,
    std::f64::consts::PI,
};

#[derive(Msg)]
pub enum Msg {
    Calculate,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() {}

    fn update(&mut self, msg: Msg) {
        match msg {
            Calculate => match self.radius.get_text().and_then(|s| s.parse::<f64>().ok()) {
                Some(radius) if radius > 0.0 => {
                    self.circumference
                        .set_text(&format!("{:.2}", 2.0 * PI * radius));
                    self.area.set_text(&format!("{:.2}", PI * radius.powi(2)));
                }
                _ => {
                    self.circumference.set_text("");
                    self.area.set_text("");
                }
            },
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            resizable: false,
            title: "Relm Circle",

            gtk::Grid {
                gtk::Label {
                    text: "Circle Calculator",

                    cell: {
                        left_attach: 0,
                        top_attach: 0,
                        width: 2,
                    },
                },
                gtk::Label {
                    text: "Radius: ",

                    cell: {
                        left_attach: 0,
                        top_attach: 1,
                    },
                },
                #[name = "radius"]
                gtk::Entry {
                    cell: {
                        left_attach: 1,
                        top_attach: 1,
                    },

                    changed => Calculate,
                },
                gtk::Label {
                    text: "Circumference: ",

                    cell: {
                        left_attach: 0,
                        top_attach: 2,
                    },
                },
                #[name = "circumference"]
                gtk::Entry {
                    editable: false,

                    cell: {
                        left_attach: 1,
                        top_attach: 2,
                    },
                },
                gtk::Label {
                    text: "Area: ",

                    cell: {
                        left_attach: 0,
                        top_attach: 3,
                    },
                },
                #[name = "area"]
                gtk::Entry {
                    editable: false,

                    cell: {
                        left_attach: 1,
                        top_attach: 3,
                    },
                },
                gtk::Button {
                    label: "Close",

                    cell: {
                        left_attach: 0,
                        top_attach: 4,
                        width: 2,
                    },

                    clicked => Quit,
                },
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run() failed");
}
