use gtk::prelude::*;
use relm::{init, Component, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {}

pub struct Model {}

#[widget]
impl Widget for WinTitlebar {
    fn model(relm: &relm::Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {}

    view! {
        gtk::HeaderBar {
            title: Some("hello world"),
            gtk::MenuButton {
                image: Some(&gtk::Image::from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::Menu)),
                child: {
                    pack_type: gtk::PackType::End
                },
                popover: view! {
                    gtk:: Popover {
                        gtk::Box {}
                    }
                }
            }
        }
    }
}
