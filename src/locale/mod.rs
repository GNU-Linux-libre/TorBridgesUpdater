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

use sys_locale::get_locale;

#[path = "lang/locale_us.rs"]
mod locale_us;

#[allow(non_snake_case)]
pub struct Translation {
    pub APP_TITLE: &'static str,
    pub NOTIFICATION_OPEN: &'static str,
    pub FORM_OK: &'static str,
    pub FORM_APPLY: &'static str,
    pub FORM_CANCEL: &'static str,
    pub FORM_CLOSE: &'static str,
    pub FORM_RETRY: &'static str,
    pub OPEN_FILE: &'static str,
    pub SAVE_FILE: &'static str,
    pub TORRC_FILE_SELECT: &'static str,
    pub BRIDGES_FILE_SELECT: &'static str,
    pub CAPTCHA_LOADING_MESSAGE: &'static str,
    pub LAST_RETRIEVAL_TIME: &'static str,
    pub RETRIEVAL_TIME_UNKNOWN: &'static str,
    pub OPEN_FILE_SUDO_MESSAGE: &'static str,
    pub LOADING_CAPTCHA_ERROR: &'static str,
    pub LOADING_BRIDGES_ERROR: &'static str,
    pub LOADING_CAPTCHA_NOTFOUND: &'static str,
    pub LOADING_BRIDGES_NOTFOUND: &'static str,
    pub LOADING_ERROR: &'static str,
    pub LOADING_ERROR_WRONG_CAPTCHA: &'static str,
    pub LOADING_ERROR_INTERNAL: &'static str,
    pub LOADING_ERROR_NO_BRIDGES: &'static str,
    pub BRIDGES_SAVE_QR: &'static str,
    pub UPDATE_BRIDGES_NOTIFICATION_TITLE: &'static str,
    pub UPDATE_BRIDGES_NOTIFICATION_TEXT: &'static str,
    pub ABOUT_WINDOW_TITLE: &'static str,
    pub ABOUT_WINDOW_DESCRIPTION: &'static str,
    pub BRIDGES_WINDOW_TITLE: &'static str,
    pub BRIDGES_WINDOW_NEW_BRIDGES: &'static str,
    pub BRIDGES_WINDOW_CLIPBOARD_COPY: &'static str,
    pub BRIDGES_WINDOW_CLIPBOARD_COPIED: &'static str,
    pub BRIDGES_WINDOW_SHOW_QR_CODE: &'static str,
    pub CAPTCHA_WINDOW_TITLE: &'static str,
    pub CAPTCHA_WINDOW_LOADING_CAPTCHA: &'static str,
    pub MAIN_WINDOW_GET_BRIDGES_BUTTON: &'static str,
    pub MAIN_WINDOW_LAST_RETRIEVAL: &'static str,
    pub MAIN_WINDOW_UNTIL_RETRIEVAL: &'static str,
    pub MAIN_WINDOW_TIME_TO_RETRIEVE: &'static str,
    pub MAIN_WINDOW_BUTTON_SETTINGS: &'static str,
    pub MAIN_WINDOW_BUTTON_ABOUT: &'static str,
    pub MAIN_WINDOW_BUTTON_QUIT: &'static str,
    pub QRCODE_WINDOW_TITLE: &'static str,
    pub QRCODE_WINDOW_SAVE_AS_IMAGE: &'static str,
    pub SETTINGS_WINDOW_TITLE: &'static str,
    pub SETTINGS_WINDOW_PROXY: &'static str,
    pub SETTINGS_WINDOW_BRIDGES_KEEP_OLD: &'static str,
    pub SETTINGS_WINDOW_BRIDGES_KEEP_OLD_TOOLTIP: &'static str,
    pub SETTINGS_WINDOW_TIMER_TITLE: &'static str,
    pub SETTINGS_WINDOW_CONNECT_ONION: &'static str,
    pub SETTINGS_WINDOW_CONNECT_ONION_TOOLTIP: &'static str,
    pub SETTINGS_WINDOW_NOTIFICATIONS_SHOW: &'static str,
    pub SETTINGS_WINDOW_NOTIFICATIONS_SHOW_EVERY: &'static str,
    pub SETTINGS_WINDOW_RUN_BACKGROUND: &'static str,
    pub SETTINGS_WINDOW_BRIDGES_TRANSPORT: &'static str,
    pub SETTINGS_WINDOW_BRIDGES_SAVE: &'static str,
    pub SETTINGS_WINDOW_IPV6: &'static str,
    pub SETTINGS_WINDOW_IPV6_TOOLTIP: &'static str,
    pub SETTINGS_WINDOW_TORRC_MODIFY: &'static str,
    pub SETTINGS_WINDOW_TORRC_DISABLE_OLD: &'static str,
    pub SETTINGS_WINDOW_TORRC_DISABLE_OLD_TOOLTIP: &'static str,
}

pub fn get_translation() -> Translation {
    match get_locale().unwrap_or_else(|| String::from("en-US")).as_str() {
        "en-US" => {
            return locale_us::TRANSLATION;
        },
        _ => {
            return locale_us::TRANSLATION;
        },
    }
}

pub fn get_days(count: i32) -> &'static str {
    match get_locale().unwrap_or_else(|| String::from("en-US")).as_str() {
        "en-US" => {
            return locale_us::get_days(count);
        },
        _ => {
            return locale_us::get_days(count);
        },
    }
}

pub fn get_hours(count: i32) -> &'static str {
    match get_locale().unwrap_or_else(|| String::from("en-US")).as_str() {
        "en-US" => {
            return locale_us::get_hours(count);
        },
        _ => {
            return locale_us::get_hours(count);
        },
    }
}

pub fn get_minutes(count: i32) -> &'static str {
    match get_locale().unwrap_or_else(|| String::from("en-US")).as_str() {
        "en-US" => {
            return locale_us::get_minutes(count);
        },
        _ => {
            return locale_us::get_minutes(count);
        },
    }
}

pub fn get_seconds(count: i32) -> &'static str {
    match get_locale().unwrap_or_else(|| String::from("en-US")).as_str() {
        "en-US" => {
            return locale_us::get_seconds(count);
        },
        _ => {
            return locale_us::get_seconds(count);
        },
    }
}
