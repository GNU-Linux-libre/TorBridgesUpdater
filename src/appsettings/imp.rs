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

use gtk4 as gtk;

use gtk::{glib, prelude::*};
use glib::subclass::prelude::*;

use std::cell::RefCell;

#[derive(Default)]
pub struct AppSettings {
    lastretrievaltime: RefCell<i64>,
    time: RefCell<i64>,
    days: RefCell<i64>,
    hours: RefCell<i64>,
    minutes: RefCell<i64>,
    seconds: RefCell<i64>,
    savetorrc: RefCell<bool>,
    savetorrcpath: RefCell<Option<String>>,
    savebridges: RefCell<bool>,
    savebridgespath: RefCell<Option<String>>,
    torrcdisableold: RefCell<bool>,
    keepold: RefCell<bool>,
    useproxy: RefCell<bool>,
    proxytype: RefCell<i64>,
    proxyhost: RefCell<Option<String>>,
    proxyport: RefCell<i64>,
    transport: RefCell<i64>,
    ipv6: RefCell<bool>,
    proxyonion: RefCell<bool>,
    notifications: RefCell<bool>,
    runinbackground: RefCell<bool>,
    captchaid: RefCell<Option<String>>,
    captchaloading: RefCell<bool>,
    qrcodetext: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for AppSettings {
    const NAME: &'static str = "AppSettings";
    type Type = super::AppSettings;
    type ParentType = glib::Object;
}

impl ObjectImpl for AppSettings {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecInt64::new(
                    "lastretrievaltime",
                    "LastRetrievalTime",
                    "LastRetrievalTime",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "time",
                    "Time",
                    "Time",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "days",
                    "Days",
                    "Days",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "hours",
                    "Hours",
                    "Hours",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "minutes",
                    "Minutes",
                    "Minutes",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "seconds",
                    "Seconds",
                    "Seconds",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "savetorrc",
                    "SaveTorrc",
                    "SaveTorrc",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "savetorrcpath",
                    "SaveTorrcPath",
                    "SaveTorrcPath",
                    Some(""),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "savebridges",
                    "SaveBridges",
                    "SaveBridges",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "savebridgespath",
                    "SaveBridgesPath",
                    "SaveBridgesPath",
                    Some(""),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "torrcdisableold",
                    "TorrcDisableOld",
                    "TorrcDisableOld",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "keepold",
                    "KeepOld",
                    "KeepOld",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "useproxy",
                    "UseProxy",
                    "UseProxy",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "proxytype",
                    "ProxyType",
                    "ProxyType",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "proxyhost",
                    "ProxyHost",
                    "ProxyHost",
                    Some(""),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "proxyport",
                    "ProxyPort",
                    "ProxyPort",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "proxyonion",
                    "ProxyOnion",
                    "ProxyOnion",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt64::new(
                    "transport",
                    "Transport",
                    "Transport",
                    i64::MIN,
                    i64::MAX,
                    0,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "ipv6",
                    "IPv6",
                    "IPv6",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "notifications",
                    "Notifications",
                    "Notifications",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "runinbackground",
                    "RunInBackground",
                    "RunInBackground",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "captchaid",
                    "CaptchaID",
                    "CaptchaID",
                    Some(""),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "captchaloading",
                    "CaptchaLoading",
                    "CaptchaLoading",
                    false,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "qrcodetext",
                    "QRCodeText",
                    "QRCodeText",
                    Some(""),
                    glib::ParamFlags::READWRITE,
                ),

            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        match pspec.name() {
            "lastretrievaltime" => {
                let lastretrievaltime = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.lastretrievaltime.replace(lastretrievaltime);
            },
            "time" => {
                let time = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.time.replace(time);
            },
            "days" => {
                let days = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.days.replace(days);
            },
            "hours" => {
                let hours = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.hours.replace(hours);
            },
            "minutes" => {
                let minutes = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.minutes.replace(minutes);
            },
            "seconds" => {
                let seconds = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.seconds.replace(seconds);
            },
            "savetorrc" => {
                let savetorrc = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.savetorrc.replace(savetorrc);
            },
            "savetorrcpath" => {
                let savetorrcpath = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.savetorrcpath.replace(savetorrcpath);
            },
            "savebridges" => {
                let savebridges = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.savebridges.replace(savebridges);
            },
            "savebridgespath" => {
                let savebridgespath = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.savebridgespath.replace(savebridgespath);
            },
            "torrcdisableold" => {
                let torrcdisableold = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.torrcdisableold.replace(torrcdisableold);
            },
            "keepold" => {
                let keepold = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.keepold.replace(keepold);
            },
            "useproxy" => {
                let useproxy = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.useproxy.replace(useproxy);
            },
            "proxytype" => {
                let proxytype = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.proxytype.replace(proxytype);
            },
            "proxyhost" => {
                let proxyhost = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.proxyhost.replace(proxyhost);
            },
            "proxyport" => {
                let proxyport = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.proxyport.replace(proxyport);
            },
            "proxyonion" => {
                let proxyonion = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.proxyonion.replace(proxyonion);
            },
            "transport" => {
                let transport = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.transport.replace(transport);
            },
            "ipv6" => {
                let ipv6 = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.ipv6.replace(ipv6);
            },
            "notifications" => {
                let notifications = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.notifications.replace(notifications);
            },
            "runinbackground" => {
                let runinbackground = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.runinbackground.replace(runinbackground);
            },
            "captchaid" => {
                let captchaid = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.captchaid.replace(captchaid);
            },
            "captchaloading" => {
                let captchaloading = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.captchaloading.replace(captchaloading);
            },
            "qrcodetext" => {
                let qrcodetext = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.qrcodetext.replace(qrcodetext);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "lastretrievaltime" => self.lastretrievaltime.borrow().to_value(),
            "time" => self.time.borrow().to_value(),
            "days" => self.days.borrow().to_value(),
            "hours" => self.hours.borrow().to_value(),
            "minutes" => self.minutes.borrow().to_value(),
            "seconds" => self.seconds.borrow().to_value(),
            "savetorrc" => self.savetorrc.borrow().to_value(),
            "savetorrcpath" => self.savetorrcpath.borrow().to_value(),
            "savebridges" => self.savebridges.borrow().to_value(),
            "savebridgespath" => self.savebridgespath.borrow().to_value(),
            "torrcdisableold" => self.torrcdisableold.borrow().to_value(),
            "keepold" => self.keepold.borrow().to_value(),
            "useproxy" => self.useproxy.borrow().to_value(),
            "proxytype" => self.proxytype.borrow().to_value(),
            "proxyhost" => self.proxyhost.borrow().to_value(),
            "proxyport" => self.proxyport.borrow().to_value(),
            "proxyonion" => self.proxyonion.borrow().to_value(),
            "transport" => self.transport.borrow().to_value(),
            "ipv6" => self.ipv6.borrow().to_value(),
            "notifications" => self.notifications.borrow().to_value(),
            "runinbackground" => self.runinbackground.borrow().to_value(),
            "captchaid" => self.captchaid.borrow().to_value(),
            "captchaloading" => self.captchaloading.borrow().to_value(),
            "qrcodetext" => self.qrcodetext.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

}
