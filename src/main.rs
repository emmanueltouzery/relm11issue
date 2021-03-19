use gtk::prelude::*;
use relm::{Component, Widget};
use relm_derive::{widget, Msg};
use wintitlebar::WinTitlebar;

mod wintitlebar;

fn main() {
    Win::run(()).unwrap();
}

#[derive(Msg)]
pub enum Msg {}

pub struct Model {
    titlebar: Component<WinTitlebar>,
}

#[widget]
impl Widget for Win {
    fn model(relm: &relm::Relm<Self>, params: ()) -> Model {
        let titlebar = relm::init::<WinTitlebar>(()).expect("win title bar init");
        Model { titlebar }
    }

    fn update(&mut self, event: Msg) {}

    view! {
        gtk::Window {
            titlebar: Some(self.model.titlebar.widget())
        }
    }
}
