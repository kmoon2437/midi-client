use gtk::prelude::*;
use gtk::{
    glib, Application,
    ApplicationWindow, TextView, Grid,
    PositionType, ScrolledWindow, Label,
    Button, DropDown
};

const APP_ID: &str = "kr.choyunjin.MidiClient";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    return app.run();
}

const TEXTAREA_WIDTH: i32 = 300;
const TEXTAREA_HEIGHT: i32 = 500;

fn build_ui(app: &Application) {
    let input = TextView::builder()
        .monospace(true)
        .width_request(TEXTAREA_WIDTH)
        .height_request(TEXTAREA_HEIGHT)
        .build();
    let input_container = ScrolledWindow::builder()
        .width_request(TEXTAREA_WIDTH)
        .height_request(TEXTAREA_HEIGHT)
        .child(&input).build();
    let input_label = Label::new(Some("Midi input"));

    let input_dropdown = DropDown::builder().build();
    
    let output = TextView::builder()
        .monospace(true)
        .width_request(TEXTAREA_WIDTH)
        .height_request(TEXTAREA_HEIGHT)
        .build();
    let output_container = ScrolledWindow::builder()
        .width_request(TEXTAREA_WIDTH)
        .height_request(TEXTAREA_HEIGHT)
        .child(&output).build();
    let output_label = Label::new(Some("Midi output"));

    let send_button = Button::builder()
        .label("Send")
        .build();

    let grid = Grid::builder()
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .row_spacing(10)
        .column_spacing(10)
        .build();

    grid.attach(&input_container, 0, 0, 1, 1);
    grid.attach_next_to(&output_container, Some(&input_container), PositionType::Right, 1, 1);
    grid.attach_next_to(&input_label, Some(&input_container), PositionType::Top, 1, 1);
    grid.attach_next_to(&output_label, Some(&output_container), PositionType::Top, 1, 1);
    grid.attach_next_to(&send_button, Some(&output_container), PositionType::Bottom, 1, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Midi Client")
        .child(&grid)
        .resizable(false)
        .build();

    window.present();
}