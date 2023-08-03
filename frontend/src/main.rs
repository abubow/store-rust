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
        <div id="chat-container">
            <ul>
                {
                    messages.iter().map(|m| html!{<li>{m}</li>}).collect::<Html>()
                }
            </ul>
            <textarea onchange={on_text_change} value={text_input}></textarea>
            <button type="submit" onclick={on_send}>{"Send"}</button>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
