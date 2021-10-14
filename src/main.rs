use yew::prelude::*;
use yew::web_sys::HtmlInputElement;
use yew::{ NodeRef, web_sys::Element };
use wasm_logger;

use todo::TodoType;

mod todo;

enum Msg {
    AddTodo,
}

struct Model {
    link: ComponentLink<Self>,
    todos: Vec<TodoType>,
    input_ref: NodeRef,
    last_id: usize,
    
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: Vec::new(),
            input_ref: NodeRef::default(),
            last_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo => {

                let val = self.input_ref.cast::<HtmlInputElement>().unwrap().value();
                if val != "" {
                    self.todos.push(TodoType::new(
                    self.last_id,
                    val
                ));
            }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        
        log::info!("Hi");
        let submit_onclick = self.link.callback(|_| Msg::AddTodo);
        html! {
            <div>
                <h1>{ "Todo List" }</h1>
                <input
                    ref=self.input_ref.clone()
                />

                <button onclick=submit_onclick
                >{ "Submit" }</button>

                <ul>
                    { for self.todos.iter().map(|p| p.render()) }
                </ul>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>(); 
}
