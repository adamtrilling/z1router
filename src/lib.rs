extern crate serde_derive;
#[macro_use]
extern crate yew;

pub mod locations;

use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::{storage::Area, ConsoleService, StorageService};

use locations::{Location, Path, Route};

pub struct Model {
    console: ConsoleService,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    route: Route,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut storage = StorageService::new(Area::Local);
        let state = if let Json(Ok(restored_state)) = storage.restore("state") {
            restored_state
        } else {
            State {
                route: Route::new(),
            }
        };

        Model {
            console: ConsoleService::new(),
            storage: storage,
            state: state,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        self.console.log(&format!(
            "persisting to state: {}",
            serde_json::to_string(&self.state).unwrap()
        ));
        self.storage.store("state", Json(&self.state));
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                {"Paths:"}
                <ul>
                    { for self.state.route.paths.iter().map(|p| self.view_path(p)) }
                </ul>
            </div>
            <div>
                {"Location:"}
                { self.view_location(&self.state.route.location) }
            </div>
            <div>
                {"Next:"}
                { self.view_next_paths(&self.state.route) }
            </div>
        }
    }
}

impl Model {
    fn view_location(&self, location: &Location) -> Html<Model> {
        html! {
            { location }
        }
    }

    fn view_path(&self, path: &Path) -> Html<Model> {
        html! {
            <li>
                { path }
            </li>
        }
    }

    fn view_next_paths(&self, route: &Route) -> Html<Model> {
        let next_paths = route.next_paths();
        html! {
            <ul>
                { for next_paths.iter().map(|p| self.view_path(p)) }
            </ul>
        }
    }
}
