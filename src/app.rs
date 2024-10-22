use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let name = use_state(|| String::new());
    
    let oninput = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    html! {
        <div class="container">
            <div class="input-group">
                <input
                    type="text"
                    placeholder="กรุณาใส่ชื่อของคุณ"
                    value={(*name).clone()}
                    {oninput}
                />
                {if !(*name).is_empty() {
                    html! {
                        <div class="greeting">
                            { format!("สวัสดี {}", *name) }
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}