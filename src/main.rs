use yew::prelude::*;
use bmp_rust::bmp::BMP;
use gloo_console::log;
use wasm_bindgen::JsValue;

mod start;
use start::Start;
mod create_load;
use create_load::Create;

#[derive(PartialEq, Properties, Default)]
pub struct Props;

//App

pub enum AppMessage {
  Create,
  Load,
  NewBMP(BMP),
}

pub struct App {
  current_bmp: Option<BMP>,
  show_create: bool,
}

impl Component for App {
  type Message = AppMessage;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self { current_bmp: None, show_create: false }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Self::Message::Create => {
        //Create
        self.show_create = true;
        true
      },
      Self::Message::Load => {
        true
      },
      Self::Message::NewBMP(bmp_inside) => {
        self.show_create = false;
        self.current_bmp = Some(bmp_inside);
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let link = ctx.link().clone();
    let link2 = ctx.link().clone();
    //let mut from_scratch = false;

    let create_load_process = move |from_scratch: bool| {
      if from_scratch {
        link.send_message(Self::Message::Create);
      } else {
        //
      }
    };
  
    let create_load_callback = Callback::from(move |from_scratch_passed: bool| {
      create_load_process(from_scratch_passed);
    });

    let send_bmp_process = move |new_bmp: BMP| {
      log!("yes", new_bmp.contents.len());
      link2.send_message(Self::Message::NewBMP(new_bmp))
      //new_bmp so emit NewBMP message
    };
  
    let send_bmp_callback = Callback::from(move |new_bmp: BMP| {
      send_bmp_process(new_bmp);
    });
  
    html! {
      <div>
        <Start {create_load_callback} />
        <Create {send_bmp_callback} show={self.show_create} />
      </div>
    }
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}