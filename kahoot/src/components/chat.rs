use crate::services::event_bus::EventBus;
use crate::services::websocket::WebsocketService;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
extern crate iron;
use wasm_bindgen::JsCast;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage(MsgTypes),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum MsgTypes {
    Join(StructJoin),
    Start,
    Response(StructResponse),
    Wait,
}

#[derive(Debug, Serialize)]
pub struct StructResponse {
    id_question: i32,
    response_value: i32,
}

#[derive(Debug, Serialize)]
pub struct StructJoin {
    code: String,
}

#[derive(Deserialize)]
pub struct QuestionStruct {
    pub question_value: i32,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ReceivedMessage {
    Question(QuestionStruct),
}

pub struct Chat {
    chat_input: NodeRef,
    wss: WebsocketService,
    current_question: Option<i32>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Msg {
    fn new_response(id_question: i32, response_value: i32) -> Self {
        Msg::SubmitMessage(MsgTypes::Response(StructResponse {
            id_question,
            response_value,
        }))
    }
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut wss = WebsocketService::new();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        let temp = html_document.url().unwrap();
        let message = MsgTypes::Join(StructJoin { code: temp });

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        //TODO:

        Self {
            chat_input: NodeRef::default(),
            wss,
            current_question: None,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!("update");
        match msg {
            Msg::HandleMsg(s) => {
                let received_msg: ReceivedMessage = serde_json::from_str(&s).unwrap();
                match received_msg {
                    ReceivedMessage::Question(q) => {
                        self.current_question = Some(q.question_value);
                        true
                    }
                }
            }
            Msg::SubmitMessage(message) => {
                if let Err(e) = self
                    .wss
                    .tx
                    .clone()
                    .try_send(serde_json::to_string(&message).unwrap())
                {
                    log::debug!("error sending to channel: {:?}", e);
                }
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::SubmitMessage(MsgTypes::Start));
     //   let click = ctx.link().callback(|_| Msg::SubmitMessage(MsgTypes::Wait));
        let started = false;
        match self.current_question {
            None => {
                html! {
                <div class="flex w-screen">
                         <div class="grow h-screen flex flex-col">
                             <div class="container mx-auto flex flex-col justify-center items-center">
                                 <div class="flex">
                       <button {onclick} class="px-8 rounded-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r"> {"Start the game"}
                     </button>
                     </div>
                     </div>
                     </div>
                     </div>
                 }
            }
            Some(question_index) => {
                html! {
                    <div class="flex w-screen">
                        <div class="grow h-screen flex flex-col">
                            <div class="w-full h-14 border-b-2 border-gray-300"></div>
                            <div class="container mx-auto flex flex-col justify-center items-center">
                                <div class="flex">
                                    <button onclick={ctx.link().callback(|_| Msg::new_response(0,0))} class="px-8 rounded-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                                        {"réponse A"}
                                    </button>
                                    <button onclick={ctx.link().callback(|_| Msg::new_response(0,1))} class="px-8 rounded-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                                        {"réponse b"}
                                    </button>
                                </div>
                                <div class="flex">
                                    <button onclick={ctx.link().callback(|_| Msg::new_response(0,2))} class="px-8 rounded-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                                        {"réponse c"}
                                    </button>
                                    <button onclick={ctx.link().callback(|_| Msg::new_response(0,3))} class="px-8 rounded-lg bg-violet-600 text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r">
                                        {"réponse d"}
                                    </button>

                                </div>
                            </div>
                        </div>
                    </div>

                }
            }
        }
    }
}


