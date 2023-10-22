use gtk::prelude::*;
use gtk::{gio, glib, Application};
use std::fs::File;
use std::io::{BufRead, BufReader};

const APP_ID: &str = "org.manzana.lineas";
fn main() -> glib::ExitCode{
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");

    //Create the app
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| {
        let _ = adw::init();
    });
    app.connect_activate(build_ui);

    //Run the app i(idwin) dont speking english estube aqui jajajaja
    app.run()
}

fn build_ui(app: &Application){
    //Load the user interface
    let builder = gtk::Builder::from_resource("/org/manzana/lineas/window.ui");
    let window = builder
        .object::<gtk::Window>("window")
        .expect("Couldn't get window");
    let button = builder
        .object::<gtk::Button>("button1")
        .expect("Couldn't get button");git
    window.set_application(Some(app));
    let window_clone = window.clone();
    button.connect_clicked(move |_| {
        /*Create the FileDialog to open the file */
        let dialog = gtk::FileDialog::builder()
            .title("Seleccione el archivo de texto")
            .accept_label("Open")
            .build();
        /*Filters for type text on the FileDialog */
        let texto = builder
            .object::<gtk::Label>("texto")
            .expect("Couldn't get label");
        /*Add filters to let the user choose only text files */
        let filter = gtk::FileFilter::new();
        filter.add_mime_type("text/plain");
        filter.add_pattern("*.txt");
        filter.set_name(Some("Archivos de texto"));
        let list_filters = gio::ListStore::new::<gtk::FileFilter>();
        list_filters.append(&filter);
        dialog.set_filters(Some(&list_filters));
        dialog.set_default_filter(Some(&filter));
        /*Opens the dialog to let the user select a file*/
        dialog.open(Some(&window_clone), gio::Cancellable::NONE, move |file| {
            if let Ok(file) = file {
                let num_lineas = read_num_lines(file);
                /*Change the label to show the number of lines */
                let resultado = format!("El archivo tiene {} líneas de texto",num_lineas);
                texto.set_text(resultado.as_str());
            }
        });
    });
    //Present window
    window.present();
}

fn read_num_lines(file : gtk::gio::File) -> u32 {
     /*Counts the number of lines in that file */
    let filename = file.path().expect("No se pudo obtener la ruta");
    let file = File::open(&filename).expect("No se pudo obtener el archivo");
    let file = BufReader::new(file);
    let mut num_lineas = 0;
    for _lines in file.lines(){
        num_lineas += 1;
    }
    return num_lineas
}
