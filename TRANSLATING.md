# Translating

You can translate the application to your language:

1. Copy file `src/locale/lang/locale_us.rs` to `src/locale/lang/locale_YOUR_LANGUAGE_CODE_HERE.rs`, and translate the strings in your new file. (You would also need to translate the functions for multiples)

2. In file `src/locale/mod.rs` add your language to the functions.

Example:

```
pub fn get_translation() -> Translation {
    match get_locale().unwrap_or_else(|| String::from("en-US")).as_str() {
        "en-US" => {
            return locale_us::TRANSLATION;
        },
        "YOUR_LANGUAGE_CODE" => {
            return locale_YOUR_LANGUAGE_CODE::TRANSLATION;
        },
        _ => {
            return locale_us::TRANSLATION;
        },
    }
}
```

3. Add translation path to the beginning of `src/locale/mod.rs`

```
#[path = "lang/locale_YOUR_LANGUAGE_CODE.rs"]
mod locale_YOUR_LANGUAGE_CODE;
```
