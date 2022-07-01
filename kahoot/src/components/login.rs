use yew::prelude::*;

pub enum Msg {
    _SubmitMessage(i32),
}

pub struct Login;
impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
           <div class="bg-gray-800 flex w-screen">
                <div class="container mx-auto flex flex-col justify-center items-center">
                <a href="/create" class="px-8 rounded-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r" >{"Create Game!"}</a>
                <h1 class="text-white px-8">{"Or"}</h1>
                    <form class="m-4">
                        <input class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Rejoindre une partie" />
                        <button class="px-8 rounded-r-lg bg-violet-600	  text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r" >{"Join!"}</button>
                    </form>
                </div>
            </div>
        }
    }
}
