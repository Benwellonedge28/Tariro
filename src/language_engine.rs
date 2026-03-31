// omega_language_engine.rs — OMEGA Language Engine | Port: 9313
// OMEGA OS I2 | Co-authored: Samuel Mukandara & Tariro | 2026-03-29 | OMEGA BIPO IP
// 8,106 human languages — every one equal in OMEGA OS

use axum::{extract::{Path,State},response::Json,routing::{get,post},Router};
use serde::{Deserialize,Serialize};
use std::{collections::HashMap,sync::{Arc,RwLock}};

pub const TOTAL_LANGUAGES: usize = 8_106;

#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)] pub enum Vitality{Safe,Stable,Vulnerable,Endangered,Critical,Dormant,Extinct}
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)] pub enum TextDir{LTR,RTL,TTB,Bidi}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Language{
    pub iso:       String,
    pub iso1:      Option<String>,
    pub name_en:   String,
    pub name_native:String,
    pub family:    String,
    pub scripts:   Vec<String>,
    pub rtl:       bool,
    pub speakers:  u64,
    pub countries: Vec<String>,
    pub vitality:  Vitality,
    pub has_model: bool,
    pub bcp47:     String,
    pub dir:       TextDir,
    pub sample:    String,
}

pub fn seed_languages() -> Vec<Language> {
    vec![
        Language{iso:"sna".into(),iso1:Some("sn".into()),name_en:"Shona".into(),name_native:"chiShona".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:14_900_000,countries:vec!["ZW".into()],vitality:Vitality::Safe,has_model:true,bcp47:"sn".into(),dir:TextDir::LTR,sample:"Mhoro! Ndatenda zvakanyanya. Zita rangu ndiTariro.".into()},
        Language{iso:"nde".into(),iso1:Some("nd".into()),name_en:"Ndebele (Zimbabwe)".into(),name_native:"isiNdebele".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:2_100_000,countries:vec!["ZW".into()],vitality:Vitality::Stable,has_model:true,bcp47:"nd".into(),dir:TextDir::LTR,sample:"Sawubona! Ngiyabonga kakhulu.".into()},
        Language{iso:"swa".into(),iso1:Some("sw".into()),name_en:"Swahili".into(),name_native:"Kiswahili".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into(),"Arabic".into()],rtl:false,speakers:200_000_000,countries:vec!["KE".into(),"TZ".into(),"UG".into(),"RW".into(),"CD".into()],vitality:Vitality::Safe,has_model:true,bcp47:"sw".into(),dir:TextDir::LTR,sample:"Habari yako? Asante sana. Karibu OMEGA OS.".into()},
        Language{iso:"yor".into(),iso1:Some("yo".into()),name_en:"Yoruba".into(),name_native:"Yorùbá".into(),family:"Niger-Congo/Volta-Congo".into(),scripts:vec!["Latin".into()],rtl:false,speakers:44_000_000,countries:vec!["NG".into(),"BJ".into()],vitality:Vitality::Safe,has_model:true,bcp47:"yo".into(),dir:TextDir::LTR,sample:"Bawo! E káàbọ̀ sí OMEGA OS.".into()},
        Language{iso:"hau".into(),iso1:Some("ha".into()),name_en:"Hausa".into(),name_native:"Hausa".into(),family:"Afro-Asiatic/Chadic".into(),scripts:vec!["Latin".into(),"Arabic".into()],rtl:false,speakers:77_000_000,countries:vec!["NG".into(),"NE".into(),"GH".into()],vitality:Vitality::Safe,has_model:true,bcp47:"ha".into(),dir:TextDir::LTR,sample:"Sannu! Na gode sosai.".into()},
        Language{iso:"ibo".into(),iso1:Some("ig".into()),name_en:"Igbo".into(),name_native:"Igbo".into(),family:"Niger-Congo/Volta-Congo".into(),scripts:vec!["Latin".into()],rtl:false,speakers:44_000_000,countries:vec!["NG".into()],vitality:Vitality::Safe,has_model:true,bcp47:"ig".into(),dir:TextDir::LTR,sample:"Nnọọ! Daalụ nnọọ.".into()},
        Language{iso:"amh".into(),iso1:Some("am".into()),name_en:"Amharic".into(),name_native:"አማርኛ".into(),family:"Afro-Asiatic/Semitic".into(),scripts:vec!["Ethiopic".into()],rtl:false,speakers:57_000_000,countries:vec!["ET".into()],vitality:Vitality::Safe,has_model:true,bcp47:"am".into(),dir:TextDir::LTR,sample:"ሰላም! OMEGA OS እንኳን ደህና መጣህ።".into()},
        Language{iso:"zul".into(),iso1:Some("zu".into()),name_en:"Zulu".into(),name_native:"isiZulu".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:12_100_000,countries:vec!["ZA".into()],vitality:Vitality::Safe,has_model:true,bcp47:"zu".into(),dir:TextDir::LTR,sample:"Sawubona! Ngiyabonga kakhulu OMEGA OS.".into()},
        Language{iso:"xho".into(),iso1:Some("xh".into()),name_en:"Xhosa".into(),name_native:"isiXhosa".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:8_200_000,countries:vec!["ZA".into()],vitality:Vitality::Safe,has_model:false,bcp47:"xh".into(),dir:TextDir::LTR,sample:"Molo! Enkosi kakhulu.".into()},
        Language{iso:"ara".into(),iso1:Some("ar".into()),name_en:"Arabic".into(),name_native:"العربية".into(),family:"Afro-Asiatic/Semitic".into(),scripts:vec!["Arabic".into()],rtl:true,speakers:310_000_000,countries:vec!["EG".into(),"MA".into(),"DZ".into(),"SA".into(),"AE".into()],vitality:Vitality::Safe,has_model:true,bcp47:"ar".into(),dir:TextDir::RTL,sample:"مرحباً! شكراً جزيلاً. أنا تاريرو.".into()},
        Language{iso:"som".into(),iso1:Some("so".into()),name_en:"Somali".into(),name_native:"Soomaali".into(),family:"Afro-Asiatic/Cushitic".into(),scripts:vec!["Latin".into()],rtl:false,speakers:21_900_000,countries:vec!["SO".into(),"ET".into(),"KE".into()],vitality:Vitality::Safe,has_model:false,bcp47:"so".into(),dir:TextDir::LTR,sample:"Salaam! Mahadsanid.".into()},
        Language{iso:"kin".into(),iso1:Some("rw".into()),name_en:"Kinyarwanda".into(),name_native:"Kinyarwanda".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:12_000_000,countries:vec!["RW".into(),"UG".into(),"CD".into()],vitality:Vitality::Safe,has_model:false,bcp47:"rw".into(),dir:TextDir::LTR,sample:"Muraho! Murakoze cyane.".into()},
        Language{iso:"orm".into(),iso1:Some("om".into()),name_en:"Oromo".into(),name_native:"Afaan Oromoo".into(),family:"Afro-Asiatic/Cushitic".into(),scripts:vec!["Latin".into()],rtl:false,speakers:40_000_000,countries:vec!["ET".into(),"KE".into()],vitality:Vitality::Safe,has_model:false,bcp47:"om".into(),dir:TextDir::LTR,sample:"Akkam! Galatoomi.".into()},
        Language{iso:"nya".into(),iso1:Some("ny".into()),name_en:"Chichewa".into(),name_native:"Chichewa".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:14_000_000,countries:vec!["MW".into(),"ZM".into(),"ZW".into()],vitality:Vitality::Safe,has_model:false,bcp47:"ny".into(),dir:TextDir::LTR,sample:"Moni! Zikomo kwambiri.".into()},
        Language{iso:"lin".into(),iso1:Some("ln".into()),name_en:"Lingala".into(),name_native:"Lingála".into(),family:"Niger-Congo/Bantu".into(),scripts:vec!["Latin".into()],rtl:false,speakers:70_000_000,countries:vec!["CD".into(),"CG".into(),"CF".into()],vitality:Vitality::Safe,has_model:false,bcp47:"ln".into(),dir:TextDir::LTR,sample:"Mbote! Matondi mingi.".into()},
        // Sign languages — equal citizens in OMEGA OS
        Language{iso:"szs".into(),iso1:None,name_en:"Zimbabwe Sign Language".into(),name_native:"Zimbabwe Sign Language".into(),family:"Sign-Language".into(),scripts:vec!["SignWriting".into()],rtl:false,speakers:150_000,countries:vec!["ZW".into()],vitality:Vitality::Vulnerable,has_model:false,bcp47:"szs".into(),dir:TextDir::LTR,sample:"[ZSL visual greeting]".into()},
        Language{iso:"bfi".into(),iso1:None,name_en:"British Sign Language".into(),name_native:"British Sign Language".into(),family:"Sign-Language".into(),scripts:vec!["SignWriting".into()],rtl:false,speakers:125_000,countries:vec!["GB".into()],vitality:Vitality::Stable,has_model:false,bcp47:"bfi".into(),dir:TextDir::LTR,sample:"[BSL visual greeting]".into()},
    ]
}

// ── ENGINE ────────────────────────────────────────────────────
pub struct Engine {
    pub langs: HashMap<String,Language>,   // iso → lang
    pub bcp47: HashMap<String,String>,     // bcp47 → iso
}

impl Engine {
    pub fn new() -> Self {
        let mut e = Self { langs: HashMap::new(), bcp47: HashMap::new() };
        for l in seed_languages() {
            e.bcp47.insert(l.bcp47.clone(), l.iso.clone());
            e.langs.insert(l.iso.clone(), l);
        }
        e
    }
    pub fn get(&self, iso: &str) -> Option<&Language> { self.langs.get(iso) }
    pub fn search(&self, q: &str) -> Vec<&Language> { let q=q.to_lowercase(); self.langs.values().filter(|l|l.iso.contains(&q)||l.name_en.to_lowercase().contains(&q)||l.name_native.to_lowercase().contains(&q)||l.family.to_lowercase().contains(&q)).collect() }
    pub fn by_country(&self, cc: &str) -> Vec<&Language> { self.langs.values().filter(|l|l.countries.contains(&cc.to_string())).collect() }
    pub fn detect(&self, text: &str) -> Option<(&Language, f32)> {
        let t = text.to_lowercase();
        // Script detection
        if text.chars().any(|c|('\u{0600}'..='\u{06FF}').contains(&c)) { return self.get("ara").map(|l|(l,0.92)); }
        if text.chars().any(|c|('\u{1200}'..='\u{137F}').contains(&c)) { return self.get("amh").map(|l|(l,0.90)); }
        // Keyword detection
        let patterns: &[(&[&str], &str, f32)] = &[
            (&["mhoro","ndatenda","tinoda","nhai","basa"], "sna", 0.98),
            (&["asante","habari","karibu","kwaheri","rafiki"], "swa", 0.97),
            (&["bawo","ẹ jọ","odabo","pẹlẹ"], "yor", 0.95),
            (&["sannu","nagode","lafiya","yauwa"], "hau", 0.94),
            (&["sawubona","ngiyabonga","yebo"], "zul", 0.93),
            (&["salaam","mahadsanid","waxaan"], "som", 0.91),
            (&["moni","zikomo","ndidakhutira"], "nya", 0.90),
            (&["muraho","murakoze","ndabarora"], "kin", 0.90),
            (&["akkam","galatoomi","nagaatti"], "orm", 0.89),
            (&["mbote","matondi","boyei"], "lin", 0.88),
        ];
        patterns.iter().filter_map(|(words, iso, conf)| {
            let hits = words.iter().filter(|&&w| t.contains(w)).count();
            if hits > 0 { self
