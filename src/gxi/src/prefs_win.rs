use editview::MainState;
use gettextrs::gettext;
use gio::{SettingsBindFlags, SettingsExt};
use gtk::*;
use gxi_config_storage::{GSchema, GSchemaExt};
use gxi_peer::Core;
use log::{debug, trace};
use pango::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PrefsWin {
    pub core: Core,
    pub window: Window,
}

const SRC: &str = include_str!("ui/prefs_win.glade");

impl PrefsWin {
    pub fn new(
        parent: &ApplicationWindow,
        main_state: &Rc<RefCell<MainState>>,
        core: &Core,
        gschema: &GSchema,
    ) -> Self {
        let builder = Builder::new_from_string(SRC);

        let window: Window = builder.get_object("prefs_win").unwrap();
        let font_chooser_widget: FontChooserWidget =
            builder.get_object("font_chooser_widget").unwrap();
        let theme_combo_box: ComboBoxText = builder.get_object("theme_combo_box").unwrap();
        let tab_stops_checkbutton: ToggleButton =
            builder.get_object("tab_stops_checkbutton").unwrap();
        let word_wrap_checkbutton: ToggleButton =
            builder.get_object("word_wrap_checkbutton").unwrap();
        let draw_trailing_spaces_checkbutton: ToggleButton = builder
            .get_object("draw_trailing_spaces_checkbutton")
            .unwrap();
        let margin_checkbutton: ToggleButton = builder.get_object("margin_checkbutton").unwrap();
        let margin_spinbutton: SpinButton = builder.get_object("margin_spinbutton").unwrap();
        let highlight_line_checkbutton: ToggleButton =
            builder.get_object("highlight_line_checkbutton").unwrap();
        let tab_size_spinbutton: SpinButton = builder.get_object("tab_size_spinbutton").unwrap();

        let font_desc: &String = &gschema.get_key("font");
        font_chooser_widget.set_font_desc(&FontDescription::from_string(font_desc));

        {
            let main_state = main_state.borrow();
            for (i, theme_name) in main_state.themes.iter().enumerate() {
                theme_combo_box.append_text(theme_name);
                if &main_state.theme_name == theme_name {
                    trace!("{}: {}", gettext("Setting active theme"), i);
                    theme_combo_box.set_active(Some(i as u32));
                }
            }
        }

        theme_combo_box.connect_changed(enclose!((core, main_state, gschema) move |cb|{
            if let Some(theme_name) = cb.get_active_text() {
                let theme_name = theme_name.to_string();
                debug!("{} {}", gettext("Theme changed to"), &theme_name);
                core.set_theme(&theme_name);

                gschema.set_key("theme-name", theme_name.clone()).unwrap();

                let mut main_state = main_state.borrow_mut();
                main_state.theme_name = theme_name;
            }
        }));

        margin_checkbutton.connect_toggled(enclose!((margin_spinbutton) move |toggle_btn| {
            let value = toggle_btn.get_active();
            margin_spinbutton.set_sensitive(value);
        }));

        margin_spinbutton.set_sensitive(gschema.get_key("draw-right-margin"));

        gschema.settings.bind(
            "font",
            &font_chooser_widget,
            "font",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "word-wrap",
            &word_wrap_checkbutton,
            "active",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "use-tab-stops",
            &tab_stops_checkbutton,
            "active",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "draw-trailing-spaces",
            &draw_trailing_spaces_checkbutton,
            "active",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "draw-right-margin",
            &margin_checkbutton,
            "active",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "highlight-line",
            &highlight_line_checkbutton,
            "active",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "column-right-margin",
            &margin_spinbutton,
            "value",
            SettingsBindFlags::DEFAULT,
        );

        gschema.settings.bind(
            "tab-size",
            &tab_size_spinbutton,
            "value",
            SettingsBindFlags::DEFAULT,
        );

        let prefs_win = Self {
            core: core.clone(),
            window: window.clone(),
        };

        window.set_transient_for(Some(parent));
        window.show_all();

        prefs_win
    }
}
