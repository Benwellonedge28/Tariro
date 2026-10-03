//! TUMC Internationalization
//!
//! Internationalization and localization support for Tariro programs.

use std::collections::HashMap;

/// Locale identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale {
    /// Language code
    pub language: String,
    /// Script code (optional)
    pub script: Option<String>,
    /// Region code (optional)
    pub region: Option<String>,
    /// Variant (optional)
    pub variant: Option<String>,
}

impl Locale {
    /// Create from BCP 47 tag
    pub fn from_bcp47(tag: &str) -> Self {
        let parts: Vec<&str> = tag.split('-').collect();

        let language = parts.first().unwrap_or(&"en").to_string();
        let script = parts.get(1).map(|s| {
            if s.len() == 4 {
                s.to_string()
            } else {
                String::new()
            }
        }).filter(|s| !s.is_empty());

        let region = parts.get(1..).and_then(|p| {
            if script.is_some() {
                p.get(1).map(|s| s.to_string())
            } else {
                p.first().map(|s| {
                    if s.len() == 2 || s.len() == 3 {
                        s.to_string()
                    } else {
                        String::new()
                    }
                }).filter(|s| !s.is_empty())
            }
        });

        Self {
            language,
            script,
            region,
            variant: None,
        }
    }

    /// Get default locale
    pub fn default() -> Self {
        Self::from_bcp47("en")
    }

    /// Get language tag
    pub fn to_tag(&self) -> String {
        let mut tag = self.language.clone();
        if let Some(ref script) = self.script {
            tag.push('-');
            tag.push_str(script);
        }
        if let Some(ref region) = self.region {
            tag.push('-');
            tag.push_str(region);
        }
        tag
    }
}

/// Message translator
pub struct Translator {
    /// Current locale
    locale: Locale,
    /// Loaded translations
    translations: HashMap<String, TranslationMap>,
    /// Fallback locale
    fallback: Locale,
}

impl Translator {
    /// Create new translator
    pub fn new(locale: Locale) -> Self {
        Self {
            locale: locale.clone(),
            translations: HashMap::new(),
            fallback: Locale::default(),
        }
    }

    /// Add translation
    pub fn add_translation(&mut self, key: &str, locale: &Locale, message: &str) {
        let locale_tag = locale.to_tag();
        self.translations
            .entry(locale_tag.clone())
            .or_insert_with(TranslationMap::new)
            .add(key, message);
    }

    /// Translate message
    pub fn translate(&self, key: &str, params: Option<&HashMap<String, String>>) -> String {
        let locale_tag = self.locale.to_tag();

        // Try current locale
        if let Some(map) = self.translations.get(&locale_tag) {
            if let Some(msg) = map.get(key) {
                return self.interpolate(msg, params);
            }
        }

        // Try language-only (en in en-US)
        let lang_only = self.locale.language.clone();
        if let Some(map) = self.translations.get(&lang_only) {
            if let Some(msg) = map.get(key) {
                return self.interpolate(msg, params);
            }
        }

        // Try fallback
        let fallback_tag = self.fallback.to_tag();
        if let Some(map) = self.translations.get(&fallback_tag) {
            if let Some(msg) = map.get(key) {
                return self.interpolate(msg, params);
            }
        }

        // Return key as last resort
        key.to_string()
    }

    /// Interpolate parameters
    fn interpolate(&self, message: &str, params: Option<&HashMap<String, String>>) -> String {
        let mut result = message.to_string();

        if let Some(params) = params {
            for (key, value) in params {
                let placeholder = format!("{{{}}}", key);
                result = result.replace(&placeholder, value);
            }
        }

        result
    }

    /// Set locale
    pub fn set_locale(&mut self, locale: Locale) {
        self.locale = locale;
    }

    /// Get current locale
    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

/// Translation map
#[derive(Debug, Clone)]
pub struct TranslationMap {
    messages: HashMap<String, String>,
}

impl TranslationMap {
    /// Create new map
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    /// Add translation
    pub fn add(&mut self, key: &str, message: &str) {
        self.messages.insert(key.to_string(), message.to_string());
    }

    /// Get translation
    pub fn get(&self, key: &str) -> Option<&String> {
        self.messages.get(key)
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<&String> {
        self.messages.keys().collect()
    }
}

impl Default for TranslationMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Plural rules
pub struct PluralRules {
    /// Locale
    locale: Locale,
}

impl PluralRules {
    /// Create new plural rules
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Get plural form index
    pub fn get(&self, n: u64) -> PluralForm {
        let lang = self.locale.language.as_str();

        match lang {
            "en" => Self::en_plural(n),
            "es" => Self::es_plural(n),
            "fr" => Self::fr_plural(n),
            "de" => Self::de_plural(n),
            "ja" | "ko" | "zh" => PluralForm::One,
            _ => Self::en_plural(n),
        }
    }

    /// English plural rules
    fn en_plural(n: u64) -> PluralForm {
        if n == 1 {
            PluralForm::One
        } else {
            PluralForm::Other
        }
    }

    /// Spanish plural rules
    fn es_plural(n: u64) -> PluralForm {
        if n == 1 {
            PluralForm::One
        } else {
            PluralForm::Other
        }
    }

    /// French plural rules
    fn fr_plural(n: u64) -> PluralForm {
        if n == 0 || n == 1 {
            PluralForm::One
        } else {
            PluralForm::Other
        }
    }

    /// German plural rules
    fn de_plural(n: u64) -> PluralForm {
        if n == 1 {
            PluralForm::One
        } else {
            PluralForm::Other
        }
    }
}

/// Plural forms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralForm {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

/// Number formatter
pub struct NumberFormatter {
    /// Locale
    locale: Locale,
    /// Decimal separator
    decimal_sep: String,
    /// Group separator
    group_sep: String,
    /// Currency symbol
    currency_symbol: String,
}

impl NumberFormatter {
    /// Create new formatter
    pub fn new(locale: Locale) -> Self {
        let (decimal_sep, group_sep, currency_symbol) = match locale.language.as_str() {
            "de" | "fr" | "es" | "it" => (",".to_string(), ".".to_string(), "€".to_string()),
            "ja" | "zh" => (".".to_string(), ",".to_string(), "¥".to_string()),
            _ => (".".to_string(), ",".to_string(), "$".to_string()),
        };

        Self {
            locale,
            decimal_sep,
            group_sep,
            currency_symbol,
        }
    }

    /// Format integer
    pub fn format_int(&self, n: i64) -> String {
        let s = n.abs().to_string();
        let negative = n < 0;

        // Add groups
        let mut result = String::new();
        let len = s.len();
        for (i, c) in s.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push_str(&self.group_sep);
            }
            result.push(c);
        }

        if negative {
            result = format!("-{}", result);
        }

        result
    }

    /// Format decimal
    pub fn format_decimal(&self, n: f64, decimals: usize) -> String {
        let multiplier = 10_f64.powi(decimals as i32);
        let int_part = (n * multiplier).round() as i64;
        let int_str = self.format_int(int_part / multiplier as i64);
        let dec_str = format!("{:0>width$}", int_part % multiplier as i64, width = decimals);

        if decimals > 0 {
            format!("{}{}{}", int_str, self.decimal_sep, dec_str)
        } else {
            int_str
        }
    }

    /// Format currency
    pub fn format_currency(&self, amount: f64) -> String {
        let formatted = self.format_decimal(amount, 2);
        format!("{}{}", self.currency_symbol, formatted)
    }

    /// Format percentage
    pub fn format_percent(&self, n: f64) -> String {
        let formatted = self.format_decimal(n * 100.0, 0);
        format!("{}%", formatted)
    }
}

/// Date formatter
pub struct DateFormatter {
    /// Locale
    locale: Locale,
    /// Date format
    date_format: String,
    /// Time format
    time_format: String,
}

impl DateFormatter {
    /// Create new formatter
    pub fn new(locale: Locale) -> Self {
        let (date_format, time_format) = match locale.language.as_str() {
            "en" => ("MM/DD/YYYY".to_string(), "HH:mm".to_string()),
            "de" => ("DD.MM.YYYY".to_string(), "HH:mm".to_string()),
            "fr" => ("DD/MM/YYYY".to_string(), "HH:mm".to_string()),
            "ja" => ("YYYY/MM/DD".to_string(), "HH:mm".to_string()),
            _ => ("YYYY-MM-DD".to_string(), "HH:mm".to_string()),
        };

        Self {
            locale,
            date_format,
            time_format,
        }
    }

    /// Format date
    pub fn format_date(&self, date: &DateTime) -> String {
        self.date_format
            .replace("YYYY", &date.year.to_string())
            .replace("MM", &format!("{:02}", date.month))
            .replace("DD", &format!("{:02}", date.day))
    }

    /// Format time
    pub fn format_time(&self, date: &DateTime) -> String {
        self.time_format
            .replace("HH", &format!("{:02}", date.hour))
            .replace("mm", &format!("{:02}", date.minute))
    }

    /// Format datetime
    pub fn format(&self, date: &DateTime) -> String {
        format!("{} {}", self.format_date(date), self.format_time(date))
    }
}

/// Date/time structure
#[derive(Debug, Clone)]
pub struct DateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
    /// Get current UTC time
    pub fn now_utc() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        let secs = now.as_secs();
        // Simplified - would use proper calendar calculation
        Self {
            year: 2024,
            month: 1,
            day: 1,
            hour: ((secs / 3600) % 24) as u8,
            minute: ((secs / 60) % 60) as u8,
            second: (secs % 60) as u8,
        }
    }
}

/// Message catalog
pub struct MessageCatalog {
    /// Messages by locale
    messages: HashMap<String, TranslationMap>,
    /// Current locale
    current: Locale,
}

impl MessageCatalog {
    /// Create new catalog
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
            current: Locale::default(),
        }
    }

    /// Load from JSON
    pub fn load_json(&mut self, locale: &Locale, json: &str) -> Result<(), ParseError> {
        // Simplified JSON parsing
        let map = TranslationMap::new();
        self.messages.insert(locale.to_tag(), map);
        Ok(())
    }

    /// Get translator
    pub fn translator(&self) -> Translator {
        let mut t = Translator::new(self.current.clone());
        for (tag, map) in &self.messages {
            let locale = Locale::from_bcp47(tag);
            for key in map.keys() {
                if let Some(msg) = map.get(key) {
                    t.add_translation(key, &locale, msg);
                }
            }
        }
        t
    }
}

impl Default for MessageCatalog {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse error
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

/// Resource bundle
pub struct ResourceBundle {
    /// Bundle name
    name: String,
    /// Locale
    locale: Locale,
    /// Resources
    resources: HashMap<String, String>,
}

impl ResourceBundle {
    /// Create new bundle
    pub fn new(name: &str, locale: Locale) -> Self {
        Self {
            name: name.to_string(),
            locale,
            resources: HashMap::new(),
        }
    }

    /// Add string resource
    pub fn add_string(&mut self, key: &str, value: &str) {
        self.resources.insert(key.to_string(), value.to_string());
    }

    /// Get string resource
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.resources.get(key)
    }

    /// Get locale
    pub fn locale(&self) -> &Locale {
        &self.locale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_bcp47() {
        let locale = Locale::from_bcp47("en-US");
        assert_eq!(locale.language, "en");
        assert_eq!(locale.region, Some("US".to_string()));
    }

    #[test]
    fn test_translator() {
        let mut t = Translator::new(Locale::from_bcp47("en"));
        t.add_translation("hello", &Locale::default(), "Hello, World!");
        assert_eq!(t.translate("hello", None), "Hello, World!");
    }

    #[test]
    fn test_plural_rules() {
        let rules = PluralRules::new(Locale::from_bcp47("en"));
        assert_eq!(rules.get(1), PluralForm::One);
        assert_eq!(rules.get(5), PluralForm::Other);
    }

    #[test]
    fn test_number_formatter() {
        let fmt = NumberFormatter::new(Locale::from_bcp47("en"));
        assert_eq!(fmt.format_int(1234567), "1,234,567");
        assert_eq!(fmt.format_decimal(1234.56, 2), "1,234.56");
    }
}
