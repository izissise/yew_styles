use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::{utils, App};

/// # Navbar Dropdown Container component
///
/// ## Features required
///
/// navbar
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew::services::ConsoleService;
/// use yew_styles::{
///     navbar::{
///         navbar_component::{Fixed, Navbar},
///         navbar_container::NavbarContainer,
///         navbar_item::NavbarItem,
///     },
///     styles::{Palette, Style},
///     layouts::{
///         container::{JustifyContent, Mode},
///     },
/// };
///
/// pub struct App {
///   link: ComponentLink<Self>,
/// }
///
/// pub enum Msg {
///   ChangeMenu(String),
/// }
/// #[derive(Clone, Properties)]
/// pub struct Props {}
///
/// impl Component for App {
///     type Message = Msg;
///     type Properties = Props;
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         App {
///             link
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::ChangeMenu(menu) => {
///                 ConsoleService::log(format!("{}", menu));
///             }
///         }
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///        html! {
///            <Navbar
///                fixed=Fixed::None
///                navbar_style=Style::Light
///                navbar_type=Palette::Info
///                branch=html!{<img src="/assets/spielrs_logo.png"></img>}>
///                    <NavbarContainer justify_content=JustifyContent::FlexStart(Mode::NoMode)>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Home")))>
///                            <span>{"Home"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                            <span>{"Shop"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Shop")))>
///                            <span>{"Shop"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("About us")))>   
///                            <span>{"About us"}</span>
///                        </NavbarItem>
///                        <NavbarItem
///                            onclick_signal=link.callback(move |_| Msg::ChangeMenu(String::from("Contact")))>   
///                            <span>{"Contact"}</span>
///                        </NavbarItem>
///                        <NavbarDropdown main_content=html!{
///                           <span>{menu}<ControllerAssets
///                             icon=ControllerIcon::ChevronDown
///                             size=("20".to_string(), "20".to_string())
///                           /></span>
///                        }>
///                          <NavbarDropdownItem
///                            onclick_signal=link.callback(move |_: MouseEvent| Msg::ChangeType(String::from("menu 1".to_string())))>{"menu 1"}</NavbarDropdownItem>
///                          <NavbarDropdownItem
///                            onclick_signal=link.callback(move |_: MouseEvent| Msg::ChangeType(String::from("menu 2".to_string())))>{"menu 2"}</NavbarDropdownItem>
///                          <NavbarDropdownItem
///                            onclick_signal=link.callback(move |_: MouseEvent| Msg::ChangeType(String::from("menu 3".to_string())))>{"menu 3"}</NavbarDropdownItem>
///                        </NavbarDropdown>
///                    </NavbarContainer>
///              </Navbar>
///         }
///     }
/// }
/// ```
pub struct NavbarDropdown {
    props: Props,
    show: bool,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    /// clickeable content to show the dropdown. Required
    pub main_content: Html,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// show with style when the dropdown is currrently active
    #[prop_or(false)]
    pub active: bool,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    ShowDropdown,
    HideDropdown,
}

impl Component for NavbarDropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            show: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowDropdown => {
                self.show = true;
            }
            Msg::HideDropdown => {
                self.show = false;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=("navbar-dropdown", if self.props.active {
                    "active"
                } else {
                    ""
                }, self.props.class_name.clone())
                id=self.props.id
                onmouseover=self.link.callback(|_| Msg::ShowDropdown)
                onmouseleave=self.link.callback(|_| Msg::HideDropdown)
                onclick=self.link.callback(|_| Msg::HideDropdown)
                >
                <div class="main-content">{self.props.main_content.clone()}</div>
                {get_items(self.show, self.props.children.clone())}
            </div>
        }
    }
}

fn get_items(show: bool, children: Children) -> Html {
    if show {
        html! {
            <ul>
                {children.clone()}
            </ul>
        }
    } else {
        html! {}
    }
}

#[wasm_bindgen_test]
fn should_create_navbar_dropdown_container() {
    let navbar_dropdown_container_props = Props {
        main_content: html! {<div id="test">{"test"}</div>},
        active: false,
        class_name: String::from("class-test"),
        id: String::from("id-test"),
        children: Children::new(vec![html! {
            <div id="item">{"Item"}</div>
        }]),
    };

    let navbar_dropdown_container: App<NavbarDropdown> = App::new();

    navbar_dropdown_container.mount_with_props(
        utils::document().get_element_by_id("output").unwrap(),
        navbar_dropdown_container_props,
    );

    let content_element = utils::document().get_element_by_id("test").unwrap();
    assert_eq!(content_element.text_content().unwrap(), "test".to_string());
}
