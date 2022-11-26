use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        App {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Login => html! {
            // <Login />
            <h1>{"Login"}</h1>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/404")]
    NotFound,
}
