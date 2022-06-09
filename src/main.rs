use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i32,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
            <p>{ self.value }</p>
            <button onclick={link.callback(|_| Msg::AddOne)}> {"+1"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

// ciog7VxzMBJ5K2aJSfG1jgFrPKehTbCYguU cargo tocken
