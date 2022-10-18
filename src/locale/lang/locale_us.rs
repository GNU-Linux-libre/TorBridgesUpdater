use crate::locale::Translation;

pub const TRANSLATION: Translation = Translation {
    APP_TITLE: "Tor Bridges Updater",
    NOTIFICATION_OPEN: "Open",
    FORM_OK: "OK",
    FORM_APPLY: "Apply",
    FORM_CANCEL: "Cancel",
    FORM_CLOSE: "Close",
    FORM_RETRY: "Retry",
    OPEN_FILE: "Open file",
    SAVE_FILE: "Save file",
    TORRC_FILE_SELECT: "Open torrc file",
    BRIDGES_FILE_SELECT: "Save bridges in",
    CAPTCHA_LOADING_MESSAGE: "Loading captcha...",
    LAST_RETRIEVAL_TIME: "Last retrieval time",
    RETRIEVAL_TIME_UNKNOWN: "unknown",
    OPEN_FILE_SUDO_MESSAGE: "Permissions are required to edit your torrc file. Proceed?",
    LOADING_CAPTCHA_ERROR: "Error loading captcha",
    LOADING_BRIDGES_ERROR: "Error loading bridges",
    LOADING_CAPTCHA_NOTFOUND: "Captcha is missing",
    LOADING_BRIDGES_NOTFOUND: "Bridges are missing",
    LOADING_ERROR: "Can't load page!",
    LOADING_ERROR_WRONG_CAPTCHA: "Wrong captcha!",
    LOADING_ERROR_NO_BRIDGES: "No bridges are currently available",
    BRIDGES_SAVE_QR: "Save bridges QR code",
    UPDATE_BRIDGES_NOTIFICATION_TITLE: "Time to update bridges!",
    UPDATE_BRIDGES_NOTIFICATION_TEXT: "Update your Tor Bridges",
    ABOUT_WINDOW_TITLE: "About Tor Bridges Updater",
    ABOUT_WINDOW_DESCRIPTION: "Update your Tor bridges on a schedule.",
    BRIDGES_WINDOW_TITLE: "Bridges",
    BRIDGES_WINDOW_NEW_BRIDGES: "Your new bridges are",
    BRIDGES_WINDOW_CLIPBOARD_COPY: "Copy to clipboard",
    BRIDGES_WINDOW_CLIPBOARD_COPIED: "Copied to clipboard",
    BRIDGES_WINDOW_SHOW_QR_CODE: "Show QR Code",
    CAPTCHA_WINDOW_TITLE: "Enter captcha",
    CAPTCHA_WINDOW_LOADING_CAPTCHA: "Loading captcha...",
    MAIN_WINDOW_GET_BRIDGES_BUTTON: "Retrieve now",
    MAIN_WINDOW_LAST_RETRIEVAL: "Last retrieval time",
    MAIN_WINDOW_UNTIL_RETRIEVAL: "Until next bridge retrieval",
    MAIN_WINDOW_TIME_TO_RETRIEVE: "Time to retrieve bridges",
    MAIN_WINDOW_BUTTON_SETTINGS: "Settings",
    MAIN_WINDOW_BUTTON_ABOUT: "About",
    MAIN_WINDOW_BUTTON_QUIT: "Exit",
    QRCODE_WINDOW_TITLE: "QR Code",
    QRCODE_WINDOW_SAVE_AS_IMAGE: "Save as image",
    SETTINGS_WINDOW_TITLE: "Settings",
    SETTINGS_WINDOW_PROXY: "Use proxy to retrieve bridges",
    SETTINGS_WINDOW_BRIDGES_KEEP_OLD: "Keep old bridges",
    SETTINGS_WINDOW_BRIDGES_KEEP_OLD_TOOLTIP: "Append new bridges to a file, instead of overwriting",
    SETTINGS_WINDOW_TIMER_TITLE: "Get new bridges every",
    SETTINGS_WINDOW_CONNECT_ONION: "Connect to .onion website",
    SETTINGS_WINDOW_CONNECT_ONION_TOOLTIP: "Use the .onion version of bridges.torproject.org (if using Tor proxy)",
    SETTINGS_WINDOW_NOTIFICATIONS_SHOW: "Show notifications",
    SETTINGS_WINDOW_NOTIFICATIONS_SHOW_EVERY: "Show notifications every",
    SETTINGS_WINDOW_RUN_BACKGROUND: "Run in background",
    SETTINGS_WINDOW_BRIDGES_TRANSPORT: "Bridges transport type",
    SETTINGS_WINDOW_BRIDGES_SAVE: "Save bridges to file",
    SETTINGS_WINDOW_IPV6: "Get IPv6",
    SETTINGS_WINDOW_IPV6_TOOLTIP: "Get bridges with IPv6 addresses",
    SETTINGS_WINDOW_TORRC_MODIFY: "Modify bridges in torrc",
    SETTINGS_WINDOW_TORRC_DISABLE_OLD: "Disable old bridges",
    SETTINGS_WINDOW_TORRC_DISABLE_OLD_TOOLTIP: "Comment out old bridges in torrc",
};

pub fn get_days(count: i32) -> &'static str {
    if count == 1 {
        return "day";
    } else {
        return "days";
    }
}

pub fn get_hours(count: i32) -> &'static str {
    if count == 1 {
        return "hour";
    } else {
        return "hours";
    }
}

pub fn get_minutes(count: i32) -> &'static str {
    if count == 1 {
        return "minute";
    } else {
        return "minutes";
    }
}

pub fn get_seconds(count: i32) -> &'static str {
    if count == 1 {
        return "second";
    } else {
        return "seconds";
    }
}
