use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    link: ComponentLink<Self>,
    color: Color,
    other_color: OtherColor,
}

pub enum Color {
    Pink,
    Yellow,
}

pub enum OtherColor {
    Green,
    Red,
}

pub enum Actions {
    AppClicked,
    OtherClicked,
}

impl Component for App {
    type Message = Actions;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            color: Color::Pink,
            other_color: OtherColor::Green,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Actions::AppClicked => {
                self.color = match self.color {
                    Color::Pink => Color::Yellow,
                    Color::Yellow => Color::Pink,
                };
            }
            Actions::OtherClicked => {
                self.other_color = match self.other_color {
                    OtherColor::Green => OtherColor::Red,
                    OtherColor::Red => OtherColor::Green,
                };
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let color = match self.color {
            Color::Pink => "pink",
            Color::Yellow => "yellow",
        };
        let other_color = match self.other_color {
            OtherColor::Red => "red",
            OtherColor::Green => "green",
        };
        let on_app_click = self.link.callback(|_| Actions::AppClicked);
        let on_other_click = self.link.callback(|_| Actions::OtherClicked);
        html! {
            <div onclick={ on_app_click } class=classes!("app", color)>
                <div onclick={ on_other_click } class=classes!("other", other_color) />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
