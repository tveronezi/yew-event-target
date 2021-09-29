use yew::{classes, html, Component, ComponentLink, Html, ShouldRender, MouseEvent};

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
    AppClicked(MouseEvent),
    OtherClicked(MouseEvent),
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
            Actions::AppClicked(event) => {
                if event.target() == event.current_target() {
                    self.color = match self.color {
                        Color::Pink => Color::Yellow,
                        Color::Yellow => Color::Pink,
                    };
                }
            }
            Actions::OtherClicked(event) => {
                if event.target() == event.current_target() {
                    self.other_color = match self.other_color {
                        OtherColor::Green => OtherColor::Red,
                        OtherColor::Red => OtherColor::Green,
                    };
                }
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
        let on_app_click = self.link.callback(Actions::AppClicked);
        let on_other_click = self.link.callback(Actions::OtherClicked);
        html! {
            <div onclick={ on_app_click } class=classes!("app", color)>
                <span class="label">{"Main"}</span>
                <div onclick={ on_other_click } class=classes!("other", other_color)>
                    <span>{"Other"}</span>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
