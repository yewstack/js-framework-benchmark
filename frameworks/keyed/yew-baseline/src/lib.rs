#![recursion_limit = "1024"]

use rand::prelude::*;
use std::cmp::min;
use yew::prelude::*;
use yew::web_sys::window;
use wasm_bindgen::prelude::wasm_bindgen;

static ADJECTIVES: &[&str] = &[
    "pretty",
    "large",
    "big",
    "small",
    "tall",
    "short",
    "long",
    "handsome",
    "plain",
    "quaint",
    "clean",
    "elegant",
    "easy",
    "angry",
    "crazy",
    "helpful",
    "mushy",
    "odd",
    "unsightly",
    "adorable",
    "important",
    "inexpensive",
    "cheap",
    "expensive",
    "fancy",
];

static COLOURS: &[&str] = &[
    "red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black",
    "orange",
];

static NOUNS: &[&str] = &[
    "table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger",
    "pizza", "mouse", "keyboard",
];

struct Row {
    id: usize,
    label: String,
}

impl Row {
    fn new(id: usize, rng: &mut SmallRng) -> Row {
        let mut label = String::new();
        label.push_str(ADJECTIVES.choose(rng).unwrap());
        label.push(' ');
        label.push_str(COLOURS.choose(rng).unwrap());
        label.push(' ');
        label.push_str(NOUNS.choose(rng).unwrap());

        Row { id, label }
    }
}

pub struct Model {
    rows: Vec<Row>,
    next_id: usize,
    selected_id: Option<usize>,
    rng: SmallRng,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Run(usize),
    Add(usize),
    Update(usize),
    Clear,
    Swap,
    Remove(usize),
    Select(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            rows: Vec::new(),
            next_id: 1,
            selected_id: None,
            rng: SmallRng::from_entropy(),
            link,
        }
    }

    fn change(&mut self, _: ()) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Run(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                let update_amount = min(amount, self.rows.len());
                for index in 0..update_amount {
                    self.rows[index] = Row::new(next_id + index, rng);
                }
                self.rows
                    .extend((update_amount..amount).map(|index| Row::new(next_id + index, rng)));
                self.next_id += amount;
            }
            Msg::Add(amount) => {
                let rng = &mut self.rng;
                let next_id = self.next_id;
                self.rows
                    .extend((0..amount).map(|index| Row::new(next_id + index, rng)));
                self.next_id += amount;
            }
            Msg::Update(step) => {
                for index in (0..self.rows.len()).step_by(step) {
                    self.rows[index].label += " !!!";
                }
            }
            Msg::Clear => {
                self.rows.clear();
            }
            Msg::Swap => {
                if self.rows.len() > 998 {
                    self.rows.swap(1, 998);
                }
            }
            Msg::Remove(id) => {
                if let Some(index) = self.rows.iter().position(|row| row.id == id) {
                    self.rows.remove(index);
                }
            }
            Msg::Select(id) => {
                self.selected_id = Some(id);
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="jumbotron">
                    <div class="row">
                        <div class="col-md-6">
                            <h1>{ "Yew" }</h1>
                        </div>
                        <div class="col-md-6">
                            <div class="row">
                                <div class="col-sm-6 smallpad">
                                    <button type="button" id="run" class="btn btn-primary btn-block" onclick=self.link.callback(|_| Msg::Run(1_000))>{ "Create 1,000 rows" }</button>
                                </div>
                                <div class="col-sm-6 smallpad">
                                    <button type="button" class="btn btn-primary btn-block" onclick=self.link.callback(|_| Msg::Run(10_000)) id="runlots">{ "Create 10,000 rows" }</button>
                                </div>
                                <div class="col-sm-6 smallpad">
                                    <button type="button" class="btn btn-primary btn-block" onclick=self.link.callback(|_| Msg::Add(1_000)) id="add">{ "Append 1,000 rows" }</button>
                                </div>
                                <div class="col-sm-6 smallpad">
                                    <button type="button" class="btn btn-primary btn-block" onclick=self.link.callback(|_| Msg::Update(10)) id="update">{ "Update every 10th row" }</button>
                                </div>
                                <div class="col-sm-6 smallpad">
                                    <button type="button" class="btn btn-primary btn-block" onclick=self.link.callback(|_| Msg::Clear) id="clear">{ "Clear" }</button>
                                </div>
                                <div class="col-sm-6 smallpad">
                                    <button type="button" class="btn btn-primary btn-block" onclick=self.link.callback(|_| Msg::Swap) id="swaprows">{ "Swap Rows" }</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <table class="table table-hover table-striped test-data">
                    <tbody id="tbody">
                        { for self.rows.iter().map(|row| {
                            let id = row.id.clone();
                            html! {
                                <tr class=if self.selected_id == Some(id) { "danger" } else  { "" }>
                                    <td class="col-md-1">{ id.to_string() }</td>
                                    <td class="col-md-4" onclick=self.link.callback(move |_| Msg::Select(id))>
                                        <a class="lbl">{ row.label.clone() }</a>
                                    </td>
                                    <td class="col-md-1">
                                        <a class="remove" onclick=self.link.callback(move |_| Msg::Remove(id))>
                                            <span class="glyphicon glyphicon-remove remove" aria-hidden="true"></span>
                                        </a>
                                    </td>
                                    <td class="col-md-6"></td>
                                </tr>
                            }
                        } ) }
                    </tbody>
                </table>
                <span class="preloadicon glyphicon glyphicon-remove" aria-hidden="true"></span>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = window().unwrap().document().unwrap();
    let mount_el = document.query_selector("#main").unwrap().unwrap();
    App::<Model>::new().mount(mount_el);
}
