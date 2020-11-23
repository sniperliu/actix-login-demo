use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, prelude::*};

mod content;
mod login;

struct AppModel {
    link: ComponentLink<Self>,
}

struct AppMsg;

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/registration"]
    Registration,
    #[to = "/content"]
    Content,
}

type AppRouter = Router<AppRoute>;
type AppAnchor = RouterAnchor<AppRoute>;

impl Component for AppModel {
    type Message = AppMsg;

    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav>
                <AppAnchor route=AppRoute::Registration>{ "Registration" }</AppAnchor>
                <AppAnchor route=AppRoute::Content>{ "Content" }</AppAnchor>
                </nav>
                <main>
                <AppRouter render=Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Registration => html!{ <login::Registration /> },
                        AppRoute::Content => html!{ <content::Content /> },
                    }
                }) />
                </main>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppModel>::new().mount_to_body();
}
