use leptos::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::transpiler::Transpiler;

#[component]
pub fn App() -> impl IntoView {
    let (input_data, set_input_data) = create_signal("".to_string());
    let (formatted_data, set_formatted_data) = create_signal("".to_string());
    view! {
        <main class="bg-black text-white">
            <div class="flex h-[100vh] items-center justify-around">
                <div class="flex flex-col h-[80vh] justify-center items-start">
                    <textarea on:input=move |e| {
                        set_input_data.update(|i| {
                            *i = event_target_value(&e);
                        });
                    } rows="8" placeholder="Enter JSON Here" class="border-2 text-black h-[50vh] w-[30vw] border-black" />
                    <button on:click=move |_e| {
                        let mut l = Lexer::new(input_data.get());
                        l.lex();
                        let mut p = Parser::new(&l.tokens);
                        p.parse();
                        if let Some(root) = p.root_object {
                            logging::log!("wow");
                            let t = Transpiler::new(&root);
                            let s = t.format();
                            if let Some(clipboard) = window().navigator().clipboard() {
                                let _ = clipboard.write_text(&s);
                            } else {
                                // handle the lack of clipboard!
                            }
                            logging::log!("{}", s);
                            set_formatted_data.update(move |v| {
                                *v = s;
                            });
                        } else {
                            set_formatted_data.update(move |v| {
                                *v = "Invalid JSON".to_string();
                            });
                        }
                    } class="p-8 bg-gray-500 w-full">Format and copy</button>
                            </div>
                        <div class="w-[30vw] h-[80vh] flex flex-col justify-center">
                        <pre class="text-sm overflow-scroll">{formatted_data}</pre>
                </div>
            </div>
        </main>
    }
}

