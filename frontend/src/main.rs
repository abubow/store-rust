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
            copy_messages.push(ws_msg.clone());
            message_handler.set(copy_messages.clone());
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
    html!{
        <div id="chat-container" class="bg-gray-900 text-gray-200 min-h-screen flex flex-col items-center p-6 space-y-4">
            <h1 class="text-3xl font-bold text-white mb-6">{"Broadcast Chat"}</h1>
            <ul class="bg-gray-800 w-full max-w-lg rounded-lg shadow-lg mb-4 p-4 flex-grow overflow-auto space-y-2">
                {
                    messages.iter().map(|m| html! {
                        <li class="bg-gray-700 p-3 rounded-lg border border-gray-600">{m}</li>
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
