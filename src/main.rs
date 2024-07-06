use yew::prelude::*;

type Notes = u16;

#[derive(Clone, Copy)]
enum Cell {
    Empty(Notes),
    Value(u8),
}

#[function_component]
fn Sudoku() -> Html {
    let selected_cell = use_state(|| None);
    let board = use_state(|| [Cell::Empty(0b111111111); 81] );

    let on_cell_click = {
        let selected_cell = selected_cell.clone();

        move |index: u8| {
            log::info!("Cell {} clicked", index);
            selected_cell.set(Some(index));
        }
    };

    html! {
        <div class="text-xl border border-gray-600 border-2 rounded grid grid-cols-9 grid-rows-9 gap-0" style="width: 27rem; height: 27rem;">
            {for (0..81).map(|index| {
                let cell_value = &board[index as usize];

                let on_cell_click = on_cell_click.clone();
                let selected_cell = selected_cell.clone();

                let is_selected = *selected_cell == Some(index);
                let affects_selected_cell = {
                    match *selected_cell {
                        Some(selected_cell) => {
                            let selected_row = selected_cell / 9;
                            let selected_col = selected_cell % 9;
                            let cell_row = index / 9;
                            let cell_col = index % 9;

                            selected_row == cell_row ||
                            selected_col == cell_col ||
                            (
                                selected_row / 3 == cell_row / 3 &&
                                selected_col / 3 == cell_col / 3
                            )
                        }
                        None => false
                    }
                };

                let cell_color = match (is_selected, affects_selected_cell) {
                    (true, _) => "bg-blue-200",
                    (_, true) => "bg-gray-200",
                    (_, false) => "bg-gray-100",
                };

                html! {
                    // Please allow me to write class={classes!("many classes in one string literal", cell_color)}
                    <button onclick={move |_| on_cell_click(index)} class={cell_color}>
                        {match cell_value {
                            Cell::Empty(notes) => html! {
                                <div class="text-xs grid grid-cols-3 grid-rows-3">
                                    {for (0..9).map(|i| {
                                        let is_set = notes & (1 << i) != 0;

                                        match is_set {
                                            true => html! { <p>{(i + 1).to_string()}</p> },
                                            false => html! { <p>{" "}</p> },
                                        }
                                    })}
                                </div>
                            },
                            Cell::Value(cell_value) => html! { <p>{cell_value.to_string()}</p> },
                        }}
                    </button>
                }
            })}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="container mx-auto">
            <h1 class="text-4xl font-bold text-center">{"Sudoku"}</h1>
            <Sudoku />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
