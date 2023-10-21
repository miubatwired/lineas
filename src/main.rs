use gtk::prelude::*;
use gtk::{gio, glib, Application};
use gtk::ApplicationWindow;
use gtk::Button;
use std::fs::File;
use std::io::{BufRead, BufReader};

const APP_ID: &str = "org.gtk_rs.lineas";
fn main() -> glib::ExitCode{
    //Crear la aplicación
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| {
        let _ = adw::init();
    });
    app.connect_activate(build_ui);
    //Ejecutar la aplicación i(idwin) dont speking english estube aqui jajajaja
    app.run()
}

fn build_ui(app: &Application){
     let button = Button::builder()
        .label("Seleccionar Archivo")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let label = gtk::Text::new();
    label.set_placeholder_text(Some("Hola"));
    //Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Líneas")
        .child(&button)
        .build();

    let window_clone = window.clone();
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
            //Build the FileDialog object
            let dialog = gtk::FileDialog::builder()
                .title("Seleccione el archivo de texto")
                .accept_label("Open")
                .build();
            /*Filters for type text on the FileDialog */
            let filter = gtk::FileFilter::new();
            filter.add_mime_type("text/plain");
            filter.add_pattern("*.txt");
            filter.set_name(Some("Archivos de texto"));
            let list_filters = gio::ListStore::new::<gtk::FileFilter>();
            list_filters.append(&filter);
            dialog.set_filters(Some(&list_filters));
            let button_clone = button.clone();
            dialog.open(Some(&window_clone), gio::Cancellable::NONE, move |file| {
                    if let Ok(file) = file {
                        let filename = file.path().expect("No se pudo obtener la ruta");
                        let file = File::open(filename).expect("No se pudo obtener el archivo");
                        let file = BufReader::new(file);
                        let mut num_lineas = 0;
                        for _lines in file.lines(){
                            num_lineas += 1;
                        }
                        button_clone.set_label((num_lineas.to_string()).as_str());
                    }
            });
        }
    );
    //Present window
    window.present();
}
