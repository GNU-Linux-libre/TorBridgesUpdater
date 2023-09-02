/*
 * This is free and unencumbered software released into the public domain.
 *
 * Anyone is free to copy, modify, publish, use, compile, sell, or
 * distribute this software, either in source code form or as a compiled
 * binary, for any purpose, commercial or non-commercial, and by any
 * means.
 *
 * In jurisdictions that recognize copyright laws, the author or authors
 * of this software dedicate any and all copyright interest in the
 * software to the public domain. We make this dedication for the benefit
 * of the public at large and to the detriment of our heirs and
 * successors. We intend this dedication to be an overt act of
 * relinquishment in perpetuity of all present and future rights to this
 * software under copyright law.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * For more information, please refer to <http://unlicense.org/>
*/

#![windows_subsystem = "windows"]

mod locale;
mod appsettings;

use std::io::prelude::*;
use std::thread;
use std::{fs::OpenOptions, path::{Path, PathBuf}};
use std::time::{Duration, SystemTime};

use chrono::prelude::*;
use regex::Regex;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{TextBuffer, Application, ApplicationWindow, ResponseType, Image, Picture, FileChooserAction, FileChooserDialog, ScrollablePolicy, Adjustment, AboutDialog, ScrolledWindow, TextView, ComboBoxText, SpinButton, Entry, Button, Grid, Label, ProgressBar, Box, ToggleButton, CheckButton, Orientation, Align};
use gtk::{gdk::Texture, gdk_pixbuf::Pixbuf};
use gtk::{gio, gio::{MemoryInputStream, Cancellable}};
use gtk::{glib, glib::{Bytes, clone}};

const APP_ID: &str = "com.yakovlevegor.TorBridgesUpdater";
const APP_VERSION: &str = "0.1.4";
const APP_ICON: &'static [u8] = include_bytes!("../icons/logo.png");

const ABOUT_AUTHOR: &str = "Egor Yakovlev (yakovlevegor)";
const ABOUT_WEBSITE: &str = "gitlab.com/yakovlevegor/TorBridgesUpdater";

const TOR_NOTICE: &'static [u8] = include_bytes!("../tor_notice.txt");
const LICENSE: &'static [u8] = include_bytes!("../LICENSE");

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0";
const BRIDGES_URL: &str = "https://bridges.torproject.org/bridges/";
const BRIDGES_URL_ONION: &str = "http://yq5jjvr7drkjrelzhut7kgclfuro65jjlivyzfmxiq2kyv5lickrl4qd.onion/bridges/";

type RequestResult = Result<(String, String), i64>;

use base64::Engine as _;

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|app| {
        let app_settings = appsettings::AppSettings::load();
        let app_settings_backup = appsettings::AppSettings::new();
        app_settings_backup.load_from(&app_settings);

        app.connect_activate(clone!(@weak app, @strong app_settings, @strong app_settings_backup => move |_| {
            if app_settings.property::<bool>("backgroundmode") == true {
                let appwindows = app.windows();

                for wind in appwindows {
                    wind.destroy();
                }

                app_settings.set_property("backgroundmode", false);

/* Initialize windows start */

                let mainwindow = ApplicationWindow::builder()
                    .application(&app)
                    .title(locale::get_translation().APP_TITLE)
                    .default_width(320)
                    .default_height(200)
                    .modal(true)
                    .hide_on_close(true)
                    .build();


                let pict_buf = Pixbuf::from_read(APP_ICON).unwrap();
                let pict_texture = Texture::for_pixbuf(&pict_buf);
                let icon_load = Image::builder().paintable(&pict_texture).pixel_size(64).valign(Align::Center).build();
                let label_text: Label = Label::builder().valign(Align::Center).label(locale::get_translation().MAIN_WINDOW_GET_BRIDGES_BUTTON).build();
                let label_timer: Label = Label::builder().valign(Align::End).use_markup(true).build();
                let label_timer_until: Label = Label::builder().valign(Align::End).label(locale::get_translation().MAIN_WINDOW_UNTIL_RETRIEVAL).build();
                let label_timer_last_retrieval: Label = Label::builder().valign(Align::End).label(&(locale::get_translation().LAST_RETRIEVAL_TIME.to_string() + ": <" + locale::get_translation().RETRIEVAL_TIME_UNKNOWN + ">")).margin_bottom(5).build();
                let progressload: ProgressBar = ProgressBar::builder().valign(Align::End).fraction(0.5).margin_bottom(3).build();

                let timer_visible_update = clone!(@strong app_settings, @weak label_timer, @weak label_timer_until, @weak progressload => move || {
                    label_timer.set_visible(app_settings.property::<bool>("notifications"));
                    label_timer_until.set_visible(app_settings.property::<bool>("notifications"));
                    progressload.set_visible(app_settings.property::<bool>("notifications"));
                });

                timer_visible_update();

                let lastretrtime = app_settings.property::<i64>("time");
                if lastretrtime != 0 {
                    label_timer_last_retrieval.set_text(&(locale::get_translation().LAST_RETRIEVAL_TIME.to_string() + ": " + &chrono::Local.timestamp_opt(lastretrtime as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string().as_str()));
                }
                let icon_settings_button = Image::builder().valign(Align::Center).icon_name("emblem-system-symbolic").build();
                let label_settings_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().MAIN_WINDOW_BUTTON_SETTINGS).margin_start(5).build();
                let settings_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                settings_button_display.append(&icon_settings_button);
                settings_button_display.append(&label_settings_button);
                let settings_button: Button = Button::builder().valign(Align::End).halign(Align::Start).vexpand(false).hexpand(false).build();
                settings_button.set_child(Some(&settings_button_display));
                let icon_about_button = Image::builder().valign(Align::Center).icon_name("help-about-symbolic").build();
                let label_about_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().MAIN_WINDOW_BUTTON_ABOUT).margin_start(5).build();
                let about_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                about_button_display.append(&icon_about_button);
                about_button_display.append(&label_about_button);
                let about_button: Button = Button::builder().halign(Align::Start).build();
                about_button.set_child(Some(&about_button_display));
                let icon_quit_button = Image::builder().valign(Align::Center).icon_name("application-exit-symbolic").build();
                let label_quit_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().MAIN_WINDOW_BUTTON_QUIT).margin_start(5).build();
                let quit_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                quit_button_display.append(&icon_quit_button);
                quit_button_display.append(&label_quit_button);
                let quit_button: Button = Button::builder().halign(Align::End).build();
                quit_button.set_child(Some(&quit_button_display));
                let boxapp = Box::builder().orientation(Orientation::Vertical).homogeneous(false).vexpand(true).hexpand(true).margin_top(5).margin_bottom(5).margin_start(5).margin_end(5).build();
                let boxtop = Box::builder().orientation(Orientation::Vertical).homogeneous(false).vexpand(true).build();
                let boxbottom = Box::builder().orientation(Orientation::Horizontal).homogeneous(true).vexpand(true).hexpand(true).build();
                let boxbottomright = Box::builder().orientation(Orientation::Horizontal).homogeneous(true).vexpand(false).hexpand(false).valign(Align::End).halign(Align::End).build();
                let buttongetbridges = Button::builder().valign(Align::Center).vexpand(true).margin_bottom(5).build();
                let button_display: Box = Box::builder().halign(Align::Center).hexpand(true).vexpand(true).build();
                button_display.append(&icon_load);
                button_display.append(&label_text);
                buttongetbridges.set_child(Some(&button_display));

                boxtop.append(&buttongetbridges);
                boxtop.append(&progressload);
                boxtop.append(&label_timer);
                boxtop.append(&label_timer_until);
                boxtop.append(&label_timer_last_retrieval);
                boxbottomright.append(&about_button);
                boxbottomright.append(&quit_button);
                boxbottom.append(&settings_button);
                boxbottom.append(&boxbottomright);
                boxapp.append(&boxtop);
                boxapp.append(&boxbottom);

                boxapp.show();

                mainwindow.set_child(Some(&boxapp));

                mainwindow.connect_hide(clone!(@strong app_settings, @weak app => move |_| {
                    app_settings.set_property("backgroundmode", true);

                    #[cfg(not(target_os = "macos"))]
                    {
                        if !app_settings.property::<bool>("runinbackground") || !app_settings.property::<bool>("notifications") {
                            app.quit();
                        }
                    }

                    #[cfg(target_os = "macos")]
                    {
                        app.quit();
                    }
                }));

                let notify_activate = gio::SimpleAction::new("activate", None);
                notify_activate.connect_activate(clone!(@weak app => move |_, _| {
                    app.activate();
                }));

                app.add_action(&notify_activate);

                let action_quit = gio::SimpleAction::new("quit", None);
                action_quit.connect_activate(clone!(@weak app => move |_, _| {
                    app.quit();
                }));

                app.add_action(&action_quit);

                app.set_accels_for_action("app.quit", &["<Primary>Q"]);

                about_button.connect_clicked(move |_| {
                    let input_bytesabout = Bytes::from(APP_ICON);
                    let memory_input_streamabout = MemoryInputStream::from_bytes(&input_bytesabout);
                    let about_icon = Pixbuf::from_stream(&memory_input_streamabout, Cancellable::NONE).unwrap();
                    let dialog_logo = Image::from_pixbuf(Some(&about_icon));
                    let aboutwindow = AboutDialog::builder().title(locale::get_translation().ABOUT_WINDOW_TITLE).logo(&dialog_logo.paintable().unwrap()).program_name(locale::get_translation().APP_TITLE).comments(locale::get_translation().ABOUT_WINDOW_DESCRIPTION).authors(vec![String::from(ABOUT_AUTHOR)]).website(&("https://".to_string() + ABOUT_WEBSITE)).website_label(ABOUT_WEBSITE).version(APP_VERSION).copyright(&String::from_utf8(TOR_NOTICE.to_vec()).unwrap()).license(&String::from_utf8(LICENSE.to_vec()).unwrap()).modal(true).hide_on_close(false).build();
                    aboutwindow.show();
                });

                quit_button.connect_clicked(clone!(@weak app => move |_| {
                    app.quit();
                }));

                buttongetbridges.grab_focus();
                mainwindow.show();


                settings_button.connect_clicked(clone!(@weak app, @strong app_settings, @strong app_settings_backup => move |_| {
                    app_settings_backup.load_from(&app_settings);

/* Settings window start */

                    let label_days: Label = Label::builder().halign(Align::End).margin_end(5).label("days").build();
                    let label_hours: Label = Label::builder().halign(Align::End).margin_end(5).label("hours").build();
                    let label_minutes: Label = Label::builder().halign(Align::End).margin_end(5).label("minutes").build();
                    let label_seconds: Label = Label::builder().halign(Align::End).margin_end(5).label("seconds").build();

                    let days_entry = SpinButton::builder().digits(0).adjustment(&Adjustment::new(0.0, 0.0, 364.0, 1.0, 1.0, 1.0)).build();
                    let hours_entry = SpinButton::builder().digits(0).adjustment(&Adjustment::new(0.0, 0.0, 23.0, 1.0, 1.0, 1.0)).build();
                    let minutes_entry = SpinButton::builder().digits(0).adjustment(&Adjustment::new(0.0, 0.0, 59.0, 1.0, 1.0, 1.0)).build();
                    let seconds_entry = SpinButton::builder().digits(0).adjustment(&Adjustment::new(0.0, 0.0, 59.0, 1.0, 1.0, 1.0)).build();

                    let boxtimer = Box::builder().orientation(Orientation::Vertical).homogeneous(true).vexpand(false).hexpand(true).valign(Align::Center).margin_bottom(15).margin_top(5).build();

                    let gridtimer = Grid::builder().vexpand(false).hexpand(true).row_homogeneous(true).column_homogeneous(true).row_spacing(10).valign(Align::Center).build();

                    let torrcfilechoose = Box::builder().orientation(Orientation::Horizontal).valign(Align::Center).homogeneous(false).hexpand(false).margin_bottom(5).build();
                    let torrcfileentry = Entry::builder().editable(false).hexpand(true).build();
                    let icon_torrc_button = Image::builder().valign(Align::Center).icon_name("document-open-symbolic").build();
                    let label_torrc_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().OPEN_FILE).margin_start(5).build();
                    let torrc_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    torrc_button_display.append(&icon_torrc_button);
                    torrc_button_display.append(&label_torrc_button);
                    let torrcbutton = Button::builder().margin_start(5).build();
                    torrcbutton.set_child(Some(&torrc_button_display));
                    torrcfilechoose.append(&torrcfileentry);
                    torrcfilechoose.append(&torrcbutton);

                    let bridgesfilechoose = Box::builder().orientation(Orientation::Horizontal).valign(Align::Center).homogeneous(false).build();
                    bridgesfilechoose.set_margin_bottom(5);
                    let bridgesfileentry = Entry::builder().editable(false).hexpand(true).build();
                    let icon_bridges_button = Image::builder().valign(Align::Center).icon_name("document-open-symbolic").build();
                    let label_bridges_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().OPEN_FILE).margin_start(5).build();
                    let bridges_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    bridges_button_display.append(&icon_bridges_button);
                    bridges_button_display.append(&label_bridges_button);
                    let bridgesbutton = Button::builder().margin_start(5).build();
                    bridgesbutton.set_child(Some(&bridges_button_display));
                    bridgesfilechoose.append(&bridgesfileentry);
                    bridgesfilechoose.append(&bridgesbutton);

                    let checkipv6 = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_IPV6).tooltip_text(locale::get_translation().SETTINGS_WINDOW_IPV6_TOOLTIP).halign(Align::Center).build();
                    let disableold = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_TORRC_DISABLE_OLD).tooltip_text(locale::get_translation().SETTINGS_WINDOW_TORRC_DISABLE_OLD_TOOLTIP).halign(Align::Center).build();
                    let keepold = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_BRIDGES_KEEP_OLD).tooltip_text(locale::get_translation().SETTINGS_WINDOW_BRIDGES_KEEP_OLD_TOOLTIP).halign(Align::Center).build();

                    let icon_shownotifications_check = Image::builder().vexpand(true).icon_name("alarm-symbolic").build();
                    let label_shownotifications_check: Label = Label::builder().valign(Align::Center).label(locale::get_translation().SETTINGS_WINDOW_NOTIFICATIONS_SHOW).margin_start(5).build();
                    let shownotifications_check_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    shownotifications_check_display.append(&icon_shownotifications_check);
                    shownotifications_check_display.append(&label_shownotifications_check);

                    let shownotifications = ToggleButton::builder().label(locale::get_translation().SETTINGS_WINDOW_NOTIFICATIONS_SHOW).halign(Align::Center).valign(Align::End).vexpand(true).margin_bottom(15).build();
                    shownotifications.set_child(Some(&shownotifications_check_display));

                    let runinbackground = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_RUN_BACKGROUND).halign(Align::Center).valign(Align::Start).margin_top(5).build();
                    let updbrdgs = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_TORRC_MODIFY).halign(Align::Center).build();
                    let savbrdgs = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_BRIDGES_SAVE).halign(Align::Center).build();
                    let useproxy = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_PROXY).halign(Align::Center).build();
                    let getonion = CheckButton::builder().label(locale::get_translation().SETTINGS_WINDOW_CONNECT_ONION).tooltip_text(locale::get_translation().SETTINGS_WINDOW_CONNECT_ONION_TOOLTIP).halign(Align::Center).build();

                    let boxappsettings = Box::builder().orientation(Orientation::Vertical).homogeneous(false).margin_top(5).margin_bottom(5).margin_start(5).margin_end(5).build();

                    gridtimer.attach(&Box::new(Orientation::Horizontal, 0), 0, 0, 1, 1);
                    gridtimer.attach(&label_days, 1, 0, 4, 1);
                    gridtimer.attach(&days_entry, 5, 0, 8, 1);
                    gridtimer.attach(&label_hours, 13, 0, 4, 1);
                    gridtimer.attach(&hours_entry, 17, 0, 8, 1);
                    gridtimer.attach(&Box::new(Orientation::Horizontal, 0), 25, 0, 1, 1);
                    gridtimer.attach(&Box::new(Orientation::Horizontal, 0), 0, 1, 1, 1);
                    gridtimer.attach(&label_minutes, 1, 1, 4, 1);
                    gridtimer.attach(&minutes_entry, 5, 1, 8, 1);
                    gridtimer.attach(&label_seconds, 13, 1, 4, 1);
                    gridtimer.attach(&seconds_entry, 17, 1, 8, 1);
                    gridtimer.attach(&Box::new(Orientation::Horizontal, 0), 25, 1, 1, 1);

                    let boxgridsettings = Box::builder().orientation(Orientation::Vertical).homogeneous(false).vexpand(true).valign(Align::Center).build();

                    let gridsettings = Grid::builder().vexpand(false).hexpand(true).row_homogeneous(true).column_homogeneous(true).row_spacing(5).valign(Align::Center).build();

                    let boxtorrc = Box::builder().orientation(Orientation::Vertical).valign(Align::Center).vexpand(false).homogeneous(false).build();

                    let boxbridgetype = Box::builder().orientation(Orientation::Vertical).valign(Align::Center).vexpand(false).homogeneous(false).build();

                    let boxproxy = Box::builder().orientation(Orientation::Horizontal).hexpand(true).homogeneous(false).margin_bottom(5).build();

                    let boxproxyvert = Box::builder().orientation(Orientation::Vertical).valign(Align::Center).vexpand(false).hexpand(true).homogeneous(false).build();

                    let boxsavefile = Box::builder().orientation(Orientation::Vertical).valign(Align::Center).homogeneous(false).build();

                    let label_bridgetype: Label = Label::builder().halign(Align::Center).label(locale::get_translation().SETTINGS_WINDOW_BRIDGES_TRANSPORT).build();

                    let proxytype_button = ComboBoxText::builder().build();
                    proxytype_button.append_text("http");
                    proxytype_button.append_text("socks5");
                    proxytype_button.set_active(Some(0));

                    let proxy_host = Entry::builder().width_chars(10).text("127.0.0.1").hexpand(true).margin_start(5).build();

                    let proxy_port = SpinButton::builder().digits(0).adjustment(&Adjustment::new(0.0, 0.0, 65536.0, 1.0, 1.0, 1.0)).margin_start(5).build();

                    boxproxy.append(&proxytype_button);
                    boxproxy.append(&proxy_host);
                    boxproxy.append(&proxy_port);

                    boxproxyvert.append(&boxproxy);
                    boxproxyvert.append(&getonion);

                    let bridgetype_button = ComboBoxText::new();
                    bridgetype_button.append_text("obfs4");
                    bridgetype_button.append_text("none");
                    bridgetype_button.set_active(Some(0));
                    bridgetype_button.set_margin_bottom(5);

                    boxsavefile.append(&bridgesfilechoose);
                    boxsavefile.append(&keepold);

                    boxtorrc.append(&torrcfilechoose);
                    boxtorrc.append(&disableold);

                    boxbridgetype.append(&bridgetype_button);
                    boxbridgetype.append(&checkipv6);

                    gridsettings.attach(&updbrdgs, 0, 0, 1, 1);
                    gridsettings.attach(&boxtorrc, 1, 0, 1, 1);
                    gridsettings.attach(&savbrdgs, 0, 1, 1, 1);
                    gridsettings.attach(&boxsavefile, 1, 1, 1, 1);
                    gridsettings.attach(&useproxy, 0, 2, 1, 1);
                    gridsettings.attach(&boxproxyvert, 1, 2, 1, 1);
                    gridsettings.attach(&label_bridgetype, 0, 3, 1, 1);
                    gridsettings.attach(&boxbridgetype, 1, 3, 1, 1);

                    let settings_window_buttons_box = Box::builder().orientation(Orientation::Horizontal).homogeneous(true).halign(Align::End).valign(Align::End).margin_top(5).build();
                    let icon_settings_apply_button = Image::builder().valign(Align::Center).icon_name("emblem-ok-symbolic").build();
                    let label_settings_apply_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().FORM_APPLY).margin_start(5).build();
                    let settings_apply_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    settings_apply_button_display.append(&icon_settings_apply_button);
                    settings_apply_button_display.append(&label_settings_apply_button);
                    let settings_apply = Button::builder().label(locale::get_translation().FORM_APPLY).hexpand(false).margin_start(5).build();
                    settings_apply.set_child(Some(&settings_apply_button_display));

                    label_days.set_text(&locale::get_days(app_settings.property::<i64>("days") as i32).to_string());
                    label_hours.set_text(&locale::get_hours(app_settings.property::<i64>("hours") as i32).to_string());
                    label_minutes.set_text(&locale::get_minutes(app_settings.property::<i64>("minutes") as i32).to_string());
                    label_seconds.set_text(&locale::get_seconds(app_settings.property::<i64>("seconds") as i32).to_string());

                    days_entry.set_value(app_settings.property::<i64>("days") as f64);
                    hours_entry.set_value(app_settings.property::<i64>("hours") as f64);
                    minutes_entry.set_value(app_settings.property::<i64>("minutes") as f64);
                    seconds_entry.set_value(app_settings.property::<i64>("seconds") as f64);

                    updbrdgs.set_active(app_settings.property::<bool>("savetorrc"));
                    torrcfileentry.set_text(&app_settings.property::<String>("savetorrcpath"));
                    torrcfileentry.set_sensitive(app_settings.property::<bool>("savetorrc"));
                    torrcbutton.set_sensitive(app_settings.property::<bool>("savetorrc"));
                    disableold.set_active(app_settings.property::<bool>("torrcdisableold"));
                    disableold.set_sensitive(app_settings.property::<bool>("savetorrc"));

                    savbrdgs.set_active(app_settings.property::<bool>("savebridges"));
                    bridgesfileentry.set_text(&app_settings.property::<String>("savebridgespath"));
                    bridgesfileentry.set_sensitive(app_settings.property::<bool>("savebridges"));
                    bridgesbutton.set_sensitive(app_settings.property::<bool>("savebridges"));
                    keepold.set_active(app_settings.property::<bool>("keepold"));
                    keepold.set_sensitive(app_settings.property::<bool>("savebridges"));

                    bridgetype_button.set_active(Some(app_settings.property::<i64>("transport") as u32));
                    checkipv6.set_active(app_settings.property::<bool>("ipv6"));

                    useproxy.set_active(app_settings.property::<bool>("useproxy"));
                    proxytype_button.set_active(Some(app_settings.property::<i64>("proxytype") as u32));
                    proxytype_button.set_sensitive(app_settings.property::<bool>("useproxy"));
                    proxy_host.set_text(&app_settings.property::<String>("proxyhost"));
                    proxy_host.set_sensitive(app_settings.property::<bool>("useproxy"));
                    proxy_port.set_value(app_settings.property::<i64>("proxyport") as f64);
                    proxy_port.set_sensitive(app_settings.property::<bool>("useproxy"));
                    getonion.set_active(app_settings.property::<bool>("proxyonion"));
                    getonion.set_sensitive(app_settings.property::<bool>("useproxy"));

                    gridtimer.set_visible(app_settings.property::<bool>("notifications"));
                    shownotifications.set_active(app_settings.property::<bool>("notifications"));
                    runinbackground.set_active(app_settings.property::<bool>("runinbackground"));
                    #[cfg(not(target_os = "macos"))]
                    {
                        runinbackground.set_visible(app_settings.property::<bool>("notifications"));
                    }
                    #[cfg(target_os = "macos")]
                    {
                        runinbackground.set_visible(false);
                    }
                    settings_apply.set_sensitive(false);

                    let icon_settings_submit_button = Image::builder().valign(Align::Center).icon_name("emblem-ok-symbolic").build();
                    let label_settings_submit_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().FORM_OK).margin_start(5).build();
                    let settings_submit_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    settings_submit_button_display.append(&icon_settings_submit_button);
                    settings_submit_button_display.append(&label_settings_submit_button);
                    let settings_submit = Button::builder().label(locale::get_translation().FORM_OK).hexpand(false).margin_start(5).build();
                    settings_submit.set_child(Some(&settings_submit_button_display));

                    let icon_settings_cancel_button = Image::builder().valign(Align::Center).icon_name("window-close-symbolic").build();
                    let label_settings_cancel_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().FORM_CANCEL).margin_start(5).build();
                    let settings_cancel_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    settings_cancel_button_display.append(&icon_settings_cancel_button);
                    settings_cancel_button_display.append(&label_settings_cancel_button);
                    let settings_cancel = Button::builder().label(locale::get_translation().FORM_OK).hexpand(false).margin_start(5).build();
                    settings_cancel.set_child(Some(&settings_cancel_button_display));

                    settings_window_buttons_box.append(&settings_submit);
                    settings_window_buttons_box.append(&settings_apply);
                    settings_window_buttons_box.append(&settings_cancel);

                    boxtimer.append(&shownotifications);
                    boxtimer.append(&gridtimer);
                    boxtimer.append(&runinbackground);

                    boxgridsettings.append(&boxtimer);
                    boxgridsettings.append(&gridsettings);

                    boxappsettings.append(&boxgridsettings);
                    boxappsettings.append(&settings_window_buttons_box);

                    boxappsettings.show();

                    let settingswindow = ApplicationWindow::builder()
                        .application(&app)
                        .title(locale::get_translation().SETTINGS_WINDOW_TITLE)
                        .default_width(400)
                        .default_height(200)
                        .modal(true)
                        .hide_on_close(false)
                        .build();

                    let timer_values_zero = clone!(@weak days_entry, @weak hours_entry, @weak minutes_entry, @weak seconds_entry => @default-return false, move || {
                        (days_entry.value() + hours_entry.value() + minutes_entry.value() + seconds_entry.value()) == 0.0
                    });

                    let changed_settings = clone!(@strong app_settings, @weak days_entry, @weak hours_entry, @weak minutes_entry, @weak seconds_entry, @weak updbrdgs, @weak torrcfileentry, @weak disableold, @weak savbrdgs, @weak bridgesfileentry, @weak keepold, @weak bridgetype_button, @weak checkipv6, @weak useproxy, @weak proxytype_button, @weak proxy_host, @weak proxy_port, @weak getonion, @weak shownotifications, @weak runinbackground, @weak settings_apply => move || {

                        if days_entry.value() == app_settings.property::<i64>("days") as f64 &&
                            hours_entry.value() == app_settings.property::<i64>("hours") as f64 &&
                            minutes_entry.value() == app_settings.property::<i64>("minutes") as f64 &&
                            seconds_entry.value() == app_settings.property::<i64>("seconds") as f64 &&

                            updbrdgs.is_active() == app_settings.property::<bool>("savetorrc") &&
                            torrcfileentry.text() == app_settings.property::<String>("savetorrcpath") &&
                            disableold.is_active() == app_settings.property::<bool>("torrcdisableold") &&

                            savbrdgs.is_active() == app_settings.property::<bool>("savebridges") &&
                            bridgesfileentry.text() == app_settings.property::<String>("savebridgespath") &&
                            keepold.is_active() == app_settings.property::<bool>("keepold") &&

                            bridgetype_button.active().unwrap() == app_settings.property::<i64>("transport") as u32 &&
                            checkipv6.is_active() == app_settings.property::<bool>("ipv6") &&

                            useproxy.is_active() == app_settings.property::<bool>("useproxy") &&
                            proxytype_button.active().unwrap() == app_settings.property::<i64>("proxytype") as u32 &&
                            proxy_host.text() == app_settings.property::<String>("proxyhost") &&
                            proxy_port.value() == app_settings.property::<i64>("proxyport") as f64 &&
                            getonion.is_active() == app_settings.property::<bool>("proxyonion") &&

                            shownotifications.is_active() == app_settings.property::<bool>("notifications") &&
                            runinbackground.is_active() == app_settings.property::<bool>("runinbackground") {

                            settings_apply.set_sensitive(false);

                        } else {
                            settings_apply.set_sensitive(true);
                        }
                    });


                    let apply_settings = clone!(@strong app_settings, @strong app_settings_backup, @weak days_entry, @weak hours_entry, @weak minutes_entry, @weak seconds_entry, @weak updbrdgs, @weak torrcfileentry, @weak disableold, @weak savbrdgs, @weak bridgesfileentry, @weak keepold, @weak bridgetype_button, @weak checkipv6, @weak useproxy, @weak proxytype_button, @weak proxy_host, @weak proxy_port, @weak getonion, @weak shownotifications, @weak runinbackground, @weak settings_apply, @strong timer_visible_update => move || {

                        app_settings.set_property("days", days_entry.value() as i64);
                        app_settings.set_property("hours", hours_entry.value() as i64);
                        app_settings.set_property("minutes", minutes_entry.value() as i64);
                        app_settings.set_property("seconds", seconds_entry.value() as i64);

                        app_settings.set_property("savetorrc", updbrdgs.is_active());
                        app_settings.set_property("savetorrcpath", torrcfileentry.text());
                        app_settings.set_property("torrcdisableold", disableold.is_active());

                        app_settings.set_property("savebridges", savbrdgs.is_active());
                        app_settings.set_property("savebridgespath", bridgesfileentry.text());
                        app_settings.set_property("keepold", keepold.is_active());

                        app_settings.set_property("transport", bridgetype_button.active().unwrap() as i64);
                        app_settings.set_property("ipv6", checkipv6.is_active());

                        app_settings.set_property("useproxy", useproxy.is_active());
                        app_settings.set_property("proxytype", proxytype_button.active().unwrap() as i64);
                        app_settings.set_property("proxyhost", proxy_host.text());
                        app_settings.set_property("proxyport", proxy_port.value() as i64);
                        app_settings.set_property("proxyonion", getonion.is_active());

                        app_settings.set_property("notifications", shownotifications.is_active());
                        app_settings.set_property("runinbackground", runinbackground.is_active());

                        settings_apply.set_sensitive(false);

                        if days_entry.value() != app_settings_backup.property::<i64>("days") as f64 ||
                            hours_entry.value() != app_settings_backup.property::<i64>("hours") as f64 ||
                            minutes_entry.value() != app_settings_backup.property::<i64>("minutes") as f64 ||
                            seconds_entry.value() != app_settings_backup.property::<i64>("seconds") as f64 {

                            app_settings.set_property("time", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64);
                        }

                        timer_visible_update();
                    });

                    let revert_settings = clone!(@strong app_settings, @strong app_settings_backup => move || {
                        app_settings_backup.set_property("time", app_settings.property::<i64>("time"));
                        app_settings.load_from(&app_settings_backup);
                    });

                    settings_apply.connect_clicked(clone!(@strong app_settings, @weak settings_apply, @strong apply_settings => move |_| {
                        apply_settings();
                        app_settings.save();
                        settings_apply.set_sensitive(false);
                    }));

                    settings_submit.connect_clicked(clone!(@strong app_settings, @weak settingswindow, @strong apply_settings => move |_| {
                        apply_settings();
                        app_settings.save();
                        settingswindow.destroy();
                    }));

                    settings_cancel.connect_clicked(clone!(@strong app_settings, @weak settingswindow, @strong revert_settings, @strong timer_visible_update => move |_| {
                        revert_settings();
                        app_settings.save();
                        timer_visible_update();
                        settingswindow.destroy();
                    }));

                    days_entry.connect_value_changed(clone!(@weak label_days, @weak days_entry, @strong timer_values_zero => move |_| {
                        if timer_values_zero() {
                            days_entry.set_value(1.0);
                        }
                        label_days.set_text(&locale::get_days(days_entry.value() as i32).to_string());
                    }));

                    days_entry.connect_value_changed(clone!(@strong changed_settings => move |_| changed_settings()));

                    hours_entry.connect_value_changed(clone!(@weak label_hours, @weak hours_entry, @strong timer_values_zero => move |_| {
                        if timer_values_zero() {
                            hours_entry.set_value(1.0);
                        }
                        label_hours.set_text(&locale::get_hours(hours_entry.value() as i32).to_string());
                    }));

                    hours_entry.connect_value_changed(clone!(@strong changed_settings => move |_| changed_settings()));

                    minutes_entry.connect_value_changed(clone!(@weak label_minutes, @weak minutes_entry, @strong timer_values_zero => move |_| {
                        if timer_values_zero() {
                            minutes_entry.set_value(1.0);
                        }
                        label_minutes.set_text(&locale::get_minutes(minutes_entry.value() as i32).to_string());
                    }));

                    minutes_entry.connect_value_changed(clone!(@strong changed_settings => move |_| changed_settings()));

                    seconds_entry.connect_value_changed(clone!(@weak label_seconds, @weak seconds_entry, @strong timer_values_zero => move |_| {
                        if timer_values_zero() {
                            seconds_entry.set_value(1.0);
                        }
                        label_seconds.set_text(&locale::get_seconds(seconds_entry.value() as i32).to_string());
                    }));

                    seconds_entry.connect_value_changed(clone!(@strong changed_settings => move |_| changed_settings()));

                    updbrdgs.connect_toggled(clone!(@weak updbrdgs, @weak torrcfileentry, @weak torrcbutton, @weak disableold => move |_| {
                        torrcfileentry.set_sensitive(updbrdgs.is_active());
                        torrcbutton.set_sensitive(updbrdgs.is_active());
                        disableold.set_sensitive(updbrdgs.is_active());
                    }));

                    updbrdgs.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    torrcbutton.connect_clicked(clone!(@weak settingswindow, @strong changed_settings, @weak torrcfileentry => move |_| {
                        let dialog_open = FileChooserDialog::new(Some(locale::get_translation().TORRC_FILE_SELECT), Some(&settingswindow), FileChooserAction::Open, &[(locale::get_translation().FORM_CANCEL, ResponseType::Cancel), (locale::get_translation().OPEN_FILE, ResponseType::Ok)]);
                        dialog_open.set_modal(true);

                        let mut dirpath = "".to_string();

                        if let Some(homedir) = dirs::home_dir() {
                            dirpath = homedir.to_str().unwrap().to_string();
                        };

                        if let Ok(_) = dialog_open.set_current_folder(Some(&gio::File::for_path(Path::new(&dirpath)))) {
                            dialog_open.connect_response(clone!(@weak torrcfileentry, @strong changed_settings => move |dialog, response| {

                                match response {
                                    ResponseType::Ok => {
                                        if let Some(dialogfilename) = dialog.file() {
                                            if let Some(filepath) = dialogfilename.path() {
                                                if filepath.is_file() {
                                                    torrcfileentry.set_text(filepath.as_path().to_str().unwrap());
                                                }
                                            }
                                        }
                                        changed_settings();
                                    },
                                    _ => ()
                                }
                                dialog.close();
                            }));
                        }
                        dialog_open.show();
                    }));

                    disableold.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    savbrdgs.connect_toggled(clone!(@weak savbrdgs, @weak bridgesfileentry, @weak bridgesbutton, @weak keepold => move |_| {
                        bridgesfileentry.set_sensitive(savbrdgs.is_active());
                        bridgesbutton.set_sensitive(savbrdgs.is_active());
                        keepold.set_sensitive(savbrdgs.is_active());
                    }));
                    savbrdgs.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    bridgesbutton.connect_clicked(clone!(@weak settingswindow, @strong changed_settings, @weak bridgesfileentry => move |_| {
                        let dialog_open = FileChooserDialog::new(Some(locale::get_translation().BRIDGES_FILE_SELECT), Some(&settingswindow), FileChooserAction::Save, &[(locale::get_translation().FORM_CANCEL, ResponseType::Cancel), (locale::get_translation().SAVE_FILE, ResponseType::Ok)]);
                        dialog_open.set_modal(true);
                        dialog_open.set_current_name("bridges.txt");

                        let mut dirpath = "".to_string();

                        if let Some(homedir) = dirs::home_dir() {
                            dirpath = homedir.to_str().unwrap().to_string();
                        };

                        if let Ok(_) = dialog_open.set_current_folder(Some(&gio::File::for_path(Path::new(&dirpath)))) {
                            dialog_open.connect_response(clone!(@weak bridgesfileentry, @strong changed_settings => move |dialog, response| {

                                match response {
                                    ResponseType::Ok => {
                                        if let Some(dialogfilename) = dialog.file() {
                                            if let Some(filepath) = dialogfilename.path() {
                                                if let Some(dirparent) = filepath.as_path().parent() {
                                                    if dirparent.is_dir() {
                                                        bridgesfileentry.set_text(filepath.as_path().to_str().unwrap());
                                                    }
                                                } else if filepath.as_path().is_file() {
                                                    bridgesfileentry.set_text(filepath.as_path().to_str().unwrap());
                                                }
                                            }
                                        }
                                        changed_settings();
                                    },
                                    _ => ()
                                }
                                dialog.close();
                            }));
                        }
                        dialog_open.show();
                    }));

                    keepold.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    useproxy.connect_toggled(clone!(@weak useproxy, @weak proxytype_button, @weak proxy_host, @weak proxy_port, @weak getonion => move |_| {
                        proxytype_button.set_sensitive(useproxy.is_active());
                        proxy_host.set_sensitive(useproxy.is_active());
                        proxy_port.set_sensitive(useproxy.is_active());
                        getonion.set_sensitive(useproxy.is_active());
                    }));
                    useproxy.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    proxytype_button.connect_changed(clone!(@strong changed_settings => move |_| changed_settings()));
                    proxy_host.connect_changed(clone!(@strong changed_settings => move |_| changed_settings()));
                    proxy_port.connect_value_changed(clone!(@strong changed_settings => move |_| changed_settings()));
                    getonion.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    bridgetype_button.connect_changed(clone!(@strong changed_settings => move |_| changed_settings()));
                    checkipv6.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));

                    shownotifications.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));
                    shownotifications.connect_toggled(clone!(@weak shownotifications, @weak runinbackground, @weak gridtimer, @weak label_shownotifications_check => move |_| {
                        if shownotifications.is_active() {
                            label_shownotifications_check.set_label(&(locale::get_translation().SETTINGS_WINDOW_NOTIFICATIONS_SHOW_EVERY.to_string() + ":"));
                        } else {
                            label_shownotifications_check.set_label(locale::get_translation().SETTINGS_WINDOW_NOTIFICATIONS_SHOW);
                        }
                        gridtimer.set_visible(shownotifications.is_active());
                        #[cfg(not(target_os = "macos"))]
                        {
                            runinbackground.set_visible(shownotifications.is_active());
                        }
                        #[cfg(target_os = "macos")]
                        {
                            runinbackground.set_visible(false);
                        }
                    }));

                    runinbackground.connect_toggled(clone!(@strong changed_settings => move |_| changed_settings()));
/* Settings window end */


                    settingswindow.set_child(Some(&boxappsettings));

                    settingswindow.show();
                }));

                buttongetbridges.connect_clicked(clone!(@weak app, @strong app_settings, @weak label_timer_last_retrieval, @weak label_timer_until => move |_| {
                    let retrieval_new = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

                    app_settings.set_property("time", retrieval_new as i64);
                    app_settings.set_property("lastretrievaltime", retrieval_new as i64);
                    app_settings.save();
                    label_timer_last_retrieval.set_text(&(locale::get_translation().LAST_RETRIEVAL_TIME.to_string() + ": " + &chrono::Local.timestamp_opt(retrieval_new as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string().as_str()));

                    label_timer_until.set_label(locale::get_translation().MAIN_WINDOW_UNTIL_RETRIEVAL);


/* Captcha window start */


                    let boxcaptcha = Box::builder().orientation(Orientation::Vertical).homogeneous(false).vexpand(true).hexpand(true).margin_top(5).margin_bottom(5).margin_start(5).margin_end(5).build();

                    let boxcaptchaimage = Box::builder().orientation(Orientation::Vertical).homogeneous(true).vexpand(true).hexpand(true).width_request(400).height_request(125).build();

                    let boxcaptchainput = Box::builder().orientation(Orientation::Horizontal).homogeneous(false).build();

                    let boxcaptchainputvert = Box::builder().orientation(Orientation::Vertical).homogeneous(false).vexpand(false).valign(Align::End).build();

                    let captcha_load = Picture::builder().can_shrink(true).vexpand(true).hexpand(false).build();

                    let captcha_loading_message = gtk::Label::new(Some(locale::get_translation().CAPTCHA_WINDOW_LOADING_CAPTCHA));

                    let captcha_loading_error_message: Label = Label::builder().label(locale::get_translation().LOADING_BRIDGES_ERROR).valign(Align::End).build();

                    let captcha_input = Entry::builder().hexpand(true).margin_top(5).build();
                    let captcha_window_buttons_box = Box::builder().orientation(Orientation::Horizontal).homogeneous(true).halign(Align::End).margin_top(5).build();

                    let icon_captcha_retry_button = Image::builder().valign(Align::Center).icon_name("view-refresh-symbolic").build();
                    let label_captcha_retry_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().FORM_RETRY).margin_start(5).build();
                    let captcha_retry_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    captcha_retry_button_display.append(&icon_captcha_retry_button);
                    captcha_retry_button_display.append(&label_captcha_retry_button);
                    let captcha_retry = Button::builder().label(locale::get_translation().FORM_RETRY).hexpand(false).margin_start(5).build();
                    captcha_retry.set_child(Some(&captcha_retry_button_display));

                    let icon_captcha_submit_button = Image::builder().valign(Align::Center).icon_name("emblem-ok-symbolic").build();
                    let label_captcha_submit_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().FORM_OK).margin_start(5).build();
                    let captcha_submit_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    captcha_submit_button_display.append(&icon_captcha_submit_button);
                    captcha_submit_button_display.append(&label_captcha_submit_button);
                    let captcha_submit = Button::builder().label(locale::get_translation().FORM_OK).hexpand(false).margin_start(5).build();
                    captcha_submit.set_child(Some(&captcha_submit_button_display));

                    let icon_captcha_cancel_button = Image::builder().valign(Align::Center).icon_name("window-close-symbolic").build();
                    let label_captcha_cancel_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().FORM_CANCEL).margin_start(5).build();
                    let captcha_cancel_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                    captcha_cancel_button_display.append(&icon_captcha_cancel_button);
                    captcha_cancel_button_display.append(&label_captcha_cancel_button);
                    let captcha_cancel = Button::builder().label(locale::get_translation().FORM_CANCEL).hexpand(false).margin_start(5).build();
                    captcha_cancel.set_child(Some(&captcha_cancel_button_display));

                    boxcaptchainput.append(&captcha_input);
                    captcha_window_buttons_box.append(&captcha_submit);
                    captcha_window_buttons_box.append(&captcha_retry);
                    captcha_window_buttons_box.append(&captcha_cancel);

                    boxcaptchaimage.append(&captcha_loading_message);
                    boxcaptchaimage.append(&captcha_load);
                    boxcaptchainputvert.append(&boxcaptchainput);
                    boxcaptchainputvert.append(&captcha_window_buttons_box);

                    boxcaptcha.append(&boxcaptchaimage);
                    boxcaptcha.append(&captcha_loading_error_message);
                    boxcaptcha.append(&boxcaptchainputvert);

                    boxcaptcha.show();


                    let captchawindow = ApplicationWindow::builder()
                        .application(&app)
                        .title(locale::get_translation().CAPTCHA_WINDOW_TITLE)
                        .default_width(400)
                        .default_height(250)
                        .modal(true)
                        .hide_on_close(false)
                        .build();

                    captcha_load.connect_paintable_notify(clone!(@weak captcha_load, @weak captcha_loading_message => move |_| {
                        captcha_loading_message.hide();
                        captcha_load.show();
                    }));

                    let channelcaptcha: (gtk::glib::Sender<RequestResult>, gtk::glib::Receiver<RequestResult>) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

                    channelcaptcha.1.attach(None, clone!(@strong app_settings, @weak captchawindow, @weak captcha_load, @weak captcha_loading_message => @default-return glib::Continue(false), move |captcha_result| {
                        app_settings.set_property("captchaloading", false);
                        if captchawindow.is_visible() {
                            match captcha_result {
                                Ok(captcha) =>  {

                                    let outbuf = base64::engine::general_purpose::STANDARD.decode(captcha.0.clone()).unwrap();

                                    let input_bytes = Bytes::from(&outbuf);

                                    let memory_input_stream = MemoryInputStream::from_bytes(&input_bytes);

                                    let captcha_pixbuf = Pixbuf::from_stream(&memory_input_stream, Cancellable::NONE).unwrap();

                                    let pict_texture_captcha = Texture::for_pixbuf(&captcha_pixbuf);

                                    captcha_load.set_paintable(Some(&pict_texture_captcha));

                                    app_settings.set_property("captchaid", &captcha.1.clone());


                                }
                                Err(error) => {
                                    let mut captcha_message = locale::get_translation().LOADING_CAPTCHA_ERROR.to_string() + ": ";
                                    match error {
                                        0 => {
                                            captcha_message.push_str(locale::get_translation().LOADING_ERROR);
                                        },
                                        1 => {
                                            captcha_message.push_str(locale::get_translation().LOADING_CAPTCHA_NOTFOUND);
                                        },
                                        _ => ()
                                    }
                                    captcha_load.hide();
                                    captcha_loading_message.set_text(&captcha_message);
                                    captcha_loading_message.show();
                                }
                            }
                        }
                        glib::Continue(true)
                    }));

                    let load_captcha = clone!(@strong app_settings, @weak captcha_load, @weak captcha_loading_message, @weak captcha_input => @default-return glib::Continue(false), move || {
                        if app_settings.property::<bool>("captchaloading") == false {
                            let captchachannelsend = channelcaptcha.0.clone();

                            let transport_arg = app_settings.property::<i64>("transport");
                            let ipv6_arg = app_settings.property::<bool>("ipv6");
                            let use_proxy_arg = app_settings.property::<bool>("useproxy");
                            let proxy_type_arg = app_settings.property::<i64>("proxytype");
                            let proxy_host_arg = app_settings.property::<String>("proxyhost");
                            let proxy_port_arg = app_settings.property::<i64>("proxyport");
                            let proxy_onion_arg = app_settings.property::<bool>("proxyonion");

                            app_settings.set_property("captchaloading", true);

                            captcha_load.hide();
                            captcha_loading_message.set_text(locale::get_translation().CAPTCHA_LOADING_MESSAGE);
                            captcha_loading_message.show();
                            captcha_input.set_text("");

                            thread::spawn(move || {
                                captchachannelsend.send(get_captcha(transport_arg, ipv6_arg, use_proxy_arg, proxy_type_arg, proxy_host_arg, proxy_port_arg, proxy_onion_arg)).expect("Error receiving bridges");
                            });
                        }

                        glib::Continue(false)
                    });

                    captcha_retry.connect_clicked(clone!(@weak captcha_loading_error_message, @strong load_captcha => move |_| {
                        captcha_loading_error_message.hide();
                        load_captcha();
                    }));

                    captcha_cancel.connect_clicked(clone!(@weak captchawindow => move |_| {
                        captchawindow.destroy();
                    }));

                    let channelbridges: (gtk::glib::Sender<RequestResult>, gtk::glib::Receiver<RequestResult>) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

                    channelbridges.1.attach(None, clone!(@weak app, @strong app_settings, @weak captchawindow, @weak captcha_load, @weak captcha_loading_message, @weak captcha_input, @weak captcha_loading_error_message, @strong load_captcha, @weak captcha_retry, @weak captcha_submit, @weak captcha_cancel => @default-return glib::Continue(false), move |bridges_result| {
                        if captchawindow.is_visible() {
                            match bridges_result {
                                Ok(bridges) =>  {
                                    captchawindow.destroy();
                                    let bridges_output_text_buffer = TextBuffer::new(None);
                                    let bridges_output_text = TextView::builder().buffer(&bridges_output_text_buffer).editable(false).vexpand(true).hexpand(true).vscroll_policy(ScrollablePolicy::Natural).hscroll_policy(ScrollablePolicy::Natural).build();
                                    bridges_output_text.grab_focus();

/* Bridges window start */
                                    let channelclipboardmessage: (gtk::glib::Sender<bool>, gtk::glib::Receiver<bool>) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

                                    let label_clipboard_copied: Label = Label::builder().halign(Align::Center).label(locale::get_translation().BRIDGES_WINDOW_CLIPBOARD_COPIED).build();

                                    channelclipboardmessage.1.attach(None, clone!(@weak label_clipboard_copied => @default-return glib::Continue(false), move |_| {
                                        label_clipboard_copied.hide();
                                        glib::Continue(true)
                                    }));

                                    let icon_copy_clipboard_button = Image::builder().valign(Align::Center).icon_name("edit-copy-symbolic").build();
                                    let label_copy_clipboard_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().BRIDGES_WINDOW_CLIPBOARD_COPY).margin_start(5).build();
                                    let copy_clipboard_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                                    copy_clipboard_button_display.append(&icon_copy_clipboard_button);
                                    copy_clipboard_button_display.append(&label_copy_clipboard_button);
                                    let button_copy_clipboard: Button = Button::builder().hexpand(false).margin_end(5).halign(Align::Center).build();
                                    button_copy_clipboard.set_child(Some(&copy_clipboard_button_display));

                                    button_copy_clipboard.connect_clicked(clone!(@weak bridges_output_text, @strong bridges_output_text_buffer, @weak label_clipboard_copied => move |_| {
                                        let clipboardmessagehide = channelclipboardmessage.0.clone();
                                        let clipboard = bridges_output_text.clipboard();
                                        bridges_output_text_buffer.select_range(&bridges_output_text_buffer.start_iter(), &bridges_output_text_buffer.end_iter());
                                        bridges_output_text_buffer.copy_clipboard(&clipboard);
                                        label_clipboard_copied.show();
                                        thread::spawn(move || {
                                            thread::sleep(Duration::from_secs(5));
                                            clipboardmessagehide.send(true).unwrap();
                                        });
                                    }));

                                    let icon_show_qrcode_button = Image::builder().valign(Align::Center).icon_name("emblem-photos-symbolic").build();
                                    let label_show_qrcode_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().BRIDGES_WINDOW_SHOW_QR_CODE).margin_start(5).build();
                                    let show_qrcode_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                                    show_qrcode_button_display.append(&icon_show_qrcode_button);
                                    show_qrcode_button_display.append(&label_show_qrcode_button);

                                    let button_show_qrcode: Button = Button::builder().hexpand(false).margin_start(5).halign(Align::Center).build();
                                    button_show_qrcode.set_child(Some(&show_qrcode_button_display));

                                    button_show_qrcode.connect_clicked(clone!(@weak app, @strong app_settings => move |_| {
                                        let boxqrcode = Box::builder().orientation(Orientation::Vertical).homogeneous(false).margin_top(5).margin_bottom(5).margin_start(5).margin_end(5).build();

                                        let qr_load = Picture::builder().can_shrink(false).vexpand(true).hexpand(true).build();

                                        let icon_qr_save_button = Image::builder().valign(Align::Center).icon_name("document-save-symbolic").build();
                                        let label_qr_save_button: Label = Label::builder().valign(Align::Center).label(locale::get_translation().QRCODE_WINDOW_SAVE_AS_IMAGE).margin_start(5).build();
                                        let qr_save_button_display: Box = Box::builder().halign(Align::Center).hexpand(false).vexpand(false).build();
                                        qr_save_button_display.append(&icon_qr_save_button);
                                        qr_save_button_display.append(&label_qr_save_button);
                                        let qr_save_button = Button::builder().hexpand(false).halign(Align::Center).build();
                                        qr_save_button.set_child(Some(&qr_save_button_display));

                                        boxqrcode.append(&qr_load);
                                        boxqrcode.append(&qr_save_button);

                                        boxqrcode.show();

                                        let qr_text = app_settings.property::<String>("qrcodetext");

                                        let outbuf = base64::engine::general_purpose::STANDARD.decode(qr_text.clone()).unwrap();

                                        let input_bytes = Bytes::from(&outbuf);

                                        let memory_input_stream = MemoryInputStream::from_bytes(&input_bytes);

                                        let qr_pixbuf = Pixbuf::from_stream(&memory_input_stream, Cancellable::NONE).unwrap();

                                        let pict_texture_qrcode = Texture::for_pixbuf(&qr_pixbuf);

                                        qr_load.set_paintable(Some(&pict_texture_qrcode));

                                        qr_load.set_width_request(qr_pixbuf.width());

                                        qr_load.set_height_request(qr_pixbuf.height());
/* QR code window start */
                                        let qrcodewindow = ApplicationWindow::builder()
                                            .application(&app)
                                            .title(locale::get_translation().QRCODE_WINDOW_TITLE)
                                            .default_width(400)
                                            .default_height(200)
                                            .modal(true)
                                            .hide_on_close(false)
                                            .build();

                                        qr_save_button.connect_clicked(clone!(@strong app_settings, @weak qrcodewindow => move |_| {
                                            let dialog_open = FileChooserDialog::new(Some(locale::get_translation().BRIDGES_FILE_SELECT), Some(&qrcodewindow), FileChooserAction::Save, &[(locale::get_translation().FORM_CANCEL, ResponseType::Cancel), (locale::get_translation().SAVE_FILE, ResponseType::Ok)]);
                                            dialog_open.set_modal(true);
                                            let now_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                                            dialog_open.set_current_name(&("bridges_qrcode_".to_string() + &chrono::Local.timestamp_opt(now_time as i64, 0).unwrap().format("%Y-%m-%d_%H_%M_%S").to_string().as_str() + ".jpg"));

                                            let mut dirpath = "".to_string();

                                            if let Some(homedir) = dirs::home_dir() {
                                                dirpath = homedir.to_str().unwrap().to_string();
                                            };

                                            if let Ok(_) = dialog_open.set_current_folder(Some(&gio::File::for_path(Path::new(&dirpath)))) {
                                                dialog_open.connect_response(clone!(@strong app_settings => move |dialog, response| {

                                                    match response {
                                                        ResponseType::Ok => {
                                                            if let Some(dialogfilename) = dialog.file() {
                                                                if let Some(filepath) = dialogfilename.path() {
                                                                    if let Some(pathstem) = filepath.as_path().parent() {
                                                                        if pathstem.exists() {
                                                                            let mut imageqr = OpenOptions::new().write(true).create(true).open(filepath.as_path()).unwrap();

                                                                            let qr_text = app_settings.property::<String>("qrcodetext");

                                                                            let outbufqr = base64::engine::general_purpose::STANDARD.decode(qr_text.clone()).unwrap();

                                                                            imageqr.write_all(outbufqr.as_slice()).unwrap();

                                                                            imageqr.sync_all().unwrap();
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        _ => ()
                                                    }
                                                    dialog.close();
                                                }));
                                            }
                                            dialog_open.show();
                                        }));
/* QR code window end */

                                        qrcodewindow.set_child(Some(&boxqrcode));

                                        qrcodewindow.show();
                                    }));
/* Bridges window end */

                                    let bridgeswindow = ApplicationWindow::builder()
                                       .application(&app)
                                       .title(locale::get_translation().BRIDGES_WINDOW_TITLE)
                                       .default_width(400)
                                       .default_height(200)
                                       .modal(true)
                                       .hide_on_close(false)
                                       .build();


                                    let boxbridges = Box::builder().orientation(Orientation::Vertical).homogeneous(false).margin_top(5).margin_bottom(5).margin_start(5).margin_end(5).build();

                                    let boxbridgesbuttons = Box::builder().orientation(Orientation::Horizontal).homogeneous(false).halign(Align::Center).build();

                                    let bridges_output_window = ScrolledWindow::new();






                                    bridges_output_window.set_child(Some(&bridges_output_text));

                                    boxbridgesbuttons.append(&button_copy_clipboard);
                                    boxbridgesbuttons.append(&button_show_qrcode);

                                    boxbridges.append(&bridges_output_window);
                                    boxbridges.append(&label_clipboard_copied);
                                    boxbridges.append(&boxbridgesbuttons);

                                    boxbridges.show();

                                    label_clipboard_copied.hide();



                                    bridgeswindow.set_child(Some(&boxbridges));
                                    bridgeswindow.show();

                                    bridges_output_text_buffer.set_text(&bridges.1.clone());

                                    app_settings.set_property("qrcodetext", &bridges.0.clone());

                                    if app_settings.property::<bool>("savebridges") {
                                        let bridgespath = app_settings.property::<String>("savebridgespath");

                                        let mut bridgesfilegetcontentsdata = String::new();

                                        if Path::new(&bridgespath).exists() {
                                            let mut bridgesfilegetcontents = OpenOptions::new().read(true).open(&bridgespath).unwrap();

                                            bridgesfilegetcontents.read_to_string(&mut bridgesfilegetcontentsdata).unwrap();
                                        }


                                        let brdg_entries_str = bridges.1.clone();
                                        let mut brdg_entries = brdg_entries_str.split("\n\n").collect::<Vec<&str>>();

                                        brdg_entries.retain(|x| !bridgesfilegetcontentsdata.contains(x));

                                        if brdg_entries.len() > 0 {
                                            let mut bridgestypeload = "none".to_string();

                                            if app_settings.property::<i64>("transport") == 0 {
                                                bridgestypeload = "obfs4".to_string();
                                            }

                                            let bridgestimestamp = chrono::Local.timestamp_opt(app_settings.property::<i64>("time"), 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();

                                            let bridges_contents = "#Retrieved on ".to_string() + &bridgestimestamp + ". Bridges type - " + &bridgestypeload + "\n\n" + &brdg_entries.join("\n\n") + "\n\n\n";

                                            if let Some(pathstem) = Path::new(&bridgespath).parent() {
                                                if pathstem.exists() {
                                                    if app_settings.property::<bool>("keepold") {
                                                        let mut bridgesfile = OpenOptions::new().write(true).create(true).append(true).open(&bridgespath).unwrap();

                                                        bridgesfile.write_all(bridges_contents.as_bytes()).unwrap();

                                                        bridgesfile.sync_all().unwrap();
                                                    } else {
                                                        let mut bridgesfile = OpenOptions::new().write(true).create(true).truncate(true).open(&bridgespath).unwrap();

                                                        bridgesfile.write_all(bridges_contents.as_bytes()).unwrap();

                                                        bridgesfile.sync_all().unwrap();
                                                    }

                                                }
                                            }
                                        }
                                    }

                                    if app_settings.property::<bool>("savetorrc") {
                                        let torrcpath = app_settings.property::<String>("savetorrcpath");

                                        let mut torrcfilegetcontentsdata = String::new();

                                        if Path::new(&torrcpath).exists() {
                                            let mut torrcfilegetcontents = OpenOptions::new().read(true).open(&torrcpath).unwrap();

                                            torrcfilegetcontents.read_to_string(&mut torrcfilegetcontentsdata).unwrap();
                                        }


                                        let torrc_entries_str = bridges.1.clone();
                                        let mut torrc_entries = torrc_entries_str.split("\n\n").collect::<Vec<&str>>();

                                        torrc_entries.retain(|x| !torrcfilegetcontentsdata.contains(x));

                                        if torrc_entries.len() > 0 {

                                            let bridges_torrc_contents = "\n\nBridge ".to_string() + &torrc_entries.join("\n\nBridge ") + "\n";

                                            let pathfile = PathBuf::from(torrcpath);
                                            if pathfile.as_path().exists() {

                                                match OpenOptions::new().write(true).append(true).open(pathfile.as_path()) {
                                                    Ok(mut bridgesfile) => {

                                                        if app_settings.property::<bool>("torrcdisableold") {

                                                            let mut filetorrctext = OpenOptions::new().read(true).open(pathfile.clone()).unwrap();
                                                            let mut torrc_file_text = String::new();
                                                            filetorrctext.read_to_string(&mut torrc_file_text).unwrap();
                                                            torrc_file_text = torrc_file_text.replace("\nBridge", "\n#Bridge") + &bridges_torrc_contents;

                                                            let mut bridgesfilerewrite = OpenOptions::new().write(true).truncate(true).open(pathfile).unwrap();

                                                            bridgesfilerewrite.write_all(torrc_file_text.as_bytes()).unwrap();

                                                            bridgesfilerewrite.sync_all().unwrap();

                                                        } else {

                                                            bridgesfile.write_all(bridges_torrc_contents.as_bytes()).unwrap();

                                                            bridgesfile.sync_all().unwrap();

                                                        }

                                                    },
                                                    #[cfg(target_os = "linux")]
                                                    Err(error) => {
                                                        use gtk::{MessageDialog, DialogFlags, MessageType, ButtonsType};
                                                        use std::io::ErrorKind;
                                                        use std::process::{Command, Stdio};

                                                        if error.kind() == ErrorKind::PermissionDenied && Path::new("/usr/bin/pkexec").exists() && Path::new("/usr/bin/tee").exists() && Path::new("/usr/bin/printf").exists() {
                                                            let permissions_ask_dialog = MessageDialog::new(Some(&bridgeswindow), DialogFlags::MODAL, MessageType::Question, ButtonsType::YesNo, locale::get_translation().OPEN_FILE_SUDO_MESSAGE);

                                                            permissions_ask_dialog.connect_response(clone!(@strong app_settings => move |dialog, permissions_ask_result_button| {
                                                                let pathfile_permissions = pathfile.clone();

                                                                if permissions_ask_result_button == ResponseType::Yes {
                                                                    if app_settings.property::<bool>("torrcdisableold") {

                                                                        let mut filetorrctext = OpenOptions::new().read(true).open(pathfile_permissions.as_path()).unwrap();
                                                                        let mut torrc_file_text = String::new();
                                                                        filetorrctext.read_to_string(&mut torrc_file_text).unwrap();
                                                                        torrc_file_text = torrc_file_text.replace("\nBridge", "\n#Bridge") + &bridges_torrc_contents;

                                                                        if let Ok(printf_child) = Command::new("printf")
                                                                            .arg(&torrc_file_text)
                                                                            .stdout(Stdio::piped())
                                                                            .spawn() {
                                                                                match Command::new("pkexec")
                                                                                    .arg("tee")
                                                                                    .arg(pathfile_permissions.as_path().to_str().unwrap())
                                                                                    .stdin(Stdio::from(printf_child.stdout.expect("Failed to open echo stdout")))
                                                                                    .stdout(Stdio::piped())
                                                                                    .spawn() {
                                                                                        Err(_) => {
                                                                                            println!("Error opening file!");
                                                                                        },
                                                                                        _ => ()
                                                                                    }
                                                                            }

                                                                    } else {

                                                                        if let Ok(printf_child) = Command::new("printf")
                                                                            .arg(&bridges_torrc_contents)
                                                                            .stdout(Stdio::piped())
                                                                            .spawn() {
                                                                                match Command::new("pkexec")
                                                                                    .arg("tee")
                                                                                    .arg("-a")
                                                                                    .arg(pathfile_permissions.as_path().to_str().unwrap())
                                                                                    .stdin(Stdio::from(printf_child.stdout.expect("Failed to open echo stdout")))
                                                                                    .stdout(Stdio::piped())
                                                                                    .spawn() {
                                                                                        Err(_) => {
                                                                                            println!("Error opening file");
                                                                                        },
                                                                                        _ => ()
                                                                                    }
                                                                            }

                                                                    }
                                                                }
                                                                dialog.close();
                                                            }));
                                                            permissions_ask_dialog.show();
                                                        }
                                                    },
                                                    #[cfg(not(target_os = "linux"))]
                                                    _ => (),
                                                }
                                            }
                                        }
                                    }

                                },
                                Err(error) => {
                                    let mut captcha_message = locale::get_translation().LOADING_BRIDGES_ERROR.to_string() + ": ";
                                    match error {
                                        0 => {
                                            captcha_message.push_str(locale::get_translation().LOADING_ERROR);
                                        },
                                        1 => {
                                            captcha_message.push_str(locale::get_translation().LOADING_ERROR_WRONG_CAPTCHA);
                                        },
                                        2 => {
                                            captcha_message.push_str(locale::get_translation().LOADING_ERROR_NO_BRIDGES);
                                        },
                                        3 => {
                                            captcha_message.push_str(locale::get_translation().LOADING_ERROR_INTERNAL);
                                        },
                                        _ => ()
                                    }
                                    captcha_loading_error_message.show();
                                    captcha_loading_error_message.set_markup(&("<span color=\"red\">".to_string() + &captcha_message + "</span>"));
                                    load_captcha();
                                },
                            }

                            captcha_submit.set_sensitive(true);
                            captcha_retry.set_sensitive(true);
                            captcha_cancel.set_sensitive(true);
                        }

                        glib::Continue(true)
                    }));

                    let load_bridges = clone!(@strong app_settings, @strong load_captcha, @weak captcha_input, @weak captcha_submit, @weak captcha_retry, @weak captcha_cancel, @weak captcha_loading_error_message => move || {
                        if app_settings.property::<String>("captchaid") != "" {
                            captcha_retry.set_sensitive(false);
                            captcha_submit.set_sensitive(false);
                            captcha_cancel.set_sensitive(false);
                            captcha_loading_error_message.hide();

                            let bridgeschannelsend = channelbridges.0.clone();

                            let transport_arg = app_settings.property::<i64>("transport");
                            let ipv6_arg = app_settings.property::<bool>("ipv6");
                            let use_proxy_arg = app_settings.property::<bool>("useproxy");
                            let proxy_type_arg = app_settings.property::<i64>("proxytype");
                            let proxy_host_arg = app_settings.property::<String>("proxyhost");
                            let proxy_port_arg = app_settings.property::<i64>("proxyport");
                            let proxy_onion_arg = app_settings.property::<bool>("proxyonion");
                            let captcha_id_arg = app_settings.property::<String>("captchaid");
                            let captcha_input_arg = captcha_input.text().to_string();

                            thread::spawn(move || {
                                bridgeschannelsend.send(get_bridges(transport_arg, ipv6_arg, use_proxy_arg, proxy_type_arg, proxy_host_arg, proxy_port_arg, proxy_onion_arg, captcha_id_arg.clone(), captcha_input_arg)).expect("Error receiving bridges");
                            });
                        } else {
                            captcha_loading_error_message.hide();

                            load_captcha();
                        }
                    });

                    captcha_submit.connect_clicked(clone!(@strong load_bridges, @weak captcha_input => move |_| {
                        load_bridges();
                        captcha_input.set_text("");
                    }));

                    captcha_input.connect_activate(clone!(@strong load_captcha, @weak captcha_input => move |_| {
                        load_bridges();
                        captcha_input.set_text("");
                    }));


                    captcha_retry.set_sensitive(true);
                    captcha_submit.set_sensitive(true);
                    captcha_cancel.set_sensitive(true);
                    captcha_loading_error_message.hide();
                    captcha_input.grab_focus();

                    captchawindow.set_child(Some(&boxcaptcha));
                    captcha_load.hide();

                    captchawindow.show();
                    load_captcha();
                }));
/* Captcha window end */

                let action_close = gio::SimpleAction::new("close", None);
                action_close.connect_activate(clone!(@weak app, @strong app_settings, @weak mainwindow => move |_, _| {
                    if mainwindow.is_active() && mainwindow.is_visible() {
                        if !app_settings.property::<bool>("runinbackground") || !app_settings.property::<bool>("notifications") {
                            app.quit();
                        } else {
                            mainwindow.hide();
                        }
                    }
                    let appwindows = app.windows();

                    for wind in appwindows {
                        if wind.hides_on_close() == false {
                            wind.destroy();
                        } else {
                            wind.hide();
                        }
                    }

                }));

                app.add_action(&action_close);

                app.set_accels_for_action("app.close", &["<Primary>W"]);

                let channeltimer: (gtk::glib::Sender<bool>, gtk::glib::Receiver<bool>) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

                channeltimer.1.attach(None, clone!(@weak app, @weak label_timer, @weak label_timer_until, @weak progressload, @weak mainwindow, @strong app_settings => @default-return glib::Continue(true), move |_| {
                    let mut next_time: u64 = (app_settings.property::<i64>("time") + (app_settings.property::<i64>("days")*24*60*60) + (app_settings.property::<i64>("hours")*60*60) + (app_settings.property::<i64>("minutes")*60) + app_settings.property::<i64>("seconds")) as u64;
                    let next_time_since_retrieval: u64 = (app_settings.property::<i64>("lastretrievaltime") + (app_settings.property::<i64>("days")*24*60*60) + (app_settings.property::<i64>("hours")*60*60) + (app_settings.property::<i64>("minutes")*60) + app_settings.property::<i64>("seconds")) as u64;
                    let now_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    if now_time > next_time_since_retrieval {
                        label_timer_until.set_markup(&("<b>".to_string() + locale::get_translation().MAIN_WINDOW_TIME_TO_RETRIEVE + "</b>"));
                    }
                    if now_time >= next_time {
                        app_settings.set_property("time", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64);
                        next_time = (app_settings.property::<i64>("time") + (app_settings.property::<i64>("days")*24*60*60) + (app_settings.property::<i64>("hours")*60*60) + (app_settings.property::<i64>("minutes")*60) + app_settings.property::<i64>("seconds")) as u64;
                        if !mainwindow.is_visible() || !mainwindow.is_active() {

                            if app_settings.property::<bool>("notifications") {

                                #[cfg(not(target_os = "windows"))]
                                {
                                    use gtk::gio::Notification;
                                    let pictb = Pixbuf::from_read(APP_ICON).unwrap();
                                    let notif_new = Notification::new(locale::get_translation().UPDATE_BRIDGES_NOTIFICATION_TITLE);
                                    notif_new.set_body(Some(locale::get_translation().UPDATE_BRIDGES_NOTIFICATION_TEXT));
                                    notif_new.set_icon(&pictb);
                                    notif_new.add_button(locale::get_translation().NOTIFICATION_OPEN, "app.activate");
                                    notif_new.set_default_action("app.activate");
                                    app.send_notification(None, &notif_new);
                                }

                                #[cfg(target_os = "windows")]
                                {
                                    app.activate();
                                }
                            }

                        }
                    }
                    let mut time_state = String::new();
                    let until_next: i64 = next_time as i64 - now_time as i64;
                    if until_next > 0 {
                        let days = until_next/(24*60*60);
                        let hours = (until_next%(24*60*60))/(60*60);
                        let minutes = (until_next/60)%60;
                        let seconds = until_next%60;
                        if days > 0 {
                            time_state.push_str("<b>");
                            time_state.push_str(days.to_string().as_str());
                            time_state.push_str("</b>");
                            time_state.push_str(&(" ".to_string() + locale::get_days(days as i32).to_string().as_str()));
                        }
                        if hours > 0 {
                            if days > 0 {
                                time_state.push(' ');
                            }
                            time_state.push_str("<b>");
                            time_state.push_str(hours.to_string().as_str());
                            time_state.push_str("</b>");
                            time_state.push_str(&(" ".to_string() + locale::get_hours(hours as i32).to_string().as_str()));
                        }
                        if minutes > 0 {
                            if days > 0 || hours > 0 {
                                time_state.push(' ');
                            }
                            time_state.push_str("<b>");
                            time_state.push_str(minutes.to_string().as_str());
                            time_state.push_str("</b>");
                            time_state.push_str(&(" ".to_string() + locale::get_minutes(minutes as i32).to_string().as_str()));
                        }
                        if seconds > 0 {
                            if days > 0 || hours > 0 || minutes > 0 {
                                time_state.push(' ');
                            }
                            time_state.push_str("<b>");
                            time_state.push_str(seconds.to_string().as_str());
                            time_state.push_str("</b>");
                            time_state.push_str(&(" ".to_string() + locale::get_seconds(seconds as i32).to_string().as_str()));
                        }
                        label_timer.set_markup(&time_state);

                        let full_progress: u64 = ((app_settings.property::<i64>("days")*24*60*60) + (app_settings.property::<i64>("hours")*60*60) + (app_settings.property::<i64>("minutes")*60) + app_settings.property::<i64>("seconds")) as u64;

                        let last_retrieve: u64 = app_settings.property::<i64>("lastretrievaltime") as u64;

                        if last_retrieve > 0 {
                            label_timer_last_retrieval.set_text(&(locale::get_translation().LAST_RETRIEVAL_TIME.to_string() + ": " + &chrono::Local.timestamp_opt(last_retrieve as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string().as_str()));
                        }

                        progressload.set_fraction(until_next as f64/full_progress as f64);

                    }
                    glib::Continue(true)
                }));

                thread::spawn(move || {
                    let chantimer = channeltimer.0.clone();
                    loop {
                        chantimer.send(true).expect("Failed updating timer");
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }
/* Initialize windows end */
        }));
    });

    app.run();
}

fn get_captcha(transport: i64, ipv6: bool, use_proxy: bool, proxy_protocol: i64, proxy_host: String, proxy_port: i64, onion: bool) -> RequestResult {
    let mut url = String::from(BRIDGES_URL);

    let user_agent = String::from(USER_AGENT);

    let mut client = reqwest::blocking::Client::builder()
        .user_agent(user_agent);

    if use_proxy {
        if onion {
            url = String::from(BRIDGES_URL_ONION);
        }

        let mut proxy_string = String::new();

        match proxy_protocol {
            0 => {
                proxy_string.push_str("http://");
            },
            1 => {
                proxy_string.push_str("socks5h://");
            },
            _ => ()
        }

        proxy_string.push_str(&proxy_host);

        proxy_string.push(':');

        proxy_string.push_str(&proxy_port.to_string());

        if let Ok(proxy) = reqwest::Proxy::all(proxy_string) {
            client = client.proxy(proxy);
        } else {
            return Err(0);
        }
    }

    if transport == 0 {

        url.push_str("?transport=obfs4");

    } else {

        url.push_str("?transport=0");

    }

    if ipv6 {

        url.push_str("&ipv6=yes");

    }

    if let Ok(response) = client.build().unwrap().get(url).send() {
        if let Ok(body) = response.text() {

            let captchapattern_old = Regex::new("<div id=\"bridgedb-captcha-container\">\\s+<div id=\"bridgedb-captcha\" class=\"pb-3\">\\s+<img alt=\"Your browser is not displaying images properly.\" src=\"data:image/jpeg;base64,([^\"]*)").unwrap();

            let mut captchapattern = Regex::new("<div class=\"container-narrow\" id=\"captcha-submission-container\">\\s+<div class=\"container-fluid container-fluid-inner-5\">\\s+<div class=\"box\" id=\"captcha-box\">\\s+<img alt=\"Your browser is not displaying images properly.\" src=\"data:image/jpeg;base64,([^\"]*)").unwrap();

            let captchaidpattern_old = Regex::new("<input type=\"hidden\"\\s+name=\"captcha_challenge_field\"\\s+value=\"([^\"]*)").unwrap();

            let mut captchaidpattern = Regex::new("<input type=\"hidden\"\\s+form=\"captchaSubmission\"\\s+name=\"captcha_challenge_field\"\\s+id=\"captcha_challenge_field\"\\s+value=\"([^\"]*)").unwrap();

            if captchapattern_old.captures(&body).iter().count() > 0 && captchaidpattern_old.captures(&body).iter().count() > 0 {
               captchapattern = captchapattern_old;
               captchaidpattern = captchaidpattern_old;
            }

            if captchapattern.captures(&body).iter().count() > 0 {

                let captchaimagestr = captchapattern.captures(&body).unwrap().get(1).unwrap().as_str().to_string();

                if captchaidpattern.captures(&body).iter().count() > 0 {

                    let captchaidstr = captchaidpattern.captures(&body).unwrap().get(1).unwrap().as_str().to_string();
                    return Ok((captchaimagestr, captchaidstr));
                } else {
                    return Err(1);
                }
            } else {
                return Err(1);
            }
        } else {
            return Err(0);
        }

    } else {
        return Err(0);
    }
}

fn get_bridges(transport: i64, ipv6: bool, use_proxy: bool, proxy_protocol: i64, proxy_host: String, proxy_port: i64, onion: bool, captchaid: String, captchasolved: String) -> RequestResult {
    let mut url = String::from(BRIDGES_URL);

    let user_agent = String::from(USER_AGENT);

    let mut client = reqwest::blocking::Client::builder()
        .user_agent(user_agent);

    if use_proxy {
        if onion {
            url = String::from(BRIDGES_URL_ONION);
        }

        let mut proxy_string = String::new();

        match proxy_protocol {
            0 => {
                proxy_string.push_str("http://");
            },
            1 => {
                proxy_string.push_str("socks5h://");
            },
            _ => ()
        }

        proxy_string.push_str(&proxy_host);

        proxy_string.push(':');

        proxy_string.push_str(&proxy_port.to_string());

        if let Ok(proxy) = reqwest::Proxy::all(proxy_string) {
            client = client.proxy(proxy);
        } else {
            return Err(0);
        }
    }

    if transport == 0 {

        url.push_str("?transport=obfs4");

    } else {

        url.push_str("?transport=0");

    }

    if ipv6 {

        url.push_str("&ipv6=yes");

    }

    if let Ok(response) = client.build().unwrap().post(url).form(&[("captcha_challenge_field", captchaid), ("captcha_response_field", captchasolved)]).send() {
        if let Ok(body) = response.text() {

            let parsebridgelines_old = Regex::new("<div id=\"bridgelines\" class=\"p-4 mb-3\">\\s+\\n([^\"]*)<br />\\s+</div>\\s").unwrap();

            let mut parsebridgelines = Regex::new("<div class=\"bridge-lines\" id=\"bridgelines\">\n(([^<]*<br />\n)+)").unwrap();
            let parsebridgeqrcode = Regex::new("<img title=\"QRCode for your bridge lines from BridgeDB\"\\s+src=\"data:image/jpeg;base64,([^<]*)\"\\s+alt=\"\"\\s+/>\\s").unwrap();

            let captchapattern_old = Regex::new("<div id=\"bridgedb-captcha-container\">\\s+<div id=\"bridgedb-captcha\" class=\"pb-3\">\\s+<img alt=\"Your browser is not displaying images properly.\" src=\"data:image/jpeg;base64,([^\"]*)").unwrap();

            let mut captchapattern = Regex::new("<div class=\"container-narrow\" id=\"captcha-submission-container\">\\s+<div class=\"container-fluid container-fluid-inner-5\">\\s+<div class=\"box\" id=\"captcha-box\">\\s+<img alt=\"Your browser is not displaying images properly.\" src=\"data:image/jpeg;base64,([^\"]*)").unwrap();

            if parsebridgelines_old.captures(&body).iter().count() > 0 {
                parsebridgelines = parsebridgelines_old;
            }

            if captchapattern_old.captures(&body).iter().count() > 0 {
                captchapattern = captchapattern_old;
            }

            if parsebridgelines.captures(&body).iter().count() == 0 || parsebridgeqrcode.captures(&body).iter().count() == 0 {
                return Err(1);
            }

            if captchapattern.captures(&body).iter().count() > 0 {
                return Err(2);
            }

            if body.contains("BridgeDB encountered an internal error") {
                return Err(3);
            }

            if body.contains("There currently aren't any bridges available") {
                return Err(4);
            }

            let parsedlines = String::from(parsebridgelines.captures(&body).unwrap().get(1).unwrap().as_str()).replacen("        ", "", 1).replace("\n        ", "\n").replace("<br />", "\n");
            let qrcodeimage = String::from(parsebridgeqrcode.captures(&body).unwrap().get(1).unwrap().as_str());

            return Ok((qrcodeimage, parsedlines));
        } else {
            return Err(0);
        }

    } else {
        return Err(0);
    }
}
