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

mod imp;

use gtk4 as gtk;

use gtk::glib;
use std::path::Path;

use std::{fs, fs::OpenOptions};
use json::{object, parse};
use std::io::{Read, Write};

use std::time::SystemTime;

use gtk::prelude::ObjectExt;

glib::wrapper! {
    pub struct AppSettings(ObjectSubclass<imp::AppSettings>);
}

fn get_config_paths() -> (String, String) {
    let mut dirpath: (String, String) = ("".to_string(), "updaterconfig.json".to_string());

    if let Some(configdir) = dirs::config_dir() {
        let mut configdirname = configdir.to_str().unwrap().to_string();
        if std::env::consts::OS == "windows" {
            configdirname.push_str("\\TorBridgesUpdater\\");
        } else {
            configdirname.push_str("/TorBridgesUpdater/");
        }
        let mut config_path = configdirname.clone();
        config_path.push_str("updaterconfig.json");
        dirpath = (configdirname, config_path);
    };
    dirpath
}

fn get_save_paths() -> (String, String) {
    let mut dirpath: (String, String) = ("torrc".to_string(), "bridges.txt".to_string());

    if let Some(homedir) = dirs::home_dir() {
        let mut torrc_path = homedir.to_str().unwrap().to_string();
        let mut bridges_path = homedir.to_str().unwrap().to_string();
        if std::env::consts::OS == "windows" {
            torrc_path.push_str("\\");
            bridges_path.push_str("\\");
        } else {
            torrc_path.push_str("/");
            bridges_path.push_str("/");
        }
        torrc_path.push_str("torrc");
        bridges_path.push_str("bridges.txt");
        dirpath = (torrc_path, bridges_path);
    };
    dirpath
}

impl AppSettings {
    pub fn new() -> AppSettings {
        glib::Object::new(&[
            ("lastretrievaltime", &(0 as i64)),
            ("time", &(0 as i64)),
            ("days", &(0 as i64)),
            ("hours", &(0 as i64)),
            ("minutes", &(0 as i64)),
            ("seconds", &(0 as i64)),
            ("savetorrc", &false),
            ("savetorrcpath", &"".to_string()),
            ("savebridges", &false),
            ("savebridgespath", &"".to_string()),
            ("torrcdisableold", &false),
            ("keepold", &false),
            ("useproxy", &false),
            ("proxytype", &(0 as i64)),
            ("proxyhost", &"".to_string()),
            ("proxyport", &(0 as i64)),
            ("proxyonion", &false),
            ("transport", &(0 as i64)),
            ("ipv6", &false),
            ("notifications", &false),
            ("runinbackground", &false),
            ("captchaid", &"".to_string()),
            ("captchaloading", &false),
            ("qrcodetext", &"".to_string()),
        ]).expect("Failed to load app settings.")
    }


    pub fn load() -> AppSettings {
        let config_paths = get_config_paths();

        let dirpath = get_save_paths();

        let config_default = object!{
            lastretrievaltime: 0,
            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            days: 0,
            hours: 1,
            minutes: 0,
            seconds: 0,
            savetorrc: false,
            savetorrcpath: dirpath.clone().0,
            savebridges: true,
            savebridgespath: dirpath.clone().1,
            torrcdisableold: true,
            keepold: true,
            useproxy: false,
            proxytype: 0,
            proxyhost: "127.0.0.1".to_string(),
            proxyport: 8118,
            transport: 0,
            ipv6: false,
            proxyonion: false,
            notifications: false,
            runinbackground: false,
        };

        #[allow(unused_assignments)]
        let mut config = object!{};
        if !Path::new(&config_paths.1).exists() {
            if !Path::new(&config_paths.0).exists() {
                fs::create_dir(&config_paths.0).expect("Error accessing system config directory");
            }

            let mut fileconfig = OpenOptions::new().write(true).create(true).open(config_paths.1.clone()).unwrap();

            fileconfig.write_all(config_default.clone().dump().as_bytes()).unwrap();

            fileconfig.sync_all().unwrap();

        } else {
            let mut fileconfig = OpenOptions::new().read(true).open(config_paths.1.clone()).unwrap();
            let mut config_text = String::new();
            fileconfig.read_to_string(&mut config_text).unwrap();
            match parse(&config_text) {
                Ok(config_parsed) => {
                    config = config_parsed
                },
                Err(_) => {
                    config = config_default.clone();
                    let mut fileconfigoverwrite = OpenOptions::new().write(true).create(true).truncate(true).open(config_paths.1.clone()).unwrap();
                    fileconfigoverwrite.write_all(config_default.clone().dump().as_bytes()).unwrap();
                    fileconfigoverwrite.sync_all().unwrap();
                },
            }

        }

        if config["lastretrievaltime"].as_i64() == None ||
        config["time"].as_i64() == None ||
        config["days"].as_i64() == None ||
        config["hours"].as_i64() == None ||
        config["minutes"].as_i64() == None ||
        config["seconds"].as_i64() == None ||
        config["savetorrc"].as_bool() == None ||
        config["savetorrcpath"].as_str() == None ||
        config["savebridges"].as_bool() == None ||
        config["savebridgespath"].as_str() == None ||
        config["torrcdisableold"].as_bool() == None ||
        config["keepold"].as_bool() == None ||
        config["useproxy"].as_bool() == None ||
        config["proxytype"].as_i64() == None ||
        config["proxyhost"].as_str() == None ||
        config["proxyport"].as_i64() == None ||
        config["proxyonion"].as_bool() == None ||
        config["transport"].as_i64() == None ||
        config["ipv6"].as_bool() == None ||
        config["notifications"].as_bool() == None ||
        config["runinbackground"].as_bool() == None {
            let mut fileconfigoverwrite = OpenOptions::new().write(true).create(true).truncate(true).open(config_paths.1.clone()).unwrap();
            fileconfigoverwrite.write_all(config_default.clone().dump().as_bytes()).unwrap();
            fileconfigoverwrite.sync_all().unwrap();

            return glib::Object::new(&[
                ("lastretrievaltime", &config_default["lastretrievaltime"].as_i64().unwrap()),
                ("time", &config_default["time"].as_i64().unwrap()),
                ("days", &config_default["days"].as_i64().unwrap()),
                ("hours", &config_default["hours"].as_i64().unwrap()),
                ("minutes", &config_default["minutes"].as_i64().unwrap()),
                ("seconds", &config_default["seconds"].as_i64().unwrap()),
                ("savetorrc", &config_default["savetorrc"].as_bool().unwrap()),
                ("savetorrcpath", &config_default["savetorrcpath"].as_str().unwrap()),
                ("savebridges", &config_default["savebridges"].as_bool().unwrap()),
                ("savebridgespath", &config_default["savebridgespath"].as_str().unwrap()),
                ("torrcdisableold", &config_default["torrcdisableold"].as_bool().unwrap()),
                ("keepold", &config_default["keepold"].as_bool().unwrap()),
                ("useproxy", &config_default["useproxy"].as_bool().unwrap()),
                ("proxytype", &config_default["proxytype"].as_i64().unwrap()),
                ("proxyhost", &config_default["proxyhost"].as_str().unwrap()),
                ("proxyport", &config_default["proxyport"].as_i64().unwrap()),
                ("proxyonion", &config_default["proxyonion"].as_bool().unwrap()),
                ("transport", &config_default["transport"].as_i64().unwrap()),
                ("ipv6", &config_default["ipv6"].as_bool().unwrap()),
                ("notifications", &config_default["notifications"].as_bool().unwrap()),
                ("runinbackground", &config_default["runinbackground"].as_bool().unwrap()),
                ("captchaid", &"".to_string()),
                ("captchaloading", &false),
                ("qrcodetext", &"".to_string()),
            ]).expect("Failed to load app settings.");
        }

        glib::Object::new(&[
            ("lastretrievaltime", &config["lastretrievaltime"].as_i64().unwrap()),
            ("time", &config["time"].as_i64().unwrap()),
            ("days", &config["days"].as_i64().unwrap()),
            ("hours", &config["hours"].as_i64().unwrap()),
            ("minutes", &config["minutes"].as_i64().unwrap()),
            ("seconds", &config["seconds"].as_i64().unwrap()),
            ("savetorrc", &config["savetorrc"].as_bool().unwrap()),
            ("savetorrcpath", &config["savetorrcpath"].as_str().unwrap()),
            ("savebridges", &config["savebridges"].as_bool().unwrap()),
            ("savebridgespath", &config["savebridgespath"].as_str().unwrap()),
            ("torrcdisableold", &config["torrcdisableold"].as_bool().unwrap()),
            ("keepold", &config["keepold"].as_bool().unwrap()),
            ("useproxy", &config["useproxy"].as_bool().unwrap()),
            ("proxytype", &config["proxytype"].as_i64().unwrap()),
            ("proxyhost", &config["proxyhost"].as_str().unwrap()),
            ("proxyport", &config["proxyport"].as_i64().unwrap()),
            ("proxyonion", &config["proxyonion"].as_bool().unwrap()),
            ("transport", &config["transport"].as_i64().unwrap()),
            ("ipv6", &config["ipv6"].as_bool().unwrap()),
            ("notifications", &config["notifications"].as_bool().unwrap()),
            ("runinbackground", &config["runinbackground"].as_bool().unwrap()),
            ("captchaid", &"".to_string()),
            ("captchaloading", &false),
            ("qrcodetext", &"".to_string()),
        ]).expect("Failed to load app settings.")
    }

    pub fn save(&self) {
        let config = object!{
            lastretrievaltime: self.property_value("lastretrievaltime").get::<i64>().unwrap(),
            time: self.property_value("time").get::<i64>().unwrap(),
            days: self.property_value("days").get::<i64>().unwrap(),
            hours: self.property_value("hours").get::<i64>().unwrap(),
            minutes: self.property_value("minutes").get::<i64>().unwrap(),
            seconds: self.property_value("seconds").get::<i64>().unwrap(),
            savetorrc: self.property_value("savetorrc").get::<bool>().unwrap(),
            savetorrcpath: self.property_value("savetorrcpath").get::<String>().unwrap(),
            savebridges: self.property_value("savebridges").get::<bool>().unwrap(),
            savebridgespath: self.property_value("savebridgespath").get::<String>().unwrap(),
            torrcdisableold: self.property_value("torrcdisableold").get::<bool>().unwrap(),
            keepold: self.property_value("keepold").get::<bool>().unwrap(),
            useproxy: self.property_value("useproxy").get::<bool>().unwrap(),
            proxytype: self.property_value("proxytype").get::<i64>().unwrap(),
            proxyhost: self.property_value("proxyhost").get::<String>().unwrap(),
            proxyport: self.property_value("proxyport").get::<i64>().unwrap(),
            transport: self.property_value("transport").get::<i64>().unwrap(),
            ipv6: self.property_value("ipv6").get::<bool>().unwrap(),
            proxyonion: self.property_value("proxyonion").get::<bool>().unwrap(),
            notifications: self.property_value("notifications").get::<bool>().unwrap(),
            runinbackground: self.property_value("runinbackground").get::<bool>().unwrap(),
        };

        let mut fileconfig = OpenOptions::new().write(true).truncate(true).open(get_config_paths().1).unwrap();

        fileconfig.write_all(config.dump().as_bytes()).unwrap();
        fileconfig.sync_all().unwrap();
    }

    pub fn load_from(&self, other: &AppSettings) {
        self.set_property("lastretrievaltime", other.property_value("lastretrievaltime").get::<i64>().unwrap());
        self.set_property("time", other.property_value("time").get::<i64>().unwrap());
        self.set_property("days", other.property_value("days").get::<i64>().unwrap());
        self.set_property("hours", other.property_value("hours").get::<i64>().unwrap());
        self.set_property("minutes", other.property_value("minutes").get::<i64>().unwrap());
        self.set_property("seconds", other.property_value("seconds").get::<i64>().unwrap());
        self.set_property("savetorrc", other.property_value("savetorrc").get::<bool>().unwrap());
        self.set_property("savetorrcpath", other.property_value("savetorrcpath").get::<String>().unwrap());
        self.set_property("savebridges", other.property_value("savebridges").get::<bool>().unwrap());
        self.set_property("savebridgespath", other.property_value("savebridgespath").get::<String>().unwrap());
        self.set_property("torrcdisableold", other.property_value("torrcdisableold").get::<bool>().unwrap());
        self.set_property("keepold", other.property_value("keepold").get::<bool>().unwrap());
        self.set_property("useproxy", other.property_value("useproxy").get::<bool>().unwrap());
        self.set_property("proxytype", other.property_value("proxytype").get::<i64>().unwrap());
        self.set_property("proxyhost", other.property_value("proxyhost").get::<String>().unwrap());
        self.set_property("proxyport", other.property_value("proxyport").get::<i64>().unwrap());
        self.set_property("transport", other.property_value("transport").get::<i64>().unwrap());
        self.set_property("ipv6", other.property_value("ipv6").get::<bool>().unwrap());
        self.set_property("proxyonion", other.property_value("proxyonion").get::<bool>().unwrap());
        self.set_property("notifications", other.property_value("notifications").get::<bool>().unwrap());
        self.set_property("runinbackground", other.property_value("runinbackground").get::<bool>().unwrap());
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self::new()
    }
}
