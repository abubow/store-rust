use common::WSMessageType;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;
#[function_component]
fn App() -> Html{
    let message_handler = use_state(Vec::default);
    let messages = (*message_handler).clone();

    let text_input_handler = use_state(|| String::from(""));
    let text_input = (*text_input_handler).clone();

    let ws = use_websocket("ws://localhost:8000/".to_string());

    let mut copy_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            let ws_msg: common::WSMessage = serde_json::from_str::<common::WSMessage>(&ws_msg).unwrap();
            match ws_msg.message_type {
                WSMessageType::NewMessage => {
                    let msg = ws_msg.message.expect("Error: Message missing payload");
                    copy_messages.push(msg);
                    message_handler.set(copy_messages.clone());
                }
                WSMessageType::UserList => {

                }
            }
        }
    });

    let input_handler_clone = text_input_handler.clone();
    let on_text_change = Callback::from(
        move |e:Event|{
            let target = e.target_dyn_into::<HtmlTextAreaElement>();
            if let Some(input) = target {
                input_handler_clone.set(input.value());
            } 
        }
    );

    let input = text_input.clone();
    let ws_clone = ws.clone();
    let input_handler_clone = text_input_handler.clone();
    let on_send =  Callback::from(
        move |_e:MouseEvent| { 
            ws_clone.send(input.clone());
            input_handler_clone.set(String::from(""));
        }
    );
    html! {
        <div id="chat-container" class="bg-gray-900 text-gray-200 min-h-screen flex flex-col items-center p-6 space-y-4">
            <h1 class="text-3xl font-bold text-white mb-6">{"Broadcast Chat"}</h1>
            <ul class="bg-gray-800 w-full max-w-lg rounded-lg shadow-lg mb-4 p-4 flex-grow overflow-auto space-y-2">
                {
                    messages.iter().map(|m| {
                        let profile_picture_url = format!("https://api.dicebear.com/8.x/pixel-art/svg?seed={}", m.user);
                        html! {
                            <li class="bg-gray-700 p-3 rounded-lg border border-gray-600 flex items-center space-x-3">
                                <img src={profile_picture_url} class="w-8 h-8 rounded-full" alt="Profile picture" />
                                <div>
                                    <p class="font-bold">{m.user.clone()}</p>
                                    <p>{m.msg.clone()}</p>
                                    <p class="text-xs text-gray-400">{m.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                                </div>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
            <textarea
                onchange={on_text_change}
                value={text_input}
                class="w-full max-w-lg mb-2 p-3 rounded-lg border border-gray-600 bg-gray-800 text-gray-200 placeholder-gray-400"
                placeholder="Type your message here..."
            ></textarea>
            <button
                type="submit"
                onclick={on_send}
                class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg shadow-lg focus:outline-none focus:ring-4 focus:ring-blue-400"
            >
                {"Send"}
            </button>
        </div>
    }    
}
fn main() {
    yew::Renderer::<App>::new().render();
}
