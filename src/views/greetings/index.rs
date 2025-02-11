use crate::views::layouts::MainLayout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewArgs {
    milliseconds: f64,
    total_rows: i32,
}

impl ViewArgs {
    pub fn new(milliseconds: f64, rows: i32) -> ViewArgs {
        ViewArgs {
            milliseconds,
            total_rows: rows,
        }
    }
}

#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {
    html! {
      <MainLayout>

        <div class="w-fill m-auto">

          <h1 class="text-4xl">{ format!("DB round trip: {} ms", args.milliseconds) }</h1>
          <h1 class="text-4xl">{ format!("Rows Loaded: {}", args.total_rows) }</h1>

        </div>

      </MainLayout>
    }
}
