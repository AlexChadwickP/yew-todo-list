use yew::prelude::*;

#[derive(Clone)]
pub struct TodoInfo {
    id: usize,
    title: String,
    completed: bool,
}

impl TodoInfo {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    fn render(&self) -> Html {
        html! {
            <>
                <p>{ &self.title } </p>   
            </>
        }
    }
}

#[derive(Properties, Clone)]
pub struct TodoComponent {
    info: TodoInfo,
}

impl Component for TodoComponent {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <li class="todo" id=self.info.id.to_string()>
                { self.info.render() }
            </li>
        }
    } 
}   

pub enum TodoType {
    Component(TodoInfo),
}

impl TodoType {
    pub fn info(&self) -> &TodoInfo {
        match self {
            Self::Component(info) => info,
        }
    }

    pub fn new(id: usize, title: String) -> Self {
        let info = TodoInfo::new(id, title);
        Self::Component(info)
    }

    pub fn render(&self) -> Html {
        match self {
            Self::Component(info) => {
                html! { <TodoComponent info=info.clone() /> }
            }
        }
    }
}
