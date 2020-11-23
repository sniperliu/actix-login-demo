use anyhow::Error;
use yew::format::Json;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    // pub create_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub struct Registration {
    link: ComponentLink<Self>,
    first_name: String,
    last_name: String,
    email: String,
    active_tasks: Vec<FetchTask>,
}

pub enum RegisterMsg {
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateEmail(String),
    Submit,
    Success(User),
    Fail,
}

impl Component for Registration {
    type Message = RegisterMsg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            active_tasks: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RegisterMsg::UpdateFirstName(first_name) => { self.first_name = first_name; false },
            RegisterMsg::UpdateLastName(last_name) => { self.last_name = last_name; false },
            RegisterMsg::UpdateEmail(email) => { self.email = email; false },
            RegisterMsg::Success(user) => { self.first_name = String::from(user.first_name);
                                            self.last_name = String::from(user.last_name);
                                            self.email = String::from(user.email);
                                            true },
            RegisterMsg::Fail => { false },
            RegisterMsg::Submit => {
                let new_user = NewUser {
                    first_name: String::from(&self.first_name),
                    last_name: String::from(&self.last_name),
                    email: String::from(&self.email),
                };

                let request = Request::post("http://localhost:8080/users")
                    .header("Content-Type", "application/json")
                    .body(Json(&new_user))
                    .expect("Failed to build request.");

                let task = FetchService::fetch(
                    request,
                    self.link.callback(|response: Response<Json<Result<User, Error>>>| {
                        if let (meta, Json(Ok(user))) = response.into_parts() {
                            if meta.status.is_success() {
                                return RegisterMsg::Success(user);
                            }
                        }

                        RegisterMsg::Fail
                    }));

                self.active_tasks.push(task.unwrap());

                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let Self { link, first_name, last_name, email, .. } = self;

        html! {
            <>
                <div>
                <input
                placeholder="First Name"
                value=&first_name
                oninput=link.callback(|e: InputData| RegisterMsg::UpdateFirstName(e.value))
                />
                <input
                placeholder="Last Name"
                value=&last_name
                oninput=link.callback(|e: InputData| RegisterMsg::UpdateLastName(e.value))
                />
                <input
                placeholder="Email"
                value=&email
                oninput=link.callback(|e: InputData| RegisterMsg::UpdateEmail(e.value))
                />
                </div>
                <button onclick=link.callback(|_| RegisterMsg::Submit)> { "Register" } </button>
            </>
        }
    }
}
