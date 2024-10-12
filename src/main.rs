use gtk4::{
    prelude::*,
    Application, ApplicationWindow, Box, Button, ComboBoxText, Orientation, TextView, Frame, Image,
    Settings, FileChooserAction, FileChooserDialog, ResponseType, Label,
};
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};
use fast_qr::qr::QRBuilder;
use std::path::Path;

/*

TODO:

- add support for saving in svg
- make qr code colors customizable
- get the app theme from system theme
- optimize

*/

fn main() {
    let app = Application::builder()
        .application_id("dev.aapelix.qr")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1500)
            .default_height(900)
            .title("aapelix/qr")
            .resizable(true)
            .build();

        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_start(20);
        hbox.set_margin_end(20);
        hbox.set_margin_top(20);
        hbox.set_margin_bottom(20);

        let vbox = Box::new(Orientation::Vertical, 10);
        vbox.set_hexpand(true);
        vbox.set_vexpand(false);

        let qr_entry = TextView::new();
        qr_entry.set_hexpand(true);
        qr_entry.set_vexpand(true);

        let qr_gen_btn = Button::with_label("Generate QR");
        let save_to_file_btn = Button::with_label("Save to File");
        save_to_file_btn.set_visible(false);

        let shape_selector = ComboBoxText::new();
        shape_selector.append_text("Square");
        shape_selector.append_text("RoundedSquare");
        shape_selector.append_text("Circle");
        shape_selector.append_text("Diamond");
        shape_selector.append_text("Horizontal");
        shape_selector.append_text("Vertical");
        shape_selector.set_active(Some(0));

        let theme_selector = ComboBoxText::new();
        theme_selector.append_text("System Default");
        theme_selector.append_text("Light");
        theme_selector.append_text("Dark");

        theme_selector.set_active(Some(0));

        vbox.append(&qr_entry);
        vbox.append(&shape_selector);
        vbox.append(&theme_selector);
        vbox.append(&qr_gen_btn);
        vbox.append(&save_to_file_btn);

        let qr_frame = Frame::new(Some("QR Code Display"));
        qr_frame.set_hexpand(true);
        qr_frame.set_vexpand(true);

        let qr_image = Image::new();
        qr_image.set_hexpand(true);
        qr_image.set_vexpand(true);
        let disclaimer_label = Label::new(Some("Note: The saved QR codes will always have black data dots for optimal scanning."));
        disclaimer_label.set_margin_top(10);

        let qr_vbox = Box::new(Orientation::Vertical, 5);
        qr_vbox.append(&qr_image);
        qr_vbox.append(&disclaimer_label);
        qr_frame.set_child(Some(&qr_vbox));

        hbox.append(&vbox);
        hbox.append(&qr_frame);

        window.set_child(Some(&hbox));

        let update_theme = |theme: &str| {
            let settings = Settings::default().unwrap();
            match theme {
                "Dark" => settings.set_gtk_application_prefer_dark_theme(true),
                "Light" => settings.set_gtk_application_prefer_dark_theme(false),
                _ => settings.reset_property("gtk-application-prefer-dark-theme"),
            }
        };

        theme_selector.connect_changed(move |combo| {
            if let Some(theme) = combo.active_text() {
                update_theme(&theme);
            }
        });

        let save_to_file_btn_clone = save_to_file_btn.clone();
        qr_gen_btn.connect_clicked(move |_| {
            let buf = qr_entry.buffer();
            let text = buf.text(&buf.start_iter(), &buf.end_iter(), false);

            if text.is_empty() {
                return;
            }

            let selected_shape = match shape_selector.active_text().as_deref() {
                Some("Square") => Shape::Square,
                Some("RoundedSquare") => Shape::Circle,
                Some("Circle") => Shape::RoundedSquare,
                Some("Diamond") => Shape::Diamond,
                Some("Horizontal") => Shape::Horizontal,
                Some("Vertical") => Shape::Vertical,
                _ => Shape::Square,
            };

            let qr = QRBuilder::new(text).build().unwrap();

            let settings = Settings::default().unwrap();
            let dark_theme_active = settings.is_gtk_application_prefer_dark_theme();

            if dark_theme_active {
                let _img = ImageBuilder::default()
                    .shape(selected_shape)
                    .background_color([0, 0, 0, 255])
                    .module_color([255, 255, 255, 255])
                    .fit_width(1000)
                    .to_file(&qr, "out_inverted.png");

                let _img2 = ImageBuilder::default()
                    .shape(selected_shape)
                    .background_color([255, 255, 255, 0])
                    .fit_width(1000)
                    .to_file(&qr, "out.png");

                qr_image.set_from_file(Some(Path::new("out_inverted.png")));
            } else {
                let _img = ImageBuilder::default()
                    .shape(selected_shape)
                    .background_color([255, 255, 255, 0])
                    .fit_width(1000)
                    .to_file(&qr, "out.png");

                qr_image.set_from_file(Some(Path::new("out.png")));
            }

            save_to_file_btn_clone.set_visible(true);
        });

        let window_clone = window.clone();
        save_to_file_btn.connect_clicked(move |_| {
            let file_chooser = FileChooserDialog::new(
                Some("Save File"),
                Some(&window_clone),
                FileChooserAction::Save,
                &[("_Cancel", ResponseType::Cancel), ("_Save", ResponseType::Accept)]
            );

            file_chooser.set_current_name("qr_code.png");

            file_chooser.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(filename) = dialog.file() {
                        let dest_path = filename.path().unwrap();

                        if let Err(e) = std::fs::copy("out.png", &dest_path) {
                            eprintln!("Failed to save file: {}", e);
                        } else {
                            let _ = std::fs::remove_file("out.png");
                        }
                    }
                }
                dialog.close();
            });
            file_chooser.show();
        });

        window.present();
    });

    app.run();
}
