use crate::styles::{get_pallete, get_size, get_style, Palette, Size, Style};
use yew::prelude::*;

pub struct Dropdown {
    props: Props,
    active: bool,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// clickeable content to show the dropdown. Required
    pub main_content: Html,
    /// Palette style color for dropdown
    #[prop_or(Palette::Standard)]
    pub dropdown_palette: Palette,
    /// Style for dropdown
    #[prop_or(Style::Regular)]
    pub dropdown_style: Style,
    /// Size for dropdown
    #[prop_or(Size::Medium)]
    pub dropdown_size: Size,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    ShowDropdown,
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowDropdown => {
                self.active = !self.active;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=("dropdown", self.props.class_name.clone(), get_style(self.props.dropdown_style.clone()), get_pallete(self.props.dropdown_palette.clone()), get_size(self.props.dropdown_size.clone()))
                id=self.props.id
                onclick=self.link.callback(|_| Msg::ShowDropdown)
                >
                <div class="main-content">{self.props.main_content.clone()}</div>
                {get_items(self.active, self.props.children.clone())}
            </div>
        }
    }
}

fn get_items(active: bool, children: Children) -> Html {
    if active {
        html! {
            <ul>
                {children.clone()}
            </ul>
        }
    } else {
        html! {}
    }
}
