#![allow(dead_code, unused)]
use crate::common::{deserialize_date, serialize_date, Data, File, Filter, GeoPoint2d, Order};
use geojson::GeoJson;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

#[doc = "# Bev\u{f6}lkerungsbestand nach Geschlecht, Alter, Gemeinde und Jahr (seit 2003)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10010/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10010/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik</p>"]
#[cfg(feature = "bl10010")]
pub mod bevoelkerungsbestand_nach_geschlecht_alter_gemeinde_und_jahr_seit_2003 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub gemeinde_nummer: Option<String>,
        pub gemeinde: Option<String>,
        pub bezirk_nummer: Option<String>,
        pub bezirk: Option<String>,
        pub versorgungsregion_code: Option<i64>,
        pub versorgungsregion: Option<String>,
        pub versorgungsregion_aggregiert_code: Option<i64>,
        pub versorgungsregion_aggregiert: Option<String>,
        pub geschlecht_code: Option<i64>,
        pub geschlecht: Option<String>,
        pub altersjahr_100_plus: Option<i64>,
        pub altersklasse_5_jahre_code: Option<i64>,
        pub altersklasse_5_jahre: Option<String>,
        pub anzahl_personen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        GemeindeNummer,
        Gemeinde,
        BezirkNummer,
        Bezirk,
        VersorgungsregionCode,
        Versorgungsregion,
        VersorgungsregionAggregiertCode,
        VersorgungsregionAggregiert,
        GeschlechtCode,
        Geschlecht,
        Altersjahr100Plus,
        Altersklasse5JahreCode,
        Altersklasse5Jahre,
        AnzahlPersonen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::GemeindeNummer => "gemeinde_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::VersorgungsregionCode => "versorgungsregion_code",
                Field::Versorgungsregion => "versorgungsregion",
                Field::VersorgungsregionAggregiertCode => "versorgungsregion_aggregiert_code",
                Field::VersorgungsregionAggregiert => "versorgungsregion_aggregiert",
                Field::GeschlechtCode => "geschlecht_code",
                Field::Geschlecht => "geschlecht",
                Field::Altersjahr100Plus => "altersjahr_100_plus",
                Field::Altersklasse5JahreCode => "altersklasse_5_jahre_code",
                Field::Altersklasse5Jahre => "altersklasse_5_jahre",
                Field::AnzahlPersonen => "anzahl_personen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10010/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bev\u{f6}lkerungsbestand nach Nationalit\u{e4}t, Konfession, Gemeinde und Quartal (seit 2003)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10020/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10020/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik (Quartalserhebung)</p>"]
#[cfg(feature = "bl10020")]
pub mod bevoelkerungsbestand_nach_nationalitaet_konfession_gemeinde_und_quartal_seit_2003 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub quartal: Option<i64>,
        pub gemeinde_nummer: Option<String>,
        pub gemeinde: Option<String>,
        pub bezirk_nummer: Option<String>,
        pub bezirk: Option<String>,
        pub nationalitaet_code: Option<i64>,
        pub nationalitaet: Option<String>,
        pub konfession_code: Option<i64>,
        pub konfession: Option<String>,
        pub anzahl_personen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Quartal,
        GemeindeNummer,
        Gemeinde,
        BezirkNummer,
        Bezirk,
        NationalitaetCode,
        Nationalitaet,
        KonfessionCode,
        Konfession,
        AnzahlPersonen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Quartal => "quartal",
                Field::GemeindeNummer => "gemeinde_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::NationalitaetCode => "nationalitaet_code",
                Field::Nationalitaet => "nationalitaet",
                Field::KonfessionCode => "konfession_code",
                Field::Konfession => "konfession",
                Field::AnzahlPersonen => "anzahl_personen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10020/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bev\u{f6}lkerungsbestand nach Geschlecht, Nationalit\u{e4}t, Zivilstand und Konfession (seit 1980)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10030/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10030/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik</p>"]
#[cfg(feature = "bl10030")]
pub mod bevoelkerungsbestand_nach_geschlecht_nationalitaet_zivilstand_und_konfession_seit_1980 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub geschlecht_code: Option<i64>,
        pub geschlecht: Option<String>,
        pub nationalitaet_code: Option<i64>,
        pub nationalitaet: Option<String>,
        pub zivilstand_aggregiert_code: Option<i64>,
        pub zivilstand_aggregiert: Option<String>,
        pub konfession_code: Option<i64>,
        pub konfession: Option<String>,
        pub anzahl_personen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        GeschlechtCode,
        Geschlecht,
        NationalitaetCode,
        Nationalitaet,
        ZivilstandAggregiertCode,
        ZivilstandAggregiert,
        KonfessionCode,
        Konfession,
        AnzahlPersonen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::GeschlechtCode => "geschlecht_code",
                Field::Geschlecht => "geschlecht",
                Field::NationalitaetCode => "nationalitaet_code",
                Field::Nationalitaet => "nationalitaet",
                Field::ZivilstandAggregiertCode => "zivilstand_aggregiert_code",
                Field::ZivilstandAggregiert => "zivilstand_aggregiert",
                Field::KonfessionCode => "konfession_code",
                Field::Konfession => "konfession",
                Field::AnzahlPersonen => "anzahl_personen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10030/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bev\u{f6}lkerungsbilanz nach Gemeinde und Jahr (seit 1980)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10040/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10040/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik</p>"]
#[cfg(feature = "bl10040")]
pub mod bevoelkerungsbilanz_nach_gemeinde_und_jahr_seit_1980 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub gemeinde_nummer: Option<String>,
        pub gemeinde: Option<String>,
        pub bezirk_nummer: Option<String>,
        pub bezirk: Option<String>,
        pub versorgungsregion_code: Option<i64>,
        pub versorgungsregion: Option<String>,
        pub versorgungsregion_aggriegiert_code: Option<i64>,
        pub versorgungsregion_aggriegiert: Option<String>,
        pub anfangsbestand: Option<i64>,
        pub geburten: Option<i64>,
        pub todesfaelle: Option<i64>,
        pub geburtenueberschuss: Option<i64>,
        pub zuzuege: Option<i64>,
        pub wegzuege: Option<i64>,
        pub wanderungssaldo: Option<i64>,
        pub bereinigung_saldo: Option<i64>,
        pub gesamtveraenderung: Option<i64>,
        pub endbestand: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        GemeindeNummer,
        Gemeinde,
        BezirkNummer,
        Bezirk,
        VersorgungsregionCode,
        Versorgungsregion,
        VersorgungsregionAggriegiertCode,
        VersorgungsregionAggriegiert,
        Anfangsbestand,
        Geburten,
        Todesfaelle,
        Geburtenueberschuss,
        Zuzuege,
        Wegzuege,
        Wanderungssaldo,
        BereinigungSaldo,
        Gesamtveraenderung,
        Endbestand,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::GemeindeNummer => "gemeinde_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::VersorgungsregionCode => "versorgungsregion_code",
                Field::Versorgungsregion => "versorgungsregion",
                Field::VersorgungsregionAggriegiertCode => "versorgungsregion_aggriegiert_code",
                Field::VersorgungsregionAggriegiert => "versorgungsregion_aggriegiert",
                Field::Anfangsbestand => "anfangsbestand",
                Field::Geburten => "geburten",
                Field::Todesfaelle => "todesfaelle",
                Field::Geburtenueberschuss => "geburtenueberschuss",
                Field::Zuzuege => "zuzuege",
                Field::Wegzuege => "wegzuege",
                Field::Wanderungssaldo => "wanderungssaldo",
                Field::BereinigungSaldo => "bereinigung_saldo",
                Field::Gesamtveraenderung => "gesamtveraenderung",
                Field::Endbestand => "endbestand",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10040/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Altersprognose nach Versorgungsregion, Geschlecht, Alter und Jahr (2020 mit Basis 2018)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10050/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10050/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik, Altersprognose BL 2020<br></p>"]
#[cfg(feature = "bl10050")]
pub mod altersprognose_nach_versorgungsregion_geschlecht_alter_und_jahr_2020_mit_basis_2018 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Erhebungsjahr
        pub jahr: Option<String>,
        /// Code der Versorgungsregion
        pub versorgungsregion_code: Option<i64>,
        /// Name der Versorgungsregion
        pub versorgungsregion: Option<String>,
        /// Geschlechter-Code
        pub geschlecht_code: Option<i64>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Alter in ganzen Jahren (Personen ≥100 werden mit 100 erfasst)
        pub altersjahr_100_plus: Option<i64>,
        /// Code der Altersklasse (in 5-Jahr-Schritten)
        pub altersklasse_5_jahre_code: Option<i64>,
        /// Altersklasse (in 5-Jahr-Schritten)
        pub altersklasse_5_jahre: Option<String>,
        /// Anzahl Personen
        pub anzahl_personen: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        VersorgungsregionCode,
        Versorgungsregion,
        GeschlechtCode,
        Geschlecht,
        Altersjahr100Plus,
        Altersklasse5JahreCode,
        Altersklasse5Jahre,
        AnzahlPersonen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::VersorgungsregionCode => "versorgungsregion_code",
                Field::Versorgungsregion => "versorgungsregion",
                Field::GeschlechtCode => "geschlecht_code",
                Field::Geschlecht => "geschlecht",
                Field::Altersjahr100Plus => "altersjahr_100_plus",
                Field::Altersklasse5JahreCode => "altersklasse_5_jahre_code",
                Field::Altersklasse5Jahre => "altersklasse_5_jahre",
                Field::AnzahlPersonen => "anzahl_personen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10050/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Haushalte nach Haushaltsgr\u{f6}sse, Gemeinde und Jahr (seit 2012)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10060/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10060/</a>\n"]
#[doc = "<p>Statistik der Bev\u{f6}lkerung und der Haushalte (STATPOP)<br></p>"]
#[cfg(feature = "bl10060")]
pub mod haushalte_nach_haushaltsgroesse_gemeinde_und_jahr_seit_2012 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Haushaltgrösse
        pub haushaltgrosse: Option<String>,
        /// Wert
        pub wert: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Haushaltgrosse,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Haushaltgrosse => "haushaltgrosse",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10060/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Vornamen der Neugeborenen nach Geschlecht und Jahr (seit 2021)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10070/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10070/</a>\n"]
#[doc = "<p>Statistik der nat\u{fc}rlichen Bev\u{f6}lkerungsbewegung (BEVNAT)</p><p>Nur die ersten 200 Vornamen (Rang) mit mindestens zwei Beobachtungen pro Vorname wurden ber\u{fc}cksichtigt.<br></p>"]
#[cfg(feature = "bl10070")]
pub mod vornamen_der_neugeborenen_nach_geschlecht_und_jahr_seit_2021 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Anzahl
        pub anzahl: Option<i64>,
        /// Rang_nach_Jahr
        pub rang_nach_jahr: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Vorname,
        Geschlecht,
        Anzahl,
        RangNachJahr,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Anzahl => "anzahl",
                Field::RangNachJahr => "rang_nach_jahr",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10070/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Mittlere Wohnbev\u{f6}lkerung nach Nationalit\u{e4}t, Gemeinde und Jahr (seit 1980)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10080/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10080/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik</p><p>Die mittlere Wohnbev\u{f6}lkerung entspricht dem gewichteten Durchschnitt der Quartalsbest\u{e4}nde aus der kantonalen Bev\u{f6}lkerungsstatistik. Sie hat damit einen anderen zeitlichen Bezug als der in der kantonalen Bev\u{f6}lkerungsstatistik \u{fc}bliche Jahresendbestand. Die mittlere Wohnbev\u{f6}lkerung dient als Grundlage, wenn das Mittel der im Kanton niedergelassenen Personen \u{fc}ber das gesamte Jahr interessiert und wird beispielsweise f\u{fc}r die Berechnung des Baselbieter Finanzausgleichs verwendet. Die Berechnungsformel lautet von unten nach oben gerechnet wie folgt: ((1 x 4. Quartal Vorjahr) + (2 x 1. Quartal Jahr) + (2 x 2. Quartal Jahr) + (2 x 3. Quartal Jahr) + (1 x 4. Quartal Jahr)) / 8.<br></p>"]
#[cfg(feature = "bl10080")]
pub mod mittlere_wohnbevoelkerung_nach_nationalitaet_gemeinde_und_jahr_seit_1980 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub gemeinde_nummer: Option<String>,
        pub gemeinde: Option<String>,
        pub bfs_bezirk: Option<String>,
        pub bezirk: Option<String>,
        pub nationalitaet_code: Option<i64>,
        pub nationalitaet: Option<String>,
        pub anzahl_personen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        GemeindeNummer,
        Gemeinde,
        BfsBezirk,
        Bezirk,
        NationalitaetCode,
        Nationalitaet,
        AnzahlPersonen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::GemeindeNummer => "gemeinde_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BfsBezirk => "bfs_bezirk",
                Field::Bezirk => "bezirk",
                Field::NationalitaetCode => "nationalitaet_code",
                Field::Nationalitaet => "nationalitaet",
                Field::AnzahlPersonen => "anzahl_personen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10080/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{dc}berbauungsstand nach Zone, Erschliessung, Gemeinde und Jahr (seit 2016)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10090/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10090/</a>\n"]
#[doc = "<p>Raumbeobachtung</p><p>\u{dc}berbaut: 1 = \u{fc}berbaut, 0 = nicht \u{fc}berbaut</p><p>Erschlossen: 1 = erschlossen, 0 = nicht erschlossen</p>"]
#[cfg(feature = "bl10090")]
pub mod ueberbauungsstand_nach_zone_erschliessung_gemeinde_und_jahr_seit_2016 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Ueberbaut
        pub ueberbaut: Option<i64>,
        /// Erschlossen
        pub erschlossen: Option<i64>,
        /// Zone_Code
        pub zone_code: Option<i64>,
        /// Zone
        pub zone: Option<String>,
        /// Flaeche_m2
        pub flaeche_m2: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Ueberbaut,
        Erschlossen,
        ZoneCode,
        Zone,
        FlaecheM2,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Ueberbaut => "ueberbaut",
                Field::Erschlossen => "erschlossen",
                Field::ZoneCode => "zone_code",
                Field::Zone => "zone",
                Field::FlaecheM2 => "flaeche_m2",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10090/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# R\u{e4}umliche Grundlagedaten nach Gemeinde (Januar 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10100/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10100/</a>\n"]
#[doc = "<p>GEOSTAT</p><p>Das Merkmal AREA_HA wird von swisstopo j\u{e4}hrlich basierend auf swissBOUNDARIES3D berechnet und weist \u{ab}offizielle\u{bb}, auf Hektaren gerundete Fl\u{e4}chenangaben aus. Die Daten sind f\u{fc}r beliebige Summenbildungen geeignet. F\u{fc}r die Gemeinden entspricht diese Fl\u{e4}chenangabe der Landfl\u{e4}che ohne allf\u{e4}llige Seefl\u{e4}chenanteile an Seen &gt; 5 km2. Kleinere Gew\u{e4}sser sind hingegen in dieser Fl\u{e4}che inbegriffen.<br></p><p><br></p>"]
#[cfg(feature = "bl10100")]
pub mod raeumliche_grundlagedaten_nach_gemeinde_januar_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Nummer
        ///
        /// Gemeindenummer
        pub gmdnr: Option<String>,
        /// Gemeinde
        pub gmdname: Option<String>,
        /// AREA_HA
        pub area_ha: Option<i64>,
        /// E_MIN
        ///
        /// E-Koordinate Minimum
        pub e_min: Option<i64>,
        /// E_MAX
        ///
        /// E-Koordinate Maximum
        pub e_max: Option<i64>,
        /// N_MIN
        ///
        /// N-Koordinate Minimum
        pub n_min: Option<i64>,
        /// N_MAX
        ///
        /// N-Koordinate Maximum
        pub n_max: Option<i64>,
        /// E_CNTR
        ///
        /// E-Koordinate Zentrum
        pub e_cntr: Option<i64>,
        /// N_CNTR
        ///
        /// N-Koordinate Zentrum
        pub n_cntr: Option<i64>,
        /// Z_MIN
        ///
        /// Minimale Höhe über Meer
        pub z_min: Option<i64>,
        /// Z_MAX
        ///
        /// Maximale Höhe über Meer
        pub z_max: Option<i64>,
        /// Z_AVG
        ///
        /// Durchschnittliche Höhe über Meer
        pub z_avg: Option<i64>,
        /// Z_CNTR
        ///
        /// Zentrale Höhe über Meer
        pub z_cntr: Option<i64>,
        /// Zentrumskoordinaten
        pub zentrumskoordinaten: Option<GeoPoint2d>,
        /// Geometrie
        pub geometry: Option<GeoJson>,
        /// Geometrisches Zentrum
        pub centroid: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Gmdnr,
        Gmdname,
        AreaHa,
        EMin,
        EMax,
        NMin,
        NMax,
        ECntr,
        NCntr,
        ZMin,
        ZMax,
        ZAvg,
        ZCntr,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Gmdnr => "gmdnr",
                Field::Gmdname => "gmdname",
                Field::AreaHa => "area_ha",
                Field::EMin => "e_min",
                Field::EMax => "e_max",
                Field::NMin => "n_min",
                Field::NMax => "n_max",
                Field::ECntr => "e_cntr",
                Field::NCntr => "n_cntr",
                Field::ZMin => "z_min",
                Field::ZMax => "z_max",
                Field::ZAvg => "z_avg",
                Field::ZCntr => "z_cntr",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10100/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Raumgliederungen nach Gemeinde (M\u{e4}rz 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10110/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10110/</a>\n"]
#[doc = "<p>R\u{e4}umliche Gliederungen</p>"]
#[cfg(feature = "bl10110")]
pub mod raumgliederungen_nach_gemeinde_maerz_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Bezirk_Nummer
        pub bezirk_nummer: Option<String>,
        /// Bezirk
        pub bezirk: Option<String>,
        /// Grossregion_Code
        pub grossregion_code: Option<i64>,
        /// Grossregion
        pub grossregion: Option<String>,
        /// Agglomeration_2020_Code
        pub agglomeration_2020_code: Option<String>,
        /// Agglomeration_2020
        pub agglomeration_2020: Option<String>,
        /// Agglomerationsgrössenklasse_2020_Code
        pub agglomerationsgrossenklasse_2020_code: Option<i64>,
        /// Agglomerationsgrössenklasse_2020
        pub agglomerationsgrossenklasse_2020: Option<String>,
        /// Städtischer_Charakter_2020_Code
        pub stadtischer_charakter_2020_code: Option<i64>,
        /// Städtischer_Charakter_2020
        pub stadtischer_charakter_2020: Option<String>,
        /// Statistische_Stadt_2020_Code
        pub statistische_stadt_2020_code: Option<i64>,
        /// Statistische_Stadt_2020
        pub statistische_stadt_2020: Option<String>,
        /// Städtisch_Ländlich_2020_Code
        pub stadtisch_landlich_2020_code: Option<i64>,
        /// Städtisch_Ländlich_2020
        pub stadtisch_landlich_2020: Option<String>,
        /// Gemeindetypologie_2020_9_Code
        pub gemeindetypologie_2020_9_code: Option<i64>,
        /// Gemeindetypologie_2020_9
        pub gemeindetypologie_2020_9: Option<String>,
        /// Gemeindetypologie_2020_25_Code
        pub gemeindetypologie_2020_25_code: Option<i64>,
        /// Gemeindetypologie_2020_25
        pub gemeindetypologie_2020_25: Option<String>,
        /// Arbeitsmarktgrossregion_2018_Code
        pub arbeitsmarktgrossregion_2018_code: Option<String>,
        /// Arbeitsmarktgrossregion_2018
        pub arbeitsmarktgrossregion_2018: Option<String>,
        /// Arbeitsmarktregion_2018_Code
        pub arbeitsmarktregion_2018_code: Option<String>,
        /// Arbeitsmarktregion_2018
        pub arbeitsmarktregion_2018: Option<String>,
        /// Berggebiet_2019_Code
        pub berggebiet_2019_code: Option<i64>,
        /// Berggebiet_2019
        pub berggebiet_2019: Option<String>,
        /// Urbanisierungsgrad_2011_Code
        pub urbanisierungsgrad_2011_code: Option<i64>,
        /// Urbanisierungsgrad_2011
        pub urbanisierungsgrad_2011: Option<String>,
        /// Erweiterte_Stadt_2011_Code
        pub erweiterte_stadt_2011_code: Option<String>,
        /// Erweiterte_Stadt_2011
        pub erweiterte_stadt_2011: Option<String>,
        /// Funktionales_städtisches_Gebiet_2014_Code
        pub funktionales_stadtisches_gebiet_2014_code: Option<String>,
        /// Funktionales_städtisches_Gebiet_2014
        pub funktionales_stadtisches_gebiet_2014: Option<String>,
        /// Geometrie
        pub geometry: Option<GeoJson>,
        /// Geometrisches_Zentrum
        pub centroid: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsNummer,
        Gemeinde,
        BezirkNummer,
        Bezirk,
        GrossregionCode,
        Grossregion,
        Agglomeration2020Code,
        Agglomeration2020,
        Agglomerationsgrossenklasse2020Code,
        Agglomerationsgrossenklasse2020,
        StadtischerCharakter2020Code,
        StadtischerCharakter2020,
        StatistischeStadt2020Code,
        StatistischeStadt2020,
        StadtischLandlich2020Code,
        StadtischLandlich2020,
        Gemeindetypologie20209Code,
        Gemeindetypologie20209,
        Gemeindetypologie202025Code,
        Gemeindetypologie202025,
        Arbeitsmarktgrossregion2018Code,
        Arbeitsmarktgrossregion2018,
        Arbeitsmarktregion2018Code,
        Arbeitsmarktregion2018,
        Berggebiet2019Code,
        Berggebiet2019,
        Urbanisierungsgrad2011Code,
        Urbanisierungsgrad2011,
        ErweiterteStadt2011Code,
        ErweiterteStadt2011,
        FunktionalesStadtischesGebiet2014Code,
        FunktionalesStadtischesGebiet2014,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::GrossregionCode => "grossregion_code",
                Field::Grossregion => "grossregion",
                Field::Agglomeration2020Code => "agglomeration_2020_code",
                Field::Agglomeration2020 => "agglomeration_2020",
                Field::Agglomerationsgrossenklasse2020Code => {
                    "agglomerationsgrossenklasse_2020_code"
                }
                Field::Agglomerationsgrossenklasse2020 => "agglomerationsgrossenklasse_2020",
                Field::StadtischerCharakter2020Code => "stadtischer_charakter_2020_code",
                Field::StadtischerCharakter2020 => "stadtischer_charakter_2020",
                Field::StatistischeStadt2020Code => "statistische_stadt_2020_code",
                Field::StatistischeStadt2020 => "statistische_stadt_2020",
                Field::StadtischLandlich2020Code => "stadtisch_landlich_2020_code",
                Field::StadtischLandlich2020 => "stadtisch_landlich_2020",
                Field::Gemeindetypologie20209Code => "gemeindetypologie_2020_9_code",
                Field::Gemeindetypologie20209 => "gemeindetypologie_2020_9",
                Field::Gemeindetypologie202025Code => "gemeindetypologie_2020_25_code",
                Field::Gemeindetypologie202025 => "gemeindetypologie_2020_25",
                Field::Arbeitsmarktgrossregion2018Code => "arbeitsmarktgrossregion_2018_code",
                Field::Arbeitsmarktgrossregion2018 => "arbeitsmarktgrossregion_2018",
                Field::Arbeitsmarktregion2018Code => "arbeitsmarktregion_2018_code",
                Field::Arbeitsmarktregion2018 => "arbeitsmarktregion_2018",
                Field::Berggebiet2019Code => "berggebiet_2019_code",
                Field::Berggebiet2019 => "berggebiet_2019",
                Field::Urbanisierungsgrad2011Code => "urbanisierungsgrad_2011_code",
                Field::Urbanisierungsgrad2011 => "urbanisierungsgrad_2011",
                Field::ErweiterteStadt2011Code => "erweiterte_stadt_2011_code",
                Field::ErweiterteStadt2011 => "erweiterte_stadt_2011",
                Field::FunktionalesStadtischesGebiet2014Code => {
                    "funktionales_stadtisches_gebiet_2014_code"
                }
                Field::FunktionalesStadtischesGebiet2014 => "funktionales_stadtisches_gebiet_2014",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10110/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wetterstation Basel / Binningen: Monatswerte Klimamessnetz (seit 1901)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10130/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10130/</a>\n"]
#[doc = "<p>Monatsdaten der NBCN-Station (Swiss National Basic Climate Network) Basel-Binningen</p>"]
#[cfg(feature = "bl10130")]
pub mod wetterstation_basel_binningen_monatswerte_klimamessnetz_seit_1901 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Erster Tag des Monats
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// station/location
        ///
        /// Messstation_Code
        pub station_location: Option<String>,
        /// Messstation
        pub station_name: Option<String>,
        /// Globalstrahlung (Monatsmittel)
        pub gre000m0: Option<i64>,
        /// Gesamtschneehöhe (Monatsmittel)
        pub hto000m0: Option<i64>,
        /// Gesamtbewölkung (Monatsmittel)
        pub nto000m0: Option<i64>,
        /// Luftdruck auf Stationshöhe (Monatsmittel)
        pub prestam0: Option<f64>,
        /// Niederschlag (Monatssumme)
        pub rre150m0: Option<f64>,
        /// Sonnenscheindauer (Monatssumme)
        pub sre000m0: Option<i64>,
        /// Lufttemperatur 2 m über Boden (Monatsmittel)
        pub tre200m0: Option<f64>,
        /// Lufttemperatur 2 m über Boden (absolutes Monatsminimum)
        pub tre200mn: Option<f64>,
        /// Lufttemperatur 2 m über Boden (absolutes Monatsmaximum)
        pub tre200mx: Option<f64>,
        /// Relative Luftfeuchtigkeit 2 m über Boden (Monatsmittel)
        pub ure200m0: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        StationLocation,
        StationName,
        Gre000m0,
        Hto000m0,
        Nto000m0,
        Prestam0,
        Rre150m0,
        Sre000m0,
        Tre200m0,
        Tre200mn,
        Tre200mx,
        Ure200m0,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::StationLocation => "station_location",
                Field::StationName => "station_name",
                Field::Gre000m0 => "gre000m0",
                Field::Hto000m0 => "hto000m0",
                Field::Nto000m0 => "nto000m0",
                Field::Prestam0 => "prestam0",
                Field::Rre150m0 => "rre150m0",
                Field::Sre000m0 => "sre000m0",
                Field::Tre200m0 => "tre200m0",
                Field::Tre200mn => "tre200mn",
                Field::Tre200mx => "tre200mx",
                Field::Ure200m0 => "ure200m0",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10130/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Klimanormwerte nach ausgew\u{e4}hlten Messstationen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10140/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10140/</a>\n"]
#[doc = "<p>Messgr\u{f6}ssen der Normperiode 1991-2020</p>"]
#[cfg(feature = "bl10140")]
pub mod klimanormwerte_nach_ausgewaehlten_messstationen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Referenzperiode
        pub referenzperiode: Option<String>,
        /// Station
        pub station: Option<String>,
        /// Parameter
        pub parameter: Option<String>,
        /// Einheit
        pub einheit: Option<String>,
        /// Jan
        pub jan: Option<f64>,
        /// Feb
        pub feb: Option<f64>,
        /// Mar
        pub mar: Option<f64>,
        /// Apr
        pub apr: Option<f64>,
        /// Mai
        pub mai: Option<f64>,
        /// Jun
        pub jun: Option<f64>,
        /// Jul
        pub jul: Option<f64>,
        /// Aug
        pub aug: Option<f64>,
        /// Sep
        pub sep: Option<f64>,
        /// Okt
        pub okt: Option<f64>,
        /// Nov
        pub nov: Option<f64>,
        /// Dez
        pub dez: Option<f64>,
        /// Jahr
        pub jahr: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Referenzperiode,
        Station,
        Parameter,
        Einheit,
        Jan,
        Feb,
        Mar,
        Apr,
        Mai,
        Jun,
        Jul,
        Aug,
        Sep,
        Okt,
        Nov,
        Dez,
        Jahr,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Referenzperiode => "referenzperiode",
                Field::Station => "station",
                Field::Parameter => "parameter",
                Field::Einheit => "einheit",
                Field::Jan => "jan",
                Field::Feb => "feb",
                Field::Mar => "mar",
                Field::Apr => "apr",
                Field::Mai => "mai",
                Field::Jun => "jun",
                Field::Jul => "jul",
                Field::Aug => "aug",
                Field::Sep => "sep",
                Field::Okt => "okt",
                Field::Nov => "nov",
                Field::Dez => "dez",
                Field::Jahr => "jahr",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10140/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Hotels und Kurbetriebe: Angebot und Nachfrage nach Gemeinde und Jahr (seit 2005)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10160/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10160/</a>\n"]
#[doc = "<p>Beherbergungsstatistik (HESTA). Klammern = Datenschutz bei weniger als 3 Betrieben. Die Daten des laufenden Jahres beinhalten die aktuell verf\u{fc}gbaren Monate.<br></p>"]
#[cfg(feature = "bl10160")]
pub mod hotels_und_kurbetriebe_angebot_und_nachfrage_nach_gemeinde_und_jahr_seit_2005 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Geoeffnete_Betriebe
        pub geoeffnete_betriebe: Option<f64>,
        /// Verfuegbare_Zimmer
        pub verfuegbare_zimmer: Option<i64>,
        /// Verfuegbare_Betten
        pub verfuegbare_betten: Option<i64>,
        /// Ankuenfte
        pub ankuenfte: Option<String>,
        /// Logiernaechte
        pub logiernaechte: Option<String>,
        /// Zimmernaechte
        pub zimmernaechte: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        GeoeffneteBetriebe,
        VerfuegbareZimmer,
        VerfuegbareBetten,
        Ankuenfte,
        Logiernaechte,
        Zimmernaechte,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::GeoeffneteBetriebe => "geoeffnete_betriebe",
                Field::VerfuegbareZimmer => "verfuegbare_zimmer",
                Field::VerfuegbareBetten => "verfuegbare_betten",
                Field::Ankuenfte => "ankuenfte",
                Field::Logiernaechte => "logiernaechte",
                Field::Zimmernaechte => "zimmernaechte",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10160/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{d6}ffentlich zug\u{e4}ngliche Gastwirtschaften nach Betriebsart und Standort (Februar 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10170/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10170/</a>\n"]
#[doc = "<p>Liste der vom Kanton BL bewilligten und \u{f6}ffentlich zug\u{e4}nglichen Gastwirtschaften. Stand: 29.02.2024</p><p>F\u{fc}r die F\u{fc}hrung eines Restaurants, Bistros, Caf\u{e9}s, etc. ist eine Betriebsbewilligung erforderlich.<br></p>"]
#[cfg(feature = "bl10170")]
pub mod oeffentlich_zugaengliche_gastwirtschaften_nach_betriebsart_und_standort_februar_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Betriebsnummer
        pub betriebsnummer: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Betriebsart
        pub betriebsart: Option<String>,
        /// Post_Adresse
        pub post_adresse: Option<String>,
        /// PLZ_Ort
        pub plz_ort: Option<String>,
        /// Bemerkung
        pub bemerkung: Option<String>,
        /// GWR_Adresse
        pub gwr_adresse: Option<String>,
        pub e_eingangskoordinate: Option<f64>,
        pub n_eingangskoordinate: Option<f64>,
        pub koordinaten: Option<GeoPoint2d>,
        pub egid: Option<i64>,
        pub baujahr_des_gebaeudes: Option<String>,
        pub name_des_gebaeudes: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Betriebsnummer,
        BfsGemeindenummer,
        Gemeinde,
        Name,
        Betriebsart,
        PostAdresse,
        PlzOrt,
        Bemerkung,
        GwrAdresse,
        EEingangskoordinate,
        NEingangskoordinate,
        Egid,
        BaujahrDesGebaeudes,
        NameDesGebaeudes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Betriebsnummer => "betriebsnummer",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Name => "name",
                Field::Betriebsart => "betriebsart",
                Field::PostAdresse => "post_adresse",
                Field::PlzOrt => "plz_ort",
                Field::Bemerkung => "bemerkung",
                Field::GwrAdresse => "gwr_adresse",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
                Field::Egid => "egid",
                Field::BaujahrDesGebaeudes => "baujahr_des_gebaeudes",
                Field::NameDesGebaeudes => "name_des_gebaeudes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10170/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Unternehmensneugr\u{fc}ndungen und Unternehmensschliessungen nach Wirtschaftssektor, Gemeinde und Jahr (seit 2013)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10180/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10180/</a>\n"]
#[doc = "Statistik der Unternehmensdemografie (UDEMO). Klammern = Datenschutz bei weniger als 4 Beobachtungen. Die Werte der Unternehmensschliessungen folgen jeweils mit 2 Jahren Verz\u{f6}gerung im Vergleich zu den Unternehmensneugr\u{fc}ndungen."]
#[cfg(feature = "bl10180")]
pub mod unternehmensneugruendungen_und_unternehmensschliessungen_nach_wirtschaftssektor_gemeinde_und_jahr_seit_2013 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// BFS_Bezeichnung
        pub bfs_bezeichnung: Option<String>,
        /// Administrative_Ebene
        pub administrative_ebene: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wirtschaftssektor
        pub wirtschaftssektor: Option<String>,
        /// Anzahl
        pub anzahl: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        BfsBezeichnung,
        AdministrativeEbene,
        Indikator,
        Wirtschaftssektor,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::BfsBezeichnung => "bfs_bezeichnung",
                Field::AdministrativeEbene => "administrative_ebene",
                Field::Indikator => "indikator",
                Field::Wirtschaftssektor => "wirtschaftssektor",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10180/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Endverbrauch von Elektrizit\u{e4}t nach Gemeinde und Jahr (seit 1990)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10190/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10190/</a>\n"]
#[doc = "<p>Energiestatistik</p>"]
#[cfg(feature = "bl10190")]
pub mod endverbrauch_von_elektrizitaet_nach_gemeinde_und_jahr_seit_1990 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        pub gemeinde: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10190/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Durchschnittlicher Quadratmeterpreis von Wohnbauland nach Gemeinde und Jahr (seit 1979)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10200/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10200/</a>\n"]
#[doc = "<p>Bodenpreisstatistik. (Klammern = Datenschutz bei weniger als 3 Transaktionen; leer =\u{a0} im entsprechenden Jahr wurden keine Transaktionen vorgenommen<font face=\"inherit\"><span style=\"font-size: 0.875rem;\">)</span></font></p><p>Vor 1994 ohne Daten f\u{fc}r den Bezirk Laufen<br></p>"]
#[cfg(feature = "bl10200")]
pub mod durchschnittlicher_quadratmeterpreis_von_wohnbauland_nach_gemeinde_und_jahr_seit_1979 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Fälle
        pub falle: Option<String>,
        /// Fläche_in_m2
        pub flache_in_m2: Option<String>,
        /// Quadratmeterpreis_CHF
        pub quadratmeterpreis_chf: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Falle,
        FlacheInM2,
        QuadratmeterpreisChf,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Falle => "falle",
                Field::FlacheInM2 => "flache_in_m2",
                Field::QuadratmeterpreisChf => "quadratmeterpreis_chf",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10200/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Durchschnittlicher Verkaufspreis von Eigentumswohnungen nach Zimmerzahl, Bezirk und Jahr (seit 2011)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10210/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10210/</a>\n"]
#[doc = "<p>Bodenpreisstatistik. (Klammern = Datenschutz bei weniger als 3 Transaktionen; fehlende Zeilen =\u{a0} im entsprechenden Jahr wurden keine Transaktionen f\u{fc}r die betreffende Wohnungsgr\u{f6}sse vorgenommen)</p>"]
#[cfg(feature = "bl10210")]
pub mod durchschnittlicher_verkaufspreis_von_eigentumswohnungen_nach_zimmerzahl_bezirk_und_jahr_seit_2011 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Bezirk_Nummer
        pub bezirk_nummer: Option<String>,
        /// Bezirk
        pub bezirk: Option<String>,
        /// Zimmerzahl
        pub zimmerzahl: Option<String>,
        /// Verkaufspreis_CHF
        pub verkaufspreis_chf: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BezirkNummer,
        Bezirk,
        Zimmerzahl,
        VerkaufspreisChf,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::Zimmerzahl => "zimmerzahl",
                Field::VerkaufspreisChf => "verkaufspreis_chf",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10210/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wohnungsbestand nach Zimmerzahl, Gemeinde und Jahr (seit 1994)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10220/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10220/</a>\n"]
#[doc = "<p>Fortschreibung des Wohnungsbestands (bis 2014), Geb\u{e4}ude- und Wohnungsstatistik (ab 2015)<br></p>"]
#[cfg(feature = "bl10220")]
pub mod wohnungsbestand_nach_zimmerzahl_gemeinde_und_jahr_seit_1994 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Zimmerzahl
        pub zimmerzahl: Option<String>,
        /// Anzahl_Wohnungen
        pub anzahl_wohnungen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Zimmerzahl,
        AnzahlWohnungen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Zimmerzahl => "zimmerzahl",
                Field::AnzahlWohnungen => "anzahl_wohnungen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10220/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Neu erstellte Wohnungen nach Gemeinde und Jahr (seit 1994)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10230/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10230/</a>\n"]
#[doc = "<p>Bau- und Wohnbaustatistik</p>"]
#[cfg(feature = "bl10230")]
pub mod neu_erstellte_wohnungen_nach_gemeinde_und_jahr_seit_1994 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Neu_erstellte_Wohnungen
        pub neu_erstellte_wohnungen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        NeuErstellteWohnungen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::NeuErstellteWohnungen => "neu_erstellte_wohnungen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10230/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Baukosten nach Art und Kategorie der Auftraggeber, Bezirk und Jahr (seit 1994)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10240/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10240/</a>\n"]
#[doc = "<p>Bau- und Wohnbaustatistik. Seit 2014 ohne die vom Bund erhobenen Tiefbauprojekte (SBB, Post, Swisscom, usw.).<br></p>"]
#[cfg(feature = "bl10240")]
pub mod baukosten_nach_art_und_kategorie_der_auftraggeber_bezirk_und_jahr_seit_1994 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Bezirk_Nummer
        pub bezirk_nummer: Option<String>,
        /// Bezirk
        pub bezirk: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BezirkNummer,
        Bezirk,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10240/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Leerwohnungsbestand nach Zimmerzahl, Gemeinde und Jahr (seit 2002)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10250/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10250/</a>\n"]
#[doc = "<p>Leer stehende Wohnungen am 1. Juni des jeweiligen Jahres</p>"]
#[cfg(feature = "bl10250")]
pub mod leerwohnungsbestand_nach_zimmerzahl_gemeinde_und_jahr_seit_2002 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Zimmerzahl
        pub zimmerzahl: Option<String>,
        /// Leer_stehende_Wohnungen
        pub leer_stehende_wohnungen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Zimmerzahl,
        LeerStehendeWohnungen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Zimmerzahl => "zimmerzahl",
                Field::LeerStehendeWohnungen => "leer_stehende_wohnungen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10250/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Leerwohnungsziffer nach Zimmerzahl, Gemeinde und Jahr (seit 2002)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10260/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10260/</a>\n"]
#[doc = "Leerwohnungsziffer (leer stehende Wohnungen im Verh\u{e4}ltnis zum Wohnungsbestand des Vorjahres) am 1. Juni des jeweiligen Jahres"]
#[cfg(feature = "bl10260")]
pub mod leerwohnungsziffer_nach_zimmerzahl_gemeinde_und_jahr_seit_2002 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Zimmerzahl
        pub zimmerzahl: Option<String>,
        /// Leerwohnungsziffer
        pub leerwohnungsziffer: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Zimmerzahl,
        Leerwohnungsziffer,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Zimmerzahl => "zimmerzahl",
                Field::Leerwohnungsziffer => "leerwohnungsziffer",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10260/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Baugesuche und Baubewilligungen nach Geb\u{e4}udeart, Gemeinde und Jahr (seit 1991/1992)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10270/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10270/</a>\n"]
#[doc = "<p>Bewilligte Gesuche, einschliesslich Nachtragsbewilligungen. Achtung: Die Daten k\u{f6}nnen sich r\u{fc}ckwirkend \u{e4}ndern! Die Gemeinde Liesberg wird aufgesplittet nach den beiden Ortschaften Liesberg und Liesberg Dorf ausgewiesen.</p><p><br></p>"]
#[cfg(feature = "bl10270")]
pub mod baugesuche_und_baubewilligungen_nach_gebaeudeart_gemeinde_und_jahr_seit_1991_1992 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Typ
        pub typ: Option<String>,
        /// Gebäudeart
        pub gebaudeart: Option<String>,
        /// Anzahl
        pub anzahl: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Typ,
        Gebaudeart,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Typ => "typ",
                Field::Gebaudeart => "gebaudeart",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10270/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Shared Mobility Angebote nach Anbieter und Standort"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10290/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10290/</a>\n"]
#[doc = "<p class=\"\" style=\"font-family: sans-serif;\">Shared Mobility Angebote, entnommen von <a href=\"sharedmobility.ch\" target=\"_blank\">sharedmobility.ch</a>. Erg\u{e4}nzt mit Gemeindename, gefiltert nach Kanton Basel-Landschaft.</p><p class=\"\" style=\"font-family: sans-serif;\"><b>Spalten</b></p><table class=\"table table-bordered\"><tbody><tr><td><b>coordinates, station_id, provider_id, name</b><br></td><td>Direkt \u{fc}bernommen gem\u{e4}ss <a href=\"github.com/SFOE/sharedmobility\" target=\"_blank\">github.com/SFOE/sharedmobility</a><br></td></tr><tr><td><b>gemeinde</b><br></td><td>Gemeindename, lokalisiert mit WGS84 Koordinaten in coordinates<br></td></tr><tr><td><b>record_date</b><br></td><td>Datum des letzten Abgleichs mit sharedmobility.ch<br></td></tr></tbody></table>"]
#[cfg(feature = "bl10290")]
pub mod shared_mobility_angebote_nach_anbieter_und_standort {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub coordinates: Option<GeoPoint2d>,
        pub station_id: Option<String>,
        pub provider_id: Option<String>,
        pub name: Option<String>,
        pub gemeinde: Option<String>,
        pub record_date: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        StationId,
        ProviderId,
        Name,
        Gemeinde,
        RecordDate,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::StationId => "station_id",
                Field::ProviderId => "provider_id",
                Field::Name => "name",
                Field::Gemeinde => "gemeinde",
                Field::RecordDate => "record_date",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10290/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Kennzahlen der Sozialhilfe nach Gemeinde und Jahr (seit 2005)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10300/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10300/</a>\n"]
#[doc = "<p>Sozialhilfeempf\u{e4}ngerstatistik (SHS), Bundesamt f\u{fc}r Statistik; Kantonale Bev\u{f6}lkerungsstatistik und Gemeindefinanzstatistik,\u{a0}Amt f\u{fc}r Daten und Statistik BL</p><p>Methodische Hinweise</p><p>Die Kennzahlen der Sozialhilfe BL werden nur f\u{fc}r Gemeinden mit 50 und mehr unterst\u{fc}tzten Personen im Erhebungsjahr ausgewiesen. Die Anzahl ausgewiesener Gemeinden kann daher von Jahr zu Jahr variieren.</p><p>Sozialhilfefall/unterst\u{fc}tzte Person</p><p>Gez\u{e4}hlt werden alle F\u{e4}lle/Personen, welche mindestens einmal im relevanten Jahr Sozialhilfeleistungen bezogen haben. Ein Fall kann mehrere unterst\u{fc}tzte Personen umfassen.\u{a0}</p><p>Sozialhilfequote</p><p>Die Sozialhilfequote entspricht dem Anteil der Sozialhilfebeziehenden an der Wohnbev\u{f6}lkerung. Als Referenzbev\u{f6}lkerung f\u{fc}r die Berechnung dient der Vorjahresendbestand der st\u{e4}ndigen Wohnbev\u{f6}lkerung gem\u{e4}ss STATPOP (Bundesamt f\u{fc}r Statistik; bis 2010 der Vorjahresendbestand der Wohnbev\u{f6}lkerung gem\u{e4}ss kantonaler Bev\u{f6}lkerungsstatistik).</p><p>Doppelz\u{e4}hlungen</p><p>Eine Unterst\u{fc}tzungseinheit kann pro Jahr in mehreren F\u{e4}llen unterst\u{fc}tzt werden. Zum Beispiel, wenn ein Wohnortswechsel erfolgt. Bei den Sozialhilfef\u{e4}llen und den unterst\u{fc}tzten Personen werden im Bezirks- und Kantonstotal Doppelz\u{e4}hlungen ausgeklammert, jedoch nicht bei den neuen/abgeschlossenen F\u{e4}llen und den neu eingetretenen/ausgetretenen Personen.</p><p>Abschl\u{fc}sse</p><p>Sozialhilfef\u{e4}lle werden abgeschlossen, wenn seit mehr als sechs Monaten keine Auszahlung mehr erfolgt ist. Es werden auch Abschl\u{fc}sse von F\u{e4}llen zum aktuellen Jahr gez\u{e4}hlt, welche im Vorjahr eine letzte Auszahlung erhalten haben und im laufenden Jahr abgeschlossen wurden. Falls nach einem Unterbruch von mehr als sechs Monaten erneut ein Antrag auf Sozialhilfe gestellt wird, wird ein neuer Fall er\u{f6}ffnet.</p><p>Nettoaufwand aus der Gemeindefinanzstatistik</p><p>Als Nettoaufwand gelten diejenigen Kosten (Aufwand), welche den Sozialhilfebeziehenden \u{ab}direkt\u{bb} zugutekommen abz\u{fc}glich den erhaltenen R\u{fc}ckerstattungen (Ertrag). Darin enthalten sind auch Eingliederungsmassnahmen. Hingegen ist der Verwaltungsaufwand (Sozialdienst und Sozialhilfebeh\u{f6}rde) nicht in der Auswertung enthalten. Die Kosten des vom Bund finanzierten Asylwesens sind nicht enthalten. Es werden somit die Kosten derjenigen Personen betrachtet, welche Sozialhilfeempf\u{e4}ngerstatistik (SHS) des Bundesamts f\u{fc}r Statistik gef\u{fc}hrt werden. Der Nettoaufwand steht ab dem Jahr 2014 zur Verf\u{fc}gung.</p><div><br></div>"]
#[cfg(feature = "bl10300")]
pub mod kennzahlen_der_sozialhilfe_nach_gemeinde_und_jahr_seit_2005 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub bfs_nummer: Option<String>,
        pub bfs_bezeichnung: Option<String>,
        pub administrative_ebene: Option<String>,
        pub kennzahl_mind_50_unterstuetze_personen: Option<String>,
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        BfsBezeichnung,
        AdministrativeEbene,
        KennzahlMind50UnterstuetzePersonen,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::BfsBezeichnung => "bfs_bezeichnung",
                Field::AdministrativeEbene => "administrative_ebene",
                Field::KennzahlMind50UnterstuetzePersonen => {
                    "kennzahl_mind_50_unterstuetze_personen"
                }
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10300/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Alterszentren und Pflegeheime nach Standort (Januar 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10310/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10310/</a>\n"]
#[doc = "<p>Liste der Pflegeheime</p>"]
#[cfg(feature = "bl10310")]
pub mod alterszentren_und_pflegeheime_nach_standort_januar_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Nummer
        ///
        /// BFS-Gemeindenummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        ///
        /// Gemeindename
        pub gemeinde: Option<String>,
        /// Versorgungsregion_Code
        pub versorgungsregion_code: Option<i64>,
        /// Versorgungsregion
        pub versorgungsregion: Option<String>,
        /// Institution
        ///
        /// Name der Institution
        pub institution: Option<String>,
        /// Post_Adresse
        ///
        /// Post-Adresse
        pub post_adresse: Option<String>,
        /// PLZ_Ort
        ///
        /// Postleitzahl Ort
        pub plz_ort: Option<String>,
        /// Telefon
        ///
        /// Telefonnummer
        pub telefon: Option<String>,
        /// E_Mail
        ///
        /// E-Mail-Adresse
        pub e_mail: Option<String>,
        /// Website
        ///
        /// Internetadresse
        pub website: Option<String>,
        /// GWR_Adresse
        ///
        /// GWR-Adresse
        pub gwr_adresse: Option<String>,
        /// GKODE
        ///
        /// Gebäudekoordinate Ost
        pub gkode: Option<f64>,
        /// GKODN
        ///
        /// Gebäudekoordinate West
        pub gkodn: Option<f64>,
        /// Koordinaten
        ///
        /// Koordinaten gemäss World Geodetic System 1984 (WGS84)
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsNummer,
        Gemeinde,
        VersorgungsregionCode,
        Versorgungsregion,
        Institution,
        PostAdresse,
        PlzOrt,
        Telefon,
        EMail,
        Website,
        GwrAdresse,
        Gkode,
        Gkodn,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::VersorgungsregionCode => "versorgungsregion_code",
                Field::Versorgungsregion => "versorgungsregion",
                Field::Institution => "institution",
                Field::PostAdresse => "post_adresse",
                Field::PlzOrt => "plz_ort",
                Field::Telefon => "telefon",
                Field::EMail => "e_mail",
                Field::Website => "website",
                Field::GwrAdresse => "gwr_adresse",
                Field::Gkode => "gkode",
                Field::Gkodn => "gkodn",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10310/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Apotheken mit Betriebsbewilligung oder Impfberechtigung nach Standort (April 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10320/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10320/</a>\n"]
#[doc = "<p>Liste der vom Kanton BL bewilligten Apotheken</p>"]
#[cfg(feature = "bl10320")]
pub mod apotheken_mit_betriebsbewilligung_oder_impfberechtigung_nach_standort_april_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Firma
        pub firma: Option<String>,
        /// Adresse
        pub adresse: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Ort
        pub ort: Option<String>,
        /// Telefon
        pub telefon: Option<String>,
        /// Herstellung
        pub herstellung: Option<String>,
        /// Impfen
        pub impfen: Option<String>,
        /// Vorname_Name
        pub vorname_name: Option<String>,
        pub e_eingangskoordinate: Option<i64>,
        pub n_eingangskoordinate: Option<i64>,
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Firma,
        Adresse,
        Plz,
        Ort,
        Telefon,
        Herstellung,
        Impfen,
        VornameName,
        EEingangskoordinate,
        NEingangskoordinate,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Firma => "firma",
                Field::Adresse => "adresse",
                Field::Plz => "plz",
                Field::Ort => "ort",
                Field::Telefon => "telefon",
                Field::Herstellung => "herstellung",
                Field::Impfen => "impfen",
                Field::VornameName => "vorname_name",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10320/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bewilligte Spitex-Organisationen nach Standort (August 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10330/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10330/</a>\n"]
#[doc = "<p>Liste der vom Kanton BL bewilligten Spitex-Organisationen</p>"]
#[cfg(feature = "bl10330")]
pub mod bewilligte_spitex_organisationen_nach_standort_august_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// ID
        pub id: Option<i64>,
        /// Organisation
        pub organisation: Option<String>,
        /// Post_Adresse
        pub post_adresse: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Ort
        pub ort: Option<String>,
        /// Telefon
        pub telefon: Option<String>,
        /// E_Mail
        pub e_mail: Option<String>,
        /// GWR_Adresse
        pub gwr_adresse: Option<String>,
        /// GKODE
        pub gkode: Option<f64>,
        /// GKODN
        pub gkodn: Option<f64>,
        /// Koordinaten
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Id,
        Organisation,
        PostAdresse,
        Plz,
        Ort,
        Telefon,
        EMail,
        GwrAdresse,
        Gkode,
        Gkodn,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Id => "id",
                Field::Organisation => "organisation",
                Field::PostAdresse => "post_adresse",
                Field::Plz => "plz",
                Field::Ort => "ort",
                Field::Telefon => "telefon",
                Field::EMail => "e_mail",
                Field::GwrAdresse => "gwr_adresse",
                Field::Gkode => "gkode",
                Field::Gkodn => "gkodn",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10330/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: T\u{e4}gliche Fallzahlen (Februar 2020 - Januar 2023)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10340/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10340/</a>\n"]
#[doc = "<p>Covid-19-Monitoring.\u{a0}T\u{e4}gliche Fallzahlen von Personen mit Wohnsitz BL.\u{a0}Die Daten werden nach dem 17.01.23 nicht mehr aktualisiert.<br></p>"]
#[cfg(feature = "bl10340")]
pub mod covid_19_taegliche_fallzahlen_februar_2020_januar_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
        /// geoRegion
        ///
        /// Kanton
        pub georegion: Option<String>,
        /// Tägliche Fälle
        pub entries: Option<i64>,
        /// sumTotal
        ///
        /// Total Fälle
        pub sumtotal: Option<i64>,
        /// Ständige Wohnbevölkerung 2020
        pub pop: Option<i64>,
        /// Version
        pub version: Option<String>,
        /// Tägliche Fälle pro 1000 Personen
        pub per1000persons: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Datum,
        Georegion,
        Entries,
        Sumtotal,
        Pop,
        Version,
        Per1000persons,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Datum => "datum",
                Field::Georegion => "georegion",
                Field::Entries => "entries",
                Field::Sumtotal => "sumtotal",
                Field::Pop => "pop",
                Field::Version => "version",
                Field::Per1000persons => "per1000persons",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10340/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: T\u{e4}gliche Todesf\u{e4}lle (Februar 2020 - Januar 2023)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10350/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10350/</a>\n"]
#[doc = "<p>Covid-19-Monitoring.\u{a0}T\u{e4}gliche Todesf\u{e4}lle von Personen mit Wohnsitz BL. Die Daten werden nach dem 17.01.23 nicht mehr aktualisiert.</p><p><br></p>"]
#[cfg(feature = "bl10350")]
pub mod covid_19_taegliche_todesfaelle_februar_2020_januar_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
        /// geoRegion
        ///
        /// Kanton
        pub georegion: Option<String>,
        /// Tägliche Todesfälle
        pub entries: Option<i64>,
        /// sumTotal
        ///
        /// Total Todesfälle
        pub sumtotal: Option<i64>,
        /// Version
        pub version: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Datum,
        Georegion,
        Entries,
        Sumtotal,
        Version,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Datum => "datum",
                Field::Georegion => "georegion",
                Field::Entries => "entries",
                Field::Sumtotal => "sumtotal",
                Field::Version => "version",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10350/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: T\u{e4}gliche Hospitalisierungen (Februar 2020 - Januar 2023)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10360/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10360/</a>\n"]
#[doc = "<p>Covid-19-Monitoring. T\u{e4}gliche Hospitalisierungen von Personen mit Wohnsitz BL.\u{a0}Die Daten werden nach dem 17.01.23 nicht mehr aktualisiert.<br></p>"]
#[cfg(feature = "bl10360")]
pub mod covid_19_taegliche_hospitalisierungen_februar_2020_januar_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
        /// geoRegion
        ///
        /// Kanton
        pub georegion: Option<String>,
        /// Tägliche Hospitalisierungen
        pub entries: Option<i64>,
        /// sumTotal
        ///
        /// Total Hospitalisierungen
        pub sumtotal: Option<i64>,
        /// Version
        pub version: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Datum,
        Georegion,
        Entries,
        Sumtotal,
        Version,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Datum => "datum",
                Field::Georegion => "georegion",
                Field::Entries => "entries",
                Field::Sumtotal => "sumtotal",
                Field::Version => "version",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10360/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: T\u{e4}gliche Spitalkapazit\u{e4}t (M\u{e4}rz 2020 - Mai 2023)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10370/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10370/</a>\n"]
#[doc = "<p>Covid-19-Monitoring.\u{a0}T\u{e4}gliche Spitalkapazit\u{e4}t von Spit\u{e4}lern im Kanton BL.\u{a0}Die Daten werden nach dem 02.05.23 nicht mehr aktualisiert.<br></p>"]
#[cfg(feature = "bl10370")]
pub mod covid_19_taegliche_spitalkapazitaet_maerz_2020_mai_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// geoRegion
        ///
        /// Kanton
        pub georegion: Option<String>,
        /// ICU_AllPatients
        ///
        /// Patienten Intensivstation
        pub icu_allpatients: Option<i64>,
        /// ICU_Covid19Patients
        ///
        /// Covid-19-Patienten
        pub icu_covid19patients: Option<i64>,
        /// ICU_Capacity
        ///
        /// Kapazität Intensivstation
        pub icu_capacity: Option<i64>,
        /// Total_AllPatients
        ///
        /// Total Patienten
        pub total_allpatients: Option<i64>,
        /// Total_Covid19Patients
        ///
        /// Total Covid-19-Patienten
        pub total_covid19patients: Option<i64>,
        /// Total_Capacity
        ///
        /// Total Kapazität
        pub total_capacity: Option<i64>,
        /// ICU_NonCovid19Patients
        ///
        /// Non-Covid-19-Patienten Intensivstation
        pub icu_noncovid19patients: Option<i64>,
        /// ICU_FreeCapacity
        ///
        /// Freie Kapazität Intensivstation
        pub icu_freecapacity: Option<i64>,
        /// Total_NonCovid19Patients
        ///
        /// Total Non-Covid-19-Patienten
        pub total_noncovid19patients: Option<i64>,
        /// Total_FreeCapacity
        ///
        /// Total Freie Kapazität
        pub total_freecapacity: Option<i64>,
        /// Typ_Variante
        pub type_variant: Option<String>,
        /// ICUPercent_AllPatients
        ///
        /// Anteil Patienten Intensivstation
        pub icupercent_allpatients: Option<f64>,
        /// ICUPercent_NonCovid19Patients
        ///
        /// Anteil Non-Covid-19-Patienten Intensivstation
        pub icupercent_noncovid19patients: Option<f64>,
        /// ICUPercent_Covid19Patients
        ///
        /// Anteil Covid-19-Patienten Intensivstation
        pub icupercent_covid19patients: Option<f64>,
        /// ICUPercent_FreeCapacity
        ///
        /// Anteil Freie Kapazität Intensivstation
        pub icupercent_freecapacity: Option<f64>,
        /// TotalPercent_AllPatients
        ///
        /// Total Auslastung
        pub totalpercent_allpatients: Option<f64>,
        /// TotalPercent_NonCovid19Patients
        ///
        /// Anteil Non-Covid-19-Patienten
        pub totalpercent_noncovid19patients: Option<f64>,
        /// TotalPercent_Covid19Patients
        ///
        /// Anteil Covid-19-Patienten
        pub totalpercent_covid19patients: Option<f64>,
        /// TotalPercent_FreeCapacity
        ///
        /// Anteil Freie Kapazität
        pub totalpercent_freecapacity: Option<f64>,
        /// Version
        pub version: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        Georegion,
        IcuAllpatients,
        IcuCovid19patients,
        IcuCapacity,
        TotalAllpatients,
        TotalCovid19patients,
        TotalCapacity,
        IcuNoncovid19patients,
        IcuFreecapacity,
        TotalNoncovid19patients,
        TotalFreecapacity,
        TypeVariant,
        IcupercentAllpatients,
        IcupercentNoncovid19patients,
        IcupercentCovid19patients,
        IcupercentFreecapacity,
        TotalpercentAllpatients,
        TotalpercentNoncovid19patients,
        TotalpercentCovid19patients,
        TotalpercentFreecapacity,
        Version,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::Georegion => "georegion",
                Field::IcuAllpatients => "icu_allpatients",
                Field::IcuCovid19patients => "icu_covid19patients",
                Field::IcuCapacity => "icu_capacity",
                Field::TotalAllpatients => "total_allpatients",
                Field::TotalCovid19patients => "total_covid19patients",
                Field::TotalCapacity => "total_capacity",
                Field::IcuNoncovid19patients => "icu_noncovid19patients",
                Field::IcuFreecapacity => "icu_freecapacity",
                Field::TotalNoncovid19patients => "total_noncovid19patients",
                Field::TotalFreecapacity => "total_freecapacity",
                Field::TypeVariant => "type_variant",
                Field::IcupercentAllpatients => "icupercent_allpatients",
                Field::IcupercentNoncovid19patients => "icupercent_noncovid19patients",
                Field::IcupercentCovid19patients => "icupercent_covid19patients",
                Field::IcupercentFreecapacity => "icupercent_freecapacity",
                Field::TotalpercentAllpatients => "totalpercent_allpatients",
                Field::TotalpercentNoncovid19patients => "totalpercent_noncovid19patients",
                Field::TotalpercentCovid19patients => "totalpercent_covid19patients",
                Field::TotalpercentFreecapacity => "totalpercent_freecapacity",
                Field::Version => "version",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10370/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: T\u{e4}gliche Tests nach Typ des Tests (Februar 2020 - Januar 2023)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10380/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10380/</a>\n"]
#[doc = "<p>Covid-19-Monitoring.\u{a0}T\u{e4}gliche Tests von Personen mit Wohnsitz BL.\u{a0}Die Daten werden nach dem 17.01.23 nicht mehr aktualisiert.<br></p>"]
#[cfg(feature = "bl10380")]
pub mod covid_19_taegliche_tests_nach_typ_des_tests_februar_2020_januar_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
        /// geoRegion
        ///
        /// Kanton
        pub georegion: Option<String>,
        /// Tägliche Tests
        pub entries: Option<i64>,
        /// sumTotal
        ///
        /// Total Tests
        pub sumtotal: Option<i64>,
        /// Tägliche positive Tests
        pub entries_pos: Option<i64>,
        /// Tägliche negative Tests
        pub entries_neg: Option<i64>,
        /// Anteil tägliche positive Tests
        pub pos_anteil: Option<f64>,
        /// Version
        pub version: Option<String>,
        /// Typ des Tests
        pub nachweismethode: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Datum,
        Georegion,
        Entries,
        Sumtotal,
        EntriesPos,
        EntriesNeg,
        PosAnteil,
        Version,
        Nachweismethode,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Datum => "datum",
                Field::Georegion => "georegion",
                Field::Entries => "entries",
                Field::Sumtotal => "sumtotal",
                Field::EntriesPos => "entries_pos",
                Field::EntriesNeg => "entries_neg",
                Field::PosAnteil => "pos_anteil",
                Field::Version => "version",
                Field::Nachweismethode => "nachweismethode",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10380/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: T\u{e4}glich geimpfte Personen nach Impfstoff und Typ der Impfung (Dezember 2020 - Mai 2023)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10390/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10390/</a>\n"]
#[doc = "<p>Covid-19-Monitoring. T\u{e4}glich geimpfte Personen mit Wohnsitz BL. Die Daten werden nach dem 02.05.23 nicht mehr aktualisiert.<br></p>"]
#[cfg(feature = "bl10390")]
pub mod covid_19_taeglich_geimpfte_personen_nach_impfstoff_und_typ_der_impfung_dezember_2020_mai_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// geoRegion
        ///
        /// Kanton
        pub georegion: Option<String>,
        /// Impfstoff
        pub vaccine: Option<String>,
        /// Tägliche Impfungen
        pub entries: Option<i64>,
        /// Ständige Wohnbevölkerung 2020
        pub pop: Option<i64>,
        /// sumTotal
        ///
        /// Total Impfungen
        pub sumtotal: Option<i64>,
        /// per100Persons
        ///
        /// Tägliche Impfungen pro 100 Personen
        pub per100persons: Option<f64>,
        /// per100PersonsTotal
        ///
        /// Total Impfungen pro 100 Personen
        pub per100personstotal: Option<f64>,
        /// type
        ///
        /// Typ der Impfung
        pub r#type: Option<String>,
        /// Version
        pub version: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        Georegion,
        Vaccine,
        Entries,
        Pop,
        Sumtotal,
        Per100persons,
        Per100personstotal,
        RType,
        Version,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::Georegion => "georegion",
                Field::Vaccine => "vaccine",
                Field::Entries => "entries",
                Field::Pop => "pop",
                Field::Sumtotal => "sumtotal",
                Field::Per100persons => "per100persons",
                Field::Per100personstotal => "per100personstotal",
                Field::RType => "type",
                Field::Version => "version",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10390/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Drogerien mit Betriebsbewilligung nach Standort (April 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10400/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10400/</a>\n"]
#[doc = "<p>Liste der vom Kanton BL bewilligten Drogerien</p>"]
#[cfg(feature = "bl10400")]
pub mod drogerien_mit_betriebsbewilligung_nach_standort_april_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Drogerie
        pub drogerie: Option<String>,
        /// Adresse
        pub adresse: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Ort
        pub ort: Option<String>,
        /// Telefon
        pub telefon: Option<String>,
        /// Herstellungsbewilligung
        pub herstellungsbewilligung: Option<String>,
        /// Vorname_Name
        pub vorname_name: Option<String>,
        /// GKODE
        pub gkode: Option<f64>,
        /// GKODN
        pub gkodn: Option<f64>,
        /// Koordinaten
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Drogerie,
        Adresse,
        Plz,
        Ort,
        Telefon,
        Herstellungsbewilligung,
        VornameName,
        Gkode,
        Gkodn,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Drogerie => "drogerie",
                Field::Adresse => "adresse",
                Field::Plz => "plz",
                Field::Ort => "ort",
                Field::Telefon => "telefon",
                Field::Herstellungsbewilligung => "herstellungsbewilligung",
                Field::VornameName => "vorname_name",
                Field::Gkode => "gkode",
                Field::Gkodn => "gkodn",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10400/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Lernende an Baselbieter Schulen nach Schulstufe und Geschlecht (seit 1986)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10410/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10410/</a>\n"]
#[doc = "<p class=\"\">Lernende Primarstufe, Sekundarstufe I und Sekundarstufe II, ohne Terti\u{e4}rstufe (Hochschulen und h\u{f6}here Berufsbildung)<br></p><p class=\"\"><b>Hinweise</b></p><ul><li>Sonderklassen umfassen die Einf\u{fc}hrungsklassen, Kleinklassen und Fremdsprachenintegrationsklassen</li><li>Zur separativen Sonderschulung werden auch die Heimschulen gez\u{e4}hlt</li><li>Ab 2015 aufgrund von Harmos sechs- anstatt f\u{fc}nfj\u{e4}hrige Primarschule, daf\u{fc}r drei- anstatt vierj\u{e4}hrige Sekundarschule.</li><li>Ab 1994 inkl. Bezirk Laufen.</li><li>EBA: Eidgen\u{f6}ssisches Berufsattest (2-j\u{e4}hrige Berufslehren)</li><li>EFZ: Eidgen\u{f6}ssisches F\u{e4}higkeitszeugnis (3- und 4-j\u{e4}hrige Berufslehren)</li></ul>"]
#[cfg(feature = "bl10410")]
pub mod lernende_an_baselbieter_schulen_nach_schulstufe_und_geschlecht_seit_1986 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Schulstufe_grob_Code
        pub schulstufe_grob_code: Option<i64>,
        /// Schulstufe_grob
        pub schulstufe_grob: Option<String>,
        /// Schulstufe_mittel_Code
        pub schulstufe_mittel_code: Option<i64>,
        /// Schulstufe_mittel
        pub schulstufe_mittel: Option<String>,
        /// Schulstufe_fein_Code
        pub schulstufe_fein_code: Option<i64>,
        /// Schulstufe_fein
        pub schulstufe_fein: Option<String>,
        /// Geschlecht_Code
        pub geschlecht_code: Option<i64>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        SchulstufeGrobCode,
        SchulstufeGrob,
        SchulstufeMittelCode,
        SchulstufeMittel,
        SchulstufeFeinCode,
        SchulstufeFein,
        GeschlechtCode,
        Geschlecht,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::SchulstufeGrobCode => "schulstufe_grob_code",
                Field::SchulstufeGrob => "schulstufe_grob",
                Field::SchulstufeMittelCode => "schulstufe_mittel_code",
                Field::SchulstufeMittel => "schulstufe_mittel",
                Field::SchulstufeFeinCode => "schulstufe_fein_code",
                Field::SchulstufeFein => "schulstufe_fein",
                Field::GeschlechtCode => "geschlecht_code",
                Field::Geschlecht => "geschlecht",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10410/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Lernende mit Wohnkanton BL an Schulen in der Schweiz nach Schulstufe, Geschlecht, Wohngemeinde und Jahr (seit 2014)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10420/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10420/</a>\n"]
#[doc = "<p>Statistik der Lernenden</p><p>Ab 2015 aufgrund von HarmoS 6- statt 5-j\u{e4}hrige Primarschule, daf\u{fc}r 3- statt 4-j\u{e4}hrige Sekundarschule.</p><p>Separative Sonderschulung inkl. Heimschulen.\u{a0}</p><p>EBA = Eidg. Berufsattest (2-j\u{e4}hrige Berufslehren). </p><p>EFZ = Eidg. F\u{e4}higkeitszeugnis (3- und 4-j\u{e4}hrige Berufslehren).\u{a0}</p>"]
#[cfg(feature = "bl10420")]
pub mod lernende_mit_wohnkanton_bl_an_schulen_in_der_schweiz_nach_schulstufe_geschlecht_wohngemeinde_und_jahr_seit_2014 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub bfs_nummer: Option<String>,
        pub wohngemeinde: Option<String>,
        pub schulstufe_grob_code: Option<i64>,
        pub schulstufe_grob: Option<String>,
        pub schulstufe_mittel_code: Option<i64>,
        pub schulstufe_mittel: Option<String>,
        pub schulstufe_fein_code: Option<i64>,
        pub schulstufe_fein: Option<String>,
        pub geschlecht_code: Option<i64>,
        pub geschlecht: Option<String>,
        pub indikator: Option<String>,
        pub wert: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Wohngemeinde,
        SchulstufeGrobCode,
        SchulstufeGrob,
        SchulstufeMittelCode,
        SchulstufeMittel,
        SchulstufeFeinCode,
        SchulstufeFein,
        GeschlechtCode,
        Geschlecht,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Wohngemeinde => "wohngemeinde",
                Field::SchulstufeGrobCode => "schulstufe_grob_code",
                Field::SchulstufeGrob => "schulstufe_grob",
                Field::SchulstufeMittelCode => "schulstufe_mittel_code",
                Field::SchulstufeMittel => "schulstufe_mittel",
                Field::SchulstufeFeinCode => "schulstufe_fein_code",
                Field::SchulstufeFein => "schulstufe_fein",
                Field::GeschlechtCode => "geschlecht_code",
                Field::Geschlecht => "geschlecht",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10420/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Abschl\u{fc}sse von Studierenden mit Wohnkanton BL an Schweizer Hochschulen nach Hochschultyp, Fachbereich, Geschlecht, Examensstufe und Jahr (seit 1980)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10430/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10430/</a>\n"]
#[doc = "<p class=\"\">Schweizerisches Hochschulinformationssystem (SHIS)</p><p class=\"\">Die \u{fc}brigen Abschl\u{fc}sse umfassen Abschlussexamen ohne akademischen Grad, Gymnasiallehrer/innen, Nachdiplome (bis 2004), Sekundarlehrer/innen sowie universit\u{e4}re Aufbau- und Vertiefungsstudien.</p>"]
#[cfg(feature = "bl10430")]
pub mod abschluesse_von_studierenden_mit_wohnkanton_bl_an_schweizer_hochschulen_nach_hochschultyp_fachbereich_geschlecht_examensstufe_und_jahr_seit_1980 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Hochschultyp
        pub hochschultyp: Option<String>,
        /// Fachbereich
        pub fachbereich: Option<String>,
        /// Geschlecht_Code
        pub geschlecht_code: Option<i64>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Examensstufe
        pub examensstufe: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Hochschultyp,
        Fachbereich,
        GeschlechtCode,
        Geschlecht,
        Examensstufe,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Hochschultyp => "hochschultyp",
                Field::Fachbereich => "fachbereich",
                Field::GeschlechtCode => "geschlecht_code",
                Field::Geschlecht => "geschlecht",
                Field::Examensstufe => "examensstufe",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10430/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bewilligte Tagesbetreuungseinrichtungen f\u{fc}r Kinder nach Standort (Oktober 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10440/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10440/</a>\n"]
#[doc = "<p>Liste der vom Kanton BL bewilligten Tagesbetreuungseinrichtungen f\u{fc}r Kinder. Stand per Anfang Monat.</p>"]
#[cfg(feature = "bl10440")]
pub mod bewilligte_tagesbetreuungseinrichtungen_fuer_kinder_nach_standort_oktober_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS
        ///
        /// Gemeindenummer gemäss Bundesamt für Statistik
        pub bfs: Option<String>,
        /// Gemeinde
        ///
        /// Gemeindename
        pub gemeinde: Option<String>,
        /// Institution
        pub institution: Option<String>,
        /// Post_Adresse
        ///
        /// Post-Adresse
        pub post_adresse: Option<String>,
        /// PLZ_Ort
        ///
        /// Postleitzahl und Ort
        pub plz_ort: Option<String>,
        /// Telefon
        ///
        /// Telefonnummer
        pub telefon: Option<String>,
        /// Mail
        ///
        /// E-Mail-Adresse
        pub mail: Option<String>,
        /// URL
        ///
        /// Webpage
        pub url: Option<String>,
        /// Trägerschaft
        pub tragerschaft: Option<String>,
        /// Rechtsform
        pub rechtsform: Option<String>,
        /// Alter
        ///
        /// Betreuungsalter
        pub alter: Option<String>,
        /// Anzahl_Plätze_Tagesbetreuung
        ///
        /// Anzahl Plätze Tagesbetreuung
        pub anzahl_platze_tagesbetreuung: Option<i64>,
        /// Anzahl_Plätze_Mittagstisch
        ///
        /// [...] = Anzahl Plätze für beide Einrichtungen zusammen
        pub anzahl_platze_mittagstisch: Option<i64>,
        /// Anzahl_Plätze_Nachmittagsbetreuung
        ///
        /// [...] = Anzahl Plätze für beide Einrichtungen zusammen
        pub anzahl_platze_nachmittagsbetreuung: Option<i64>,
        /// Wochentage
        ///
        /// Wochentage mit Betreuung
        pub wochentage: Option<String>,
        /// GWR_Adresse
        ///
        /// Gebäudeadresse gemäss kantonalem Gebäude- und Wohnungsregister
        pub gwr_adresse: Option<String>,
        /// GKODE
        ///
        /// Gebäudekoordinate Ost
        pub gkode: Option<f64>,
        /// GKODN
        ///
        /// Gebäudekoordinate Nord
        pub gkodn: Option<f64>,
        /// Koordinaten
        ///
        /// Koordinaten gemäss World Geodetic System 1984 (WGS84)
        pub koordinaten: Option<GeoPoint2d>,
        /// GBEZ
        ///
        /// Gebäudebezeichnung
        pub gbez: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Bfs,
        Gemeinde,
        Institution,
        PostAdresse,
        PlzOrt,
        Telefon,
        Mail,
        Url,
        Tragerschaft,
        Rechtsform,
        Alter,
        AnzahlPlatzeTagesbetreuung,
        AnzahlPlatzeMittagstisch,
        AnzahlPlatzeNachmittagsbetreuung,
        Wochentage,
        GwrAdresse,
        Gkode,
        Gkodn,
        Gbez,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Bfs => "bfs",
                Field::Gemeinde => "gemeinde",
                Field::Institution => "institution",
                Field::PostAdresse => "post_adresse",
                Field::PlzOrt => "plz_ort",
                Field::Telefon => "telefon",
                Field::Mail => "mail",
                Field::Url => "url",
                Field::Tragerschaft => "tragerschaft",
                Field::Rechtsform => "rechtsform",
                Field::Alter => "alter",
                Field::AnzahlPlatzeTagesbetreuung => "anzahl_platze_tagesbetreuung",
                Field::AnzahlPlatzeMittagstisch => "anzahl_platze_mittagstisch",
                Field::AnzahlPlatzeNachmittagsbetreuung => "anzahl_platze_nachmittagsbetreuung",
                Field::Wochentage => "wochentage",
                Field::GwrAdresse => "gwr_adresse",
                Field::Gkode => "gkode",
                Field::Gkodn => "gkodn",
                Field::Gbez => "gbez",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10440/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung vom 1. Oktober 2021 des Bundesgesetzes \u{fc}ber Filmproduktion und Filmkultur"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10450/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10450/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 15. Mai 2022<br></p>"]
#[cfg(feature = "bl10450")]
pub mod aenderung_vom_1_oktober_2021_des_bundesgesetzes_ueber_filmproduktion_und_filmkultur {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<String>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10450/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung vom 1. Oktober 2021 des Bundesgesetzes \u{fc}ber die Transplantation von Organen, Geweben und ZeIlen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10460/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10460/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 15. Mai 2022<br></p>"]
#[cfg(feature = "bl10460")]
pub mod aenderung_vom_1_oktober_2021_des_bundesgesetzes_ueber_die_transplantation_von_organen_geweben_und_zeilen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<String>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10460/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{dc}bernahme der EU-Verordnung \u{fc}ber die Europ\u{e4}ische Grenz- und K\u{fc}stenwache"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10470/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10470/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 15. Mai 2022<br></p>"]
#[cfg(feature = "bl10470")]
pub mod uebernahme_der_eu_verordnung_ueber_die_europaeische_grenz_und_kuestenwache {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub eligible_voters: Option<i64>,
        pub expats: Option<String>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        TitleDeCh,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::TitleDeCh => "title_de_ch",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10470/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung der Kantonsverfassung vom 13. Januar 2022 betreffend Anpassung der Bestimmungen \u{fc}ber die Ombudsperson"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10480/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10480/</a>\n"]
#[doc = "<p>Kantonale Abstimmung vom 15. Mai 2022<br></p>"]
#[cfg(feature = "bl10480")]
pub mod aenderung_der_kantonsverfassung_vom_13_januar_2022_betreffend_anpassung_der_bestimmungen_ueber_die_ombudsperson {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// BFS-Gemeindenummer (0 = Auslandschweizer)
        pub entity_id: Option<String>,
        /// Gemeindename
        pub name: Option<String>,
        /// Anzahl Stimmberechtigter
        pub eligible_voters: Option<i64>,
        /// Anzahl leerer Stimmen
        pub empty: Option<i64>,
        /// Anzahl Auslandschweizer
        pub expats: Option<String>,
        /// Anzahl ungültiger Stimmen
        pub invalid: Option<i64>,
        /// Anzahl Ja-Stimmen
        pub yeas: Option<i64>,
        /// Anzahl Nein-Stimmen
        pub nays: Option<i64>,
        /// title_de_CH
        ///
        /// Vorlagentitel
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10480/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Teilrevision des Sozialhilfegesetzes vom 4. November 2021 betreffend \u{ab}Anreize st\u{e4}rken \u{2013} Arbeitsintegration f\u{f6}rdern\u{bb}"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10490/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10490/</a>\n"]
#[doc = "<p>Kantonale Abstimmung vom 15. Mai 2022<br></p>"]
#[cfg(feature = "bl10490")]
pub mod teilrevision_des_sozialhilfegesetzes_vom_4_november_2021_betreffend_anreize_staerken_arbeitsintegration_foerdern {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<String>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10490/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Abstimmungsarchiv nach Vorlage und Datum (seit 2003)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10500/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10500/</a>\n"]
#[doc = "<p>Kantonsresultate aller eidgen\u{f6}ssischen und kantonalen Vorlagen</p><p><b>Hinweis</b>: Der Datensatz wird an Abstimmungssonntagen nach der abgeschlossenen Ausz\u{e4}hlung aktualisiert.\u{a0}Detaillierte Angaben zu den Ergebnissen der Gemeinden befinden sich im Datensatz\u{a0}<a href=\"https://data.bl.ch/explore/dataset/11990/table/?disjunctive.name&amp;disjunctive.domain0&amp;sort=date\" target=\"_blank\">Abstimmungsarchiv nach Vorlage, Gemeinde und Datum (seit 2003)</a>.</p><p><b>Spaltenbeschriebe</b></p><li><strong>date</strong>: Das Datum, an dem die Abstimmung stattgefunden hat.</li><li><strong>vote_id</strong>: Eine eindeutige Identifikationsnummer f\u{fc}r jede Abstimmungsvorlage bestehend aus dem Datum des Abstimmungstags und einem pro Abstimmungstag eindeutigen K\u{fc}rzel.</li><li><strong>domain</strong>: Der Geltungsbereich der Abstimmung, z. B. \"federation\" f\u{fc}r nationale Abstimmungen oder \"canton\" f\u{fc}r kantonale Abstimmungen.</li><li><strong>type</strong>: Die Art der Vorlage: \"proposal\" f\u{fc}r einen Vorschlag, \"counter-proposal\" f\u{fc}r einen Gegenvorschlag oder \"tie-breaker\" f\u{fc}r eine Stichfrage.</li><li><strong>title_de_CH</strong>: Der Titel der Vorlage in deutscher Sprache.</li><li><strong>entities_total</strong>: Die Gesamtzahl der Gemeinden, die an der Abstimmung teilnehmen.</li><li><strong>entities_counted</strong>: Die Anzahl der Gemeinden, deren Ergebnisse bereits ausgez\u{e4}hlt wurden.</li><li><strong>answer</strong>: Das Ergebnis der Abstimmung: \"accepted\" = Vorschlag oder Gegenvorschlag angenommen, \"rejected\" = Vorschlag oder Gegenvorschlag abgelehnt, \"proposal\" = Stichentscheid f\u{fc}r Vorschlag, \"counter-proposal\" = Stichentscheid f\u{fc}r Gegenvorschlag.</li><li><strong>percent_yeas</strong>: Der Prozentsatz der abgegebenen Stimmen, die f\u{fc}r die Vorlage gestimmt haben.</li><li><strong>percent_nays</strong>: Der Prozentsatz der abgegebenen Stimmen, die gegen die Vorlage gestimmt haben.</li><li><strong>percent_turnout</strong>: Die Wahlbeteiligung, gemessen als Prozentsatz der Stimmberechtigten, die an der Abstimmung teilgenommen haben.</li><li><strong>eligible_voters</strong>: Die Anzahl der Personen, die wahlberechtigt waren.</li><li><strong>expats</strong>: Die Anzahl der stimmberechtigten Schweizer B\u{fc}rger, die im Ausland leben und an der Abstimmung teilgenommen haben.</li><li><strong>empty</strong>: Die Anzahl der abgegebenen, aber leeren Stimmzettel.</li><li><strong>invalid</strong>: Die Anzahl der abgegebenen, aber ung\u{fc}ltigen Stimmzettel.</li><li><strong>yeas</strong>: Die absolute Anzahl der Ja-Stimmen.</li><li><strong>nays</strong>: Die absolute Anzahl der Nein-Stimmen.</li><li><strong>url</strong>: Der Weblink zu den detaillierten Abstimmungsergebnissen auf wahlen.bl.ch.</li><p><br></p><p><br></p>"]
#[cfg(feature = "bl10500")]
pub mod abstimmungsarchiv_nach_vorlage_und_datum_seit_2003 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub vote_id: Option<String>,
        /// domain
        ///
        /// Ebene
        pub domain0: Option<String>,
        /// type
        ///
        /// Typ
        pub r#type: Option<String>,
        /// title_de_CH
        ///
        /// Titel
        pub title_de_ch: Option<String>,
        /// Auszuzählende Gemeinden
        pub entities_total: Option<i64>,
        /// Ausgezählte Gemeinden
        pub entities_counted: Option<i64>,
        /// Resultat
        pub answer: Option<String>,
        /// Ja-Anteil
        pub percent_yeas: Option<f64>,
        /// Nein-Anteil
        pub percent_nays: Option<f64>,
        /// Stimmbeteiligung
        pub percent_turnout: Option<f64>,
        /// Stimmberechtigte
        pub eligible_voters: Option<i64>,
        /// Stimmberechtigte Auslandschweizer/innen
        pub expats: Option<i64>,
        /// Leere Stimmen
        pub empty: Option<i64>,
        /// Ungültige Stimmen
        pub invalid: Option<i64>,
        /// Ja-Stimmen
        pub yeas: Option<i64>,
        /// Nein-Stimmen
        pub nays: Option<i64>,
        pub link_to_municipality_results: Option<String>,
        pub url_web: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        VoteId,
        Domain0,
        RType,
        TitleDeCh,
        EntitiesTotal,
        EntitiesCounted,
        Answer,
        PercentYeas,
        PercentNays,
        PercentTurnout,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        LinkToMunicipalityResults,
        UrlWeb,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::VoteId => "vote_id",
                Field::Domain0 => "domain0",
                Field::RType => "type",
                Field::TitleDeCh => "title_de_ch",
                Field::EntitiesTotal => "entities_total",
                Field::EntitiesCounted => "entities_counted",
                Field::Answer => "answer",
                Field::PercentYeas => "percent_yeas",
                Field::PercentNays => "percent_nays",
                Field::PercentTurnout => "percent_turnout",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::LinkToMunicipalityResults => "link_to_municipality_results",
                Field::UrlWeb => "url_web",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10500/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Adressen der Gemeindeverwaltungen (August 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10510/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10510/</a>\n"]
#[doc = "<p>Liste der Gemeindeadressen</p>"]
#[cfg(feature = "bl10510")]
pub mod adressen_der_gemeindeverwaltungen_august_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Nummer
        ///
        /// BFS-Gemeindenummer
        pub bfs_nummer: Option<String>,
        /// Name
        ///
        /// Name der Gemeindeverwaltung
        pub name: Option<String>,
        /// Post_Adresse
        ///
        /// Post-Adresse
        pub post_adresse: Option<String>,
        /// PLZ_Ort
        ///
        /// Postleitzahl Ort
        pub plz_ort: Option<String>,
        /// Telefon
        ///
        /// Telefonnummer
        pub telefon: Option<String>,
        /// Fax
        ///
        /// Faxnummer
        pub fax: Option<String>,
        /// E_Mail
        ///
        /// E-Mail-Adresse
        pub e_mail: Option<String>,
        /// Website
        pub website: Option<String>,
        /// GKODE
        ///
        /// Gebäudekoordinate Ost
        pub gkode: Option<f64>,
        /// GKODN
        ///
        /// Gebäudekoordinate Nord
        pub gkodn: Option<f64>,
        /// Koordinaten
        ///
        /// Koordinaten gemäss World Geodetic System 1984 (WGS84)
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsNummer,
        Name,
        PostAdresse,
        PlzOrt,
        Telefon,
        Fax,
        EMail,
        Website,
        Gkode,
        Gkodn,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsNummer => "bfs_nummer",
                Field::Name => "name",
                Field::PostAdresse => "post_adresse",
                Field::PlzOrt => "plz_ort",
                Field::Telefon => "telefon",
                Field::Fax => "fax",
                Field::EMail => "e_mail",
                Field::Website => "website",
                Field::Gkode => "gkode",
                Field::Gkodn => "gkodn",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10510/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Parteistimmen und Parteist\u{e4}rken bei den Nationalratswahlen nach Gemeinde und Jahr (seit 1971)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10520/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10520/</a>\n"]
#[doc = "<p>Statistik der Wahlen und Abstimmungen</p>"]
#[cfg(feature = "bl10520")]
pub mod parteistimmen_und_parteistaerken_bei_den_nationalratswahlen_nach_gemeinde_und_jahr_seit_1971 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Partei
        pub partei: Option<String>,
        /// Stimmen
        pub stimmen: Option<i64>,
        /// Parteistärke
        pub parteistarke: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Partei,
        Stimmen,
        Parteistarke,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Partei => "partei",
                Field::Stimmen => "stimmen",
                Field::Parteistarke => "parteistarke",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10520/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Volksinitiative vom 17. September 2019 \"Keine Massentierhaltung in der Schweiz\" (Massentierhaltungsinitiative)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10530/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10530/</a>\n"]
#[doc = "Eidgen\u{f6}ssische Abstimmung vom 25. September 2022"]
#[cfg(feature = "bl10530")]
pub mod volksinitiative_vom_17_september_2019_keine_massentierhaltung_in_der_schweiz_massentierhaltungsinitiative {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10530/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bundesbeschluss vom 17. Dezember 2021 \u{fc}ber die Zusatzfinanzierung der AHV durch eine Erh\u{f6}hung der Mehrwertsteuer"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10540/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10540/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 25. September 2022<br></p><p><br></p><p><br></p>"]
#[cfg(feature = "bl10540")]
pub mod bundesbeschluss_vom_17_dezember_2021_ueber_die_zusatzfinanzierung_der_ahv_durch_eine_erhoehung_der_mehrwertsteuer {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10540/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung vom 17. Dezember 2021 des Bundesgesetzes \u{fc}ber die Alters- und Hinterlassenenversicherung (AHVG) (AHV 21)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10550/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10550/</a>\n"]
#[doc = "Eidgen\u{f6}ssische Abstimmung vom 25. September 2022"]
#[cfg(feature = "bl10550")]
pub mod aenderung_vom_17_dezember_2021_des_bundesgesetzes_ueber_die_alters_und_hinterlassenenversicherung_ahvg_ahv_21 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10550/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung vom 17. Dezember 2021 des Bundesgesetzes \u{fc}ber die Verrechnungssteuer (Verrechnungssteuergesetz, VStG) (St\u{e4}rkung des Fremdkapitalmarkts)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10560/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10560/</a>\n"]
#[doc = "Eidgen\u{f6}ssische Abstimmung vom 25. September 2022"]
#[cfg(feature = "bl10560")]
pub mod aenderung_vom_17_dezember_2021_des_bundesgesetzes_ueber_die_verrechnungssteuer_verrechnungssteuergesetz_vstg_staerkung_des_fremdkapitalmarkts {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10560/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Finanzausgleich nach Gemeinde und Jahr (seit 2010)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10570/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10570/</a>\n"]
#[doc = "<p>Finanzausgleichsverf\u{fc}gung<br></p>"]
#[cfg(feature = "bl10570")]
pub mod finanzausgleich_nach_gemeinde_und_jahr_seit_2010 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10570/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Steuerf\u{fc}sse und Steuers\u{e4}tze nach Gemeinde und Jahr (seit 1975)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10580/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10580/</a>\n"]
#[doc = "<p class=\"\">Statistik der Steuerf\u{fc}sse und -s\u{e4}tze, Geb\u{fc}hren und Ersatzabgaben<br></p>"]
#[cfg(feature = "bl10580")]
pub mod steuerfuesse_und_steuersaetze_nach_gemeinde_und_jahr_seit_1975 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10580/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Staatssteuern der nat\u{fc}rlichen Personen nach Einkommensklasse und Jahr (seit 2013)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10590/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10590/</a>\n"]
#[doc = "Steuerstatistik"]
#[cfg(feature = "bl10590")]
pub mod staatssteuern_der_natuerlichen_personen_nach_einkommensklasse_und_jahr_seit_2013 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Klasse_Steuerbares_Einkommen_Code
        pub klasse_steuerbares_einkommen_code: Option<f64>,
        /// Klasse_Steuerbares_Einkommen_CHF
        pub klasse_steuerbares_einkommen_chf: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        KlasseSteuerbaresEinkommenCode,
        KlasseSteuerbaresEinkommenChf,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::KlasseSteuerbaresEinkommenCode => "klasse_steuerbares_einkommen_code",
                Field::KlasseSteuerbaresEinkommenChf => "klasse_steuerbares_einkommen_chf",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10590/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Staatssteuern der nat\u{fc}rlichen Personen nach Verm\u{f6}gensklasse und Jahr (seit 2013)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10600/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10600/</a>\n"]
#[doc = "<p>Steuerstatistik<br></p>"]
#[cfg(feature = "bl10600")]
pub mod staatssteuern_der_natuerlichen_personen_nach_vermoegensklasse_und_jahr_seit_2013 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<i64>,
        /// Klasse_Steuerbares_Vermoegen_Code
        pub klasse_steuerbares_vermoegen_code: Option<f64>,
        /// Klasse_Steuerbares_Vermoegen_CHF
        pub klasse_steuerbares_vermoegen_chf: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        KlasseSteuerbaresVermoegenCode,
        KlasseSteuerbaresVermoegenChf,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::KlasseSteuerbaresVermoegenCode => "klasse_steuerbares_vermoegen_code",
                Field::KlasseSteuerbaresVermoegenChf => "klasse_steuerbares_vermoegen_chf",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10600/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Staatssteuern der juristischen Personen nach Gewinnklasse und Jahr (seit 2013)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10610/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10610/</a>\n"]
#[doc = "<p>Steuerstatistik<br></p>"]
#[cfg(feature = "bl10610")]
pub mod staatssteuern_der_juristischen_personen_nach_gewinnklasse_und_jahr_seit_2013 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Klasse_Steuerbarer_Gewinn_Code
        pub klasse_steuerbarer_gewinn_code: Option<f64>,
        /// Klasse_Steuerbarer_Gewinn_CHF
        pub klasse_steuerbarer_gewinn_chf: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        KlasseSteuerbarerGewinnCode,
        KlasseSteuerbarerGewinnChf,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::KlasseSteuerbarerGewinnCode => "klasse_steuerbarer_gewinn_code",
                Field::KlasseSteuerbarerGewinnChf => "klasse_steuerbarer_gewinn_chf",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10610/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Staatssteuern der juristischen Personen nach Kapitalklasse und Jahr (seit 2013)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10620/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10620/</a>\n"]
#[doc = "<p>Steuerstatistik</p><p>Ab 2020 Umsetzung der <a href=\"https://www.baselland.ch/politik-und-behorden/direktionen/finanz-und-kirchendirektion/wissenswertes-zur-steuervorlage-17-sv17\" target=\"_blank\">Steuervorlage 17 (SV17)</a>. Es handelt sich dabei um eine umfassende Unternehmenssteuerreform, welche in erster Linie die privilegierte Gewinnbesteuerung von Statusgesellschaften aufhob und dabei verschiedene Ausgleichsmassnahmen einf\u{fc}hrte, was sich deutlich auf die Entwicklung der Zahlen auswirkte.<br></p>"]
#[cfg(feature = "bl10620")]
pub mod staatssteuern_der_juristischen_personen_nach_kapitalklasse_und_jahr_seit_2013 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Klasse_Steuerbares_Kapital_Code
        pub klasse_steuerbares_kapital_code: Option<f64>,
        /// Klasse_Steuerbares_Kapital_CHF
        pub klasse_steuerbares_kapital_chf: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        KlasseSteuerbaresKapitalCode,
        KlasseSteuerbaresKapitalChf,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::KlasseSteuerbaresKapitalCode => "klasse_steuerbares_kapital_code",
                Field::KlasseSteuerbaresKapitalChf => "klasse_steuerbares_kapital_chf",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10620/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Staatssteuern der nat\u{fc}rlichen Personen nach Gemeinde und Jahr (seit 2013)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10630/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10630/</a>\n"]
#[doc = "<p>Steuerstatistik</p>"]
#[cfg(feature = "bl10630")]
pub mod staatssteuern_der_natuerlichen_personen_nach_gemeinde_und_jahr_seit_2013 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10630/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeindefinanzen nach Rechnungsteil, Funktion, Kontenart und Jahr (seit 2014)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10640/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10640/</a>\n"]
#[doc = "<p>Gemeindefinanzstatistik (HRM2)<br></p>"]
#[cfg(feature = "bl10640")]
pub mod gemeindefinanzen_nach_rechnungsteil_funktion_kontenart_und_jahr_seit_2014 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Rechnungsteil
        pub rechnungsteil: Option<String>,
        /// Funktion_Nr
        pub funktion_nr: Option<String>,
        /// Funktion_Name
        pub funktion_name: Option<String>,
        /// Kontenart_aggregiert
        pub kontenart_aggregiert: Option<i64>,
        /// Kontenart_aggregiert_Name
        pub kontenart_aggregiert_name: Option<String>,
        /// Kontenart_Nr
        pub kontenart_nr: Option<String>,
        /// Kontenart_Name
        pub kontenart_name: Option<String>,
        /// Betrag_CHF
        pub betrag_chf: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Rechnungsteil,
        FunktionNr,
        FunktionName,
        KontenartAggregiert,
        KontenartAggregiertName,
        KontenartNr,
        KontenartName,
        BetragChf,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Rechnungsteil => "rechnungsteil",
                Field::FunktionNr => "funktion_nr",
                Field::FunktionName => "funktion_name",
                Field::KontenartAggregiert => "kontenart_aggregiert",
                Field::KontenartAggregiertName => "kontenart_aggregiert_name",
                Field::KontenartNr => "kontenart_nr",
                Field::KontenartName => "kontenart_name",
                Field::BetragChf => "betrag_chf",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10640/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeindekennzahlen (2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10650/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10650/</a>\n"]
#[doc = "<p>Daten und Kennziffern aus den <a href=\"https://gemeindeportraets.bl.ch\" target=\"_blank\">Gemeindeportr\u{e4}ts</a></p><p>Aktuell verf\u{fc}gbare Daten je nach Indikator.</p>"]
#[cfg(feature = "bl10650")]
pub mod gemeindekennzahlen_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Bezirk_Nummer
        pub bezirk_nummer: Option<String>,
        /// Bezirk
        pub bezirk: Option<String>,
        /// Höhe_über_Meer
        pub hohe_uber_meer: Option<i64>,
        /// Gesamtfläche_ha_2014_15
        pub gesamtflache_ha_2014_15: Option<i64>,
        /// Siedlungsfläche_Prozent
        pub siedlungsflache_prozent: Option<f64>,
        /// Landwirtschaftsfläche_Prozent
        pub landwirtschaftsflache_prozent: Option<f64>,
        /// Bestockte_Fläche_Wald_Prozent
        pub bestockte_flache_wald_prozent: Option<f64>,
        /// Unproduktive_Fläche_Prozent
        pub unproduktive_flache_prozent: Option<f64>,
        /// Haushalte_2023
        pub haushalte_2023: Option<i64>,
        /// Bevölkerung_2023
        pub bevolkerung_2023: Option<i64>,
        /// 0_bis_14jährige_Prozent
        pub x0_bis_14jahrige_prozent: Option<f64>,
        /// 15_bis_64jährige_Prozent
        pub x15_bis_64jahrige_prozent: Option<f64>,
        /// 65jährige_und_älter_Prozent
        pub x65jahrige_und_alter_prozent: Option<f64>,
        /// Ausländeranteil_2023_Prozent
        pub auslanderanteil_2023_prozent: Option<f64>,
        /// Kinder_öffentliche_Primarstufe_Schulort_2023
        pub kinder_offentliche_primarstufe_schulort_2023: Option<f64>,
        /// Arbeitsstätten_2022
        pub arbeitsstatten_2022: Option<f64>,
        /// Beschäftigte_2022
        pub beschaftigte_2022: Option<f64>,
        /// Beschäftigte_Sektor_1_Prozent
        pub beschaftigte_sektor_1_prozent: Option<f64>,
        /// Beschäftigte_Sektor_2_Prozent
        pub beschaftigte_sektor_2_prozent: Option<f64>,
        /// Beschäftigte_Sektor_3_Prozent
        pub beschaftigte_sektor_3_prozent: Option<f64>,
        /// Wohnungsbestand_2023
        pub wohnungsbestand_2023: Option<f64>,
        /// Einfamilienhäuser_Prozent
        pub einfamilienhauser_prozent: Option<f64>,
        /// Fertigerstellte_Wohnungen_2023
        pub fertigerstellte_wohnungen_2023: Option<f64>,
        /// Leerwohnungsziffer_2024_Prozent
        pub leerwohnungsziffer_2024_prozent: Option<f64>,
        /// Bodenpreis_m2_Wohnbauland_2021_2023_CHF
        pub bodenpreis_m2_wohnbauland_2021_2023_chf: Option<String>,
        /// Steuerertrag_2022_1000_CHF
        pub steuerertrag_2022_1000_chf: Option<i64>,
        /// Steuerfuss_2024
        pub steuerfuss_2024: Option<f64>,
        /// Webseite
        pub webseite: Option<String>,
        /// Geometrie
        pub geometrie: Option<GeoJson>,
        /// Geometrisches_Zentrum
        pub geometrisches_zentrum: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsNummer,
        Gemeinde,
        BezirkNummer,
        Bezirk,
        HoheUberMeer,
        GesamtflacheHa201415,
        SiedlungsflacheProzent,
        LandwirtschaftsflacheProzent,
        BestockteFlacheWaldProzent,
        UnproduktiveFlacheProzent,
        Haushalte2023,
        Bevolkerung2023,
        X0Bis14jahrigeProzent,
        X15Bis64jahrigeProzent,
        X65jahrigeUndAlterProzent,
        Auslanderanteil2023Prozent,
        KinderOffentlichePrimarstufeSchulort2023,
        Arbeitsstatten2022,
        Beschaftigte2022,
        BeschaftigteSektor1Prozent,
        BeschaftigteSektor2Prozent,
        BeschaftigteSektor3Prozent,
        Wohnungsbestand2023,
        EinfamilienhauserProzent,
        FertigerstellteWohnungen2023,
        Leerwohnungsziffer2024Prozent,
        BodenpreisM2Wohnbauland20212023Chf,
        Steuerertrag20221000Chf,
        Steuerfuss2024,
        Webseite,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::HoheUberMeer => "hohe_uber_meer",
                Field::GesamtflacheHa201415 => "gesamtflache_ha_2014_15",
                Field::SiedlungsflacheProzent => "siedlungsflache_prozent",
                Field::LandwirtschaftsflacheProzent => "landwirtschaftsflache_prozent",
                Field::BestockteFlacheWaldProzent => "bestockte_flache_wald_prozent",
                Field::UnproduktiveFlacheProzent => "unproduktive_flache_prozent",
                Field::Haushalte2023 => "haushalte_2023",
                Field::Bevolkerung2023 => "bevolkerung_2023",
                Field::X0Bis14jahrigeProzent => "0_bis_14jahrige_prozent",
                Field::X15Bis64jahrigeProzent => "15_bis_64jahrige_prozent",
                Field::X65jahrigeUndAlterProzent => "65jahrige_und_alter_prozent",
                Field::Auslanderanteil2023Prozent => "auslanderanteil_2023_prozent",
                Field::KinderOffentlichePrimarstufeSchulort2023 => {
                    "kinder_offentliche_primarstufe_schulort_2023"
                }
                Field::Arbeitsstatten2022 => "arbeitsstatten_2022",
                Field::Beschaftigte2022 => "beschaftigte_2022",
                Field::BeschaftigteSektor1Prozent => "beschaftigte_sektor_1_prozent",
                Field::BeschaftigteSektor2Prozent => "beschaftigte_sektor_2_prozent",
                Field::BeschaftigteSektor3Prozent => "beschaftigte_sektor_3_prozent",
                Field::Wohnungsbestand2023 => "wohnungsbestand_2023",
                Field::EinfamilienhauserProzent => "einfamilienhauser_prozent",
                Field::FertigerstellteWohnungen2023 => "fertigerstellte_wohnungen_2023",
                Field::Leerwohnungsziffer2024Prozent => "leerwohnungsziffer_2024_prozent",
                Field::BodenpreisM2Wohnbauland20212023Chf => {
                    "bodenpreis_m2_wohnbauland_2021_2023_chf"
                }
                Field::Steuerertrag20221000Chf => "steuerertrag_2022_1000_chf",
                Field::Steuerfuss2024 => "steuerfuss_2024",
                Field::Webseite => "webseite",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10650/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Datensatz-Katalog"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10660/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10660/</a>\n"]
#[doc = "<p>Liste aller Datens\u{e4}tze auf dem OGD-Portal BL (data.bl.ch)</p>"]
#[cfg(feature = "bl10660")]
pub mod datensatz_katalog {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub dataset_id: Option<String>,
        pub title: Option<String>,
        pub description: Option<String>,
        pub theme: Option<String>,
        pub keyword: Option<String>,
        pub license: Option<String>,
        #[serde(with = "time::serde::iso8601::option")]
        pub modified: Option<OffsetDateTime>,
        /// Data_processed
        #[serde(with = "time::serde::iso8601::option")]
        pub data_processed: Option<OffsetDateTime>,
        #[serde(with = "time::serde::iso8601::option")]
        pub metadata_processed: Option<OffsetDateTime>,
        pub publisher: Option<String>,
        pub references: Option<String>,
        pub records_count: Option<i64>,
        pub attributions: Option<String>,
        pub contact_email: Option<String>,
        pub accrualperiodicity: Option<String>,
        pub rights: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        DatasetId,
        Title,
        Description,
        Theme,
        Keyword,
        License,
        Modified,
        DataProcessed,
        MetadataProcessed,
        Publisher,
        References,
        RecordsCount,
        Attributions,
        ContactEmail,
        Accrualperiodicity,
        Rights,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::DatasetId => "dataset_id",
                Field::Title => "title",
                Field::Description => "description",
                Field::Theme => "theme",
                Field::Keyword => "keyword",
                Field::License => "license",
                Field::Modified => "modified",
                Field::DataProcessed => "data_processed",
                Field::MetadataProcessed => "metadata_processed",
                Field::Publisher => "publisher",
                Field::References => "references",
                Field::RecordsCount => "records_count",
                Field::Attributions => "attributions",
                Field::ContactEmail => "contact_email",
                Field::Accrualperiodicity => "accrualperiodicity",
                Field::Rights => "rights",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10660/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung des Steuergesetzes, Verm\u{f6}genssteuerreform I"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10670/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10670/</a>\n"]
#[doc = "<p>Kantonale Abstimmung vom 27. November 2022</p>"]
#[cfg(feature = "bl10670")]
pub mod aenderung_des_steuergesetzes_vermoegenssteuerreform_i {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10670/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bev\u{f6}lkerungsbilanz nach Gemeinde und Quartal (seit 2003)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10680/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10680/</a>\n"]
#[doc = "<p>Kantonale Bev\u{f6}lkerungsstatistik</p>"]
#[cfg(feature = "bl10680")]
pub mod bevoelkerungsbilanz_nach_gemeinde_und_quartal_seit_2003 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub jahr: Option<String>,
        pub quartal: Option<i64>,
        pub gemeinde_nummer: Option<String>,
        pub gemeinde: Option<String>,
        pub bezirk_nummer: Option<String>,
        pub bezirk: Option<String>,
        pub anfangsbestand: Option<i64>,
        pub geburten: Option<i64>,
        pub todesfaelle: Option<i64>,
        pub geburtenueberschuss: Option<i64>,
        pub zuzuege: Option<i64>,
        pub wegzuege: Option<i64>,
        pub wanderungssaldo: Option<i64>,
        pub bereinigung_saldo: Option<i64>,
        pub gesamtveraenderung: Option<i64>,
        pub endbestand: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Quartal,
        GemeindeNummer,
        Gemeinde,
        BezirkNummer,
        Bezirk,
        Anfangsbestand,
        Geburten,
        Todesfaelle,
        Geburtenueberschuss,
        Zuzuege,
        Wegzuege,
        Wanderungssaldo,
        BereinigungSaldo,
        Gesamtveraenderung,
        Endbestand,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Quartal => "quartal",
                Field::GemeindeNummer => "gemeinde_nummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkNummer => "bezirk_nummer",
                Field::Bezirk => "bezirk",
                Field::Anfangsbestand => "anfangsbestand",
                Field::Geburten => "geburten",
                Field::Todesfaelle => "todesfaelle",
                Field::Geburtenueberschuss => "geburtenueberschuss",
                Field::Zuzuege => "zuzuege",
                Field::Wegzuege => "wegzuege",
                Field::Wanderungssaldo => "wanderungssaldo",
                Field::BereinigungSaldo => "bereinigung_saldo",
                Field::Gesamtveraenderung => "gesamtveraenderung",
                Field::Endbestand => "endbestand",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10680/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeinderatswahlen 2024: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10700/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10700/</a>\n"]
#[doc = "<p>Kommunale Wahlen vom 3. M\u{e4}rz 2024\u{a0}(offiziell kandidierende Personen)</p><p>Keine Angaben (...) zur Stimmenzahl bei stillen Wahlen</p><p>Teilweise fehlende Angaben (...) zu Kandidaten-Nr., Jahrgang und Parteizugeh\u{f6}rigkeit<br></p>"]
#[cfg(feature = "bl10700")]
pub mod gemeinderatswahlen_2024_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Kandidaten-Nr
        pub kandidaten_nr: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Anzahl Stimmen
        pub anzahl_stimmen: Option<String>,
        /// Gewählt
        pub gewahlt: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        KandidatenNr,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        Bisher,
        AnzahlStimmen,
        Gewahlt,
        Parteibezeichnung,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::KandidatenNr => "kandidaten_nr",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::Bisher => "bisher",
                Field::AnzahlStimmen => "anzahl_stimmen",
                Field::Gewahlt => "gewahlt",
                Field::Parteibezeichnung => "parteibezeichnung",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10700/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeinderatswahlen 2024: Anzahl Sitze, Wahlberechtigte und Wahlzettel nach Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10710/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10710/</a>\n"]
#[doc = "<p>Kommunale Wahlen vom 3. M\u{e4}rz 2024</p><p>Keine Angaben (...) zu Stimmberechtigten, Wahlzetteln und Stimmen bei stillen Wahlen</p>"]
#[cfg(feature = "bl10710")]
pub mod gemeinderatswahlen_2024_anzahl_sitze_wahlberechtigte_und_wahlzettel_nach_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Wahlverfahren
        pub wahlverfahren: Option<String>,
        /// Stille Wahl
        pub stille_wahl: Option<String>,
        /// Anzahl Sitze
        pub anzahl_sitze: Option<i64>,
        /// Stimmberechtigte
        pub stimmberechtigte: Option<String>,
        /// Abgegebene Wahlzettel
        pub abgegebene_wahlzettel: Option<String>,
        /// Leere Wahlzettel
        pub leere_wahlzettel: Option<String>,
        /// Ungültige Wahlzettel
        pub ungultige_wahlzettel: Option<String>,
        /// Gültige Wahlzettel
        pub gultige_wahlzettel: Option<String>,
        /// Leere Stimmen
        pub leere_stimmen: Option<String>,
        /// Ungültige Stimmen
        pub ungultige_stimmen: Option<String>,
        /// Gültige Stimmen
        pub gultige_stimmen: Option<String>,
        /// Absolutes Mehr
        pub absolutes_mehr: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        Wahlverfahren,
        StilleWahl,
        AnzahlSitze,
        Stimmberechtigte,
        AbgegebeneWahlzettel,
        LeereWahlzettel,
        UngultigeWahlzettel,
        GultigeWahlzettel,
        LeereStimmen,
        UngultigeStimmen,
        GultigeStimmen,
        AbsolutesMehr,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Wahlverfahren => "wahlverfahren",
                Field::StilleWahl => "stille_wahl",
                Field::AnzahlSitze => "anzahl_sitze",
                Field::Stimmberechtigte => "stimmberechtigte",
                Field::AbgegebeneWahlzettel => "abgegebene_wahlzettel",
                Field::LeereWahlzettel => "leere_wahlzettel",
                Field::UngultigeWahlzettel => "ungultige_wahlzettel",
                Field::GultigeWahlzettel => "gultige_wahlzettel",
                Field::LeereStimmen => "leere_stimmen",
                Field::UngultigeStimmen => "ungultige_stimmen",
                Field::GultigeStimmen => "gultige_stimmen",
                Field::AbsolutesMehr => "absolutes_mehr",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10710/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Altersbetreuung: Versorgungsregionen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10740/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10740/</a>\n"]
#[doc = "<p>Baselbieter Versorgungsregionen (VR) gem\u{e4}ss Altersbetreuungs- und Pflegegesetz (APG) 2021.\u{a0}Polygondaten als Shapefile oder GeoJSON.</p><p>Das <a href=\"https://bl.clex.ch/app/de/texts_of_law/941\" target=\"_blank\">Altersbetreuungs- und Pflegegesetz (APG)</a> verpflichtet die Gemeinden dazu, sich zur Planung und Sicherstellung der Versorgung der Bev\u{f6}lkerung mit Angeboten zur Betreuung und Pflege zu sogenannten Versorgungsregionen zusammenschliessen. Da einzelne Versorgungsregionen sehr klein sind, verwendet der Kanton f\u{fc}r Planungszwecke die aggregierten Versorgungsregionen. Bei den aggregierten Versorgungsregionen werden die Regionen Obereres Homburgertal und Farnsberg plus dem Oberbaselbiet zugeschlagen.<br></p>"]
#[cfg(feature = "bl10740")]
pub mod altersbetreuung_versorgungsregionen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Versorgungsregion_Code
        pub versorgung: Option<i64>,
        /// Versorgungsregion
        pub versorgu_1: Option<String>,
        /// Geo Shape
        pub geo_shape: Option<GeoJson>,
        /// Geo Point
        pub geo_point_2d: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Versorgung,
        Versorgu1,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Versorgung => "versorgung",
                Field::Versorgu1 => "versorgu_1",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10740/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Einwohnerratswahlen 2024: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10840/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10840/</a>\n"]
#[doc = "<p>Kommunale Wahlen vom 3. M\u{e4}rz 2024\u{a0}(offiziell kandidierende Personen)<br></p>"]
#[cfg(feature = "bl10840")]
pub mod einwohnerratswahlen_2024_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// Anzahl Sitze
        pub anzahl_sitze: Option<i64>,
        /// Wahlkreis-Nr
        pub wahlkreis_nr: Option<i64>,
        /// Wahlkreis-Code
        pub wahlkreis_code: Option<i64>,
        /// Wahlkreisbezeichnung
        pub wahlkreisbezeichnung: Option<String>,
        /// Stimmberechtigte
        pub stimmberechtigte: Option<i64>,
        /// Wahlzettel
        pub wahlzettel: Option<i64>,
        /// Ungestempelte Wahlzettel
        pub ungestempelte_wahlzettel: Option<i64>,
        /// Ungültige Wahlzettel
        pub ungultige_wahlzettel: Option<i64>,
        /// Leere Wahlzettel
        pub leere_wahlzettel: Option<i64>,
        /// Unveränderte Wahlzettel
        pub unveranderte_wahlzettel: Option<i64>,
        /// Veränderte Wahlzettel mit Bezeichnung
        pub veranderte_wahlzettel_mit_bezeichnung: Option<i64>,
        /// Veränderte Wahlzettel ohne Bezeichnung
        pub veranderte_wahlzettel_ohne_bezeichnung: Option<i64>,
        /// Leere Stimmen
        pub leere_stimmen: Option<i64>,
        /// Listen-Nr
        pub listen_nr: Option<String>,
        /// Partei-ID
        pub partei_id: Option<i64>,
        /// Parteikurzbezeichnung
        pub parteikurzbezeichnung: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
        /// Anzahl Sitze Liste
        pub anzahl_sitze_liste: Option<i64>,
        /// Unveränderte Wahlzettel Liste
        pub unveranderte_wahlzettel_liste: Option<i64>,
        /// Veränderte Wahlzettel Liste
        pub veranderte_wahlzettel_liste: Option<i64>,
        /// Kandidatenstimmen unveränderte Wahlzettel
        pub kandidatenstimmen_unveranderte_wahlzettel: Option<i64>,
        /// Zusatzstimmen unveränderte Wahlzettel
        pub zusatzstimmen_unveranderte_wahlzettel: Option<i64>,
        /// Kandidatenstimmen veränderte Wahlzettel
        pub kandidatenstimmen_veranderte_wahlzettel: Option<i64>,
        /// Zusatzstimmen veränderte Wahlzettel
        pub zusatzstimmen_veranderte_wahlzettel: Option<i64>,
        /// Kandidaten-Nr
        pub kandidaten_nr: Option<i64>,
        /// Personen-ID
        pub personen_id: Option<i64>,
        /// Kumulation
        pub kumulation: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Gewählt
        pub gewahlt: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Stimmen unveränderte Wahlzettel
        pub stimmen_unveranderte_wahlzettel: Option<i64>,
        /// Stimmen veränderte Wahlzettel
        pub stimmen_veranderte_wahlzettel: Option<i64>,
        /// Stimmen total
        pub stimmen_total: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        AnzahlSitze,
        WahlkreisNr,
        WahlkreisCode,
        Wahlkreisbezeichnung,
        Stimmberechtigte,
        Wahlzettel,
        UngestempelteWahlzettel,
        UngultigeWahlzettel,
        LeereWahlzettel,
        UnveranderteWahlzettel,
        VeranderteWahlzettelMitBezeichnung,
        VeranderteWahlzettelOhneBezeichnung,
        LeereStimmen,
        ListenNr,
        ParteiId,
        Parteikurzbezeichnung,
        Parteibezeichnung,
        AnzahlSitzeListe,
        UnveranderteWahlzettelListe,
        VeranderteWahlzettelListe,
        KandidatenstimmenUnveranderteWahlzettel,
        ZusatzstimmenUnveranderteWahlzettel,
        KandidatenstimmenVeranderteWahlzettel,
        ZusatzstimmenVeranderteWahlzettel,
        KandidatenNr,
        PersonenId,
        Kumulation,
        Bisher,
        Gewahlt,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        StimmenUnveranderteWahlzettel,
        StimmenVeranderteWahlzettel,
        StimmenTotal,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::AnzahlSitze => "anzahl_sitze",
                Field::WahlkreisNr => "wahlkreis_nr",
                Field::WahlkreisCode => "wahlkreis_code",
                Field::Wahlkreisbezeichnung => "wahlkreisbezeichnung",
                Field::Stimmberechtigte => "stimmberechtigte",
                Field::Wahlzettel => "wahlzettel",
                Field::UngestempelteWahlzettel => "ungestempelte_wahlzettel",
                Field::UngultigeWahlzettel => "ungultige_wahlzettel",
                Field::LeereWahlzettel => "leere_wahlzettel",
                Field::UnveranderteWahlzettel => "unveranderte_wahlzettel",
                Field::VeranderteWahlzettelMitBezeichnung => {
                    "veranderte_wahlzettel_mit_bezeichnung"
                }
                Field::VeranderteWahlzettelOhneBezeichnung => {
                    "veranderte_wahlzettel_ohne_bezeichnung"
                }
                Field::LeereStimmen => "leere_stimmen",
                Field::ListenNr => "listen_nr",
                Field::ParteiId => "partei_id",
                Field::Parteikurzbezeichnung => "parteikurzbezeichnung",
                Field::Parteibezeichnung => "parteibezeichnung",
                Field::AnzahlSitzeListe => "anzahl_sitze_liste",
                Field::UnveranderteWahlzettelListe => "unveranderte_wahlzettel_liste",
                Field::VeranderteWahlzettelListe => "veranderte_wahlzettel_liste",
                Field::KandidatenstimmenUnveranderteWahlzettel => {
                    "kandidatenstimmen_unveranderte_wahlzettel"
                }
                Field::ZusatzstimmenUnveranderteWahlzettel => {
                    "zusatzstimmen_unveranderte_wahlzettel"
                }
                Field::KandidatenstimmenVeranderteWahlzettel => {
                    "kandidatenstimmen_veranderte_wahlzettel"
                }
                Field::ZusatzstimmenVeranderteWahlzettel => "zusatzstimmen_veranderte_wahlzettel",
                Field::KandidatenNr => "kandidaten_nr",
                Field::PersonenId => "personen_id",
                Field::Kumulation => "kumulation",
                Field::Bisher => "bisher",
                Field::Gewahlt => "gewahlt",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::StimmenUnveranderteWahlzettel => "stimmen_unveranderte_wahlzettel",
                Field::StimmenVeranderteWahlzettel => "stimmen_veranderte_wahlzettel",
                Field::StimmenTotal => "stimmen_total",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10840/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Luftqualit\u{e4}t Station Sissach West (halbst\u{fc}ndliche Messdaten Januar 2007 - April 2017)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10910/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10910/</a>\n"]
#[doc = "<p></p><p>Historische Daten der Luftmessstation Sissach West. Die Messwerte sind halbst\u{fc}ndlich ausgewiesen und bereinigt. Seit Fr\u{fc}hjahr 2017 ist die Station stillgelegt. Seit November 2017 wird die Station <a href=\"https://data.bl.ch/explore/dataset/12450/information/?sort=anfangszeit\" target=\"_blank\">Sissach B\u{fc}tzenen</a> betrieben.</p><p class=\"\">Das Auftreten allf\u{e4}lliger Negativwerte stammt von messtechnischen Ungenauigkeiten. Diese Messunsicherheit ist bei der Interpretation der Zahlen entsprechend mit einzubeziehen.</p><p class=\"\">Die Zeitstempel entsprechen der Zeitzone Europe/Zurich obwohl sie im Zeitformat UTC angegeben sind. Allf\u{e4}llige Fragen zum Zeitformat beantwortet das Amt f\u{fc}r Lufthygiene beider Basel auf Anfrage.</p><p></p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Ausgewiesene Werte</span></p><ul><li>Anfangszeit: Zeitstempel des Beginns der halbst\u{fc}ndlichen Messung im Format %Y-%m-%dT%H:%M:%S</li><li>Lungeng\u{e4}ngiger Feinstaub PM10 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM10 in Mikrogramm pro Kubikmeter.</li><li>Stickstoffdioxid NO2 (\u{b5}g/m3): Gemessene Stickstoffdioxid-Konzentration in Mikrogramm pro Kubikmeter.</li><li>Ozon O3 (\u{b5}g/m3): Gemessene Ozon-Konzentration in Mikrogramm pro Kubikmeter.</li></ul><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Standortbeschreibung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Die Messstation befand sich westlich vom Chienbergtunnel. Aufgrund der Strassenn\u{e4}he war die Hauptbelastungsquelle der Verkehr.\u{a0}</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Lage</span></p><p class=\"\" style=\"font-family: sans-serif;\">Kleinst\u{e4}dtisch/Vorst\u{e4}dtisch, verkehrsbelastet</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Koordinaten</span></p><p class=\"\" style=\"font-family: sans-serif;\">2627260 / 1257595; 362 m \u{fc}. M.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Bebauung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Offene Bebauung</p><p></p>"]
#[cfg(feature = "bl10910")]
pub mod luftqualitaet_station_sissach_west_halbstuendliche_messdaten_januar_2007_april_2017 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Anfangszeit
        #[serde(with = "time::serde::iso8601::option")]
        pub anfangszeit: Option<OffsetDateTime>,
        /// PM10
        ///
        /// Lungengängiger Feinstaub PM10
        pub pm10: Option<f64>,
        /// NO2
        ///
        /// Stickstoffdioxid
        pub no2: Option<f64>,
        /// O3
        ///
        /// Ozon
        pub o3: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Anfangszeit,
        Pm10,
        No2,
        O3,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Anfangszeit => "anfangszeit",
                Field::Pm10 => "pm10",
                Field::No2 => "no2",
                Field::O3 => "o3",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10910/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Erwerbst\u{e4}tige nach Wohngemeinde, Arbeitsort und Jahr (seit 2014)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10950/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10950/</a>\n"]
#[doc = "<p>Pendlermobilit\u{e4}t</p><p><br></p><p>Methodik</p><p>Der vorliegende Datensatz zu den Pendlerstr\u{f6}men stammt aus einer Registerverkn\u{fc}pfung des Bundesamts f\u{fc}r Statistik (BFS), die auf Basis der folgenden Datenquellen erstellt wurde:</p><ol><li>Statistik der Bev\u{f6}lkerung und der Haushalte (STATPOP): Enth\u{e4}lt Meldungen der Einwohnerregister der Gemeinden und liefert den Wohnort.</li><li>Register der Alters- und Hinterlassenenversicherung (AHV-Register): Enth\u{e4}lt Meldungen der AHV-Ausgleichskassen. Liefert das Unternehmen, in welchem eine Person arbeitet (ab Jahreseinkommen von 2\u{2019}300 Franken).\u{a0}</li><li>Unternehmensstatistik (STATENT): Gibt Auskunft \u{fc}ber den Standort der verschiedenen Arbeitsst\u{e4}tten (d.h. Filialen, Niederlassungen usw.) der Unternehmen und die Anzahl der dort arbeitenden Personen.</li></ol><p>Fehlende Angaben wurden mithilfe eines Algorithmus modelliert.</p>"]
#[cfg(feature = "bl10950")]
pub mod erwerbstaetige_nach_wohngemeinde_arbeitsort_und_jahr_seit_2014 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Wohngemeinde
        pub wohngemeinde: Option<String>,
        /// Wohnbezirk_Code
        pub wohnbezirk_code: Option<String>,
        /// Wohnbezirk
        pub wohnbezirk: Option<String>,
        /// Arbeitsort
        pub arbeitsort: Option<String>,
        /// Kennzahl
        pub kennzahl: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Wohngemeinde,
        WohnbezirkCode,
        Wohnbezirk,
        Arbeitsort,
        Kennzahl,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Wohngemeinde => "wohngemeinde",
                Field::WohnbezirkCode => "wohnbezirk_code",
                Field::Wohnbezirk => "wohnbezirk",
                Field::Arbeitsort => "arbeitsort",
                Field::Kennzahl => "kennzahl",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10950/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Betriebe mit einer Verkaufsbewilligung f\u{fc}r Spirituosen nach Standort (Februar 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10960/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10960/</a>\n"]
#[doc = "<p>Liste der Betriebe mit vom Kanton BL bewilligten Spirituosenverkauf. Stand: 29.02.2024</p><p>Der gewerbsm\u{e4}ssige Verkauf von alkoholischen Getr\u{e4}nken ist bewilligungspflichtig. In gastgewerblichen Bewilligungen ist der Verkauf von alkoholischen Getr\u{e4}nken bereits mitenthalten. In diesen F\u{e4}llen ist kein zus\u{e4}tzlicher Antrag notwendig.<br></p>"]
#[cfg(feature = "bl10960")]
pub mod betriebe_mit_einer_verkaufsbewilligung_fuer_spirituosen_nach_standort_februar_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Betriebsnummer
        pub betriebsnummer: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Firma
        pub firma: Option<String>,
        /// Post_Adresse
        pub post_adresse: Option<String>,
        /// PLZ_Ort
        pub plz_ort: Option<String>,
        /// Bemerkung
        pub bemerkung: Option<String>,
        /// GWR_Adresse
        pub gwr_adresse: Option<String>,
        pub e_eingangskoordinate: Option<f64>,
        pub n_eingangskoordinate: Option<f64>,
        pub koordinaten: Option<GeoPoint2d>,
        pub egid: Option<i64>,
        pub name_des_gebaeudes: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Betriebsnummer,
        BfsGemeindenummer,
        Gemeinde,
        Firma,
        PostAdresse,
        PlzOrt,
        Bemerkung,
        GwrAdresse,
        EEingangskoordinate,
        NEingangskoordinate,
        Egid,
        NameDesGebaeudes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Betriebsnummer => "betriebsnummer",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Firma => "firma",
                Field::PostAdresse => "post_adresse",
                Field::PlzOrt => "plz_ort",
                Field::Bemerkung => "bemerkung",
                Field::GwrAdresse => "gwr_adresse",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
                Field::Egid => "egid",
                Field::NameDesGebaeudes => "name_des_gebaeudes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10960/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Arbeitsst\u{e4}tten und Besch\u{e4}ftigte nach Wirtschaftssektor, Gemeinde und Jahr (seit 2011)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/10990/\" target=\"_blank\">https://data.bl.ch/explore/dataset/10990/</a>\n"]
#[doc = "<p>Statistik der Unternehmensstruktur (STATENT)<br></p>"]
#[cfg(feature = "bl10990")]
pub mod arbeitsstaetten_und_beschaeftigte_nach_wirtschaftssektor_gemeinde_und_jahr_seit_2011 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeindename
        pub gemeindename: Option<String>,
        /// Wirtschaftssektor
        pub wirtschaftssektor: Option<String>,
        /// Arbeitsstätten
        pub arbeitsstatten: Option<i64>,
        /// Beschäftigte
        pub beschaftigte: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Gemeindename,
        Wirtschaftssektor,
        Arbeitsstatten,
        Beschaftigte,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeindename => "gemeindename",
                Field::Wirtschaftssektor => "wirtschaftssektor",
                Field::Arbeitsstatten => "arbeitsstatten",
                Field::Beschaftigte => "beschaftigte",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/10990/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Lernendenprognose nach Bildungsinstitution, Schulstufe und Klassentyp"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11010/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11010/</a>\n"]
#[doc = "<p>Primarschule und Sekundarstufe I</p>"]
#[cfg(feature = "bl11010")]
pub mod lernendenprognose_nach_bildungsinstitution_schulstufe_und_klassentyp {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Bildungsinstitution_ID
        pub bildungsinstitution_id: Option<i64>,
        /// Bildungsinstitution
        pub bildungsinstitution: Option<String>,
        /// Schulstufe_Code
        pub schulstufe_code: Option<i64>,
        /// Schulstufe
        pub schulstufe: Option<String>,
        /// Klassentyp
        pub klassentyp: Option<String>,
        /// Anzahl_Lernende
        pub anzahl_lernende: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BildungsinstitutionId,
        Bildungsinstitution,
        SchulstufeCode,
        Schulstufe,
        Klassentyp,
        AnzahlLernende,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BildungsinstitutionId => "bildungsinstitution_id",
                Field::Bildungsinstitution => "bildungsinstitution",
                Field::SchulstufeCode => "schulstufe_code",
                Field::Schulstufe => "schulstufe",
                Field::Klassentyp => "klassentyp",
                Field::AnzahlLernende => "anzahl_lernende",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11010/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19: W\u{f6}chentliche Fallzahlen, Hospitalisierungen und Tests (seit Februar 2020)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11050/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11050/</a>\n"]
#[doc = "<p>F\u{e4}lle (aktuell), Hospitalisierungen (bis Ende 2023) und Tests (aktuell) aus dem obligatorischen Meldesystem.\u{a0}Personen mit Wohnsitz BL.<br></p>"]
#[cfg(feature = "bl11050")]
pub mod covid_19_woechentliche_fallzahlen_hospitalisierungen_und_tests_seit_februar_2020 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// valueCategory
        pub valuecategory: Option<String>,
        pub temporal: Option<String>,
        pub temporal_type: Option<String>,
        pub georegion: Option<String>,
        /// testResult
        pub testresult: Option<String>,
        /// testResult_type
        pub testresult_type: Option<String>,
        pub value: Option<i64>,
        pub pop: Option<i64>,
        /// incValue
        pub incvalue: Option<f64>,
        pub prct: Option<f64>,
        pub inc14d: Option<f64>,
        pub trend: Option<String>,
        /// dataComplete
        pub datacomplete: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Valuecategory,
        Temporal,
        TemporalType,
        Georegion,
        Testresult,
        TestresultType,
        Value,
        Pop,
        Incvalue,
        Prct,
        Inc14d,
        Trend,
        Datacomplete,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Valuecategory => "valuecategory",
                Field::Temporal => "temporal",
                Field::TemporalType => "temporal_type",
                Field::Georegion => "georegion",
                Field::Testresult => "testresult",
                Field::TestresultType => "testresult_type",
                Field::Value => "value",
                Field::Pop => "pop",
                Field::Incvalue => "incvalue",
                Field::Prct => "prct",
                Field::Inc14d => "inc14d",
                Field::Trend => "trend",
                Field::Datacomplete => "datacomplete",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11050/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nachnamen der st\u{e4}ndigen Wohnbev\u{f6}lkerung nach Gemeinde (seit 2022)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11080/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11080/</a>\n"]
#[doc = "<p>Statistik der Bev\u{f6}lkerung und der Haushalte (STATPOP)</p><p>Nachnamen mit weniger als 3 Nennungen werden nicht ver\u{f6}ffentlicht.<br></p>"]
#[cfg(feature = "bl11080")]
pub mod nachnamen_der_staendigen_wohnbevoelkerung_nach_gemeinde_seit_2022 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// TIME_PERIOD
        ///
        /// Jahr
        pub time_period: Option<String>,
        /// LASTNAME
        ///
        /// Nachname
        pub lastname: Option<String>,
        /// GDENR
        ///
        /// BFS-Gemeindenummer
        pub gdenr: Option<String>,
        /// GDENAME
        ///
        /// Gemeinde
        pub gdename: Option<String>,
        /// RANG_GDE
        ///
        /// Rang des Nachnamens in der Gemeinde
        pub rang_gde: Option<i64>,
        /// VALUE
        ///
        /// Anzahl Personen
        pub value: Option<i64>,
        /// PCT_GDE
        ///
        /// Prozentanteil des Nachnamens in der Gemeinde
        pub pct_gde: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        TimePeriod,
        Lastname,
        Gdenr,
        Gdename,
        RangGde,
        Value,
        PctGde,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::TimePeriod => "time_period",
                Field::Lastname => "lastname",
                Field::Gdenr => "gdenr",
                Field::Gdename => "gdename",
                Field::RangGde => "rang_gde",
                Field::Value => "value",
                Field::PctGde => "pct_gde",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11080/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Historische Geb\u{e4}ude: Firstst\u{e4}nderbauten nach Haustyp und Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11100/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11100/</a>\n"]
#[doc = "<p>Der Datensatz beinhaltet die untersuchten und aufgearbeiteten Firstst\u{e4}nderbauten des Kantons BL. Die Angaben zu den Adressen und Koordinaten sowie die Kommentar-Spalte werden aus Datenschutzgr\u{fc}nden nicht ausgewiesen.</p>"]
#[cfg(feature = "bl11100")]
pub mod historische_gebaeude_firststaenderbauten_nach_haustyp_und_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Nummer
        ///
        /// Gemeindenummer gemäss Bundesamt für Statistik
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        ///
        /// Gemeindename
        pub gemeinde: Option<String>,
        /// Akte
        ///
        /// Aktennummer
        pub akte: Option<String>,
        /// Strasse
        pub strasse: Option<String>,
        /// Koordinate_X_LV95
        ///
        /// X-Koordinate, auf Meter genau gerundet
        pub koordinate_x_lv95: Option<String>,
        /// Koordinate_Y_LV95
        ///
        /// Y-Koordinate, auf Meter genau gerundet
        pub koordinate_y_lv95: Option<String>,
        /// Schutzstatus
        ///
        /// Rechtlicher Schutzstatus des Gebäudes. keiner; Kantonal; Kommunal; Ortsbild.
        pub schutzstatus: Option<String>,
        /// Abgebrochen
        ///
        /// Vollständiger Abbruch des Gebäudes. Ja; Nein; -.
        pub abgebrochen: Option<String>,
        /// Erhaltung_Holzkonstruktion_Prozent
        ///
        /// Erhaltung der Holzkonstruktion des Kernbaus in Prozent, auf 25% gerundet.
        pub erhaltung_holzkonstruktion_prozent: Option<f64>,
        /// Kernbau_Datierung
        ///
        /// Datierung des Kernbaus, Angabe des Jahrhunderts.
        pub kernbau_datierung: Option<String>,
        /// Kernbau_Datierung_von
        ///
        /// Älteste mögliche Jahreszahl der Datierung des Kernbaus (t.p.q). Bei einer Inschrift oder Dendro bleibt das Feld leer.
        pub kernbau_datierung_von: Option<String>,
        /// Kernbau_Datierung_bis
        ///
        /// Jüngste mögliche Jahreszahl der Datierung Kernbau (t.a.q). Bei Dendro jüngst mögliches Schlagjahr.
        pub kernbau_datierung_bis: Option<String>,
        /// Kernbau_Datierung_Quelle
        ///
        /// Art der Datierung des Kernbaus. Dendro; Inschrift; Historisch; Quelle.
        pub kernbau_datierung_quelle: Option<String>,
        /// Haustyp
        ///
        /// Haustyp Firstständerbau. Siehe pdf-Beilage.
        pub haustyp: Option<String>,
        /// Holzarten
        ///
        /// In der Kernkonstruktion vorhandene Holzarten. Bei mehr als einer Angabe Trennung durch Semikolon.
        pub holzarten: Option<String>,
        /// Firstständer
        ///
        /// Anzahl der ursprünglich vorhandenen Firstständer im Kernbau.
        pub firststander: Option<f64>,
        /// Geschosse_Wohnteil
        ///
        /// Anzahl Geschosse des Wohnteils im Kernbau, ohne Dachgeschosse.
        pub geschosse_wohnteil: Option<f64>,
        /// Funktionsachsen
        ///
        /// Anzahl Funktionsachsen des Kernbaus.
        pub funktionsachsen: Option<f64>,
        /// Wohnbereich_Prozent
        ///
        /// Prozentualer Anteil der Fläche des Wohnbereiches im Verhältnis zum gesamten Kernbau, auf 5% gerundet.
        pub wohnbereich_prozent: Option<f64>,
        /// Keller
        ///
        /// Bauzeitlicher Keller vorhanden. Ja; Nein.
        pub keller: Option<String>,
        /// Bundflucht_Regelkonform
        ///
        /// Regelkonforme Bundseiten in Stube (rundherum voneinander abgewandte Bundseiten) und Tenn (zueinander blickende Bundseiten) nach Gut 2018. Angaben Stube und Tenn mit Semikolon getrennt. Ja; Nein.
        pub bundflucht_regelkonform: Option<String>,
        /// Wandverschluss
        ///
        /// Art der Wandverschlüsse im Kernbau. Bohlen; Flecklinge; Lehmflechtwerk.
        pub wandverschluss: Option<String>,
        /// Abstand_Bundfluchten
        ///
        /// Abstände der Firstständer im Kernbau, gemessen von Bundflucht zu Bundflucht. Die einzelnen Zahlen werden mit einem Semikolon getrennt.
        pub abstand_bundfluchten: Option<String>,
        /// Raumtiefe_Wohnteil
        ///
        /// Anzahl hintereinanderliegender Räume im Wohnteil des Kernbaus.
        pub raumtiefe_wohnteil: Option<String>,
        /// Länge_m
        ///
        /// Länge des Kernbaus in Metern, auf 0.1 m gerundet.
        pub lange_m: Option<f64>,
        /// Breite_m
        ///
        /// Breite des Kernbaus in Metern, auf 0.1 m gerundet.
        pub breite_m: Option<f64>,
        /// Höhe_m
        ///
        /// Höhe des Kernbaus in Metern, auf 0.1 m gerundet.
        pub hohe_m: Option<f64>,
        /// Dachform
        ///
        /// Art der bauzeitlichen Dachform des Kernbaus. Vollwalmdach; Satteldach; Schopfwalmdach.
        pub dachform: Option<String>,
        /// Dachbedeckung
        ///
        /// Material / Art der bauzeitlichen Dachbedeckung. Stroh; Brettschindel; Ziegel.
        pub dachbedeckung: Option<String>,
        /// Neigungs_Winkel_Rafen-Bundbalken_Grad
        ///
        /// Bauzeitliche Dachneigung. Gemessen zwischen Rafen und Bundbalken, auf 1 Grad gerundet.
        pub neigungs_winkel_rafen_bundbalken_grad: Option<f64>,
        /// Versteinerung
        ///
        /// Nachträgliche Versteinerung der Holzkonstruktion des Kernbaus. Ja; Nein.
        pub versteinerung: Option<String>,
        /// Versteinerung_Datierung
        ///
        /// Datierung der Versteinerung des Kernbaus, Angabe des Jahrhunderts.
        pub versteinerung_datierung: Option<String>,
        /// Versteinerung_Datierung_von
        ///
        /// Älteste mögliche Jahreszahl der Datierung der Versteinerung (t.p.q). Bei einer Inschrift oder Dendro bleibt das Feld leer.
        pub versteinerung_datierung_von: Option<String>,
        /// Versteinerung_Datierung_bis
        ///
        /// Jüngste mögliche Jahreszahl der Datierung der Versteinerung (t.a.q). Bei Dendro jüngst mögliches Schlagjahr.
        pub versteinerung_datierung_bis: Option<String>,
        /// Versteinerung_Datierung_Quelle
        ///
        /// Art der Datierung der Versteinerung. Dendro; Inschrift; Historisch; Quelle.
        pub versteinerung_datierung_quelle: Option<String>,
        /// Kommentar
        ///
        /// Ergänzende Bemerkungen zum Objekt.
        pub kommentar: Option<String>,
        pub geom: Option<GeoJson>,
        pub centroid: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsNummer,
        Gemeinde,
        Akte,
        Strasse,
        KoordinateXLv95,
        KoordinateYLv95,
        Schutzstatus,
        Abgebrochen,
        ErhaltungHolzkonstruktionProzent,
        KernbauDatierung,
        KernbauDatierungVon,
        KernbauDatierungBis,
        KernbauDatierungQuelle,
        Haustyp,
        Holzarten,
        Firststander,
        GeschosseWohnteil,
        Funktionsachsen,
        WohnbereichProzent,
        Keller,
        BundfluchtRegelkonform,
        Wandverschluss,
        AbstandBundfluchten,
        RaumtiefeWohnteil,
        LangeM,
        BreiteM,
        HoheM,
        Dachform,
        Dachbedeckung,
        NeigungsWinkelRafenBundbalkenGrad,
        Versteinerung,
        VersteinerungDatierung,
        VersteinerungDatierungVon,
        VersteinerungDatierungBis,
        VersteinerungDatierungQuelle,
        Kommentar,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Akte => "akte",
                Field::Strasse => "strasse",
                Field::KoordinateXLv95 => "koordinate_x_lv95",
                Field::KoordinateYLv95 => "koordinate_y_lv95",
                Field::Schutzstatus => "schutzstatus",
                Field::Abgebrochen => "abgebrochen",
                Field::ErhaltungHolzkonstruktionProzent => "erhaltung_holzkonstruktion_prozent",
                Field::KernbauDatierung => "kernbau_datierung",
                Field::KernbauDatierungVon => "kernbau_datierung_von",
                Field::KernbauDatierungBis => "kernbau_datierung_bis",
                Field::KernbauDatierungQuelle => "kernbau_datierung_quelle",
                Field::Haustyp => "haustyp",
                Field::Holzarten => "holzarten",
                Field::Firststander => "firststander",
                Field::GeschosseWohnteil => "geschosse_wohnteil",
                Field::Funktionsachsen => "funktionsachsen",
                Field::WohnbereichProzent => "wohnbereich_prozent",
                Field::Keller => "keller",
                Field::BundfluchtRegelkonform => "bundflucht_regelkonform",
                Field::Wandverschluss => "wandverschluss",
                Field::AbstandBundfluchten => "abstand_bundfluchten",
                Field::RaumtiefeWohnteil => "raumtiefe_wohnteil",
                Field::LangeM => "lange_m",
                Field::BreiteM => "breite_m",
                Field::HoheM => "hohe_m",
                Field::Dachform => "dachform",
                Field::Dachbedeckung => "dachbedeckung",
                Field::NeigungsWinkelRafenBundbalkenGrad => "neigungs_winkel_rafen_bundbalken_grad",
                Field::Versteinerung => "versteinerung",
                Field::VersteinerungDatierung => "versteinerung_datierung",
                Field::VersteinerungDatierungVon => "versteinerung_datierung_von",
                Field::VersteinerungDatierungBis => "versteinerung_datierung_bis",
                Field::VersteinerungDatierungQuelle => "versteinerung_datierung_quelle",
                Field::Kommentar => "kommentar",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11100/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Adressen der Primar-, Sekundar- und Musikschulen (Juni 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11150/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11150/</a>\n"]
#[doc = "<p>Liste der \u{f6}ffentlichen\u{a0}Primar-, Sekundar- und Musikschulen</p><p>Die Sekundarschulkreise sind im\u{a0}<a href=\"https://bl.clex.ch/app/de/texts_of_law/642.1\" target=\"_blank\">Dekret \u{fc}ber die Sekundarschulkreise und Sekundarschulstandorte</a> definiert.</p>"]
#[cfg(feature = "bl11150")]
pub mod adressen_der_primar_sekundar_und_musikschulen_juni_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Schule_Code
        pub schule_code: Option<String>,
        /// Schule_Name
        pub schule_name: Option<String>,
        /// Sekundarschulkreis_Code
        pub sekundarschulkreis_code: Option<String>,
        /// Sekundarschulkreis
        pub sekundarschulkreis: Option<String>,
        /// Adresse
        pub adresse: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Ort
        pub ort: Option<String>,
        /// Kategorie
        pub kategorie: Option<String>,
        /// Telefon
        pub telefon: Option<String>,
        /// E_Mail
        pub e_mail: Option<String>,
        /// URL
        pub url: Option<String>,
        pub e_eingangskoordinate: Option<i64>,
        pub n_eingangskoordinate: Option<i64>,
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsGemeindenummer,
        Gemeinde,
        SchuleCode,
        SchuleName,
        SekundarschulkreisCode,
        Sekundarschulkreis,
        Adresse,
        Plz,
        Ort,
        Kategorie,
        Telefon,
        EMail,
        Url,
        EEingangskoordinate,
        NEingangskoordinate,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::SchuleCode => "schule_code",
                Field::SchuleName => "schule_name",
                Field::SekundarschulkreisCode => "sekundarschulkreis_code",
                Field::Sekundarschulkreis => "sekundarschulkreis",
                Field::Adresse => "adresse",
                Field::Plz => "plz",
                Field::Ort => "ort",
                Field::Kategorie => "kategorie",
                Field::Telefon => "telefon",
                Field::EMail => "e_mail",
                Field::Url => "url",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11150/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Adressen der Privatschulen (Juni 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11160/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11160/</a>\n"]
#[doc = "<p>Liste der vom Kanton BL bewilligten Privatschulen</p>"]
#[cfg(feature = "bl11160")]
pub mod adressen_der_privatschulen_juni_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Schule_Code
        pub schule_code: Option<String>,
        /// Schule_Name
        pub schule_name: Option<String>,
        /// Adresse
        pub adresse: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Ort
        pub ort: Option<String>,
        /// Kategorie
        pub kategorie: Option<String>,
        /// Schulstufe
        pub schulstufe: Option<String>,
        /// Telefon
        pub telefon: Option<String>,
        /// E_Mail
        pub e_mail: Option<String>,
        /// URL
        pub url: Option<String>,
        pub e_eingangskoordinate: Option<i64>,
        pub n_eingangskoordinate: Option<i64>,
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsGemeindenummer,
        Gemeinde,
        SchuleCode,
        SchuleName,
        Adresse,
        Plz,
        Ort,
        Kategorie,
        Schulstufe,
        Telefon,
        EMail,
        Url,
        EEingangskoordinate,
        NEingangskoordinate,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::SchuleCode => "schule_code",
                Field::SchuleName => "schule_name",
                Field::Adresse => "adresse",
                Field::Plz => "plz",
                Field::Ort => "ort",
                Field::Kategorie => "kategorie",
                Field::Schulstufe => "schulstufe",
                Field::Telefon => "telefon",
                Field::EMail => "e_mail",
                Field::Url => "url",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11160/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Adressen der Sonderschulen und Schulheime (Juni 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11200/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11200/</a>\n"]
#[doc = "<p>Liste der Tagessonderschulen und Schulheime</p>"]
#[cfg(feature = "bl11200")]
pub mod adressen_der_sonderschulen_und_schulheime_juni_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Standortgemeinde
        pub standortgemeinde: Option<String>,
        /// Schule_Code
        pub schule_code: Option<String>,
        /// Schule_Name
        pub schule_name: Option<String>,
        /// Adresse
        pub adresse: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Ort
        pub ort: Option<String>,
        /// Kategorie
        pub kategorie: Option<String>,
        /// Telefon
        pub telefon: Option<String>,
        /// E_Mail
        pub e_mail: Option<String>,
        /// URL
        pub url: Option<String>,
        pub e_eingangskoordinate: Option<i64>,
        pub n_eingangskoordinate: Option<i64>,
        pub koordinaten: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsGemeindenummer,
        Standortgemeinde,
        SchuleCode,
        SchuleName,
        Adresse,
        Plz,
        Ort,
        Kategorie,
        Telefon,
        EMail,
        Url,
        EEingangskoordinate,
        NEingangskoordinate,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Standortgemeinde => "standortgemeinde",
                Field::SchuleCode => "schule_code",
                Field::SchuleName => "schule_name",
                Field::Adresse => "adresse",
                Field::Plz => "plz",
                Field::Ort => "ort",
                Field::Kategorie => "kategorie",
                Field::Telefon => "telefon",
                Field::EMail => "e_mail",
                Field::Url => "url",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11200/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Swisslos Sportfonds: Bilanz nach Gesuchsteller, Kategorie, Objekt und Jahr (seit 2011)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11450/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11450/</a>\n"]
#[doc = "<p>Beitr\u{e4}ge zur F\u{f6}rderung des kantonalen Breitensports. Bei Betr\u{e4}gen an nat\u{fc}rliche Personen werden keine Werte ausgewiesen.</p>"]
#[cfg(feature = "bl11450")]
pub mod swisslos_sportfonds_bilanz_nach_gesuchsteller_kategorie_objekt_und_jahr_seit_2011 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Gesuchsteller
        pub gesuchsteller: Option<String>,
        /// Kategorie
        pub kategorie: Option<String>,
        /// Objekt
        pub objekt: Option<String>,
        /// Ausbezahlter_Betrag_CHF
        pub ausbezahlter_betrag_chf: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Gesuchsteller,
        Kategorie,
        Objekt,
        AusbezahlterBetragChf,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Gesuchsteller => "gesuchsteller",
                Field::Kategorie => "kategorie",
                Field::Objekt => "objekt",
                Field::AusbezahlterBetragChf => "ausbezahlter_betrag_chf",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11450/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Swisslos-Fonds: Unterst\u{fc}tzte Projekte nach Sparte und Betrag (seit 2011)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11460/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11460/</a>\n"]
#[doc = "<p>Durch den Kanton Basel-Landschaft geleistete Beitr\u{e4}ge aus dem kantonalen Fonds der Genossenschaft der Interkantonalen Landeslotterie SWISSLOS an wohlt\u{e4}tige, gemeinn\u{fc}tzige und kulturelle Projekte seit 2011.</p><p>Kriterien f\u{fc}r unterst\u{fc}tzte Projekte k\u{f6}nnen der Verordnung \u{fc}ber den Swisslos-Fonds (<a href=\"https://bl.clex.ch/app/de/texts_of_law/543.12\" target=\"_blank\">SGS 543.12</a>) entnommen werden.</p><p>\u{dc}ber den <a href=\"https://data.bl.ch/explore/dataset/11450/table/?disjunctive.jahr&amp;disjunctive.kategorie\" target=\"_blank\">Swisslos-Sportfonds</a> des Kantons Basel-Landschaft geleistete Beitr\u{e4}ge sind nicht Teil dieses Datensatzes.</p>"]
#[cfg(feature = "bl11460")]
pub mod swisslos_fonds_unterstuetzte_projekte_nach_sparte_und_betrag_seit_2011 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Begünstigte
        pub begunstigte: Option<String>,
        /// Unterstütztes_Projekt
        pub unterstutztes_projekt: Option<String>,
        /// Sparte
        pub sparte: Option<String>,
        /// Beitrag_CHF
        pub beitrag_chf: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Begunstigte,
        UnterstutztesProjekt,
        Sparte,
        BeitragChf,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Begunstigte => "begunstigte",
                Field::UnterstutztesProjekt => "unterstutztes_projekt",
                Field::Sparte => "sparte",
                Field::BeitragChf => "beitrag_chf",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11460/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Jugend und Sport: Anzahl Kurse, Teilnehmende und Leitende nach Sportart und Jahr (seit 2005)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11470/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11470/</a>\n"]
#[doc = "<p>J+S-Statistik</p><p>Die Sportarten wurden teilweise zusammengefasst.</p>"]
#[cfg(feature = "bl11470")]
pub mod jugend_und_sport_anzahl_kurse_teilnehmende_und_leitende_nach_sportart_und_jahr_seit_2005 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Sportart
        pub sportart: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Anzahl
        pub anzahl: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Sportart,
        Indikator,
        Geschlecht,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Sportart => "sportart",
                Field::Indikator => "indikator",
                Field::Geschlecht => "geschlecht",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11470/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Luftqualit\u{e4}t Station Liestal (halbst\u{fc}ndliche Messdaten Januar 2000 - November 2016)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11540/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11540/</a>\n"]
#[doc = "<p class=\"\">Historische Daten der Luftmessstation Liestal von Anfang 2000 bis Ende 2016. \u{c4}ltere Daten k\u{f6}nnen beim Lufthygieneamt beider Basel direkt bezogen werden. Die Messwerte sind halbst\u{fc}ndlich ausgewiesen und bereinigt. Seit Ende November 2016 ist die Station stillgelegt.</p><p class=\"\">Das Auftreten allf\u{e4}lliger Negativwerte stammt von messtechnischen Ungenauigkeiten. Diese Messunsicherheit ist bei der Interpretation der Zahlen entsprechend mit einzubeziehen.</p><p class=\"\">Die Zeitstempel entsprechen der Zeitzone Europe/Zurich obwohl sie im Zeitformat UTC angegeben sind. Allf\u{e4}llige Fragen zum Zeitformat beantwortet das Amt f\u{fc}r Lufthygiene beider Basel auf Anfrage.</p><p style=\"\"><span style=\"font-weight: 700;\">Ausgewiesene Werte</span><br></p><ul><li>Anfangszeit: Zeitstempel des Beginns der halbst\u{fc}ndlichen Messung im Format %Y-%m-%dT%H:%M:%S</li><li>Stickstoffdioxid NO2 (\u{b5}g/m3): Gemessene Stickstoffdioxid-Konzentration in Mikrogramm pro Kubikmeter.</li><li>Ozon O3 (\u{b5}g/m3): Gemessene Ozon-Konzentration in Mikrogramm pro Kubikmeter.</li></ul><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Standortbeschreibung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Die Messstation befand sich an der Rheinstrasse 44 im ehemaligen Sitz des Lufthygieneamts beider Basel. Aufgrund der Strassenn\u{e4}he war die Hauptbelastungsquelle der Verkehr.\u{a0}</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Lage</span></p><p class=\"\" style=\"font-family: sans-serif;\">Kleinst\u{e4}dtisch/Vorst\u{e4}dtisch, verkehrsbelastet</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Koordinaten</span></p><p class=\"\" style=\"font-family: sans-serif;\">2621790 / 1259900; 308 m \u{fc}. M.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Bebauung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Offene Bebauung</p>"]
#[cfg(feature = "bl11540")]
pub mod luftqualitaet_station_liestal_halbstuendliche_messdaten_januar_2000_november_2016 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Anfangszeit
        #[serde(with = "time::serde::iso8601::option")]
        pub anfangszeit: Option<OffsetDateTime>,
        /// NO2
        ///
        /// Stickstoffdioxid
        pub no2: Option<f64>,
        /// O3
        ///
        /// Ozon
        pub o3: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Anfangszeit,
        No2,
        O3,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Anfangszeit => "anfangszeit",
                Field::No2 => "no2",
                Field::O3 => "o3",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11540/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2023: Kandidierendenresultate, Wahlberechtigte und Parteistimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11590/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11590/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 12. Februar 2023</p>"]
#[cfg(feature = "bl11590")]
pub mod landratswahlen_2023_kandidierendenresultate_wahlberechtigte_und_parteistimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_gender: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub compound_id: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        /// Parteistimmen
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateGender,
        CandidateId,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        CompoundId,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateGender => "candidate_gender",
                Field::CandidateId => "candidate_id",
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CompoundId => "compound_id",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11590/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratswahlen 2023: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11600/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11600/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 12. Februar 2023</p>"]
#[cfg(feature = "bl11600")]
pub mod regierungsratswahlen_2023_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11600/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2023: Panaschierstimmen der Kandidierenden"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11610/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11610/</a>\n"]
#[doc = "Kantonale Wahlen vom 12. Februar 2023. Die jeweilige Spalte mit den Panaschierstimmen der eigenen Partei enth\u{e4}lt auch die unver\u{e4}nderten Stimmen der kandidierenden Person."]
#[cfg(feature = "bl11610")]
pub mod landratswahlen_2023_panaschierstimmen_der_kandidierenden {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub entity_district_id: Option<i64>,
        pub entity_district_name: Option<String>,
        pub election_status: Option<String>,
        pub entity_id: Option<i64>,
        pub entity_name: Option<String>,
        pub list_name: Option<String>,
        pub list_id: Option<i64>,
        pub list_number_of_mandates: Option<i64>,
        pub list_votes: Option<i64>,
        pub candidate_id: Option<String>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_elected: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_gender: Option<String>,
        pub candidate_year_of_birth: Option<String>,
        pub candidate_votes: Option<i64>,
        /// votes_from_FDP
        pub votes_from_fdp: Option<i64>,
        /// votes_from_SP
        pub votes_from_sp: Option<i64>,
        /// votes_from_SVP
        pub votes_from_svp: Option<i64>,
        /// votes_from_EVP
        pub votes_from_evp: Option<i64>,
        /// votes_from_DieMitteBL
        pub votes_from_diemittebl: Option<i64>,
        /// votes_from_Grüne
        pub votes_from_grune: Option<i64>,
        /// votes_from_GLP
        pub votes_from_glp: Option<i64>,
        /// votes_from_CuP
        pub votes_from_cup: Option<i64>,
        /// votes_from_AVP
        pub votes_from_avp: Option<i64>,
        /// votes_from_Leer
        pub votes_from_leer: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        EntityDistrictId,
        EntityDistrictName,
        ElectionStatus,
        EntityId,
        EntityName,
        ListName,
        ListId,
        ListNumberOfMandates,
        ListVotes,
        CandidateId,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateElected,
        CandidateParty,
        CandidateGender,
        CandidateYearOfBirth,
        CandidateVotes,
        VotesFromFdp,
        VotesFromSp,
        VotesFromSvp,
        VotesFromEvp,
        VotesFromDiemittebl,
        VotesFromGrune,
        VotesFromGlp,
        VotesFromCup,
        VotesFromAvp,
        VotesFromLeer,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::EntityDistrictId => "entity_district_id",
                Field::EntityDistrictName => "entity_district_name",
                Field::ElectionStatus => "election_status",
                Field::EntityId => "entity_id",
                Field::EntityName => "entity_name",
                Field::ListName => "list_name",
                Field::ListId => "list_id",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
                Field::CandidateId => "candidate_id",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateElected => "candidate_elected",
                Field::CandidateParty => "candidate_party",
                Field::CandidateGender => "candidate_gender",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CandidateVotes => "candidate_votes",
                Field::VotesFromFdp => "votes_from_fdp",
                Field::VotesFromSp => "votes_from_sp",
                Field::VotesFromSvp => "votes_from_svp",
                Field::VotesFromEvp => "votes_from_evp",
                Field::VotesFromDiemittebl => "votes_from_diemittebl",
                Field::VotesFromGrune => "votes_from_grune",
                Field::VotesFromGlp => "votes_from_glp",
                Field::VotesFromCup => "votes_from_cup",
                Field::VotesFromAvp => "votes_from_avp",
                Field::VotesFromLeer => "votes_from_leer",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11610/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2023: Kandidierende nach Liste, Geschlecht, Jahrgang, Beruf und Wahlkreis"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11660/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11660/</a>\n"]
#[doc = "<p>Wahlvorschl\u{e4}ge f\u{fc}r die Landratswahlen vom 12. Februar 2023</p>"]
#[cfg(feature = "bl11660")]
pub mod landratswahlen_2023_kandidierende_nach_liste_geschlecht_jahrgang_beruf_und_wahlkreis {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlkreis-Nr
        pub wahlkreis_nr: Option<i64>,
        /// Wahlkreis
        pub wahlkreis: Option<String>,
        /// Region
        pub region: Option<String>,
        /// Wahltermin
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub wahltermin: Option<Date>,
        /// Anzahl_Sitze
        pub anzahl_sitze: Option<i64>,
        /// Listen-Nr
        pub listen_nr: Option<String>,
        /// Parteikurzbezeichnung
        pub parteikurzbezeichnung: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
        /// Anzahl_leere_Linien
        pub anzahl_leere_linien: Option<i64>,
        /// Zeilen-Nr
        pub zeilen_nr: Option<i64>,
        /// Kandidaten-Nr
        pub kandidaten_nr: Option<String>,
        /// Kumulation
        pub kumulation: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Titel
        pub titel: Option<String>,
        /// Beruf_Tätigkeit
        pub beruf_tatigkeit: Option<String>,
        /// Zusatz
        pub zusatz: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Wohnort
        pub ort: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        WahlkreisNr,
        Wahlkreis,
        Region,
        Wahltermin,
        AnzahlSitze,
        ListenNr,
        Parteikurzbezeichnung,
        Parteibezeichnung,
        AnzahlLeereLinien,
        ZeilenNr,
        KandidatenNr,
        Kumulation,
        Bisher,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        Titel,
        BerufTatigkeit,
        Zusatz,
        Plz,
        Ort,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::WahlkreisNr => "wahlkreis_nr",
                Field::Wahlkreis => "wahlkreis",
                Field::Region => "region",
                Field::Wahltermin => "wahltermin",
                Field::AnzahlSitze => "anzahl_sitze",
                Field::ListenNr => "listen_nr",
                Field::Parteikurzbezeichnung => "parteikurzbezeichnung",
                Field::Parteibezeichnung => "parteibezeichnung",
                Field::AnzahlLeereLinien => "anzahl_leere_linien",
                Field::ZeilenNr => "zeilen_nr",
                Field::KandidatenNr => "kandidaten_nr",
                Field::Kumulation => "kumulation",
                Field::Bisher => "bisher",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::Titel => "titel",
                Field::BerufTatigkeit => "beruf_tatigkeit",
                Field::Zusatz => "zusatz",
                Field::Plz => "plz",
                Field::Ort => "ort",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11660/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen: Wahlkreise"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11710/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11710/</a>\n"]
#[doc = "<p>Polygondaten als Shapefile oder GeoJSON</p>"]
#[cfg(feature = "bl11710")]
pub mod landratswahlen_wahlkreise {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlkreis_Nr
        pub wahlkreisn: Option<i64>,
        /// Wahlkreis
        pub wahlkreis: Option<String>,
        /// Geo Shape
        pub geo_shape: Option<GeoJson>,
        pub geo_point_2d: Option<GeoPoint2d>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlkreisn,
        Wahlkreis,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlkreisn => "wahlkreisn",
                Field::Wahlkreis => "wahlkreis",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11710/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2019: Kandidierendenresultate, Wahlberechtigte und Parteistimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11720/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11720/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 31. M\u{e4}rz 2019</p>"]
#[cfg(feature = "bl11720")]
pub mod landratswahlen_2019_kandidierendenresultate_wahlberechtigte_und_parteistimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub compound_id: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        CompoundId,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CompoundId => "compound_id",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11720/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatswahlen 2019: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11730/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11730/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 20. Oktober 2019<br></p>"]
#[cfg(feature = "bl11730")]
pub mod staenderatswahlen_2019_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<String>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11730/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2019: Kandidierendenresultate, Wahlberechtigte und Listenstimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11740/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11740/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 20. Oktober 2019<br></p>"]
#[cfg(feature = "bl11740")]
pub mod nationalratswahlen_2019_kandidierendenresultate_wahlberechtigte_und_listenstimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_gender: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_panachage_votes_from_list_01: Option<String>,
        pub candidate_panachage_votes_from_list_02: Option<String>,
        pub candidate_panachage_votes_from_list_03: Option<String>,
        pub candidate_panachage_votes_from_list_04: Option<String>,
        pub candidate_panachage_votes_from_list_05: Option<String>,
        pub candidate_panachage_votes_from_list_06: Option<String>,
        pub candidate_panachage_votes_from_list_07: Option<String>,
        pub candidate_panachage_votes_from_list_08: Option<i64>,
        pub candidate_panachage_votes_from_list_11: Option<String>,
        pub candidate_panachage_votes_from_list_12: Option<String>,
        pub candidate_panachage_votes_from_list_13: Option<String>,
        pub candidate_panachage_votes_from_list_22: Option<String>,
        pub candidate_panachage_votes_from_list_23: Option<String>,
        pub candidate_panachage_votes_from_list_33: Option<String>,
        pub candidate_panachage_votes_from_list_34: Option<String>,
        pub candidate_panachage_votes_from_list_44: Option<String>,
        pub candidate_panachage_votes_from_list_55: Option<String>,
        pub candidate_panachage_votes_from_list_56: Option<String>,
        pub candidate_panachage_votes_from_list_70: Option<String>,
        pub candidate_panachage_votes_from_list_77: Option<String>,
        pub candidate_panachage_votes_from_list_999: Option<i64>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_connection: Option<i64>,
        pub list_connection_parent: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_panachage_votes_from_list_01: Option<i64>,
        pub list_panachage_votes_from_list_02: Option<i64>,
        pub list_panachage_votes_from_list_03: Option<i64>,
        pub list_panachage_votes_from_list_04: Option<i64>,
        pub list_panachage_votes_from_list_05: Option<i64>,
        pub list_panachage_votes_from_list_06: Option<i64>,
        pub list_panachage_votes_from_list_07: Option<i64>,
        pub list_panachage_votes_from_list_08: Option<String>,
        pub list_panachage_votes_from_list_11: Option<i64>,
        pub list_panachage_votes_from_list_12: Option<i64>,
        pub list_panachage_votes_from_list_13: Option<i64>,
        pub list_panachage_votes_from_list_22: Option<i64>,
        pub list_panachage_votes_from_list_23: Option<i64>,
        pub list_panachage_votes_from_list_33: Option<i64>,
        pub list_panachage_votes_from_list_34: Option<String>,
        pub list_panachage_votes_from_list_44: Option<i64>,
        pub list_panachage_votes_from_list_55: Option<i64>,
        pub list_panachage_votes_from_list_56: Option<i64>,
        pub list_panachage_votes_from_list_70: Option<i64>,
        pub list_panachage_votes_from_list_77: Option<i64>,
        pub list_panachage_votes_from_list_999: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateGender,
        CandidateId,
        CandidatePanachageVotesFromList01,
        CandidatePanachageVotesFromList02,
        CandidatePanachageVotesFromList03,
        CandidatePanachageVotesFromList04,
        CandidatePanachageVotesFromList05,
        CandidatePanachageVotesFromList06,
        CandidatePanachageVotesFromList07,
        CandidatePanachageVotesFromList08,
        CandidatePanachageVotesFromList11,
        CandidatePanachageVotesFromList12,
        CandidatePanachageVotesFromList13,
        CandidatePanachageVotesFromList22,
        CandidatePanachageVotesFromList23,
        CandidatePanachageVotesFromList33,
        CandidatePanachageVotesFromList34,
        CandidatePanachageVotesFromList44,
        CandidatePanachageVotesFromList55,
        CandidatePanachageVotesFromList56,
        CandidatePanachageVotesFromList70,
        CandidatePanachageVotesFromList77,
        CandidatePanachageVotesFromList999,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListConnection,
        ListConnectionParent,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListPanachageVotesFromList01,
        ListPanachageVotesFromList02,
        ListPanachageVotesFromList03,
        ListPanachageVotesFromList04,
        ListPanachageVotesFromList05,
        ListPanachageVotesFromList06,
        ListPanachageVotesFromList07,
        ListPanachageVotesFromList08,
        ListPanachageVotesFromList11,
        ListPanachageVotesFromList12,
        ListPanachageVotesFromList13,
        ListPanachageVotesFromList22,
        ListPanachageVotesFromList23,
        ListPanachageVotesFromList33,
        ListPanachageVotesFromList34,
        ListPanachageVotesFromList44,
        ListPanachageVotesFromList55,
        ListPanachageVotesFromList56,
        ListPanachageVotesFromList70,
        ListPanachageVotesFromList77,
        ListPanachageVotesFromList999,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateGender => "candidate_gender",
                Field::CandidateId => "candidate_id",
                Field::CandidatePanachageVotesFromList01 => {
                    "candidate_panachage_votes_from_list_01"
                }
                Field::CandidatePanachageVotesFromList02 => {
                    "candidate_panachage_votes_from_list_02"
                }
                Field::CandidatePanachageVotesFromList03 => {
                    "candidate_panachage_votes_from_list_03"
                }
                Field::CandidatePanachageVotesFromList04 => {
                    "candidate_panachage_votes_from_list_04"
                }
                Field::CandidatePanachageVotesFromList05 => {
                    "candidate_panachage_votes_from_list_05"
                }
                Field::CandidatePanachageVotesFromList06 => {
                    "candidate_panachage_votes_from_list_06"
                }
                Field::CandidatePanachageVotesFromList07 => {
                    "candidate_panachage_votes_from_list_07"
                }
                Field::CandidatePanachageVotesFromList08 => {
                    "candidate_panachage_votes_from_list_08"
                }
                Field::CandidatePanachageVotesFromList11 => {
                    "candidate_panachage_votes_from_list_11"
                }
                Field::CandidatePanachageVotesFromList12 => {
                    "candidate_panachage_votes_from_list_12"
                }
                Field::CandidatePanachageVotesFromList13 => {
                    "candidate_panachage_votes_from_list_13"
                }
                Field::CandidatePanachageVotesFromList22 => {
                    "candidate_panachage_votes_from_list_22"
                }
                Field::CandidatePanachageVotesFromList23 => {
                    "candidate_panachage_votes_from_list_23"
                }
                Field::CandidatePanachageVotesFromList33 => {
                    "candidate_panachage_votes_from_list_33"
                }
                Field::CandidatePanachageVotesFromList34 => {
                    "candidate_panachage_votes_from_list_34"
                }
                Field::CandidatePanachageVotesFromList44 => {
                    "candidate_panachage_votes_from_list_44"
                }
                Field::CandidatePanachageVotesFromList55 => {
                    "candidate_panachage_votes_from_list_55"
                }
                Field::CandidatePanachageVotesFromList56 => {
                    "candidate_panachage_votes_from_list_56"
                }
                Field::CandidatePanachageVotesFromList70 => {
                    "candidate_panachage_votes_from_list_70"
                }
                Field::CandidatePanachageVotesFromList77 => {
                    "candidate_panachage_votes_from_list_77"
                }
                Field::CandidatePanachageVotesFromList999 => {
                    "candidate_panachage_votes_from_list_999"
                }
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListConnection => "list_connection",
                Field::ListConnectionParent => "list_connection_parent",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListPanachageVotesFromList01 => "list_panachage_votes_from_list_01",
                Field::ListPanachageVotesFromList02 => "list_panachage_votes_from_list_02",
                Field::ListPanachageVotesFromList03 => "list_panachage_votes_from_list_03",
                Field::ListPanachageVotesFromList04 => "list_panachage_votes_from_list_04",
                Field::ListPanachageVotesFromList05 => "list_panachage_votes_from_list_05",
                Field::ListPanachageVotesFromList06 => "list_panachage_votes_from_list_06",
                Field::ListPanachageVotesFromList07 => "list_panachage_votes_from_list_07",
                Field::ListPanachageVotesFromList08 => "list_panachage_votes_from_list_08",
                Field::ListPanachageVotesFromList11 => "list_panachage_votes_from_list_11",
                Field::ListPanachageVotesFromList12 => "list_panachage_votes_from_list_12",
                Field::ListPanachageVotesFromList13 => "list_panachage_votes_from_list_13",
                Field::ListPanachageVotesFromList22 => "list_panachage_votes_from_list_22",
                Field::ListPanachageVotesFromList23 => "list_panachage_votes_from_list_23",
                Field::ListPanachageVotesFromList33 => "list_panachage_votes_from_list_33",
                Field::ListPanachageVotesFromList34 => "list_panachage_votes_from_list_34",
                Field::ListPanachageVotesFromList44 => "list_panachage_votes_from_list_44",
                Field::ListPanachageVotesFromList55 => "list_panachage_votes_from_list_55",
                Field::ListPanachageVotesFromList56 => "list_panachage_votes_from_list_56",
                Field::ListPanachageVotesFromList70 => "list_panachage_votes_from_list_70",
                Field::ListPanachageVotesFromList77 => "list_panachage_votes_from_list_77",
                Field::ListPanachageVotesFromList999 => "list_panachage_votes_from_list_999",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11740/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratswahlen 2019: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11750/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11750/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 31. M\u{e4}rz 2019</p>"]
#[cfg(feature = "bl11750")]
pub mod regierungsratswahlen_2019_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11750/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatsnachwahl 2019: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11760/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11760/</a>\n"]
#[doc = "<p>Kantonale Nachwahl vom 24. November 2019</p>"]
#[cfg(feature = "bl11760")]
pub mod staenderatsnachwahl_2019_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11760/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2015: Kandidierendenresultate, Wahlberechtigte und Parteistimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11770/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11770/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 8. Februar 2015</p>"]
#[cfg(feature = "bl11770")]
pub mod landratswahlen_2015_kandidierendenresultate_wahlberechtigte_und_parteistimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub compound_id: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        CompoundId,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CompoundId => "compound_id",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11770/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatswahlen 2015: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11780/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11780/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 18. Oktober 2015<br></p>"]
#[cfg(feature = "bl11780")]
pub mod staenderatswahlen_2015_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11780/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2015: Kandidierendenresultate, Wahlberechtigte und Listenstimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11790/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11790/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 18. Oktober 2015<br></p>"]
#[cfg(feature = "bl11790")]
pub mod nationalratswahlen_2015_kandidierendenresultate_wahlberechtigte_und_listenstimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_gender: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_panachage_votes_from_list_01: Option<String>,
        pub candidate_panachage_votes_from_list_02: Option<i64>,
        pub candidate_panachage_votes_from_list_03: Option<i64>,
        pub candidate_panachage_votes_from_list_04: Option<String>,
        pub candidate_panachage_votes_from_list_05: Option<String>,
        pub candidate_panachage_votes_from_list_06: Option<String>,
        pub candidate_panachage_votes_from_list_07: Option<String>,
        pub candidate_panachage_votes_from_list_08: Option<i64>,
        pub candidate_panachage_votes_from_list_10: Option<String>,
        pub candidate_panachage_votes_from_list_11: Option<i64>,
        pub candidate_panachage_votes_from_list_14: Option<String>,
        pub candidate_panachage_votes_from_list_22: Option<String>,
        pub candidate_panachage_votes_from_list_34: Option<String>,
        pub candidate_panachage_votes_from_list_55: Option<String>,
        pub candidate_panachage_votes_from_list_70: Option<String>,
        pub candidate_panachage_votes_from_list_77: Option<String>,
        pub candidate_panachage_votes_from_list_999: Option<i64>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_connection: Option<i64>,
        pub list_connection_parent: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_panachage_votes_from_list_01: Option<i64>,
        pub list_panachage_votes_from_list_02: Option<i64>,
        pub list_panachage_votes_from_list_03: Option<i64>,
        pub list_panachage_votes_from_list_04: Option<i64>,
        pub list_panachage_votes_from_list_05: Option<i64>,
        pub list_panachage_votes_from_list_06: Option<i64>,
        pub list_panachage_votes_from_list_07: Option<i64>,
        pub list_panachage_votes_from_list_08: Option<String>,
        pub list_panachage_votes_from_list_10: Option<i64>,
        pub list_panachage_votes_from_list_11: Option<i64>,
        pub list_panachage_votes_from_list_14: Option<i64>,
        pub list_panachage_votes_from_list_22: Option<i64>,
        pub list_panachage_votes_from_list_34: Option<i64>,
        pub list_panachage_votes_from_list_55: Option<i64>,
        pub list_panachage_votes_from_list_70: Option<i64>,
        pub list_panachage_votes_from_list_77: Option<i64>,
        pub list_panachage_votes_from_list_999: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateGender,
        CandidateId,
        CandidatePanachageVotesFromList01,
        CandidatePanachageVotesFromList02,
        CandidatePanachageVotesFromList03,
        CandidatePanachageVotesFromList04,
        CandidatePanachageVotesFromList05,
        CandidatePanachageVotesFromList06,
        CandidatePanachageVotesFromList07,
        CandidatePanachageVotesFromList08,
        CandidatePanachageVotesFromList10,
        CandidatePanachageVotesFromList11,
        CandidatePanachageVotesFromList14,
        CandidatePanachageVotesFromList22,
        CandidatePanachageVotesFromList34,
        CandidatePanachageVotesFromList55,
        CandidatePanachageVotesFromList70,
        CandidatePanachageVotesFromList77,
        CandidatePanachageVotesFromList999,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListConnection,
        ListConnectionParent,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListPanachageVotesFromList01,
        ListPanachageVotesFromList02,
        ListPanachageVotesFromList03,
        ListPanachageVotesFromList04,
        ListPanachageVotesFromList05,
        ListPanachageVotesFromList06,
        ListPanachageVotesFromList07,
        ListPanachageVotesFromList08,
        ListPanachageVotesFromList10,
        ListPanachageVotesFromList11,
        ListPanachageVotesFromList14,
        ListPanachageVotesFromList22,
        ListPanachageVotesFromList34,
        ListPanachageVotesFromList55,
        ListPanachageVotesFromList70,
        ListPanachageVotesFromList77,
        ListPanachageVotesFromList999,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateGender => "candidate_gender",
                Field::CandidateId => "candidate_id",
                Field::CandidatePanachageVotesFromList01 => {
                    "candidate_panachage_votes_from_list_01"
                }
                Field::CandidatePanachageVotesFromList02 => {
                    "candidate_panachage_votes_from_list_02"
                }
                Field::CandidatePanachageVotesFromList03 => {
                    "candidate_panachage_votes_from_list_03"
                }
                Field::CandidatePanachageVotesFromList04 => {
                    "candidate_panachage_votes_from_list_04"
                }
                Field::CandidatePanachageVotesFromList05 => {
                    "candidate_panachage_votes_from_list_05"
                }
                Field::CandidatePanachageVotesFromList06 => {
                    "candidate_panachage_votes_from_list_06"
                }
                Field::CandidatePanachageVotesFromList07 => {
                    "candidate_panachage_votes_from_list_07"
                }
                Field::CandidatePanachageVotesFromList08 => {
                    "candidate_panachage_votes_from_list_08"
                }
                Field::CandidatePanachageVotesFromList10 => {
                    "candidate_panachage_votes_from_list_10"
                }
                Field::CandidatePanachageVotesFromList11 => {
                    "candidate_panachage_votes_from_list_11"
                }
                Field::CandidatePanachageVotesFromList14 => {
                    "candidate_panachage_votes_from_list_14"
                }
                Field::CandidatePanachageVotesFromList22 => {
                    "candidate_panachage_votes_from_list_22"
                }
                Field::CandidatePanachageVotesFromList34 => {
                    "candidate_panachage_votes_from_list_34"
                }
                Field::CandidatePanachageVotesFromList55 => {
                    "candidate_panachage_votes_from_list_55"
                }
                Field::CandidatePanachageVotesFromList70 => {
                    "candidate_panachage_votes_from_list_70"
                }
                Field::CandidatePanachageVotesFromList77 => {
                    "candidate_panachage_votes_from_list_77"
                }
                Field::CandidatePanachageVotesFromList999 => {
                    "candidate_panachage_votes_from_list_999"
                }
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListConnection => "list_connection",
                Field::ListConnectionParent => "list_connection_parent",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListPanachageVotesFromList01 => "list_panachage_votes_from_list_01",
                Field::ListPanachageVotesFromList02 => "list_panachage_votes_from_list_02",
                Field::ListPanachageVotesFromList03 => "list_panachage_votes_from_list_03",
                Field::ListPanachageVotesFromList04 => "list_panachage_votes_from_list_04",
                Field::ListPanachageVotesFromList05 => "list_panachage_votes_from_list_05",
                Field::ListPanachageVotesFromList06 => "list_panachage_votes_from_list_06",
                Field::ListPanachageVotesFromList07 => "list_panachage_votes_from_list_07",
                Field::ListPanachageVotesFromList08 => "list_panachage_votes_from_list_08",
                Field::ListPanachageVotesFromList10 => "list_panachage_votes_from_list_10",
                Field::ListPanachageVotesFromList11 => "list_panachage_votes_from_list_11",
                Field::ListPanachageVotesFromList14 => "list_panachage_votes_from_list_14",
                Field::ListPanachageVotesFromList22 => "list_panachage_votes_from_list_22",
                Field::ListPanachageVotesFromList34 => "list_panachage_votes_from_list_34",
                Field::ListPanachageVotesFromList55 => "list_panachage_votes_from_list_55",
                Field::ListPanachageVotesFromList70 => "list_panachage_votes_from_list_70",
                Field::ListPanachageVotesFromList77 => "list_panachage_votes_from_list_77",
                Field::ListPanachageVotesFromList999 => "list_panachage_votes_from_list_999",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11790/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratswahlen 2015: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11800/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11800/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 8. Februar 2015</p>"]
#[cfg(feature = "bl11800")]
pub mod regierungsratswahlen_2015_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11800/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2011: Kandidierendenresultate, Wahlberechtigte und Parteistimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11810/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11810/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 27. M\u{e4}rz 2011</p>"]
#[cfg(feature = "bl11810")]
pub mod landratswahlen_2011_kandidierendenresultate_wahlberechtigte_und_parteistimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub compound_id: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        CompoundId,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CompoundId => "compound_id",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11810/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatswahlen 2011: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11820/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11820/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 23. Oktober 2011<br></p>"]
#[cfg(feature = "bl11820")]
pub mod staenderatswahlen_2011_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11820/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2011: Kandidierendenresultate, Wahlberechtigte und Listenstimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11830/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11830/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 23. Oktober 2011<br></p>"]
#[cfg(feature = "bl11830")]
pub mod nationalratswahlen_2011_kandidierendenresultate_wahlberechtigte_und_listenstimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_panachage_votes_from_list_01: Option<i64>,
        pub candidate_panachage_votes_from_list_02: Option<i64>,
        pub candidate_panachage_votes_from_list_03: Option<i64>,
        pub candidate_panachage_votes_from_list_04: Option<String>,
        pub candidate_panachage_votes_from_list_05: Option<i64>,
        pub candidate_panachage_votes_from_list_06: Option<String>,
        pub candidate_panachage_votes_from_list_07: Option<String>,
        pub candidate_panachage_votes_from_list_08: Option<i64>,
        pub candidate_panachage_votes_from_list_09: Option<String>,
        pub candidate_panachage_votes_from_list_11: Option<String>,
        pub candidate_panachage_votes_from_list_22: Option<String>,
        pub candidate_panachage_votes_from_list_44: Option<String>,
        pub candidate_panachage_votes_from_list_55: Option<String>,
        pub candidate_panachage_votes_from_list_77: Option<String>,
        pub candidate_panachage_votes_from_list_999: Option<i64>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_connection: Option<i64>,
        pub list_connection_parent: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_panachage_votes_from_list_01: Option<i64>,
        pub list_panachage_votes_from_list_02: Option<i64>,
        pub list_panachage_votes_from_list_03: Option<i64>,
        pub list_panachage_votes_from_list_04: Option<i64>,
        pub list_panachage_votes_from_list_05: Option<i64>,
        pub list_panachage_votes_from_list_06: Option<i64>,
        pub list_panachage_votes_from_list_07: Option<i64>,
        pub list_panachage_votes_from_list_08: Option<String>,
        pub list_panachage_votes_from_list_09: Option<i64>,
        pub list_panachage_votes_from_list_11: Option<i64>,
        pub list_panachage_votes_from_list_22: Option<i64>,
        pub list_panachage_votes_from_list_44: Option<i64>,
        pub list_panachage_votes_from_list_55: Option<i64>,
        pub list_panachage_votes_from_list_77: Option<i64>,
        pub list_panachage_votes_from_list_999: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidatePanachageVotesFromList01,
        CandidatePanachageVotesFromList02,
        CandidatePanachageVotesFromList03,
        CandidatePanachageVotesFromList04,
        CandidatePanachageVotesFromList05,
        CandidatePanachageVotesFromList06,
        CandidatePanachageVotesFromList07,
        CandidatePanachageVotesFromList08,
        CandidatePanachageVotesFromList09,
        CandidatePanachageVotesFromList11,
        CandidatePanachageVotesFromList22,
        CandidatePanachageVotesFromList44,
        CandidatePanachageVotesFromList55,
        CandidatePanachageVotesFromList77,
        CandidatePanachageVotesFromList999,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListConnection,
        ListConnectionParent,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListPanachageVotesFromList01,
        ListPanachageVotesFromList02,
        ListPanachageVotesFromList03,
        ListPanachageVotesFromList04,
        ListPanachageVotesFromList05,
        ListPanachageVotesFromList06,
        ListPanachageVotesFromList07,
        ListPanachageVotesFromList08,
        ListPanachageVotesFromList09,
        ListPanachageVotesFromList11,
        ListPanachageVotesFromList22,
        ListPanachageVotesFromList44,
        ListPanachageVotesFromList55,
        ListPanachageVotesFromList77,
        ListPanachageVotesFromList999,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidatePanachageVotesFromList01 => {
                    "candidate_panachage_votes_from_list_01"
                }
                Field::CandidatePanachageVotesFromList02 => {
                    "candidate_panachage_votes_from_list_02"
                }
                Field::CandidatePanachageVotesFromList03 => {
                    "candidate_panachage_votes_from_list_03"
                }
                Field::CandidatePanachageVotesFromList04 => {
                    "candidate_panachage_votes_from_list_04"
                }
                Field::CandidatePanachageVotesFromList05 => {
                    "candidate_panachage_votes_from_list_05"
                }
                Field::CandidatePanachageVotesFromList06 => {
                    "candidate_panachage_votes_from_list_06"
                }
                Field::CandidatePanachageVotesFromList07 => {
                    "candidate_panachage_votes_from_list_07"
                }
                Field::CandidatePanachageVotesFromList08 => {
                    "candidate_panachage_votes_from_list_08"
                }
                Field::CandidatePanachageVotesFromList09 => {
                    "candidate_panachage_votes_from_list_09"
                }
                Field::CandidatePanachageVotesFromList11 => {
                    "candidate_panachage_votes_from_list_11"
                }
                Field::CandidatePanachageVotesFromList22 => {
                    "candidate_panachage_votes_from_list_22"
                }
                Field::CandidatePanachageVotesFromList44 => {
                    "candidate_panachage_votes_from_list_44"
                }
                Field::CandidatePanachageVotesFromList55 => {
                    "candidate_panachage_votes_from_list_55"
                }
                Field::CandidatePanachageVotesFromList77 => {
                    "candidate_panachage_votes_from_list_77"
                }
                Field::CandidatePanachageVotesFromList999 => {
                    "candidate_panachage_votes_from_list_999"
                }
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListConnection => "list_connection",
                Field::ListConnectionParent => "list_connection_parent",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListPanachageVotesFromList01 => "list_panachage_votes_from_list_01",
                Field::ListPanachageVotesFromList02 => "list_panachage_votes_from_list_02",
                Field::ListPanachageVotesFromList03 => "list_panachage_votes_from_list_03",
                Field::ListPanachageVotesFromList04 => "list_panachage_votes_from_list_04",
                Field::ListPanachageVotesFromList05 => "list_panachage_votes_from_list_05",
                Field::ListPanachageVotesFromList06 => "list_panachage_votes_from_list_06",
                Field::ListPanachageVotesFromList07 => "list_panachage_votes_from_list_07",
                Field::ListPanachageVotesFromList08 => "list_panachage_votes_from_list_08",
                Field::ListPanachageVotesFromList09 => "list_panachage_votes_from_list_09",
                Field::ListPanachageVotesFromList11 => "list_panachage_votes_from_list_11",
                Field::ListPanachageVotesFromList22 => "list_panachage_votes_from_list_22",
                Field::ListPanachageVotesFromList44 => "list_panachage_votes_from_list_44",
                Field::ListPanachageVotesFromList55 => "list_panachage_votes_from_list_55",
                Field::ListPanachageVotesFromList77 => "list_panachage_votes_from_list_77",
                Field::ListPanachageVotesFromList999 => "list_panachage_votes_from_list_999",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11830/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratswahlen 2011: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11840/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11840/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 27. M\u{e4}rz 2011</p>"]
#[cfg(feature = "bl11840")]
pub mod regierungsratswahlen_2011_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11840/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2007: Kandidierendenresultate, Wahlberechtigte und Parteistimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11850/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11850/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 11. Februar 2007</p>"]
#[cfg(feature = "bl11850")]
pub mod landratswahlen_2007_kandidierendenresultate_wahlberechtigte_und_parteistimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub compound_id: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        CompoundId,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CompoundId => "compound_id",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11850/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatswahlen 2007: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11860/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11860/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 21. Oktober 2007<br></p>"]
#[cfg(feature = "bl11860")]
pub mod staenderatswahlen_2007_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11860/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2007: Kandidierendenresultate, Wahlberechtigte und Listenstimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11870/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11870/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 21. Oktober 2007<br></p>"]
#[cfg(feature = "bl11870")]
pub mod nationalratswahlen_2007_kandidierendenresultate_wahlberechtigte_und_listenstimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_panachage_votes_from_list_01: Option<i64>,
        pub candidate_panachage_votes_from_list_02: Option<i64>,
        pub candidate_panachage_votes_from_list_03: Option<i64>,
        pub candidate_panachage_votes_from_list_04: Option<i64>,
        pub candidate_panachage_votes_from_list_05: Option<i64>,
        pub candidate_panachage_votes_from_list_06: Option<String>,
        pub candidate_panachage_votes_from_list_07: Option<i64>,
        pub candidate_panachage_votes_from_list_08: Option<String>,
        pub candidate_panachage_votes_from_list_09: Option<String>,
        pub candidate_panachage_votes_from_list_12: Option<String>,
        pub candidate_panachage_votes_from_list_33: Option<String>,
        pub candidate_panachage_votes_from_list_55: Option<i64>,
        pub candidate_panachage_votes_from_list_77: Option<String>,
        pub candidate_panachage_votes_from_list_999: Option<i64>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_connection: Option<i64>,
        pub list_connection_parent: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_panachage_votes_from_list_01: Option<i64>,
        pub list_panachage_votes_from_list_02: Option<i64>,
        pub list_panachage_votes_from_list_03: Option<i64>,
        pub list_panachage_votes_from_list_04: Option<i64>,
        pub list_panachage_votes_from_list_05: Option<String>,
        pub list_panachage_votes_from_list_06: Option<i64>,
        pub list_panachage_votes_from_list_07: Option<i64>,
        pub list_panachage_votes_from_list_08: Option<i64>,
        pub list_panachage_votes_from_list_09: Option<i64>,
        pub list_panachage_votes_from_list_12: Option<i64>,
        pub list_panachage_votes_from_list_33: Option<i64>,
        pub list_panachage_votes_from_list_55: Option<i64>,
        pub list_panachage_votes_from_list_77: Option<i64>,
        pub list_panachage_votes_from_list_999: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidatePanachageVotesFromList01,
        CandidatePanachageVotesFromList02,
        CandidatePanachageVotesFromList03,
        CandidatePanachageVotesFromList04,
        CandidatePanachageVotesFromList05,
        CandidatePanachageVotesFromList06,
        CandidatePanachageVotesFromList07,
        CandidatePanachageVotesFromList08,
        CandidatePanachageVotesFromList09,
        CandidatePanachageVotesFromList12,
        CandidatePanachageVotesFromList33,
        CandidatePanachageVotesFromList55,
        CandidatePanachageVotesFromList77,
        CandidatePanachageVotesFromList999,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListConnection,
        ListConnectionParent,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListPanachageVotesFromList01,
        ListPanachageVotesFromList02,
        ListPanachageVotesFromList03,
        ListPanachageVotesFromList04,
        ListPanachageVotesFromList05,
        ListPanachageVotesFromList06,
        ListPanachageVotesFromList07,
        ListPanachageVotesFromList08,
        ListPanachageVotesFromList09,
        ListPanachageVotesFromList12,
        ListPanachageVotesFromList33,
        ListPanachageVotesFromList55,
        ListPanachageVotesFromList77,
        ListPanachageVotesFromList999,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidatePanachageVotesFromList01 => {
                    "candidate_panachage_votes_from_list_01"
                }
                Field::CandidatePanachageVotesFromList02 => {
                    "candidate_panachage_votes_from_list_02"
                }
                Field::CandidatePanachageVotesFromList03 => {
                    "candidate_panachage_votes_from_list_03"
                }
                Field::CandidatePanachageVotesFromList04 => {
                    "candidate_panachage_votes_from_list_04"
                }
                Field::CandidatePanachageVotesFromList05 => {
                    "candidate_panachage_votes_from_list_05"
                }
                Field::CandidatePanachageVotesFromList06 => {
                    "candidate_panachage_votes_from_list_06"
                }
                Field::CandidatePanachageVotesFromList07 => {
                    "candidate_panachage_votes_from_list_07"
                }
                Field::CandidatePanachageVotesFromList08 => {
                    "candidate_panachage_votes_from_list_08"
                }
                Field::CandidatePanachageVotesFromList09 => {
                    "candidate_panachage_votes_from_list_09"
                }
                Field::CandidatePanachageVotesFromList12 => {
                    "candidate_panachage_votes_from_list_12"
                }
                Field::CandidatePanachageVotesFromList33 => {
                    "candidate_panachage_votes_from_list_33"
                }
                Field::CandidatePanachageVotesFromList55 => {
                    "candidate_panachage_votes_from_list_55"
                }
                Field::CandidatePanachageVotesFromList77 => {
                    "candidate_panachage_votes_from_list_77"
                }
                Field::CandidatePanachageVotesFromList999 => {
                    "candidate_panachage_votes_from_list_999"
                }
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListConnection => "list_connection",
                Field::ListConnectionParent => "list_connection_parent",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListPanachageVotesFromList01 => "list_panachage_votes_from_list_01",
                Field::ListPanachageVotesFromList02 => "list_panachage_votes_from_list_02",
                Field::ListPanachageVotesFromList03 => "list_panachage_votes_from_list_03",
                Field::ListPanachageVotesFromList04 => "list_panachage_votes_from_list_04",
                Field::ListPanachageVotesFromList05 => "list_panachage_votes_from_list_05",
                Field::ListPanachageVotesFromList06 => "list_panachage_votes_from_list_06",
                Field::ListPanachageVotesFromList07 => "list_panachage_votes_from_list_07",
                Field::ListPanachageVotesFromList08 => "list_panachage_votes_from_list_08",
                Field::ListPanachageVotesFromList09 => "list_panachage_votes_from_list_09",
                Field::ListPanachageVotesFromList12 => "list_panachage_votes_from_list_12",
                Field::ListPanachageVotesFromList33 => "list_panachage_votes_from_list_33",
                Field::ListPanachageVotesFromList55 => "list_panachage_votes_from_list_55",
                Field::ListPanachageVotesFromList77 => "list_panachage_votes_from_list_77",
                Field::ListPanachageVotesFromList999 => "list_panachage_votes_from_list_999",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11870/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratswahlen 2007: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11880/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11880/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 11. Februar 2007</p>"]
#[cfg(feature = "bl11880")]
pub mod regierungsratswahlen_2007_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11880/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2003: Kandidierendenresultate, Wahlberechtigte und Parteistimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11890/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11890/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 30. M\u{e4}rz 2003</p>"]
#[cfg(feature = "bl11890")]
pub mod landratswahlen_2003_kandidierendenresultate_wahlberechtigte_und_parteistimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub compound_id: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        CompoundId,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::CompoundId => "compound_id",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11890/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatswahlen 2003: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11900/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11900/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 19. Oktober 2003<br></p>"]
#[cfg(feature = "bl11900")]
pub mod staenderatswahlen_2003_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11900/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2003: Kandidierendenresultate, Wahlberechtigte und Listenstimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11910/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11910/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 19. Oktober 2003</p>"]
#[cfg(feature = "bl11910")]
pub mod nationalratswahlen_2003_kandidierendenresultate_wahlberechtigte_und_listenstimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_panachage_votes_from_list_01: Option<i64>,
        pub candidate_panachage_votes_from_list_02: Option<i64>,
        pub candidate_panachage_votes_from_list_03: Option<i64>,
        pub candidate_panachage_votes_from_list_04: Option<i64>,
        pub candidate_panachage_votes_from_list_05: Option<i64>,
        pub candidate_panachage_votes_from_list_06: Option<String>,
        pub candidate_panachage_votes_from_list_07: Option<i64>,
        pub candidate_panachage_votes_from_list_09: Option<String>,
        pub candidate_panachage_votes_from_list_10: Option<String>,
        pub candidate_panachage_votes_from_list_13: Option<String>,
        pub candidate_panachage_votes_from_list_44: Option<String>,
        pub candidate_panachage_votes_from_list_77: Option<String>,
        pub candidate_panachage_votes_from_list_999: Option<i64>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<String>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_connection: Option<i64>,
        pub list_connection_parent: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_panachage_votes_from_list_01: Option<i64>,
        pub list_panachage_votes_from_list_02: Option<i64>,
        pub list_panachage_votes_from_list_03: Option<i64>,
        pub list_panachage_votes_from_list_04: Option<i64>,
        pub list_panachage_votes_from_list_05: Option<String>,
        pub list_panachage_votes_from_list_06: Option<i64>,
        pub list_panachage_votes_from_list_07: Option<i64>,
        pub list_panachage_votes_from_list_09: Option<i64>,
        pub list_panachage_votes_from_list_10: Option<i64>,
        pub list_panachage_votes_from_list_13: Option<i64>,
        pub list_panachage_votes_from_list_44: Option<i64>,
        pub list_panachage_votes_from_list_77: Option<i64>,
        pub list_panachage_votes_from_list_999: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidatePanachageVotesFromList01,
        CandidatePanachageVotesFromList02,
        CandidatePanachageVotesFromList03,
        CandidatePanachageVotesFromList04,
        CandidatePanachageVotesFromList05,
        CandidatePanachageVotesFromList06,
        CandidatePanachageVotesFromList07,
        CandidatePanachageVotesFromList09,
        CandidatePanachageVotesFromList10,
        CandidatePanachageVotesFromList13,
        CandidatePanachageVotesFromList44,
        CandidatePanachageVotesFromList77,
        CandidatePanachageVotesFromList999,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListConnection,
        ListConnectionParent,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListPanachageVotesFromList01,
        ListPanachageVotesFromList02,
        ListPanachageVotesFromList03,
        ListPanachageVotesFromList04,
        ListPanachageVotesFromList05,
        ListPanachageVotesFromList06,
        ListPanachageVotesFromList07,
        ListPanachageVotesFromList09,
        ListPanachageVotesFromList10,
        ListPanachageVotesFromList13,
        ListPanachageVotesFromList44,
        ListPanachageVotesFromList77,
        ListPanachageVotesFromList999,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidatePanachageVotesFromList01 => {
                    "candidate_panachage_votes_from_list_01"
                }
                Field::CandidatePanachageVotesFromList02 => {
                    "candidate_panachage_votes_from_list_02"
                }
                Field::CandidatePanachageVotesFromList03 => {
                    "candidate_panachage_votes_from_list_03"
                }
                Field::CandidatePanachageVotesFromList04 => {
                    "candidate_panachage_votes_from_list_04"
                }
                Field::CandidatePanachageVotesFromList05 => {
                    "candidate_panachage_votes_from_list_05"
                }
                Field::CandidatePanachageVotesFromList06 => {
                    "candidate_panachage_votes_from_list_06"
                }
                Field::CandidatePanachageVotesFromList07 => {
                    "candidate_panachage_votes_from_list_07"
                }
                Field::CandidatePanachageVotesFromList09 => {
                    "candidate_panachage_votes_from_list_09"
                }
                Field::CandidatePanachageVotesFromList10 => {
                    "candidate_panachage_votes_from_list_10"
                }
                Field::CandidatePanachageVotesFromList13 => {
                    "candidate_panachage_votes_from_list_13"
                }
                Field::CandidatePanachageVotesFromList44 => {
                    "candidate_panachage_votes_from_list_44"
                }
                Field::CandidatePanachageVotesFromList77 => {
                    "candidate_panachage_votes_from_list_77"
                }
                Field::CandidatePanachageVotesFromList999 => {
                    "candidate_panachage_votes_from_list_999"
                }
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListConnection => "list_connection",
                Field::ListConnectionParent => "list_connection_parent",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListPanachageVotesFromList01 => "list_panachage_votes_from_list_01",
                Field::ListPanachageVotesFromList02 => "list_panachage_votes_from_list_02",
                Field::ListPanachageVotesFromList03 => "list_panachage_votes_from_list_03",
                Field::ListPanachageVotesFromList04 => "list_panachage_votes_from_list_04",
                Field::ListPanachageVotesFromList05 => "list_panachage_votes_from_list_05",
                Field::ListPanachageVotesFromList06 => "list_panachage_votes_from_list_06",
                Field::ListPanachageVotesFromList07 => "list_panachage_votes_from_list_07",
                Field::ListPanachageVotesFromList09 => "list_panachage_votes_from_list_09",
                Field::ListPanachageVotesFromList10 => "list_panachage_votes_from_list_10",
                Field::ListPanachageVotesFromList13 => "list_panachage_votes_from_list_13",
                Field::ListPanachageVotesFromList44 => "list_panachage_votes_from_list_44",
                Field::ListPanachageVotesFromList77 => "list_panachage_votes_from_list_77",
                Field::ListPanachageVotesFromList999 => "list_panachage_votes_from_list_999",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11910/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratswahlen 2003: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11920/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11920/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 30. M\u{e4}rz 2003</p>"]
#[cfg(feature = "bl11920")]
pub mod regierungsratswahlen_2003_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11920/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Regierungsratsersatzwahl 2013: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11930/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11930/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 9. Juni 2013</p>"]
#[cfg(feature = "bl11930")]
pub mod regierungsratsersatzwahl_2013_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        pub election_date: Option<String>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11930/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wohngeb\u{e4}ude nach Energietr\u{e4}ger der Heizung, Bauperiode, Gemeinde und Jahr (2022)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11940/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11940/</a>\n"]
#[doc = "<p>Energiestatistik</p><p>Die Codes im Datensatz entsprechen den Codelisten gem\u{e4}ss GWR-Merkmalskatalog 4.2</p><p>Bauperiode: <a href=\"https://www.housing-stat.ch/de/help/42.html#GBAUP\" target=\"_blank\">https://www.housing-stat.ch/de/help/42.html#GBAUP</a></p><p>Energie-/W\u{e4}rmequelle Heizung: <a href=\"https://www.housing-stat.ch/de/help/42.html#GENH\" target=\"_blank\">https://www.housing-stat.ch/de/help/42.html#GENH</a></p>"]
#[cfg(feature = "bl11940")]
pub mod wohngebaeude_nach_energietraeger_der_heizung_bauperiode_gemeinde_und_jahr_2022 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Gemeindenummer Bundesamt für Statistik
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Code Energieträger gemäss GWR-Katalog
        pub energietraeger_code: Option<String>,
        /// Code Bauperiode gemäss GWR-Katalog
        pub bauperiode_code: Option<String>,
        /// Indikator
        pub indikator: Option<String>,
        /// Absoluter Wert
        pub wert: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Gemeinde,
        EnergietraegerCode,
        BauperiodeCode,
        Indikator,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::EnergietraegerCode => "energietraeger_code",
                Field::BauperiodeCode => "bauperiode_code",
                Field::Indikator => "indikator",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11940/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19 (Breites Testen BL): W\u{f6}chentlich getestete bzw. positive Personen in Betrieben nach Kategorie (M\u{e4}rz 2021 - Dezember 2022)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11950/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11950/</a>\n"]
#[doc = "<p><font face=\"Arial, sans-serif\"><span style=\"font-size: 14.6667px;\">Anzahl getesteter (Pool-Test) bzw. positiv getesteter Personen (Nachtestung resp. R\u{fc}ckstellprobe nach positiven Pool-Tests) pro Kalenderwoche nach Art der teilnehmenden Organisation. Das Programm lief zum 31. Dezember 2022 aus.</span></font></p><p><font face=\"Arial, sans-serif\"><span style=\"font-size: 14.6667px;\">Das breite Testen Baselland war ein repetitives Testprogramm zur Identifizierung von asymptomatischen SARS-CoV-2 infizierten Personen mittels gepoolter PCR-Tests aus Speichelproben. Das Programm lief von 1. M\u{e4}rz 2021 bis 31. Dezember 2022. Grunds\u{e4}tzlich konnten Betriebe, Schulen, Alters- und Pflegeheime (APH) und Spit\u{e4}ler aus dem Kanton Basel-Landschaft daran teilnehmen.</span><br></font></p>"]
#[cfg(feature = "bl11950")]
pub mod covid_19_breites_testen_bl_woechentlich_getestete_bzw_positive_personen_in_betrieben_nach_kategorie_maerz_2021_dezember_2022 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        ///
        /// Erster Tag der Kalenderwoche
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
        /// Kalenderwoche
        ///
        /// Jahr_Kalenderwoche
        pub kalenderwoche: Option<String>,
        /// Kategorie
        ///
        /// Art der teilnehmenden Organisation
        pub kategorie: Option<String>,
        /// Anzahl_getestete_Personen
        pub anzahl_getestete_personen: Option<f64>,
        /// Anzahl_positive_Personen
        pub anzahl_positive_personen: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Datum,
        Kalenderwoche,
        Kategorie,
        AnzahlGetestetePersonen,
        AnzahlPositivePersonen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Datum => "datum",
                Field::Kalenderwoche => "kalenderwoche",
                Field::Kategorie => "kategorie",
                Field::AnzahlGetestetePersonen => "anzahl_getestete_personen",
                Field::AnzahlPositivePersonen => "anzahl_positive_personen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11950/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Covid-19 (Breites Testen BL): W\u{f6}chentliche Anzahl Pools bzw. positive Pools (M\u{e4}rz 2021 - Dezember 2022)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11960/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11960/</a>\n"]
#[doc = "<p>Anzahl\u{a0}getesteter Pools und Anzahl bzw. Anteil positiver Pools pro Kalenderwoche. Das Programm lief zum 31. Dezember 2022 aus.</p><p>Das breite Testen Baselland war ein repetitives Testprogramm zur Identifizierung von asymptomatischen SARS-CoV-2 infizierten Personen mittels gepoolter PCR-Tests aus Speichelproben. Das Programm lief von 1. M\u{e4}rz 2021 bis 31. Dezember 2022. Grunds\u{e4}tzlich konnten Betriebe, Schulen, Alters- und Pflegeheime (APH) und Spit\u{e4}ler aus dem Kanton Basel-Landschaft daran teilnehmen.<br></p>"]
#[cfg(feature = "bl11960")]
pub mod covid_19_breites_testen_bl_woechentliche_anzahl_pools_bzw_positive_pools_maerz_2021_dezember_2022 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        ///
        /// Erster Tag der Kalenderwoche
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
        /// Kalenderwoche
        ///
        /// Jahr_Kalenderwoche
        pub kalenderwoche: Option<String>,
        /// Anzahl_Pools
        ///
        /// Zahl der analysierten Pools
        pub anzahl_pools: Option<f64>,
        /// Anzahl_positive_Pools
        ///
        /// Zahl der positiven Pools
        pub anzahl_positive_pools: Option<f64>,
        /// Anteil_positive_Pools
        ///
        /// Prozent positive Pools
        pub anteil_positive_pools: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Datum,
        Kalenderwoche,
        AnzahlPools,
        AnzahlPositivePools,
        AnteilPositivePools,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Datum => "datum",
                Field::Kalenderwoche => "kalenderwoche",
                Field::AnzahlPools => "anzahl_pools",
                Field::AnzahlPositivePools => "anzahl_positive_pools",
                Field::AnteilPositivePools => "anteil_positive_pools",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11960/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Arealstatistik: Bodennutzung und -bedeckung nach Hauptbereich, Klasse und Gemeinde (seit 1982)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11970/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11970/</a>\n"]
#[doc = "<p>Bundesamt f\u{fc}r Statistik, Arealstatistiken 1979/85 - 2013/18; swisstopo,\u{a0}swissBOUNDARIES3D</p>"]
#[cfg(feature = "bl11970")]
pub mod arealstatistik_bodennutzung_und_bedeckung_nach_hauptbereich_klasse_und_gemeinde_seit_1982 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Erhebungsperiode
        ///
        /// Periode der Erhebung im Kanton BL
        pub erhebungsperiode: Option<String>,
        /// Erhebungsjahr/e
        ///
        /// Jahr(e) der Erhebung (Luftbilder) im Kanton BL
        pub erhebungsjahr_e: Option<String>,
        /// BFS_Nummer
        ///
        /// Gemeindenummer gemäss Bundesamt für Statistik
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        ///
        /// Gemeindename
        pub gemeinde: Option<String>,
        /// Hauptbereich
        ///
        /// 4 Hauptbereiche und 2 Totale. Punktfläche = Summer der 4 Hauptbereiche
        pub hauptbereich: Option<String>,
        /// Klasse
        ///
        /// 17 Klassen
        pub klasse: Option<String>,
        /// Hektaren
        ///
        /// Wert in ha
        pub wert: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Erhebungsperiode,
        ErhebungsjahrE,
        BfsNummer,
        Gemeinde,
        Hauptbereich,
        Klasse,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Erhebungsperiode => "erhebungsperiode",
                Field::ErhebungsjahrE => "erhebungsjahr_e",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Hauptbereich => "hauptbereich",
                Field::Klasse => "klasse",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11970/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Abstimmungsarchiv nach Vorlage, Gemeinde und Datum (seit 2003)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/11990/\" target=\"_blank\">https://data.bl.ch/explore/dataset/11990/</a>\n"]
#[doc = "<p class=\"\">Gemeinderesultate aller eidgen\u{f6}ssischen und kantonalen Vorlagen</p><p style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">Hinweis</span>: Der Datensatz wird an Abstimmungssonntagen viertelst\u{fc}ndlich aktualisiert. Solange die Gemeinden nicht fertig ausgez\u{e4}hlt sind, zeigen die Eintr\u{e4}ge lediglich den aktuellen Stand der Ausz\u{e4}hlung und nicht das definitive Ergebnis der Abstimmung. Die Spalte <i>counted</i>\u{a0}gibt an, ob die Gemeinde ausgez\u{e4}hlt ist. Aggregierte Angaben auf Kantonsebene befinden sich im Datensatz\u{a0}<a href=\"https://data.bl.ch/explore/dataset/10500/table/?disjunctive.type&amp;disjunctive.answer&amp;sort=date\" target=\"_blank\">Abstimmungsarchiv nach Vorlage und Datum (seit 2003)</a>.</p><p style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">Spaltenbeschriebe</span></p><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">date</span>: Das Datum, an dem die Abstimmung stattgefunden hat.</li><li style=\"font-family: Roboto, sans-serif;\"><strong>entity_id</strong>: Eine eindeutige Kennung f\u{fc}r die Einheit (BFS Code).</li><li style=\"font-family: Roboto, sans-serif;\"><strong>name</strong>: Der Name der\u{a0} Gemeine.</li><li style=\"font-family: Roboto, sans-serif;\"><strong>district</strong>: Der Bezirk, dem die Gemeinde angeh\u{f6}rt.<br></li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">vote_id</span>: Eine eindeutige Identifikationsnummer f\u{fc}r jede Abstimmungsvorlage bestehend aus dem Datum des Abstimmungstags und einem pro Abstimmungstag eindeutigen K\u{fc}rzel.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">domain</span>: Der Geltungsbereich der Abstimmung, z. B. \"federation\" f\u{fc}r nationale Abstimmungen oder \"canton\" f\u{fc}r kantonale Abstimmungen.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">type</span>: Die Art der Vorlage: \"proposal\" f\u{fc}r einen Vorschlag, \"counter-proposal\" f\u{fc}r einen Gegenvorschlag oder \"tie-breaker\" f\u{fc}r eine Stichfrage.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">title_de_CH</span>: Der Titel der Vorlage in deutscher Sprache.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">counted</span>: Hat den Wert \"True\" wenn gemeinde Ausgez\u{e4}hlt ist.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">answer</span>: Das Ergebnis der Abstimmung: \"accepted\" = Vorschlag oder Gegenvorschlag angenommen, \"rejected\" = Vorschlag oder Gegenvorschlag abgelehnt, \"proposal\" = Stichentscheid f\u{fc}r Vorschlag, \"counter-proposal\" = Stichentscheid f\u{fc}r Gegenvorschlag.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">percent_yeas</span>: Der Prozentsatz der abgegebenen Stimmen, die f\u{fc}r die Vorlage gestimmt haben.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">percent_nays</span>: Der Prozentsatz der abgegebenen Stimmen, die gegen die Vorlage gestimmt haben.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">percent_turnout</span>: Die Wahlbeteiligung, gemessen als Prozentsatz der Stimmberechtigten, die an der Abstimmung teilgenommen haben.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">eligible_voters</span>: Die Anzahl der Personen, die wahlberechtigt waren.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">expats</span>: Die Anzahl der stimmberechtigten Schweizer B\u{fc}rger, die im Ausland leben und an der Abstimmung teilgenommen haben.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">empty</span>: Die Anzahl der abgegebenen, aber leeren Stimmzettel.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">invalid</span>: Die Anzahl der abgegebenen, aber ung\u{fc}ltigen Stimmzettel.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">yeas</span>: Die absolute Anzahl der Ja-Stimmen.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">nays</span>: Die absolute Anzahl der Nein-Stimmen.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">url</span>: Der Weblink zu den detaillierten Abstimmungsergebnissen auf wahlen.bl.ch.</li><p></p>"]
#[cfg(feature = "bl11990")]
pub mod abstimmungsarchiv_nach_vorlage_gemeinde_und_datum_seit_2003 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Abstimmungsdatum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// Gemeindenummer (BFS)
        pub entity_id: Option<String>,
        /// Gemeinde
        pub name: Option<String>,
        /// Bezirk
        pub district: Option<String>,
        pub vote_id: Option<String>,
        /// domain
        ///
        /// Föderale Ebene
        pub domain0: Option<String>,
        /// type
        ///
        /// Typ
        pub r#type: Option<String>,
        /// title_de_CH
        ///
        /// Abstimmungstitel
        pub title_de_ch: Option<String>,
        /// Ausgezählt
        pub counted: Option<String>,
        /// Resultat
        pub answer: Option<String>,
        /// Ja-Stimmen (%)
        pub percent_yeas: Option<f64>,
        /// Nein-Stimmen (%)
        pub percent_nays: Option<f64>,
        /// Wahlbeteiligung
        pub percent_turnout: Option<f64>,
        /// Stimmberechtigte total
        pub eligible_voters: Option<i64>,
        /// davon Stimmberechtigte Auslandschweizer/innen
        pub expats: Option<i64>,
        /// Leere Stimmen
        pub empty: Option<i64>,
        /// Ungültige Stimmen
        pub invalid: Option<i64>,
        /// Ja-Stimmen
        pub yeas: Option<i64>,
        /// Nein-Stimmen
        pub nays: Option<i64>,
        pub link_to_canton_results: Option<String>,
        pub url_web: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        District,
        VoteId,
        Domain0,
        RType,
        TitleDeCh,
        Counted,
        Answer,
        PercentYeas,
        PercentNays,
        PercentTurnout,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        LinkToCantonResults,
        UrlWeb,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::District => "district",
                Field::VoteId => "vote_id",
                Field::Domain0 => "domain0",
                Field::RType => "type",
                Field::TitleDeCh => "title_de_ch",
                Field::Counted => "counted",
                Field::Answer => "answer",
                Field::PercentYeas => "percent_yeas",
                Field::PercentNays => "percent_nays",
                Field::PercentTurnout => "percent_turnout",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::LinkToCantonResults => "link_to_canton_results",
                Field::UrlWeb => "url_web",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/11990/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Landratswahlen 2023: Unver\u{e4}nderte und ver\u{e4}nderte Wahlzettel nach Partei und Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12000/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12000/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 12. Februar 2023<br></p>"]
#[cfg(feature = "bl12000")]
pub mod landratswahlen_2023_unveraenderte_und_veraenderte_wahlzettel_nach_partei_und_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezirk-Nummer
        pub entity_district_id: Option<i64>,
        /// Wahlbezirk-Name
        pub entity_district_name: Option<String>,
        /// Gemeindenummer gemäss Bundesamt für Statistik
        pub entity_id: Option<i64>,
        /// Gemeindename
        pub entity_name: Option<String>,
        /// Wahlberechtigte in der Gemeinde
        pub entity_eligible_voters: Option<i64>,
        /// Listennummer
        pub list_id: Option<String>,
        /// Partei
        pub party: Option<String>,
        /// Gültige Wahlzettel
        pub accounted_ballots: Option<i64>,
        /// Unveränderte Wahlzettel
        pub unmodified_ballots: Option<i64>,
        /// Veränderte Wahlzettel
        pub modified_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        EntityDistrictId,
        EntityDistrictName,
        EntityId,
        EntityName,
        EntityEligibleVoters,
        ListId,
        Party,
        AccountedBallots,
        UnmodifiedBallots,
        ModifiedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::EntityDistrictId => "entity_district_id",
                Field::EntityDistrictName => "entity_district_name",
                Field::EntityId => "entity_id",
                Field::EntityName => "entity_name",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::ListId => "list_id",
                Field::Party => "party",
                Field::AccountedBallots => "accounted_ballots",
                Field::UnmodifiedBallots => "unmodified_ballots",
                Field::ModifiedBallots => "modified_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12000/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# CO2-Emissionen nach Energietr\u{e4}ger, Gemeinde und Jahr (seit 2018)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12020/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12020/</a>\n"]
#[doc = "<p>Energiestatistik</p><p>CO2-Emissionen in Tonnen und pro Einwohner/in, die durch Energiegewinnung und -verbrauch innerhalb des Kantons Basel-Landschaft entstehen. CO2-Emissionen aus Treibstoffen (Benzin, Diesel) sind darin nicht ber\u{fc}cksichtigt, da diese nur auf Ebene des gesamten Kantons bekannt sind. Der Datensatz ist Teil der kantonalen Energiestatistik, die alle zwei Jahre durchgef\u{fc}hrt wird.</p><p>Die CO2-Emissionen pro Einwohner/in werden aufgrund der mittleren Wohnbev\u{f6}lkerung berechnet. Die Umrechnung von Energieverbrauch zu CO2-Emissionen wird anhand der Emissionsfaktoren gem\u{e4}ss dem nationalen Treibhausgasinventar gemacht. Erneuerbare Energietr\u{e4}ger gelten als klimaneutral, da \u{fc}ber den gesamten Zyklus die gleiche Menge CO2 gebunden wie emittiert wird. Deshalb werden sie hier nicht aufgef\u{fc}hrt.</p>"]
#[cfg(feature = "bl12020")]
pub mod co2_emissionen_nach_energietraeger_gemeinde_und_jahr_seit_2018 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub erhebungsjahr: Option<String>,
        pub bfs_gemeindenummer: Option<String>,
        pub gemeinde: Option<String>,
        pub energietraeger_bezeichnung: Option<String>,
        pub co2_absolut_tonnen: Option<f64>,
        pub co2_kg_pro_person: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Erhebungsjahr,
        BfsGemeindenummer,
        Gemeinde,
        EnergietraegerBezeichnung,
        Co2AbsolutTonnen,
        Co2KgProPerson,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Erhebungsjahr => "erhebungsjahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::EnergietraegerBezeichnung => "energietraeger_bezeichnung",
                Field::Co2AbsolutTonnen => "co2_absolut_tonnen",
                Field::Co2KgProPerson => "co2_kg_pro_person",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12020/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wetterstation Basel / Binningen: Tageswerte Klimamessnetz (seit 1864)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12030/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12030/</a>\n"]
#[doc = "<p>Tagesdaten der NBCN-Station (Swiss National Basic Climate Network) Basel-Binningen</p><p>Methodischer Hinweis:<br>Die Berechnung des Tagesmittels erfolgte je nach historischer Zeitperiode unterschiedlich.<br><br>bis 1970:<br>Tagesmittel - berechnet wird das 3-er Mittel aus 07, 13 und 21 Uhr<br>Min - ist der tiefere Wert der beiden Messungen um 7.30 und 21.30 Uhr<br>Max - ist der h\u{f6}here Wert der beiden Messungen\u{a0} um 7.30 und 21.30 Uhr</p><p>1971 bis 1980:<br>Tagesmittel - T<sub>m</sub>= n - k (n-Min) f\u{fc}r n = 3-er Mittel aus\u{a0}07, 13 und 21 Uhr<br>Min -\u{a0}ist der tiefere Wert der beiden Messungen um 7.30 und 19.30 Uhr<br>Max -\u{a0}ist der h\u{f6}here Wert der beiden Messungen um 19.30 und 21.30 Uhr</p><p>ab 1981:<br>Tagesmittel - Tagesmittel berechnet sich aus dem 24 Stundenmittel von 2.40 bis 23.40 UTC<br>Min -\u{a0}ist der tiefste Tageswert<br>Max -\u{a0}ist der h\u{f6}chste Tageswert</p><p>ab 2018: Die Tagesmittel werden aus den von der MeteoSchweiz gemessenen 10min-Werten von 00:10 bis 00:00 aggregiert (z.B. 26.1.2023 00:10 bis 27.1.2023: 00:00 f\u{fc}r den 26.1.2023).</p>"]
#[cfg(feature = "bl12030")]
pub mod wetterstation_basel_binningen_tageswerte_klimamessnetz_seit_1864 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// Jahr
        pub jahr: Option<String>,
        /// Globalstrahlung in W/m2
        ///
        /// Tagesmittel
        pub gre000d0: Option<f64>,
        /// Gesamtschneemenge
        ///
        /// Morgenmessung von 6 UTC
        pub hto000d0: Option<f64>,
        /// Gesamtbewölkung
        ///
        /// Tagesmittel
        pub nto000d0: Option<f64>,
        /// Luftdruck in hPa
        ///
        /// Tagesmittel auf Stationshöhe (QFE)
        pub prestad0: Option<f64>,
        /// Niederschlag
        ///
        /// Tagessumme 6 UTC - 6 UTC Folgetag
        pub rre150d0: Option<f64>,
        /// Sonnenscheindauer
        ///
        /// Tagessumme
        pub sre000d0: Option<f64>,
        /// Tagesmittel Lufttemperatur
        ///
        /// 2 m über Boden
        ///Die Berechnung erfolgte je nach historischer Zeitperiode unterschiedlich - weitere Hinweise sind im Informationstext zu finden.
        pub tre200d0: Option<f64>,
        /// Tagesminimum Lufttemperatur
        ///
        /// 2 m über Boden
        ///Die Berechnung erfolgte je nach historischer Zeitperiode unterschiedlich - weitere Hinweise sind im Informationstext zu finden.
        pub tre200dn: Option<f64>,
        /// Tagesmaximum Lufttemperatur
        ///
        /// 2 m über Boden
        ///Die Berechnung erfolgte je nach historischer Zeitperiode unterschiedlich - weitere Hinweise sind im Informationstext zu finden.
        pub tre200dx: Option<f64>,
        /// Relative Luftfeuchtigkeit
        ///
        /// Tagesmittel, 2 m über Boden
        pub ure200d0: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        Jahr,
        Gre000d0,
        Hto000d0,
        Nto000d0,
        Prestad0,
        Rre150d0,
        Sre000d0,
        Tre200d0,
        Tre200dn,
        Tre200dx,
        Ure200d0,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::Jahr => "jahr",
                Field::Gre000d0 => "gre000d0",
                Field::Hto000d0 => "hto000d0",
                Field::Nto000d0 => "nto000d0",
                Field::Prestad0 => "prestad0",
                Field::Rre150d0 => "rre150d0",
                Field::Sre000d0 => "sre000d0",
                Field::Tre200d0 => "tre200d0",
                Field::Tre200dn => "tre200dn",
                Field::Tre200dx => "tre200dx",
                Field::Ure200d0 => "ure200d0",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12030/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Abfallmengen nach Kategorie, Gemeinde und Jahr (seit 2017)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12060/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12060/</a>\n"]
#[doc = "<p>Abfallstatistik</p><p>Nur von den Gemeinden gesammelte Abf\u{e4}lle exkl. regionale Entsorgungszentren.\u{a0}Nur ein Teil der separat gesammelten Kunststoffe wird wiederverwertet. Ein grosser Teil der gesammelten Kunststoffe wird nach wie vor verbrannt.</p><p><br></p>"]
#[cfg(feature = "bl12060")]
pub mod abfallmengen_nach_kategorie_gemeinde_und_jahr_seit_2017 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Kategorie
        pub kategorie: Option<String>,
        /// Einheit
        pub einheit: Option<String>,
        /// Wert
        pub wert: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Gemeinde,
        Kategorie,
        Einheit,
        Wert,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Kategorie => "kategorie",
                Field::Einheit => "einheit",
                Field::Wert => "wert",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12060/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Durchschnittlicher Quadratmeterpreis von Bauland nach Gemeinde und Jahr (seit 1979)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12070/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12070/</a>\n"]
#[doc = "<p style=\"font-family: sans-serif;\">Bodenpreisstatistik. (Klammern = Datenschutz bei weniger als 3 Transaktionen; leer =\u{a0} im entsprechenden Jahr wurden keine Transaktionen vorgenommen<font face=\"inherit\">)</font></p><p style=\"font-family: sans-serif;\">Vor 1994 ohne Daten f\u{fc}r den Bezirk Laufen</p>"]
#[cfg(feature = "bl12070")]
pub mod durchschnittlicher_quadratmeterpreis_von_bauland_nach_gemeinde_und_jahr_seit_1979 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Nummer
        pub bfs_nummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Fälle
        pub falle: Option<String>,
        /// Fläche_in_m2
        pub flache_in_m2: Option<String>,
        /// Quadratmeterpreis_CHF
        pub quadratmeterpreis_chf: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsNummer,
        Gemeinde,
        Falle,
        FlacheInM2,
        QuadratmeterpreisChf,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsNummer => "bfs_nummer",
                Field::Gemeinde => "gemeinde",
                Field::Falle => "falle",
                Field::FlacheInM2 => "flache_in_m2",
                Field::QuadratmeterpreisChf => "quadratmeterpreis_chf",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12070/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bundesbeschluss vom 16. Dezember 2022 \u{fc}ber eine besondere Besteuerung grosser Unternehmensgruppen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12080/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12080/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 18. Juni 2023<br></p>"]
#[cfg(feature = "bl12080")]
pub mod bundesbeschluss_vom_16_dezember_2022_ueber_eine_besondere_besteuerung_grosser_unternehmensgruppen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12080/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bundesgesetz vom 30. September 2022 \u{fc}ber die Ziele im Klimaschutz, die Innovation und die St\u{e4}rkung der Energiesicherheit (KIG)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12090/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12090/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 18. Juni 2023<br></p>"]
#[cfg(feature = "bl12090")]
pub mod bundesgesetz_vom_30_september_2022_ueber_die_ziele_im_klimaschutz_die_innovation_und_die_staerkung_der_energiesicherheit_kig {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12090/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung vom 16. Dezember 2022 des Bundesgesetzes \u{fc}ber die gesetzlichen Grundlagen f\u{fc}r Verordnungen des Bundesrates zur Bew\u{e4}ltigung der Covid-19-Epidemie (Covid-19-Gesetz)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12100/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12100/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 18. Juni 2023<br></p>"]
#[cfg(feature = "bl12100")]
pub mod aenderung_vom_16_dezember_2022_des_bundesgesetzes_ueber_die_gesetzlichen_grundlagen_fuer_verordnungen_des_bundesrates_zur_bewaeltigung_der_covid_19_epidemie_covid_19_gesetz {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12100/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wohnbev\u{f6}lkerung nach Geschlecht, Altersgruppe, Gemeinde und Jahr (1941 - 2000)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12140/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12140/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Volksz\u{e4}hlungen 1941-2000</p><p>Gemeinden: Gemeindestand Jahr 2000<br></p>"]
#[cfg(feature = "bl12140")]
pub mod wohnbevoelkerung_nach_geschlecht_altersgruppe_gemeinde_und_jahr_1941_2000 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// Gemeindenummer
        pub gemeindenummer: Option<String>,
        /// Gemeindename
        pub gemeindename: Option<String>,
        /// Altersgruppe
        pub altersgruppe: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Anzahl
        pub anzahl: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Gemeindenummer,
        Gemeindename,
        Altersgruppe,
        Geschlecht,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Gemeindenummer => "gemeindenummer",
                Field::Gemeindename => "gemeindename",
                Field::Altersgruppe => "altersgruppe",
                Field::Geschlecht => "geschlecht",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12140/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wohnbev\u{f6}lkerung nach Gemeinde und Jahr (1699 - 2000)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12150/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12150/</a>\n"]
#[doc = "Volksz\u{e4}hlungen in der Schweiz vor 1850; Eidgen\u{f6}ssische Volksz\u{e4}hlungen 1850-2000<div><br></div><div>Leere Datenfelder: Daten wurden nicht erhoben</div><div><br></div><div>Quellen</div><ul><li>Schuler, Martin: <a href=\"https://www.bfs.admin.ch/bfs/de/home/aktuell/neue-veroeffentlichungen.assetdetail.25105964.html\" target=\"_blank\">Volksz\u{e4}hlungen in der Schweiz vor 1850</a>. Die Bev\u{f6}lkerungszahlen auf lokaler Ebene. Bundesamt f\u{fc}r Statistik (BFS), 2023.</li><li>Bundesamt f\u{fc}r Statistik:\u{a0}<a href=\"https://www.bfs.admin.ch/bfs/de/home/aktuell/neue-veroeffentlichungen.assetdetail.24306873.html\" target=\"_blank\">Kanton Basel-Landschaft - Die Bev\u{f6}lkerungszahlen auf lokaler Ebene vor 1850</a></li><li>Bundesamt f\u{fc}r Statistik:\u{a0}<a href=\"https://opendata.swiss/de/dataset/daten-der-eidgenossischen-volkszahlungen-ab-1850-nach-gemeinden-csv-datensatz\" target=\"_blank\">Volksz\u{e4}hlungen 1850-2000</a> (opendata.swiss)</li></ul><div><br></div><div>Gemeinden: Gemeindestand und BFS-Gemeindenummern mit Basis Jahr 2000</div><div><br></div><div>Gemeindegebiete: Langenbruck ab 1699 inkl. B\u{e4}renwil, Laufen bis 1850 bestehend aus Laufen Stadt und Laufen Vorstadt, Muttenz bis 1870 inkl. Birsfelden, Arisdorf bis 1880 inkl. Olsberg</div><div><br></div><div><br></div><div>Erl\u{e4}uterungen</div><div><br></div><div>Einwohnerinnen und Einwohner</div><div><br></div><div>Unter dem Bev\u{f6}lkerungskonzept \u{ab}Einwohnerinnen und Einwohner\u{bb} versteht man alle im Staatsgebiete anwesenden Personen an dem Orte, wo sie sich am festgelegten Stichtag befinden. In den Volksz\u{e4}hlungen 1850 und 1860 wurde die Zahl der Einwohnerinnen und Einwohner ermittelt. Zwar wurde 1860 auch die Zahl der vor\u{fc}bergehend Abwesenden bestimmt, in der Einteilung der Einwohnerinnen und Einwohner nach Geschlecht, Konfession, Zivilstand, Heimatsverh\u{e4}ltnisse wurden die vor\u{fc}bergehend Abwesenden aber nicht abgezogen.</div><div><br></div><div>Ortsanwesende oder faktische Bev\u{f6}lkerung</div><div><br></div><div>Unter der ortsanwesenden oder faktischen Bev\u{f6}lkerung versteht man die am Tag der Z\u{e4}hlung am Z\u{e4}hlungsorte anwesende Bev\u{f6}lkerung ausschliesslich der vor\u{fc}bergehend Abwesenden und einschliesslich der Durchreisenden. In den Volksz\u{e4}hlungen 1870 bis 1930 wurde zus\u{e4}tzlich zur Wohnbev\u{f6}lkerung die ortsanwesende Bev\u{f6}lkerung ermittelt.</div><div><br></div><div>Wohnbev\u{f6}lkerung</div><div><br></div><div>Die Wohnbev\u{f6}lkerung beinhaltet alle Personen, die sich zum Zeitpunkt der Erhebung in der Schweiz aufhielten oder ihre Schriften hier deponiert hatten und auch dann, wenn sie vor\u{fc}bergehend abwesend waren. Auch Saisonarbeiter, Kurzaufenthalter und Asylsuchende geh\u{f6}ren zur Wohnbev\u{f6}lkerung, nicht aber Touristen, Besucher oder Gesch\u{e4}ftsreisende. Die Wohnbev\u{f6}lkerung wurde in den Volksz\u{e4}hlungen 1870 bis 2000 ermittelt.</div><div><br></div><div>Sprache</div><div><br></div><div>In den Jahren 1860 und 1870 wurden die Angaben der Haushalte auf die Bev\u{f6}lkerung hochgerechnet.</div><div><br></div><div>Konfession/Religion</div><div><br></div><div>Aus Gr\u{fc}nden der Datenbeschaffenheit sind die Christkatholiken bis 1960 im Total der Katholiken enthalten, ab 1970 in der Kategorie \u{ab}Andere_Religion\u{bb}.</div><div><br></div><div>Weiterf\u{fc}hrende Informationen</div><ul><li>Amt f\u{fc}r Daten und Statistik: <a href=\"https://www.baselland.ch/politik-und-behorden/direktionen/finanz-und-kirchendirektion/statistisches-amt/schwerpunkt-demografie/demografiebericht-2011/11-demografiebericht.pdf\" target=\"_blank\">Demografiebericht 2011</a></li></ul><ul><li>Amt f\u{fc}r Daten und Statistik:\u{a0}<a href=\"https://www.baselland.ch/politik-und-behorden/direktionen/finanz-und-kirchendirektion/statistisches-amt/publikationen/bevoelkerung/webartikel_vom_05-04-2023_bevoelkerungsstatistik_2022\" target=\"_blank\">Bev\u{f6}lkerungsstatistik 2022</a></li></ul>"]
#[cfg(feature = "bl12150")]
pub mod wohnbevoelkerung_nach_gemeinde_und_jahr_1699_2000 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        ///
        /// Jahr der Erhebung
        pub jahr: Option<String>,
        /// Datenquelle
        pub datenquelle: Option<String>,
        /// Bevölkerungsbegriff
        ///
        /// Bevölkerungsdefinition
        pub bevolkerungsbegriff: Option<String>,
        /// BFS_Gemeindenummer
        ///
        /// Gemeindenummer gemäss Bundesamt für Statistik (Jahr 2000)
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        ///
        /// Gemeindename
        pub gemeinde: Option<String>,
        /// Bezirk_historisch
        ///
        /// Historische Bezirkszugehörigkeit
        pub bezirk_historisch: Option<String>,
        /// Kanton_historisch
        ///
        /// Historische Kantonszugehörigkeit
        pub kanton_historisch: Option<String>,
        /// Bevölkerung_total
        ///
        /// Bevölkerung total
        pub bevolkerung_total: Option<i64>,
        /// Schweizer_innen
        ///
        /// davon Schweizer/innen
        pub schweizer_innen: Option<i64>,
        /// Ausländer_innen
        ///
        /// davon Ausländer/innen
        pub auslander_innen: Option<i64>,
        /// Männer
        ///
        /// davon männliches Geschlecht
        pub manner: Option<i64>,
        /// Frauen
        ///
        /// davon weibliches Geschlecht
        pub frauen: Option<i64>,
        /// Reformierte
        ///
        /// davon reformiert
        pub reformierte: Option<i64>,
        /// Katholische
        ///
        /// davon katholisch
        pub katholische: Option<i64>,
        /// Andere_Religion
        ///
        /// davon andere Religion/Konfession
        pub andere_religion: Option<i64>,
        /// Deutschsprachige
        ///
        /// davon Deutsch sprechende
        pub deutschsprachige: Option<i64>,
        /// Französischsprachige
        ///
        /// davon Französisch sprechende
        pub franzosischsprachige: Option<i64>,
        /// Italienischsprachige
        ///
        /// davon Italienisch sprechende
        pub italienischsprachige: Option<i64>,
        /// Rätoromanischsprachige
        ///
        /// davon Rätoromanisch sprechende
        pub ratoromanischsprachige: Option<i64>,
        /// Nichtlandessprache
        ///
        /// davon andere Sprachen sprechende
        pub nichtlandessprache: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        Datenquelle,
        Bevolkerungsbegriff,
        BfsGemeindenummer,
        Gemeinde,
        BezirkHistorisch,
        KantonHistorisch,
        BevolkerungTotal,
        SchweizerInnen,
        AuslanderInnen,
        Manner,
        Frauen,
        Reformierte,
        Katholische,
        AndereReligion,
        Deutschsprachige,
        Franzosischsprachige,
        Italienischsprachige,
        Ratoromanischsprachige,
        Nichtlandessprache,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::Datenquelle => "datenquelle",
                Field::Bevolkerungsbegriff => "bevolkerungsbegriff",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::BezirkHistorisch => "bezirk_historisch",
                Field::KantonHistorisch => "kanton_historisch",
                Field::BevolkerungTotal => "bevolkerung_total",
                Field::SchweizerInnen => "schweizer_innen",
                Field::AuslanderInnen => "auslander_innen",
                Field::Manner => "manner",
                Field::Frauen => "frauen",
                Field::Reformierte => "reformierte",
                Field::Katholische => "katholische",
                Field::AndereReligion => "andere_religion",
                Field::Deutschsprachige => "deutschsprachige",
                Field::Franzosischsprachige => "franzosischsprachige",
                Field::Italienischsprachige => "italienischsprachige",
                Field::Ratoromanischsprachige => "ratoromanischsprachige",
                Field::Nichtlandessprache => "nichtlandessprache",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12150/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Kantonales Geb\u{e4}ude- und Wohnungsregister (kGWR): Geb\u{e4}ude"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12160/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12160/</a>\n"]
#[doc = "<p>Geb\u{e4}udemerkmale gem\u{e4}ss kGWR</p><p>\u{d6}ffentlich zug\u{e4}ngliche Geb\u{e4}udemerkmale gem\u{e4}ss <a href=\"https://bl.clex.ch/frontend/annex_document_dictionaries/21431\" target=\"_blank\">Verordnung \u{fc}ber das kGWR, Anhang 3</a></p><p>Neben den vom Bund definierten Merkmalen (siehe <a href=\"https://www.housing-stat.ch/de/help/42.html\" target=\"_blank\">GWR Merkmalskatalog 4.2</a>) enth\u{e4}lt der Datensatz die folgenden kantonalen Merkmale (Quelle: Energiestatistik 2022):</p><ul><li>Photovoltaik (Ja/Nein)</li><li>Leistung Photovoltaik (Kilowatt Peak)</li><li>Datenquelle Photovoltaik</li></ul><p><br></p>"]
#[cfg(feature = "bl12160")]
pub mod kantonales_gebaeude_und_wohnungsregister_kgwr_gebaeude {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Eidgenössischer Gebäudeidentifikator
        pub egid: Option<String>,
        pub gemeindenummer_bfs: Option<String>,
        pub gemeindename: Option<String>,
        /// Eidgenössischer Grundstücksidentifikator
        pub egrid: Option<String>,
        /// grundstücksnummer
        ///
        /// LPARZ
        pub grundstucksnummer: Option<String>,
        /// GBEZ
        pub name_des_gebaeudes: Option<String>,
        pub e_gebaeudekoordinate: Option<f64>,
        pub n_gebaeudekoordinate: Option<f64>,
        pub koordinatenherkunft_code: Option<i64>,
        pub koordinatenherkunft_bezeichnung: Option<String>,
        /// GSTAT
        pub gebaeudestatus_code: Option<i64>,
        pub gebaeudestatus_bezeichnung: Option<String>,
        /// GKAT
        pub gebaeudekategorie_code: Option<i64>,
        pub gebaeudekategorie_bezeichnung: Option<String>,
        /// GKLAS
        pub gebaeudeklasse_code: Option<i64>,
        pub gebaeudeklasse_bezeichnung: Option<String>,
        /// GBAUJ
        pub baujahr_des_gebaeudes: Option<String>,
        /// GBAUM
        pub baumonat_des_gebaeudes: Option<i64>,
        /// GBAUP
        pub bauperiode_code: Option<i64>,
        pub bauperiode_bezeichnung: Option<String>,
        /// GABBJ
        pub abbruchjahr_des_gebaeudes: Option<i64>,
        /// GAREA
        pub gebaeudeflaeche: Option<i64>,
        /// GASTW
        pub anzahl_geschosse: Option<i64>,
        pub anzahl_separate_wohnraeume: Option<i64>,
        /// GVOL
        pub gebaeudevolumen: Option<i64>,
        pub gebaeudevolumen_norm_code: Option<i64>,
        pub gebaeudevolumen_norm_bezeichnung: Option<String>,
        pub informationsquelle_zum_gebaeudevolumen_code: Option<i64>,
        pub informationsquelle_zum_gebaeudevolumen_bezeichnung: Option<String>,
        /// GEBF
        pub energiebezugsflaeche: Option<i64>,
        /// GSCHUTZR
        pub zivilschutzraum_code: Option<i64>,
        pub zivilschutzraum_bezeichnung: Option<String>,
        /// GWAERZH1
        pub waermeerzeuger_heizung_primaer_code: Option<i64>,
        pub waermeerzeuger_heizung_primaer_bezeichnung: Option<String>,
        /// GENH1
        pub energie_waermequelle_heizung_primaer_code: Option<i64>,
        pub energie_waermequelle_heizung_primaer_bezeichnung: Option<String>,
        pub informationsquelle_heizung_primaer_code: Option<i64>,
        pub informationsquelle_heizung_primaer_bezeichnung: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub aktualisierungsdatum_heizung_primaer: Option<Date>,
        /// GWAERZH2
        pub waermeerzeuger_heizung_sekundaer_code: Option<i64>,
        pub waermeerzeuger_heizung_sekundaer_bezeichnung: Option<String>,
        /// GENH2
        pub energie_waermequelle_heizung_sekundaer_code: Option<i64>,
        pub energie_waermequelle_heizung_sekundaer_bezeichnung: Option<String>,
        pub informationsquelle_heizung_sekundaer_code: Option<i64>,
        pub informationsquelle_heizung_sekundaer_bezeichnung: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub aktualisierungsdatum_heizung_sekundaer: Option<Date>,
        /// GWAERZW1
        pub waermeerzeuger_warmwasser_primaer_code: Option<i64>,
        pub waermeerzeuger_warmwasser_primaer_bezeichnung: Option<String>,
        /// GENW1
        pub energie_waermequelle_warmwasser_primaer_code: Option<i64>,
        pub energie_waermequelle_warmwasser_primaer_bezeichnung: Option<String>,
        pub informationsquelle_warmwasser_primaer_code: Option<i64>,
        pub informationsquelle_warmwasser_primaer_bezeichnung: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub aktualisierungsdatum_warmwasser_primaer: Option<Date>,
        /// GWAERZW2
        pub waermeerzeuger_warmwasser_sekundaer_code: Option<i64>,
        pub waermeerzeuger_warmwasser_sekundaer_bezeichnung: Option<String>,
        /// GENW2
        pub energie_waermequelle_warmwasser_sekundaer_code: Option<i64>,
        pub energie_waermequelle_warmwasser_sekundaer_bezeichnung: Option<String>,
        pub informationsquelle_warmwasser_sekundaer_code: Option<i64>,
        pub informationsquelle_warmwasser_sekundaer_bezeichnung: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub aktualisierungsdatum_warmwasser_sekundaer: Option<Date>,
        pub photovoltaik: Option<String>,
        pub leistung_photovoltaik_kwp: Option<f64>,
        pub datenquelle_photovoltaik: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub exportdatum: Option<Date>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Egid,
        GemeindenummerBfs,
        Gemeindename,
        Egrid,
        Grundstucksnummer,
        NameDesGebaeudes,
        EGebaeudekoordinate,
        NGebaeudekoordinate,
        KoordinatenherkunftCode,
        KoordinatenherkunftBezeichnung,
        GebaeudestatusCode,
        GebaeudestatusBezeichnung,
        GebaeudekategorieCode,
        GebaeudekategorieBezeichnung,
        GebaeudeklasseCode,
        GebaeudeklasseBezeichnung,
        BaujahrDesGebaeudes,
        BaumonatDesGebaeudes,
        BauperiodeCode,
        BauperiodeBezeichnung,
        AbbruchjahrDesGebaeudes,
        Gebaeudeflaeche,
        AnzahlGeschosse,
        AnzahlSeparateWohnraeume,
        Gebaeudevolumen,
        GebaeudevolumenNormCode,
        GebaeudevolumenNormBezeichnung,
        InformationsquelleZumGebaeudevolumenCode,
        InformationsquelleZumGebaeudevolumenBezeichnung,
        Energiebezugsflaeche,
        ZivilschutzraumCode,
        ZivilschutzraumBezeichnung,
        WaermeerzeugerHeizungPrimaerCode,
        WaermeerzeugerHeizungPrimaerBezeichnung,
        EnergieWaermequelleHeizungPrimaerCode,
        EnergieWaermequelleHeizungPrimaerBezeichnung,
        InformationsquelleHeizungPrimaerCode,
        InformationsquelleHeizungPrimaerBezeichnung,
        AktualisierungsdatumHeizungPrimaer,
        WaermeerzeugerHeizungSekundaerCode,
        WaermeerzeugerHeizungSekundaerBezeichnung,
        EnergieWaermequelleHeizungSekundaerCode,
        EnergieWaermequelleHeizungSekundaerBezeichnung,
        InformationsquelleHeizungSekundaerCode,
        InformationsquelleHeizungSekundaerBezeichnung,
        AktualisierungsdatumHeizungSekundaer,
        WaermeerzeugerWarmwasserPrimaerCode,
        WaermeerzeugerWarmwasserPrimaerBezeichnung,
        EnergieWaermequelleWarmwasserPrimaerCode,
        EnergieWaermequelleWarmwasserPrimaerBezeichnung,
        InformationsquelleWarmwasserPrimaerCode,
        InformationsquelleWarmwasserPrimaerBezeichnung,
        AktualisierungsdatumWarmwasserPrimaer,
        WaermeerzeugerWarmwasserSekundaerCode,
        WaermeerzeugerWarmwasserSekundaerBezeichnung,
        EnergieWaermequelleWarmwasserSekundaerCode,
        EnergieWaermequelleWarmwasserSekundaerBezeichnung,
        InformationsquelleWarmwasserSekundaerCode,
        InformationsquelleWarmwasserSekundaerBezeichnung,
        AktualisierungsdatumWarmwasserSekundaer,
        Photovoltaik,
        LeistungPhotovoltaikKwp,
        DatenquellePhotovoltaik,
        Exportdatum,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Egid => "egid",
                Field::GemeindenummerBfs => "gemeindenummer_bfs",
                Field::Gemeindename => "gemeindename",
                Field::Egrid => "egrid",
                Field::Grundstucksnummer => "grundstucksnummer",
                Field::NameDesGebaeudes => "name_des_gebaeudes",
                Field::EGebaeudekoordinate => "e_gebaeudekoordinate",
                Field::NGebaeudekoordinate => "n_gebaeudekoordinate",
                Field::KoordinatenherkunftCode => "koordinatenherkunft_code",
                Field::KoordinatenherkunftBezeichnung => "koordinatenherkunft_bezeichnung",
                Field::GebaeudestatusCode => "gebaeudestatus_code",
                Field::GebaeudestatusBezeichnung => "gebaeudestatus_bezeichnung",
                Field::GebaeudekategorieCode => "gebaeudekategorie_code",
                Field::GebaeudekategorieBezeichnung => "gebaeudekategorie_bezeichnung",
                Field::GebaeudeklasseCode => "gebaeudeklasse_code",
                Field::GebaeudeklasseBezeichnung => "gebaeudeklasse_bezeichnung",
                Field::BaujahrDesGebaeudes => "baujahr_des_gebaeudes",
                Field::BaumonatDesGebaeudes => "baumonat_des_gebaeudes",
                Field::BauperiodeCode => "bauperiode_code",
                Field::BauperiodeBezeichnung => "bauperiode_bezeichnung",
                Field::AbbruchjahrDesGebaeudes => "abbruchjahr_des_gebaeudes",
                Field::Gebaeudeflaeche => "gebaeudeflaeche",
                Field::AnzahlGeschosse => "anzahl_geschosse",
                Field::AnzahlSeparateWohnraeume => "anzahl_separate_wohnraeume",
                Field::Gebaeudevolumen => "gebaeudevolumen",
                Field::GebaeudevolumenNormCode => "gebaeudevolumen_norm_code",
                Field::GebaeudevolumenNormBezeichnung => "gebaeudevolumen_norm_bezeichnung",
                Field::InformationsquelleZumGebaeudevolumenCode => {
                    "informationsquelle_zum_gebaeudevolumen_code"
                }
                Field::InformationsquelleZumGebaeudevolumenBezeichnung => {
                    "informationsquelle_zum_gebaeudevolumen_bezeichnung"
                }
                Field::Energiebezugsflaeche => "energiebezugsflaeche",
                Field::ZivilschutzraumCode => "zivilschutzraum_code",
                Field::ZivilschutzraumBezeichnung => "zivilschutzraum_bezeichnung",
                Field::WaermeerzeugerHeizungPrimaerCode => "waermeerzeuger_heizung_primaer_code",
                Field::WaermeerzeugerHeizungPrimaerBezeichnung => {
                    "waermeerzeuger_heizung_primaer_bezeichnung"
                }
                Field::EnergieWaermequelleHeizungPrimaerCode => {
                    "energie_waermequelle_heizung_primaer_code"
                }
                Field::EnergieWaermequelleHeizungPrimaerBezeichnung => {
                    "energie_waermequelle_heizung_primaer_bezeichnung"
                }
                Field::InformationsquelleHeizungPrimaerCode => {
                    "informationsquelle_heizung_primaer_code"
                }
                Field::InformationsquelleHeizungPrimaerBezeichnung => {
                    "informationsquelle_heizung_primaer_bezeichnung"
                }
                Field::AktualisierungsdatumHeizungPrimaer => "aktualisierungsdatum_heizung_primaer",
                Field::WaermeerzeugerHeizungSekundaerCode => {
                    "waermeerzeuger_heizung_sekundaer_code"
                }
                Field::WaermeerzeugerHeizungSekundaerBezeichnung => {
                    "waermeerzeuger_heizung_sekundaer_bezeichnung"
                }
                Field::EnergieWaermequelleHeizungSekundaerCode => {
                    "energie_waermequelle_heizung_sekundaer_code"
                }
                Field::EnergieWaermequelleHeizungSekundaerBezeichnung => {
                    "energie_waermequelle_heizung_sekundaer_bezeichnung"
                }
                Field::InformationsquelleHeizungSekundaerCode => {
                    "informationsquelle_heizung_sekundaer_code"
                }
                Field::InformationsquelleHeizungSekundaerBezeichnung => {
                    "informationsquelle_heizung_sekundaer_bezeichnung"
                }
                Field::AktualisierungsdatumHeizungSekundaer => {
                    "aktualisierungsdatum_heizung_sekundaer"
                }
                Field::WaermeerzeugerWarmwasserPrimaerCode => {
                    "waermeerzeuger_warmwasser_primaer_code"
                }
                Field::WaermeerzeugerWarmwasserPrimaerBezeichnung => {
                    "waermeerzeuger_warmwasser_primaer_bezeichnung"
                }
                Field::EnergieWaermequelleWarmwasserPrimaerCode => {
                    "energie_waermequelle_warmwasser_primaer_code"
                }
                Field::EnergieWaermequelleWarmwasserPrimaerBezeichnung => {
                    "energie_waermequelle_warmwasser_primaer_bezeichnung"
                }
                Field::InformationsquelleWarmwasserPrimaerCode => {
                    "informationsquelle_warmwasser_primaer_code"
                }
                Field::InformationsquelleWarmwasserPrimaerBezeichnung => {
                    "informationsquelle_warmwasser_primaer_bezeichnung"
                }
                Field::AktualisierungsdatumWarmwasserPrimaer => {
                    "aktualisierungsdatum_warmwasser_primaer"
                }
                Field::WaermeerzeugerWarmwasserSekundaerCode => {
                    "waermeerzeuger_warmwasser_sekundaer_code"
                }
                Field::WaermeerzeugerWarmwasserSekundaerBezeichnung => {
                    "waermeerzeuger_warmwasser_sekundaer_bezeichnung"
                }
                Field::EnergieWaermequelleWarmwasserSekundaerCode => {
                    "energie_waermequelle_warmwasser_sekundaer_code"
                }
                Field::EnergieWaermequelleWarmwasserSekundaerBezeichnung => {
                    "energie_waermequelle_warmwasser_sekundaer_bezeichnung"
                }
                Field::InformationsquelleWarmwasserSekundaerCode => {
                    "informationsquelle_warmwasser_sekundaer_code"
                }
                Field::InformationsquelleWarmwasserSekundaerBezeichnung => {
                    "informationsquelle_warmwasser_sekundaer_bezeichnung"
                }
                Field::AktualisierungsdatumWarmwasserSekundaer => {
                    "aktualisierungsdatum_warmwasser_sekundaer"
                }
                Field::Photovoltaik => "photovoltaik",
                Field::LeistungPhotovoltaikKwp => "leistung_photovoltaik_kwp",
                Field::DatenquellePhotovoltaik => "datenquelle_photovoltaik",
                Field::Exportdatum => "exportdatum",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12160/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Kantonales Geb\u{e4}ude- und Wohnungsregister (kGWR): Wohnungen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12170/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12170/</a>\n"]
#[doc = "<p>Wohnungsmerkmale gem\u{e4}ss kGWR</p><p>\u{d6}ffentlich zug\u{e4}ngliche Wohnungsmerkmale gem\u{e4}ss <a href=\"https://bl.clex.ch/frontend/annex_document_dictionaries/21431\" target=\"_blank\">Verordnung \u{fc}ber das kGWR, Anhang 3</a><br></p>"]
#[cfg(feature = "bl12170")]
pub mod kantonales_gebaeude_und_wohnungsregister_kgwr_wohnungen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Eidgenössischer Gebäudeidentifikator
        pub egid: Option<String>,
        /// Eidgenössischer Wohnungsidentifikator
        pub ewid: Option<i64>,
        pub gemeindenummer_bfs: Option<String>,
        pub gemeindename: Option<String>,
        /// EDID
        pub eidgenoessischer_eingangsidentifikator: Option<i64>,
        pub strassenbezeichnung: Option<String>,
        pub eingangsnummer_gebaeude: Option<String>,
        /// WHGNR
        pub administrative_wohnungsnummer: Option<String>,
        /// WEINR
        pub physische_wohnungsnummer: Option<String>,
        /// WSTWK
        pub stockwerk_code: Option<i64>,
        ///
        pub stockwerk_bezeichnung: Option<String>,
        /// WBEZ
        pub lage_auf_dem_stockwerk: Option<String>,
        /// WMEHRG
        pub mehrgeschossige_wohnung_code: Option<i64>,
        pub mehrgeschossige_wohnung_bezeichnung: Option<String>,
        /// WBAUJ
        pub baujahr_der_wohnung: Option<String>,
        pub abbruchjahr_der_wohnung: Option<i64>,
        /// WSTAT
        pub wohnungsstatus_code: Option<i64>,
        pub wohnungsstatus_bezeichnung: Option<String>,
        /// WAREA
        pub wohnungsflaeche: Option<i64>,
        /// WAZIM
        pub anzahl_zimmer: Option<i64>,
        /// WKCHE
        pub kocheinrichtung_code: Option<i64>,
        pub kocheinrichtung_bezeichnung: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub exportdatum: Option<Date>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Egid,
        Ewid,
        GemeindenummerBfs,
        Gemeindename,
        EidgenoessischerEingangsidentifikator,
        Strassenbezeichnung,
        EingangsnummerGebaeude,
        AdministrativeWohnungsnummer,
        PhysischeWohnungsnummer,
        StockwerkCode,
        StockwerkBezeichnung,
        LageAufDemStockwerk,
        MehrgeschossigeWohnungCode,
        MehrgeschossigeWohnungBezeichnung,
        BaujahrDerWohnung,
        AbbruchjahrDerWohnung,
        WohnungsstatusCode,
        WohnungsstatusBezeichnung,
        Wohnungsflaeche,
        AnzahlZimmer,
        KocheinrichtungCode,
        KocheinrichtungBezeichnung,
        Exportdatum,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Egid => "egid",
                Field::Ewid => "ewid",
                Field::GemeindenummerBfs => "gemeindenummer_bfs",
                Field::Gemeindename => "gemeindename",
                Field::EidgenoessischerEingangsidentifikator => {
                    "eidgenoessischer_eingangsidentifikator"
                }
                Field::Strassenbezeichnung => "strassenbezeichnung",
                Field::EingangsnummerGebaeude => "eingangsnummer_gebaeude",
                Field::AdministrativeWohnungsnummer => "administrative_wohnungsnummer",
                Field::PhysischeWohnungsnummer => "physische_wohnungsnummer",
                Field::StockwerkCode => "stockwerk_code",
                Field::StockwerkBezeichnung => "stockwerk_bezeichnung",
                Field::LageAufDemStockwerk => "lage_auf_dem_stockwerk",
                Field::MehrgeschossigeWohnungCode => "mehrgeschossige_wohnung_code",
                Field::MehrgeschossigeWohnungBezeichnung => "mehrgeschossige_wohnung_bezeichnung",
                Field::BaujahrDerWohnung => "baujahr_der_wohnung",
                Field::AbbruchjahrDerWohnung => "abbruchjahr_der_wohnung",
                Field::WohnungsstatusCode => "wohnungsstatus_code",
                Field::WohnungsstatusBezeichnung => "wohnungsstatus_bezeichnung",
                Field::Wohnungsflaeche => "wohnungsflaeche",
                Field::AnzahlZimmer => "anzahl_zimmer",
                Field::KocheinrichtungCode => "kocheinrichtung_code",
                Field::KocheinrichtungBezeichnung => "kocheinrichtung_bezeichnung",
                Field::Exportdatum => "exportdatum",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12170/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Kantonales Geb\u{e4}ude- und Wohnungsregister (kGWR): Geb\u{e4}udeadressen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12180/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12180/</a>\n"]
#[doc = "<p>Geb\u{e4}udeadressen gem\u{e4}ss kGWR</p><p>\u{d6}ffentlich zug\u{e4}ngliche Geb\u{e4}udemerkmale gem\u{e4}ss Verordnung \u{fc}ber das <a href=\"https://bl.clex.ch/frontend/annex_document_dictionaries/21431\" target=\"_blank\">kGWR, Anhang 3</a><br></p>"]
#[cfg(feature = "bl12180")]
pub mod kantonales_gebaeude_und_wohnungsregister_kgwr_gebaeudeadressen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Eidgenössischer Gebäudeidentifikator
        pub egid: Option<String>,
        pub gemeindenummer_bfs: Option<String>,
        pub gemeindename: Option<String>,
        /// EDID
        pub eidgenoessischer_eingangsidentifikator: Option<i64>,
        /// EGAID
        pub eidgenoessischer_gebaeudeadressidentifikator: Option<String>,
        /// DEINR
        pub eingangsnummer_gebaeude: Option<String>,
        /// ESID
        pub eidgenoessischer_strassenidentifikator: Option<String>,
        pub strassenbezeichnung: Option<String>,
        pub strassenbezeichnung_kurz: Option<String>,
        pub strassenbezeichnung_index: Option<String>,
        pub strassenbezeichnung_offiziell_code: Option<i64>,
        pub strassenbezeichnung_offiziell_bezeichnung: Option<String>,
        /// PLZ
        pub postleitzahl: Option<String>,
        pub postleitzahl_zusatzziffer: Option<String>,
        pub postleitzahl_name: Option<String>,
        pub e_eingangskoordinate: Option<f64>,
        pub n_eingangskoordinate: Option<f64>,
        pub offizielle_adresse_code: Option<i64>,
        pub offizielle_adresse_bezeichnung: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub exportdatum: Option<Date>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Egid,
        GemeindenummerBfs,
        Gemeindename,
        EidgenoessischerEingangsidentifikator,
        EidgenoessischerGebaeudeadressidentifikator,
        EingangsnummerGebaeude,
        EidgenoessischerStrassenidentifikator,
        Strassenbezeichnung,
        StrassenbezeichnungKurz,
        StrassenbezeichnungIndex,
        StrassenbezeichnungOffiziellCode,
        StrassenbezeichnungOffiziellBezeichnung,
        Postleitzahl,
        PostleitzahlZusatzziffer,
        PostleitzahlName,
        EEingangskoordinate,
        NEingangskoordinate,
        OffizielleAdresseCode,
        OffizielleAdresseBezeichnung,
        Exportdatum,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Egid => "egid",
                Field::GemeindenummerBfs => "gemeindenummer_bfs",
                Field::Gemeindename => "gemeindename",
                Field::EidgenoessischerEingangsidentifikator => {
                    "eidgenoessischer_eingangsidentifikator"
                }
                Field::EidgenoessischerGebaeudeadressidentifikator => {
                    "eidgenoessischer_gebaeudeadressidentifikator"
                }
                Field::EingangsnummerGebaeude => "eingangsnummer_gebaeude",
                Field::EidgenoessischerStrassenidentifikator => {
                    "eidgenoessischer_strassenidentifikator"
                }
                Field::Strassenbezeichnung => "strassenbezeichnung",
                Field::StrassenbezeichnungKurz => "strassenbezeichnung_kurz",
                Field::StrassenbezeichnungIndex => "strassenbezeichnung_index",
                Field::StrassenbezeichnungOffiziellCode => "strassenbezeichnung_offiziell_code",
                Field::StrassenbezeichnungOffiziellBezeichnung => {
                    "strassenbezeichnung_offiziell_bezeichnung"
                }
                Field::Postleitzahl => "postleitzahl",
                Field::PostleitzahlZusatzziffer => "postleitzahl_zusatzziffer",
                Field::PostleitzahlName => "postleitzahl_name",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
                Field::OffizielleAdresseCode => "offizielle_adresse_code",
                Field::OffizielleAdresseBezeichnung => "offizielle_adresse_bezeichnung",
                Field::Exportdatum => "exportdatum",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12180/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2023: Kandidierende nach Liste, Geschlecht, Jahrgang und Beruf"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12190/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12190/</a>\n"]
#[doc = "<p>Definitive Wahlvorschl\u{e4}ge f\u{fc}r die Nationalratswahlen vom 22. Oktober 2023<br></p>"]
#[cfg(feature = "bl12190")]
pub mod nationalratswahlen_2023_kandidierende_nach_liste_geschlecht_jahrgang_und_beruf {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahltermin
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub wahltermin: Option<Date>,
        /// Anzahl_Sitze
        pub anzahl_sitze: Option<i64>,
        /// Listen-Nr
        pub listen_nr: Option<String>,
        /// Parteikurzbezeichnung
        pub parteikurzbezeichnung: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
        /// Anzahl_leere_Linien
        pub anzahl_leere_linien: Option<i64>,
        /// HLV-Nr
        ///
        /// Nummer der Hauptlistenverbindung
        pub hlv_nr: Option<i64>,
        /// HLV-Bezeichnung
        ///
        /// Hauptlistenverbindung
        pub hlv_bezeichnung: Option<String>,
        /// ULV-Nr
        ///
        /// Nummer der Unterlistenverbindung (mit Bezug zur Hauptlistenverbindung eindeutig)
        pub ulv_nr: Option<i64>,
        /// ULV-Bezeichnung
        ///
        /// Unterlistenverbindung
        pub ulv_bezeichnung: Option<String>,
        /// Zeilen-Nr
        ///
        /// Zeilen-Nr. auf dem Wahlzettel
        pub zeilen_nr: Option<i64>,
        /// Kandidaten-Nr
        pub kandidaten_nr: Option<String>,
        /// Kumulation
        pub kumulation: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Anrede
        pub anrede: Option<String>,
        /// Titel
        pub titel: Option<String>,
        /// Beruf_Tätigkeit
        pub beruf_tatigkeit: Option<String>,
        /// PLZ
        pub plz: Option<String>,
        /// Wohnort
        pub wohnort: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahltermin,
        AnzahlSitze,
        ListenNr,
        Parteikurzbezeichnung,
        Parteibezeichnung,
        AnzahlLeereLinien,
        HlvNr,
        HlvBezeichnung,
        UlvNr,
        UlvBezeichnung,
        ZeilenNr,
        KandidatenNr,
        Kumulation,
        Bisher,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        Anrede,
        Titel,
        BerufTatigkeit,
        Plz,
        Wohnort,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahltermin => "wahltermin",
                Field::AnzahlSitze => "anzahl_sitze",
                Field::ListenNr => "listen_nr",
                Field::Parteikurzbezeichnung => "parteikurzbezeichnung",
                Field::Parteibezeichnung => "parteibezeichnung",
                Field::AnzahlLeereLinien => "anzahl_leere_linien",
                Field::HlvNr => "hlv_nr",
                Field::HlvBezeichnung => "hlv_bezeichnung",
                Field::UlvNr => "ulv_nr",
                Field::UlvBezeichnung => "ulv_bezeichnung",
                Field::ZeilenNr => "zeilen_nr",
                Field::KandidatenNr => "kandidaten_nr",
                Field::Kumulation => "kumulation",
                Field::Bisher => "bisher",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::Anrede => "anrede",
                Field::Titel => "titel",
                Field::BerufTatigkeit => "beruf_tatigkeit",
                Field::Plz => "plz",
                Field::Wohnort => "wohnort",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12190/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Im kantonalen Personenregister abfrageberechtigte Stellen (Anmeldungs- und Registerverordnung)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12200/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12200/</a>\n"]
#[doc = "<p>Verzeichnis\u{a0}aller Stellen, die eine Zugriffsberechtigung auf das kantonale Personenregister arbo BL haben. Es umfasst die rechtlich im <a href=\"https://bl.clex.ch/app/de/texts_of_law/111.11/annex/II\" target=\"_blank\">Anhang II</a> der Anmeldungs- und Registerverordnung (ARV, <a href=\"https://bl.clex.ch/app/de/texts_of_law/111.11\" target=\"_blank\">SGS 111.11</a>) definierten Stellen (auf Grundlage <a href=\"https://bl.clex.ch/app/de/texts_of_law/111/art/14\" target=\"_blank\">\u{a7} 14</a> des Anmeldungs- und Registergesetzes; ARG, <a href=\"https://bl.clex.ch/app/de/texts_of_law/111\" target=\"_blank\">SGS 111</a>) und zeigt, seit wann diese Stellen Zugriff haben und welche Schnittstellen sie verwenden.</p>"]
#[cfg(feature = "bl12200")]
pub mod im_kantonalen_personenregister_abfrageberechtigte_stellen_anmeldungs_und_registerverordnung {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Kürzel
        pub kurzel: Option<String>,
        /// Direktion
        pub direktion: Option<String>,
        /// Abfrageberechtigte Stelle
        pub abfrageberechtigte_stelle: Option<String>,
        /// Aufgabenbereich
        pub aufgabenbereich: Option<String>,
        /// RRB-Nr. (Neuzugriff)
        ///
        /// Nummer des Regierungsratsbeschlusses betreffend Einrichtung des neuen Zugriffs
        pub rrb_nr_neuzugriff: Option<String>,
        /// Beschluss (N)
        ///
        /// Datum des Regierungsratsbeschlusses betreffend Einrichtung des neuen Zugriffs
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub beschluss_n: Option<Date>,
        /// GS-Nr. (N)
        ///
        /// Nummer in der Gesetzessammlung betreffend Einrichtung des neuen Zugriffs
        pub gs_nr_n: Option<String>,
        /// GS-Link (N)
        ///
        /// Link auf die Gesetzessammlung betreffend Einrichtung des neuen Zugriffs
        pub gs_link_n: Option<String>,
        /// Inkrafttreten (N)
        ///
        /// Rechtliches Inkrafttreten des neuen Zugriffs
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub inkrafttreten_n: Option<Date>,
        /// RRB-Nr. (Änderungen)
        ///
        /// Nummer des Regierungsratsbeschlusses betreffend Änderungen des Zugriffs
        pub rrb_nr_anderungen: Option<String>,
        /// Beschluss (Ä)
        ///
        /// Datum des Regierungsratsbeschlusses betreffend Änderungen des Zugriffs
        pub beschluss_a: Option<String>,
        /// GS-Nr. (Ä)
        ///
        /// Nummer in der Gesetzessammlung betreffend Änderungen des Zugriffs
        pub gs_nr_a: Option<String>,
        /// Inkrafttreten (Ä)
        ///
        /// Rechtliches Inkrafttreten des geänderten Zugriffs
        pub inkrafttreten_a: Option<String>,
        /// Web-GUI
        ///
        /// Zugriff über Webapplikation (manuelle Einzelabfragen)
        pub web_gui: Option<String>,
        /// Webservice synchron
        ///
        /// Zugriff über Webservice (automatisierte Einzelabfragen)
        pub webservice_synchron: Option<String>,
        /// Webservice asynchron
        ///
        /// Zugriff über Webservice (automatisierte Massenabfragen)
        pub webservice_asynchron: Option<String>,
        /// Listen
        ///
        /// Periodischer Erhalt von csv-Listen (manuell exportiert durch Fachstelle Register)
        pub listen: Option<String>,
        /// Meldungen
        ///
        /// Laufender Erhalt einzelner Mutationsmeldungen (automatisiert nach eCH-0020)
        pub meldungen: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Kurzel,
        Direktion,
        AbfrageberechtigteStelle,
        Aufgabenbereich,
        RrbNrNeuzugriff,
        BeschlussN,
        GsNrN,
        GsLinkN,
        InkrafttretenN,
        RrbNrAnderungen,
        BeschlussA,
        GsNrA,
        InkrafttretenA,
        WebGui,
        WebserviceSynchron,
        WebserviceAsynchron,
        Listen,
        Meldungen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Kurzel => "kurzel",
                Field::Direktion => "direktion",
                Field::AbfrageberechtigteStelle => "abfrageberechtigte_stelle",
                Field::Aufgabenbereich => "aufgabenbereich",
                Field::RrbNrNeuzugriff => "rrb_nr_neuzugriff",
                Field::BeschlussN => "beschluss_n",
                Field::GsNrN => "gs_nr_n",
                Field::GsLinkN => "gs_link_n",
                Field::InkrafttretenN => "inkrafttreten_n",
                Field::RrbNrAnderungen => "rrb_nr_anderungen",
                Field::BeschlussA => "beschluss_a",
                Field::GsNrA => "gs_nr_a",
                Field::InkrafttretenA => "inkrafttreten_a",
                Field::WebGui => "web_gui",
                Field::WebserviceSynchron => "webservice_synchron",
                Field::WebserviceAsynchron => "webservice_asynchron",
                Field::Listen => "listen",
                Field::Meldungen => "meldungen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12200/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Geb\u{e4}ude nach Eigent\u{fc}mertyp, Wirtschaftsabschnitt, Gemeinde und Jahr (seit 2020)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12240/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12240/</a>\n"]
#[doc = "<p>Geb\u{e4}ude- und Wohnungsstatistik (GWS)</p><p>Die Statistik der Eigent\u{fc}mertypen der Geb\u{e4}ude wird erstellt, indem die Informationen zu den Wohngeb\u{e4}uden aus dem eidgen\u{f6}ssischen Geb\u{e4}ude- und Wohnungsregister (GWR) mit den Grundbuchdaten zu einem oder mehreren Eigent\u{fc}mern verkn\u{fc}pft werden.</p><p>Gemeinschaft: Form des kollektiven Eigentums, wobei jedes Mitglied im Besitz des gesamten Objekts ist. Zu den Gemeinschaften geh\u{f6}ren einfache Gesellschaften, Erbengemeinschaften, G\u{fc}tergemeinschaften und Gemeinderschaften.</p><p>Gemischt: Eigent\u{fc}mertyp, dem Geb\u{e4}ude zugeordnet werden, die mindestens zwei verschiedene Eigent\u{fc}mertypen aufweisen. Da die Anteile jedes Eigent\u{fc}mertyps nicht immer bekannt sind, kann das Geb\u{e4}ude keiner der beiden Kategorien eindeutig zugeordnet werden.</p><p>Unbekannt (leer): Eigent\u{fc}mertyp, dem Geb\u{e4}ude zugeordnet werden, bei denen die Grundb\u{fc}cher keine Informationen zu den Eigent\u{fc}mern enthalten.<br></p>"]
#[cfg(feature = "bl12240")]
pub mod gebaeude_nach_eigentuemertyp_wirtschaftsabschnitt_gemeinde_und_jahr_seit_2020 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Eigentümertyp
        pub eigentumertyp: Option<String>,
        /// Wirtschaftsabschnitt
        pub wirtschaftsabschnitt: Option<String>,
        /// Anzahl
        ///
        /// Anzahl Gebäude
        pub anzahl: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Gemeinde,
        Eigentumertyp,
        Wirtschaftsabschnitt,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Eigentumertyp => "eigentumertyp",
                Field::Wirtschaftsabschnitt => "wirtschaftsabschnitt",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12240/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeinn\u{fc}tzige Wohnungen nach Zimmerzahl, Gemeinde und Jahr (seit 2016)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12250/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12250/</a>\n"]
#[doc = "<p>Geb\u{e4}ude- und Wohnungsstatistik GWS (Bundesamt f\u{fc}r Statistik), Bundesamt f\u{fc}r Wohnungswesen</p><p>Die Datenbank mit den Geb\u{e4}udeadressen der gemeinn\u{fc}tzigen Bautr\u{e4}ger basiert auf einer Befragung der Mitglieder der beiden Dachorganisationen (Wohnbaugenossenschaften Schweiz und WOHNEN Schweiz), den Kunden der Emissionszentrale der gemeinn\u{fc}tzigen Wohnbautr\u{e4}ger EGW sowie den Adressen der vom Bundesamt f\u{fc}r Wohnungswesen unterst\u{fc}tzten gemeinn\u{fc}tzigen Bautr\u{e4}ger. Gest\u{fc}tzt auf die Antwortquoten und der vorgenannten Auswahl wird gesch\u{e4}tzt, dass rund 90 Prozent aller sich im Besitz des gemeinn\u{fc}tzigen Sektors befindlichen Wohnungen in die Auswertungen einfliessen.<br></p>"]
#[cfg(feature = "bl12250")]
pub mod gemeinnuetzige_wohnungen_nach_zimmerzahl_gemeinde_und_jahr_seit_2016 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Zimmerzahl
        pub zimmerzahl: Option<i64>,
        /// ANZAHL
        ///
        /// Anzahl gemeinnützige Wohnungen
        pub anzahl: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Gemeinde,
        Zimmerzahl,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Zimmerzahl => "zimmerzahl",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12250/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2023: Kandidierendenresultate, Wahlberechtigte und Listenstimmen"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12270/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12270/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 22. Oktober 2023<br></p>"]
#[cfg(feature = "bl12270")]
pub mod nationalratswahlen_2023_kandidierendenresultate_wahlberechtigte_und_listenstimmen {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_gender: Option<String>,
        pub candidate_id: Option<String>,
        pub candidate_panachage_votes_from_list_01: Option<i64>,
        pub candidate_panachage_votes_from_list_02: Option<i64>,
        pub candidate_panachage_votes_from_list_03: Option<i64>,
        pub candidate_panachage_votes_from_list_04: Option<i64>,
        pub candidate_panachage_votes_from_list_05: Option<i64>,
        pub candidate_panachage_votes_from_list_06: Option<i64>,
        pub candidate_panachage_votes_from_list_07: Option<i64>,
        pub candidate_panachage_votes_from_list_08: Option<i64>,
        pub candidate_panachage_votes_from_list_11: Option<i64>,
        pub candidate_panachage_votes_from_list_12: Option<i64>,
        pub candidate_panachage_votes_from_list_13: Option<i64>,
        pub candidate_panachage_votes_from_list_14: Option<i64>,
        pub candidate_panachage_votes_from_list_16: Option<i64>,
        pub candidate_panachage_votes_from_list_17: Option<i64>,
        pub candidate_panachage_votes_from_list_18: Option<i64>,
        pub candidate_panachage_votes_from_list_22: Option<i64>,
        pub candidate_panachage_votes_from_list_23: Option<i64>,
        pub candidate_panachage_votes_from_list_24: Option<i64>,
        pub candidate_panachage_votes_from_list_33: Option<i64>,
        pub candidate_panachage_votes_from_list_44: Option<i64>,
        pub candidate_panachage_votes_from_list_55: Option<i64>,
        pub candidate_panachage_votes_from_list_56: Option<i64>,
        pub candidate_panachage_votes_from_list_57: Option<i64>,
        pub candidate_panachage_votes_from_list_58: Option<i64>,
        pub candidate_panachage_votes_from_list_70: Option<i64>,
        pub candidate_panachage_votes_from_list_71: Option<i64>,
        pub candidate_panachage_votes_from_list_77: Option<i64>,
        pub candidate_panachage_votes_from_list_999: Option<i64>,
        pub candidate_party: Option<String>,
        pub candidate_votes: Option<i64>,
        pub candidate_year_of_birth: Option<i64>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub election_date: Option<Date>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_counted: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
        pub list_connection: Option<i64>,
        pub list_connection_parent: Option<i64>,
        pub list_id: Option<String>,
        pub list_name: Option<String>,
        pub list_number_of_mandates: Option<i64>,
        pub list_panachage_votes_from_list_01: Option<i64>,
        pub list_panachage_votes_from_list_02: Option<i64>,
        pub list_panachage_votes_from_list_03: Option<i64>,
        pub list_panachage_votes_from_list_04: Option<i64>,
        pub list_panachage_votes_from_list_05: Option<i64>,
        pub list_panachage_votes_from_list_06: Option<i64>,
        pub list_panachage_votes_from_list_07: Option<i64>,
        pub list_panachage_votes_from_list_08: Option<i64>,
        pub list_panachage_votes_from_list_11: Option<i64>,
        pub list_panachage_votes_from_list_12: Option<i64>,
        pub list_panachage_votes_from_list_13: Option<i64>,
        pub list_panachage_votes_from_list_14: Option<i64>,
        pub list_panachage_votes_from_list_16: Option<i64>,
        pub list_panachage_votes_from_list_17: Option<i64>,
        pub list_panachage_votes_from_list_18: Option<i64>,
        pub list_panachage_votes_from_list_22: Option<i64>,
        pub list_panachage_votes_from_list_23: Option<i64>,
        pub list_panachage_votes_from_list_24: Option<i64>,
        pub list_panachage_votes_from_list_33: Option<i64>,
        pub list_panachage_votes_from_list_44: Option<i64>,
        pub list_panachage_votes_from_list_55: Option<i64>,
        pub list_panachage_votes_from_list_56: Option<i64>,
        pub list_panachage_votes_from_list_57: Option<i64>,
        pub list_panachage_votes_from_list_58: Option<i64>,
        pub list_panachage_votes_from_list_70: Option<i64>,
        pub list_panachage_votes_from_list_71: Option<i64>,
        pub list_panachage_votes_from_list_77: Option<i64>,
        pub list_panachage_votes_from_list_999: Option<i64>,
        pub list_votes: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateGender,
        CandidateId,
        CandidatePanachageVotesFromList01,
        CandidatePanachageVotesFromList02,
        CandidatePanachageVotesFromList03,
        CandidatePanachageVotesFromList04,
        CandidatePanachageVotesFromList05,
        CandidatePanachageVotesFromList06,
        CandidatePanachageVotesFromList07,
        CandidatePanachageVotesFromList08,
        CandidatePanachageVotesFromList11,
        CandidatePanachageVotesFromList12,
        CandidatePanachageVotesFromList13,
        CandidatePanachageVotesFromList14,
        CandidatePanachageVotesFromList16,
        CandidatePanachageVotesFromList17,
        CandidatePanachageVotesFromList18,
        CandidatePanachageVotesFromList22,
        CandidatePanachageVotesFromList23,
        CandidatePanachageVotesFromList24,
        CandidatePanachageVotesFromList33,
        CandidatePanachageVotesFromList44,
        CandidatePanachageVotesFromList55,
        CandidatePanachageVotesFromList56,
        CandidatePanachageVotesFromList57,
        CandidatePanachageVotesFromList58,
        CandidatePanachageVotesFromList70,
        CandidatePanachageVotesFromList71,
        CandidatePanachageVotesFromList77,
        CandidatePanachageVotesFromList999,
        CandidateParty,
        CandidateVotes,
        CandidateYearOfBirth,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityCounted,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
        ListConnection,
        ListConnectionParent,
        ListId,
        ListName,
        ListNumberOfMandates,
        ListPanachageVotesFromList01,
        ListPanachageVotesFromList02,
        ListPanachageVotesFromList03,
        ListPanachageVotesFromList04,
        ListPanachageVotesFromList05,
        ListPanachageVotesFromList06,
        ListPanachageVotesFromList07,
        ListPanachageVotesFromList08,
        ListPanachageVotesFromList11,
        ListPanachageVotesFromList12,
        ListPanachageVotesFromList13,
        ListPanachageVotesFromList14,
        ListPanachageVotesFromList16,
        ListPanachageVotesFromList17,
        ListPanachageVotesFromList18,
        ListPanachageVotesFromList22,
        ListPanachageVotesFromList23,
        ListPanachageVotesFromList24,
        ListPanachageVotesFromList33,
        ListPanachageVotesFromList44,
        ListPanachageVotesFromList55,
        ListPanachageVotesFromList56,
        ListPanachageVotesFromList57,
        ListPanachageVotesFromList58,
        ListPanachageVotesFromList70,
        ListPanachageVotesFromList71,
        ListPanachageVotesFromList77,
        ListPanachageVotesFromList999,
        ListVotes,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateGender => "candidate_gender",
                Field::CandidateId => "candidate_id",
                Field::CandidatePanachageVotesFromList01 => {
                    "candidate_panachage_votes_from_list_01"
                }
                Field::CandidatePanachageVotesFromList02 => {
                    "candidate_panachage_votes_from_list_02"
                }
                Field::CandidatePanachageVotesFromList03 => {
                    "candidate_panachage_votes_from_list_03"
                }
                Field::CandidatePanachageVotesFromList04 => {
                    "candidate_panachage_votes_from_list_04"
                }
                Field::CandidatePanachageVotesFromList05 => {
                    "candidate_panachage_votes_from_list_05"
                }
                Field::CandidatePanachageVotesFromList06 => {
                    "candidate_panachage_votes_from_list_06"
                }
                Field::CandidatePanachageVotesFromList07 => {
                    "candidate_panachage_votes_from_list_07"
                }
                Field::CandidatePanachageVotesFromList08 => {
                    "candidate_panachage_votes_from_list_08"
                }
                Field::CandidatePanachageVotesFromList11 => {
                    "candidate_panachage_votes_from_list_11"
                }
                Field::CandidatePanachageVotesFromList12 => {
                    "candidate_panachage_votes_from_list_12"
                }
                Field::CandidatePanachageVotesFromList13 => {
                    "candidate_panachage_votes_from_list_13"
                }
                Field::CandidatePanachageVotesFromList14 => {
                    "candidate_panachage_votes_from_list_14"
                }
                Field::CandidatePanachageVotesFromList16 => {
                    "candidate_panachage_votes_from_list_16"
                }
                Field::CandidatePanachageVotesFromList17 => {
                    "candidate_panachage_votes_from_list_17"
                }
                Field::CandidatePanachageVotesFromList18 => {
                    "candidate_panachage_votes_from_list_18"
                }
                Field::CandidatePanachageVotesFromList22 => {
                    "candidate_panachage_votes_from_list_22"
                }
                Field::CandidatePanachageVotesFromList23 => {
                    "candidate_panachage_votes_from_list_23"
                }
                Field::CandidatePanachageVotesFromList24 => {
                    "candidate_panachage_votes_from_list_24"
                }
                Field::CandidatePanachageVotesFromList33 => {
                    "candidate_panachage_votes_from_list_33"
                }
                Field::CandidatePanachageVotesFromList44 => {
                    "candidate_panachage_votes_from_list_44"
                }
                Field::CandidatePanachageVotesFromList55 => {
                    "candidate_panachage_votes_from_list_55"
                }
                Field::CandidatePanachageVotesFromList56 => {
                    "candidate_panachage_votes_from_list_56"
                }
                Field::CandidatePanachageVotesFromList57 => {
                    "candidate_panachage_votes_from_list_57"
                }
                Field::CandidatePanachageVotesFromList58 => {
                    "candidate_panachage_votes_from_list_58"
                }
                Field::CandidatePanachageVotesFromList70 => {
                    "candidate_panachage_votes_from_list_70"
                }
                Field::CandidatePanachageVotesFromList71 => {
                    "candidate_panachage_votes_from_list_71"
                }
                Field::CandidatePanachageVotesFromList77 => {
                    "candidate_panachage_votes_from_list_77"
                }
                Field::CandidatePanachageVotesFromList999 => {
                    "candidate_panachage_votes_from_list_999"
                }
                Field::CandidateParty => "candidate_party",
                Field::CandidateVotes => "candidate_votes",
                Field::CandidateYearOfBirth => "candidate_year_of_birth",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityCounted => "entity_counted",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
                Field::ListConnection => "list_connection",
                Field::ListConnectionParent => "list_connection_parent",
                Field::ListId => "list_id",
                Field::ListName => "list_name",
                Field::ListNumberOfMandates => "list_number_of_mandates",
                Field::ListPanachageVotesFromList01 => "list_panachage_votes_from_list_01",
                Field::ListPanachageVotesFromList02 => "list_panachage_votes_from_list_02",
                Field::ListPanachageVotesFromList03 => "list_panachage_votes_from_list_03",
                Field::ListPanachageVotesFromList04 => "list_panachage_votes_from_list_04",
                Field::ListPanachageVotesFromList05 => "list_panachage_votes_from_list_05",
                Field::ListPanachageVotesFromList06 => "list_panachage_votes_from_list_06",
                Field::ListPanachageVotesFromList07 => "list_panachage_votes_from_list_07",
                Field::ListPanachageVotesFromList08 => "list_panachage_votes_from_list_08",
                Field::ListPanachageVotesFromList11 => "list_panachage_votes_from_list_11",
                Field::ListPanachageVotesFromList12 => "list_panachage_votes_from_list_12",
                Field::ListPanachageVotesFromList13 => "list_panachage_votes_from_list_13",
                Field::ListPanachageVotesFromList14 => "list_panachage_votes_from_list_14",
                Field::ListPanachageVotesFromList16 => "list_panachage_votes_from_list_16",
                Field::ListPanachageVotesFromList17 => "list_panachage_votes_from_list_17",
                Field::ListPanachageVotesFromList18 => "list_panachage_votes_from_list_18",
                Field::ListPanachageVotesFromList22 => "list_panachage_votes_from_list_22",
                Field::ListPanachageVotesFromList23 => "list_panachage_votes_from_list_23",
                Field::ListPanachageVotesFromList24 => "list_panachage_votes_from_list_24",
                Field::ListPanachageVotesFromList33 => "list_panachage_votes_from_list_33",
                Field::ListPanachageVotesFromList44 => "list_panachage_votes_from_list_44",
                Field::ListPanachageVotesFromList55 => "list_panachage_votes_from_list_55",
                Field::ListPanachageVotesFromList56 => "list_panachage_votes_from_list_56",
                Field::ListPanachageVotesFromList57 => "list_panachage_votes_from_list_57",
                Field::ListPanachageVotesFromList58 => "list_panachage_votes_from_list_58",
                Field::ListPanachageVotesFromList70 => "list_panachage_votes_from_list_70",
                Field::ListPanachageVotesFromList71 => "list_panachage_votes_from_list_71",
                Field::ListPanachageVotesFromList77 => "list_panachage_votes_from_list_77",
                Field::ListPanachageVotesFromList999 => "list_panachage_votes_from_list_999",
                Field::ListVotes => "list_votes",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12270/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# St\u{e4}nderatswahlen 2023: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12280/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12280/</a>\n"]
#[doc = "<p>Kantonale Wahlen vom 22. Oktober 2023<br></p>"]
#[cfg(feature = "bl12280")]
pub mod staenderatswahlen_2023_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub candidate_elected: Option<i64>,
        pub candidate_family_name: Option<String>,
        pub candidate_first_name: Option<String>,
        pub candidate_id: Option<i64>,
        pub candidate_votes: Option<i64>,
        pub election_absolute_majority: Option<i64>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub election_date: Option<Date>,
        pub election_id: Option<String>,
        pub election_mandates: Option<i64>,
        pub election_status: Option<String>,
        /// election_title_de_CH
        pub election_title_de_ch: Option<String>,
        pub entity_accounted_ballots: Option<i64>,
        pub entity_accounted_votes: Option<i64>,
        pub entity_blank_ballots: Option<i64>,
        pub entity_blank_votes: Option<i64>,
        pub entity_counted: Option<i64>,
        pub entity_district: Option<String>,
        pub entity_eligible_voters: Option<i64>,
        pub entity_id: Option<i64>,
        pub entity_invalid_ballots: Option<i64>,
        pub entity_invalid_votes: Option<i64>,
        pub entity_name: Option<String>,
        pub entity_received_ballots: Option<i64>,
        pub entity_superregion: Option<String>,
        pub entity_unaccounted_ballots: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        CandidateElected,
        CandidateFamilyName,
        CandidateFirstName,
        CandidateId,
        CandidateVotes,
        ElectionAbsoluteMajority,
        ElectionDate,
        ElectionId,
        ElectionMandates,
        ElectionStatus,
        ElectionTitleDeCh,
        EntityAccountedBallots,
        EntityAccountedVotes,
        EntityBlankBallots,
        EntityBlankVotes,
        EntityCounted,
        EntityDistrict,
        EntityEligibleVoters,
        EntityId,
        EntityInvalidBallots,
        EntityInvalidVotes,
        EntityName,
        EntityReceivedBallots,
        EntitySuperregion,
        EntityUnaccountedBallots,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::CandidateElected => "candidate_elected",
                Field::CandidateFamilyName => "candidate_family_name",
                Field::CandidateFirstName => "candidate_first_name",
                Field::CandidateId => "candidate_id",
                Field::CandidateVotes => "candidate_votes",
                Field::ElectionAbsoluteMajority => "election_absolute_majority",
                Field::ElectionDate => "election_date",
                Field::ElectionId => "election_id",
                Field::ElectionMandates => "election_mandates",
                Field::ElectionStatus => "election_status",
                Field::ElectionTitleDeCh => "election_title_de_ch",
                Field::EntityAccountedBallots => "entity_accounted_ballots",
                Field::EntityAccountedVotes => "entity_accounted_votes",
                Field::EntityBlankBallots => "entity_blank_ballots",
                Field::EntityBlankVotes => "entity_blank_votes",
                Field::EntityCounted => "entity_counted",
                Field::EntityDistrict => "entity_district",
                Field::EntityEligibleVoters => "entity_eligible_voters",
                Field::EntityId => "entity_id",
                Field::EntityInvalidBallots => "entity_invalid_ballots",
                Field::EntityInvalidVotes => "entity_invalid_votes",
                Field::EntityName => "entity_name",
                Field::EntityReceivedBallots => "entity_received_ballots",
                Field::EntitySuperregion => "entity_superregion",
                Field::EntityUnaccountedBallots => "entity_unaccounted_ballots",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12280/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen: W\u{e4}hleranteil, Anzahl Kandidierende, Anzahl Listen, Anzahl Gew\u{e4}hlte nach Partei und Jahr (seit 1991)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12290/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12290/</a>\n"]
#[doc = "<p>Statistik der Wahlen und Abstimmungen<br></p>"]
#[cfg(feature = "bl12290")]
pub mod nationalratswahlen_waehleranteil_anzahl_kandidierende_anzahl_listen_anzahl_gewaehlte_nach_partei_und_jahr_seit_1991 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub wahl_jahr: Option<String>,
        pub partei_id: Option<i64>,
        pub partei_bezeichnung_de: Option<String>,
        pub partei_staerke: Option<f64>,
        pub anzahl_listen: Option<i64>,
        pub anzahl_kandidierende: Option<i64>,
        pub anzahl_kandidierende_f: Option<i64>,
        pub anzahl_kandidierende_m: Option<i64>,
        pub anzahl_gewaehlte: Option<i64>,
        pub anzahl_gewaehlte_f: Option<i64>,
        pub anzahl_gewaehlte_m: Option<i64>,
        pub frauen_anteil: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        WahlJahr,
        ParteiId,
        ParteiBezeichnungDe,
        ParteiStaerke,
        AnzahlListen,
        AnzahlKandidierende,
        AnzahlKandidierendeF,
        AnzahlKandidierendeM,
        AnzahlGewaehlte,
        AnzahlGewaehlteF,
        AnzahlGewaehlteM,
        FrauenAnteil,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::WahlJahr => "wahl_jahr",
                Field::ParteiId => "partei_id",
                Field::ParteiBezeichnungDe => "partei_bezeichnung_de",
                Field::ParteiStaerke => "partei_staerke",
                Field::AnzahlListen => "anzahl_listen",
                Field::AnzahlKandidierende => "anzahl_kandidierende",
                Field::AnzahlKandidierendeF => "anzahl_kandidierende_f",
                Field::AnzahlKandidierendeM => "anzahl_kandidierende_m",
                Field::AnzahlGewaehlte => "anzahl_gewaehlte",
                Field::AnzahlGewaehlteF => "anzahl_gewaehlte_f",
                Field::AnzahlGewaehlteM => "anzahl_gewaehlte_m",
                Field::FrauenAnteil => "frauen_anteil",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12290/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2023: Unver\u{e4}nderte und ver\u{e4}nderte Wahlzettel nach Liste und Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12300/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12300/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 22. Oktober 2023<br></p>"]
#[cfg(feature = "bl12300")]
pub mod nationalratswahlen_2023_unveraenderte_und_veraenderte_wahlzettel_nach_liste_und_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<i64>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Listen_Nr
        pub listen_nr: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
        /// Unveränderte_Wahlzettel_Liste
        pub unveranderte_wahlzettel_liste: Option<i64>,
        /// Veränderte_Wahlzettel_Liste
        pub veranderte_wahlzettel_liste: Option<i64>,
        /// Kandidatenstimmen_unveränderte_Wahlzettel
        pub kandidatenstimmen_unveranderte_wahlzettel: Option<i64>,
        /// Zusatzstimmen_unveränderte_Wahlzettel
        pub zusatzstimmen_unveranderte_wahlzettel: Option<i64>,
        /// Kandidatenstimmen_veränderte_Wahlzettel
        pub kandidatenstimmen_veranderte_wahlzettel: Option<i64>,
        /// Zusatzstimmen_veränderte_Wahlzettel
        pub zusatzstimmen_veranderte_wahlzettel: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsGemeindenummer,
        Gemeinde,
        ListenNr,
        Parteibezeichnung,
        UnveranderteWahlzettelListe,
        VeranderteWahlzettelListe,
        KandidatenstimmenUnveranderteWahlzettel,
        ZusatzstimmenUnveranderteWahlzettel,
        KandidatenstimmenVeranderteWahlzettel,
        ZusatzstimmenVeranderteWahlzettel,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::ListenNr => "listen_nr",
                Field::Parteibezeichnung => "parteibezeichnung",
                Field::UnveranderteWahlzettelListe => "unveranderte_wahlzettel_liste",
                Field::VeranderteWahlzettelListe => "veranderte_wahlzettel_liste",
                Field::KandidatenstimmenUnveranderteWahlzettel => {
                    "kandidatenstimmen_unveranderte_wahlzettel"
                }
                Field::ZusatzstimmenUnveranderteWahlzettel => {
                    "zusatzstimmen_unveranderte_wahlzettel"
                }
                Field::KandidatenstimmenVeranderteWahlzettel => {
                    "kandidatenstimmen_veranderte_wahlzettel"
                }
                Field::ZusatzstimmenVeranderteWahlzettel => "zusatzstimmen_veranderte_wahlzettel",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12300/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Nationalratswahlen 2023: Wahlberechtigte nach Geschlecht, briefliche Stimmabgaben, unver\u{e4}nderte und ver\u{e4}nderte Wahlzettel nach Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12310/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12310/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Wahlen vom 22. Oktober 2023<br></p>"]
#[cfg(feature = "bl12310")]
pub mod nationalratswahlen_2023_wahlberechtigte_nach_geschlecht_briefliche_stimmabgaben_unveraenderte_und_veraenderte_wahlzettel_nach_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<i64>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Stimmberechtigte
        pub stimmberechtigte: Option<i64>,
        /// Stimmberechtigte_Männer
        pub stimmberechtigte_manner: Option<i64>,
        /// Stimmberechtigte_Frauen
        pub stimmberechtigte_frauen: Option<i64>,
        /// Stimmberechtigte_Auslandschweizer
        pub stimmberechtigte_auslandschweizer: Option<i64>,
        /// Wahlzettel
        pub wahlzettel: Option<i64>,
        /// Briefliche_Stimmabgaben
        pub briefliche_stimmabgaben: Option<i64>,
        /// Ungültige_Wahlzettel
        pub ungultige_wahlzettel: Option<i64>,
        /// Leere_Wahlzettel
        pub leere_wahlzettel: Option<i64>,
        /// Unveränderte_Wahlzettel
        pub unveranderte_wahlzettel: Option<i64>,
        /// Veränderte_Wahlzettel_mit_Bezeichnung
        pub veranderte_wahlzettel_mit_bezeichnung: Option<i64>,
        /// Veränderte_Wahlzettel_ohne_Bezeichnung
        pub veranderte_wahlzettel_ohne_bezeichnung: Option<i64>,
        /// Leere_Stimmen
        pub leere_stimmen: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        BfsGemeindenummer,
        Gemeinde,
        Stimmberechtigte,
        StimmberechtigteManner,
        StimmberechtigteFrauen,
        StimmberechtigteAuslandschweizer,
        Wahlzettel,
        BrieflicheStimmabgaben,
        UngultigeWahlzettel,
        LeereWahlzettel,
        UnveranderteWahlzettel,
        VeranderteWahlzettelMitBezeichnung,
        VeranderteWahlzettelOhneBezeichnung,
        LeereStimmen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Stimmberechtigte => "stimmberechtigte",
                Field::StimmberechtigteManner => "stimmberechtigte_manner",
                Field::StimmberechtigteFrauen => "stimmberechtigte_frauen",
                Field::StimmberechtigteAuslandschweizer => "stimmberechtigte_auslandschweizer",
                Field::Wahlzettel => "wahlzettel",
                Field::BrieflicheStimmabgaben => "briefliche_stimmabgaben",
                Field::UngultigeWahlzettel => "ungultige_wahlzettel",
                Field::LeereWahlzettel => "leere_wahlzettel",
                Field::UnveranderteWahlzettel => "unveranderte_wahlzettel",
                Field::VeranderteWahlzettelMitBezeichnung => {
                    "veranderte_wahlzettel_mit_bezeichnung"
                }
                Field::VeranderteWahlzettelOhneBezeichnung => {
                    "veranderte_wahlzettel_ohne_bezeichnung"
                }
                Field::LeereStimmen => "leere_stimmen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12310/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung der Kantonsverfassung betreffend Einf\u{fc}hrung kantonaler Deponieabgaben (Massnahme des Massnahmenpakets zur F\u{f6}rderung des Baustoffkreislaufs Regio Basel)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12320/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12320/</a>\n"]
#[doc = "<p>Kantonale Abstimmung vom 19. November 2023<br></p>"]
#[cfg(feature = "bl12320")]
pub mod aenderung_der_kantonsverfassung_betreffend_einfuehrung_kantonaler_deponieabgaben_massnahme_des_massnahmenpakets_zur_foerderung_des_baustoffkreislaufs_regio_basel {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Abstimmungsdatum
        pub date: Option<String>,
        /// Gemeindenummer (BFS)
        pub entity_id: Option<String>,
        /// Gemeinde
        pub name: Option<String>,
        /// Stimmberechtigte total
        pub eligible_voters: Option<i64>,
        /// davon Stimmberechtigte Auslandschweizer/innen
        pub expats: Option<i64>,
        /// Leere Stimmen
        pub empty: Option<i64>,
        /// Ungültige Stimmen
        pub invalid: Option<i64>,
        /// Ja-Stimmen
        pub yeas: Option<i64>,
        /// Nein-Stimmen
        pub nays: Option<i64>,
        /// title_de_CH
        ///
        /// Abstimmungstitel
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12320/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung des Umweltschutzgesetzes Basel-Landschaft betreffend Einf\u{fc}hrung kantonaler Deponieabgaben (Massnahme des Massnahmenpakets zur F\u{f6}rderung des Baustoffkreislaufs Regio Basel)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12330/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12330/</a>\n"]
#[doc = "<p>Kantonale Abstimmung vom 19. November 2023<br></p>"]
#[cfg(feature = "bl12330")]
pub mod aenderung_des_umweltschutzgesetzes_basel_landschaft_betreffend_einfuehrung_kantonaler_deponieabgaben_massnahme_des_massnahmenpakets_zur_foerderung_des_baustoffkreislaufs_regio_basel {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Abstimmungsdatum
        pub date: Option<String>,
        /// Gemeindenummer (BFS)
        pub entity_id: Option<String>,
        /// Gemeinde
        pub name: Option<String>,
        /// Stimmberechtigte total
        pub eligible_voters: Option<i64>,
        /// davon Stimmberechtigte Auslandschweizer/innen
        pub expats: Option<i64>,
        /// Leere Stimmen
        pub empty: Option<i64>,
        /// Ungültige Stimmen
        pub invalid: Option<i64>,
        /// Ja-Stimmen
        pub yeas: Option<i64>,
        /// Nein-Stimmen
        pub nays: Option<i64>,
        /// Ja-Stimmen (%)
        pub yeas_percent: Option<f64>,
        /// Nein-Stimmen (%)
        pub nays_percent: Option<f64>,
        /// title_de_CH
        ///
        /// Abstimmungstitel
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        YeasPercent,
        NaysPercent,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::YeasPercent => "yeas_percent",
                Field::NaysPercent => "nays_percent",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12330/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Strompreise nach Netzbetreiber, Kategorie, Gemeinde und Jahr (seit 2018)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12340/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12340/</a>\n"]
#[doc = "<p>Die Eidgen\u{f6}ssische Elektrizit\u{e4}tskommission ElCom erfasst j\u{e4}hrlich die <a href=\"https://www.elcom.admin.ch/elcom/de/home/themen/strompreise/tarif-rohdaten-verteilnetzbetreiber.html\" target=\"_blank\">Basisdaten f\u{fc}r Tarife der Schweizer Verteilnetzbetreiber</a>.\u{a0}Diese Basisdaten beinhalten Informationen \u{fc}ber die Stromnetzbetreiber, die in den jeweiligen Gemeinden t\u{e4}tig sind, sowie Details zu deren Kostenmodellen und Angeboten.</p><p>Dieser Datensatz enth\u{e4}lt einen bereinigten Auszug der Basisdaten f\u{fc}r Tarife der Schweizer Verteilnetzbetreiber f\u{fc}r den Kanton Basel-Landschaft. Zus\u{e4}tzlich zu den Basisdaten enth\u{e4}lt dieser Datensatz eine Berechnung der monatlichen Kosten f\u{fc}r einen typischen Verbraucher der jeweiligen Verbrauchskategorie.</p><p>Hinweise:</p><ul><li>In diesem Datensatz sind lediglich die pro Verteilnetzbetreiber und Gemeinde geltenden Standardprodukte enthalten. Diese Standardprodukte entsprechen meist dem \u{f6}kologisch vertr\u{e4}glichsten Produkt des Betreibers.</li><li>Es existieren Gemeinden, in welchen mehrere Verteilnetzbetreiber aktiv sind.</li><li>Das Kostenmodell setzt sich aus einem monatlichen Grundtarif (fixe Kosten) und Arbeitspreis pro kWh (variable Kosten) zusammen. Die H\u{f6}he des Grundtarifs und der Arbeitspreis variiert je nach Verbrauchskategorie. Es existieren 15 verschiedene Verbrauchskategorien von privaten Kleinhaushalten, bis hin zu industriellen Grossbetrieben. Beschreibungen zu den Verbrauchskategorien sind im Datensatz enthalten.</li><li>Die variablen Kosten, angegeben in Rappen pro kWh, umfassen den Transportpreis f\u{fc}r den Strom, den Strompreis selbst sowie kommunale Geb\u{fc}hren und den Netzzuschlag.\u{a0}</li><li>Die fixen Kosten sind in Rappen pro Monat ausgewiesen.</li><li>Der typische Verbrauch in kWh pro Jahr ist ein zur Verbrauchskategorie geh\u{f6}riger Sch\u{e4}tzwert und gibt den ungef\u{e4}hren j\u{e4}hrlichen Verbrauch eines Verbrauchers in dieser Kategorie an.</li><li>Die typischen monatlichen Kosten ergeben sich aus dem typischen Verbrauch und dem Kostenmodell der jeweiligen Verbrauchskategorie. Die Berechnung lautet dabei wie folgt: Kosten_typischer_Verbrauch_Chf_pro_Monat = (Typischer_Verbrauch_kWh_pro_Jahr/12 * Variable_Kosten_Rp_pro_kWh + Fixe_Kosten_Rp_pro_Monat)/100<br/></li></ul>"]
#[cfg(feature = "bl12340")]
pub mod strompreise_nach_netzbetreiber_kategorie_gemeinde_und_jahr_seit_2018 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr
        pub jahr: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Verbrauchskategorie_Beschreibung
        pub verbrauchskategorie_beschreibung: Option<String>,
        /// Verbrauchskategorie
        pub verbrauchskategorie: Option<String>,
        /// Verteilnetzbetreiber
        pub verteilnetzbetreiber: Option<String>,
        /// UID_Verteilnetzbetreiber
        pub uid_verteilnetzbetreiber: Option<String>,
        /// Variable_Kosten_Rp_pro_kWh
        pub variable_kosten_rp_pro_kwh: Option<f64>,
        /// Fixe_Kosten_Rp_pro_Monat
        pub fixe_kosten_rp_pro_monat: Option<f64>,
        /// Typischer_Verbrauch_kWh_pro_Jahr
        pub typischer_verbrauch_kwh_pro_jahr: Option<i64>,
        /// Kosten_typischer_Verbrauch_CHF_pro_Monat
        pub kosten_typischer_verbrauch_chf_pro_monat: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Jahr,
        BfsGemeindenummer,
        Gemeinde,
        VerbrauchskategorieBeschreibung,
        Verbrauchskategorie,
        Verteilnetzbetreiber,
        UidVerteilnetzbetreiber,
        VariableKostenRpProKwh,
        FixeKostenRpProMonat,
        TypischerVerbrauchKwhProJahr,
        KostenTypischerVerbrauchChfProMonat,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Jahr => "jahr",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::VerbrauchskategorieBeschreibung => "verbrauchskategorie_beschreibung",
                Field::Verbrauchskategorie => "verbrauchskategorie",
                Field::Verteilnetzbetreiber => "verteilnetzbetreiber",
                Field::UidVerteilnetzbetreiber => "uid_verteilnetzbetreiber",
                Field::VariableKostenRpProKwh => "variable_kosten_rp_pro_kwh",
                Field::FixeKostenRpProMonat => "fixe_kosten_rp_pro_monat",
                Field::TypischerVerbrauchKwhProJahr => "typischer_verbrauch_kwh_pro_jahr",
                Field::KostenTypischerVerbrauchChfProMonat => {
                    "kosten_typischer_verbrauch_chf_pro_monat"
                }
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12340/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wahlen Gemeindekommissionen 2024: Anzahl Sitze, Wahlberechtigte und Wahlzettel nach Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12370/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12370/</a>\n"]
#[doc = "<p>Kommunale Wahlen vom 3. M\u{e4}rz 2024</p>"]
#[cfg(feature = "bl12370")]
pub mod wahlen_gemeindekommissionen_2024_anzahl_sitze_wahlberechtigte_und_wahlzettel_nach_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Wahlverfahren
        pub wahlverfahren: Option<String>,
        /// Stille Wahl
        pub stille_wahl: Option<String>,
        /// Anzahl Sitze
        pub anzahl_sitze: Option<i64>,
        /// Stimmberechtigte
        pub stimmberechtigte: Option<i64>,
        /// Wahlzettel
        pub wahlzettel: Option<i64>,
        /// Ungültige Wahlzettel
        pub ungultige_wahlzettel: Option<i64>,
        /// Leere Wahlzettel
        pub leere_wahlzettel: Option<i64>,
        /// Gültige Wahlzettel
        pub gultige_wahlzettel: Option<i64>,
        /// Ungültige Stimmen
        pub ungultige_stimmen: Option<String>,
        /// Leere Stimmen
        pub leere_stimmen: Option<String>,
        /// Gültige Stimmen
        pub gultige_stimmen: Option<i64>,
        /// Absolutes Mehr
        pub absolutes_mehr: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        Wahlverfahren,
        StilleWahl,
        AnzahlSitze,
        Stimmberechtigte,
        Wahlzettel,
        UngultigeWahlzettel,
        LeereWahlzettel,
        GultigeWahlzettel,
        UngultigeStimmen,
        LeereStimmen,
        GultigeStimmen,
        AbsolutesMehr,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Wahlverfahren => "wahlverfahren",
                Field::StilleWahl => "stille_wahl",
                Field::AnzahlSitze => "anzahl_sitze",
                Field::Stimmberechtigte => "stimmberechtigte",
                Field::Wahlzettel => "wahlzettel",
                Field::UngultigeWahlzettel => "ungultige_wahlzettel",
                Field::LeereWahlzettel => "leere_wahlzettel",
                Field::GultigeWahlzettel => "gultige_wahlzettel",
                Field::UngultigeStimmen => "ungultige_stimmen",
                Field::LeereStimmen => "leere_stimmen",
                Field::GultigeStimmen => "gultige_stimmen",
                Field::AbsolutesMehr => "absolutes_mehr",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12370/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wahlen Gemeindekommissionen 2024: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12380/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12380/</a>\n"]
#[doc = "<p>Kommunale Wahlen vom 3. M\u{e4}rz 2024\u{a0}(offiziell kandidierende Personen)</p>"]
#[cfg(feature = "bl12380")]
pub mod wahlen_gemeindekommissionen_2024_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Kandidaten-Nr
        pub kandidaten_nr: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Stimmen
        pub stimmen: Option<i64>,
        /// Gewählt
        pub gewahlt: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        KandidatenNr,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        Bisher,
        Stimmen,
        Gewahlt,
        Parteibezeichnung,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::KandidatenNr => "kandidaten_nr",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::Bisher => "bisher",
                Field::Stimmen => "stimmen",
                Field::Gewahlt => "gewahlt",
                Field::Parteibezeichnung => "parteibezeichnung",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12380/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Volksinitiative vom 28. Mai 2021 \u{ab}F\u{fc}r ein besseres Leben im Alter (Initiative f\u{fc}r eine 13. AHV-Rente)\u{bb}"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12390/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12390/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 3. M\u{e4}rz 2024<br></p>"]
#[cfg(feature = "bl12390")]
pub mod volksinitiative_vom_28_mai_2021_fuer_ein_besseres_leben_im_alter_initiative_fuer_eine_13_ahv_rente {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub answer: Option<String>,
        pub counted: Option<i64>,
        pub date: Option<String>,
        pub district: Option<String>,
        /// domain
        pub domain0: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub expats: Option<i64>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        pub id: Option<String>,
        pub shortcode: Option<String>,
        pub status: Option<String>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        /// type
        pub r#type: Option<String>,
        pub ballot_answer: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Answer,
        Counted,
        Date,
        District,
        Domain0,
        EntityId,
        Name,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        Id,
        Shortcode,
        Status,
        TitleDeCh,
        RType,
        BallotAnswer,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Answer => "answer",
                Field::Counted => "counted",
                Field::Date => "date",
                Field::District => "district",
                Field::Domain0 => "domain0",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::Id => "id",
                Field::Shortcode => "shortcode",
                Field::Status => "status",
                Field::TitleDeCh => "title_de_ch",
                Field::RType => "type",
                Field::BallotAnswer => "ballot_answer",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12390/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Volksinitiative vom 16. Juli 2021 \u{ab}F\u{fc}r eine sichere und nachhaltige Altersvorsorge (Renteninitiative)\u{bb}"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12400/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12400/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 3. M\u{e4}rz 2024<br></p>"]
#[cfg(feature = "bl12400")]
pub mod volksinitiative_vom_16_juli_2021_fuer_eine_sichere_und_nachhaltige_altersvorsorge_renteninitiative {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub answer: Option<String>,
        pub counted: Option<i64>,
        pub date: Option<String>,
        pub district: Option<String>,
        /// domain
        pub domain0: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub expats: Option<i64>,
        pub empty: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        pub id: Option<String>,
        pub shortcode: Option<String>,
        pub status: Option<String>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        /// type
        pub r#type: Option<String>,
        pub ballot_answer: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Answer,
        Counted,
        Date,
        District,
        Domain0,
        EntityId,
        Name,
        EligibleVoters,
        Expats,
        Empty,
        Invalid,
        Yeas,
        Nays,
        Id,
        Shortcode,
        Status,
        TitleDeCh,
        RType,
        BallotAnswer,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Answer => "answer",
                Field::Counted => "counted",
                Field::Date => "date",
                Field::District => "district",
                Field::Domain0 => "domain0",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Expats => "expats",
                Field::Empty => "empty",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::Id => "id",
                Field::Shortcode => "shortcode",
                Field::Status => "status",
                Field::TitleDeCh => "title_de_ch",
                Field::RType => "type",
                Field::BallotAnswer => "ballot_answer",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12400/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Motorfahrzeugbestand nach Fahrzeugart, Treibstoff, Gemeinde und Monat (seit Mai 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12410/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12410/</a>\n"]
#[doc = "<p>Motorfahrzeugstatistik (Motorfahrzeuge mit Nummernschild BL). Anzahl Fahrzeuge per Ende Monat.</p><p>Eine Auflistung der verschiedenen Fahrzeugarten befindet sich in den\u{a0}<a href=\"https://www.baselland.ch/politik-und-behorden/direktionen/sicherheitsdirektion/motorfahrzeugkontrolle/fahrzeuge-und-kontrollschilder/vorfuhr/intervalle-der-verschiedenen-fahrzeugarten?searchterm=fahrzeugarten\" target=\"_blank\">Intervallen der verschiedenen Fahrzeugarten</a>. Beispielsweise k\u{f6}nnen vorf\u{fc}hrpflichtige E-Bikes als Mofas mit Elektro-Treibstoff ausgelesen werden.</p>"]
#[cfg(feature = "bl12410")]
pub mod motorfahrzeugbestand_nach_fahrzeugart_treibstoff_gemeinde_und_monat_seit_mai_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Jahr_Monat
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub jahr_monat: Option<Date>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Fahrzeugart
        pub fahrzeugart: Option<String>,
        /// Treibstoff
        pub treibstoff: Option<String>,
        /// Anzahl
        pub anzahl: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        JahrMonat,
        BfsGemeindenummer,
        Gemeinde,
        Fahrzeugart,
        Treibstoff,
        Anzahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::JahrMonat => "jahr_monat",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Fahrzeugart => "fahrzeugart",
                Field::Treibstoff => "treibstoff",
                Field::Anzahl => "anzahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12410/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeinderatsnachwahlen 2024: Anzahl Sitze, Wahlberechtigte und Wahlzettel nach Gemeinde"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12420/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12420/</a>\n"]
#[doc = "<p>Kommunale Nachwahlen vom 14. April 2024, 9. Juni 2024 und\u{a0}22. September 2024</p><p>Quellen: Landeskanzlei BL / Wahlb\u{fc}ros der Gemeinden</p><p>Keine Angaben (...) zu Stimmberechtigten, Wahlzetteln und Stimmen bei stillen Wahlen</p>"]
#[cfg(feature = "bl12420")]
pub mod gemeinderatsnachwahlen_2024_anzahl_sitze_wahlberechtigte_und_wahlzettel_nach_gemeinde {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Wahlverfahren
        pub wahlverfahren: Option<String>,
        /// Stille Wahl
        pub stille_wahl: Option<String>,
        /// Anzahl Sitze
        pub anzahl_sitze: Option<i64>,
        /// Stimmberechtigte
        pub stimmberechtigte: Option<String>,
        /// Abgegebene Wahlzettel
        pub abgegebene_wahlzettel: Option<String>,
        /// Leere Wahlzettel
        pub leere_wahlzettel: Option<String>,
        /// Ungültige Wahlzettel
        pub ungultige_wahlzettel: Option<String>,
        /// Gültige Wahlzettel
        pub gultige_wahlzettel: Option<String>,
        /// Leere Stimmen
        pub leere_stimmen: Option<String>,
        /// Ungültige Stimmen
        pub ungultige_stimmen: Option<String>,
        /// Gültige Stimmen
        pub gultige_stimmen: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        Wahlverfahren,
        StilleWahl,
        AnzahlSitze,
        Stimmberechtigte,
        AbgegebeneWahlzettel,
        LeereWahlzettel,
        UngultigeWahlzettel,
        GultigeWahlzettel,
        LeereStimmen,
        UngultigeStimmen,
        GultigeStimmen,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Wahlverfahren => "wahlverfahren",
                Field::StilleWahl => "stille_wahl",
                Field::AnzahlSitze => "anzahl_sitze",
                Field::Stimmberechtigte => "stimmberechtigte",
                Field::AbgegebeneWahlzettel => "abgegebene_wahlzettel",
                Field::LeereWahlzettel => "leere_wahlzettel",
                Field::UngultigeWahlzettel => "ungultige_wahlzettel",
                Field::GultigeWahlzettel => "gultige_wahlzettel",
                Field::LeereStimmen => "leere_stimmen",
                Field::UngultigeStimmen => "ungultige_stimmen",
                Field::GultigeStimmen => "gultige_stimmen",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12420/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Gemeinderatsnachwahlen 2024: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12430/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12430/</a>\n"]
#[doc = "<p>Kommunale Nachwahlen vom 14. April 2024, 9. Juni 2024 und\u{a0}22. September 2024 (offiziell kandidierende Personen)</p><p>Quellen: Landeskanzlei BL / Wahlb\u{fc}ros der Gemeinden</p><p>Keine Angaben (...) zur Stimmenzahl bei stillen Wahlen</p><p>Teilweise fehlende Angaben (...) zu Kandidaten-Nr., Jahrgang und Parteizugeh\u{f6}rigkeit</p>"]
#[cfg(feature = "bl12430")]
pub mod gemeinderatsnachwahlen_2024_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Kandidaten-Nr
        pub kandidaten_nr: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Anzahl Stimmen
        pub anzahl_stimmen: Option<String>,
        /// Gewählt
        pub gewahlt: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        KandidatenNr,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        Bisher,
        AnzahlStimmen,
        Gewahlt,
        Parteibezeichnung,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::KandidatenNr => "kandidaten_nr",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::Bisher => "bisher",
                Field::AnzahlStimmen => "anzahl_stimmen",
                Field::Gewahlt => "gewahlt",
                Field::Parteibezeichnung => "parteibezeichnung",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12430/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# OGD-Portal: T\u{e4}gliche Nutzung (seit Januar 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12440/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12440/</a>\n"]
#[doc = "<p class=\"\">Die Daten \u{fc}ber die Nutzung des OGD-Portals BL (data.bl.ch) werden von der Fach- und Koordinationsstelle OGD BL erhoben und ver\u{f6}ffentlicht.</p><p class=\"\"><b>Spalten</b></p><ul><li><b>Datum</b>: Enth\u{e4}lt den Tag, an dem die Nutzung gemessen wurde.</li><li><b>Visitors</b>: Gibt die Anzahl der t\u{e4}glichen Besucher/innen des OGD-Portals an. Die Erfassung der Besucher/innen erfolgt durch Z\u{e4}hlen der einzigartigen (unique) IP-Adressen, die am Erhebungstag Zugriffe verzeichneten. Die IP-Adresse repr\u{e4}sentiert die Netzwerkadresse des Ger\u{e4}ts, von dem aus der Zugriff auf das Portal erfolgte.</li><li><b>Datensatzinteraktionen</b>: Umfasst alle Interaktionen mit einem beliebigen Datensatz auf data.bl.ch. Ein/e Besucher/in kann mehrere Interaktionen ausl\u{f6}sen. Zu den Interaktionen z\u{e4}hlen Klicks auf der Webseite (Durchsuchen von Datens\u{e4}tzen, Filtern, usw.) sowie API-Aufrufe (Herunterladen eines Datensatzes als JSON-Datei, usw.).</li></ul><p class=\"\"><b>Bemerkungen</b></p><ul><li>Nur Aufrufe von \u{f6}ffentlich zug\u{e4}nglichen Datens\u{e4}tzen werden ausgewiesen.</li><li>IP-Adressen sowie Interaktionen von Nutzenden mit einem Login des Kantons Basel-Landschaft \u{2013} insbesondere von Mitarbeitenden der Fach- und Koordinationsstelle OGD \u{2013} werden vor der Ver\u{f6}ffentlichung aus dem Datensatz entfernt und somit nicht ausgewiesen.</li><li>Aufrufe von Akteuren, welche durch den User-Agent header eindeutig als Bots erkennbar sind, werden ebenfalls nicht ausgewiesen.</li><li>Aufgrund von Synchronisationsproblemen k\u{f6}nnen Daten tageweise\u{a0}fehlen</li></ul>"]
#[cfg(feature = "bl12440")]
pub mod ogd_portal_taegliche_nutzung_seit_januar_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub date: Option<Date>,
        /// Visitors
        ///
        /// Unique IPs
        pub unique_ip_count: Option<i64>,
        /// Interactions
        ///
        /// API calls auf Datensätze
        pub api_calls_count: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        UniqueIpCount,
        ApiCallsCount,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::UniqueIpCount => "unique_ip_count",
                Field::ApiCallsCount => "api_calls_count",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12440/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Luftqualit\u{e4}t Station Sissach-B\u{fc}tzenen (halbst\u{fc}ndliche Messdaten seit Januar 2020)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12450/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12450/</a>\n"]
#[doc = "<p class=\"\">Echtzeitdaten der Luftmessstation Sissach-B\u{fc}tzenen. Die Messwerte werden halbst\u{fc}ndlich ausgewiesen und st\u{fc}ndlich (jeweils 20 Minuten nach der vollen Stunde mit einer m\u{f6}glichen Latenz von 1 bis 4 Stunden) aktualisiert. Die ausgewiesenen Werte werden unbereinigt von der Messstation bezogen. Validierte Messwerte sind direkt vom Lufthygieneamt beider Basel zu beziehen.<br></p><p class=\"\">Aufgrund messtechnischer Ungenauigkeiten k\u{f6}nnen bei geringer Konzentration eines Schadstoffs auch Negativwerte auftreten (Nullpunktrauschen). Die Handhabung negativer Messwerte ist in der aktuell g\u{fc}ltigen\u{a0}<a href=\"https://www.bafu.admin.ch/bafu/de/home/themen/luft/publikationen-studien/publikationen/immissionsmessung-von-luftfremdstoffen.html\" style=\"background-color: rgb(255, 255, 255); font-family: sans-serif; font-size: 14px; font-weight: 400;\" target=\"_blank\">Imissionsmessempfehlung\u{a0}</a>2021 beschrieben.</p><p class=\"\">Ein Datensatz der Messwerte zwischen 2018 und 2019 kann\u{a0}<a href=\"https://fkd-sta-files.bl.ch/ogd/luftqualitaet/airmet_sissach_buetzenen_2018_2019.csv\" style=\"background-color: rgb(255, 255, 255); font-family: sans-serif; font-size: 14px; font-weight: 400;\" target=\"_blank\">mit diesem Link</a>\u{a0}bezogen werden.<br></p><p class=\"\"><b>Ausgewiesene Werte</b></p><ul><li>Anfangszeit: Zeitstempel des Beginns der halbst\u{fc}ndlichen Messung im Format %Y-%m-%dT%H:%M:%S</li><li>Lungeng\u{e4}ngiger Feinstaub PM10 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM10 in Mikrogramm pro Kubikmeter.</li><li>Lungeng\u{e4}ngiger Feinstaub PM2.5 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM2.5 in Mikrogramm pro Kubikmeter.</li><li>Stickstoffdioxid NO2 (\u{b5}g/m3): Gemessene Stickstoffdioxid-Konzentration in Mikrogramm pro Kubikmeter.</li><li>Ozon O3 (\u{b5}g/m3): Gemessene Ozon-Konzentration in Mikrogramm pro Kubikmeter.</li></ul><p class=\"\"><b>Standortbeschreibung</b></p><p class=\"\">Die Messstation liegt in einem Wohnquartier von Sissach, direkt neben einer Schule. Sie misst den kleinst\u{e4}dtischen Hintergrund.</p><p class=\"\"><b>Lage</b></p><p class=\"\">Kleinst\u{e4}dtisch/Vorst\u{e4}dtisch, Hintergrund</p><p class=\"\"><b>Koordinaten</b></p><p class=\"\">2628410 / 1257208; 327 m \u{fc}. M.</p><p class=\"\"><b>Bebauung</b></p><p class=\"\">Offene Bebauung</p>"]
#[cfg(feature = "bl12450")]
pub mod luftqualitaet_station_sissach_buetzenen_halbstuendliche_messdaten_seit_januar_2020 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Anfangszeit
        #[serde(with = "time::serde::iso8601::option")]
        pub anfangszeit: Option<OffsetDateTime>,
        /// PM10
        ///
        /// Lungengängiger Feinstaub PM10
        pub pm10: Option<f64>,
        /// PM2.5
        ///
        /// Lungengängiger Feinstaub PM2.5
        pub pm2_5: Option<f64>,
        /// NO2
        ///
        /// Stickstoffdioxid
        pub no2: Option<f64>,
        /// O3
        ///
        /// Ozon
        pub o3: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Anfangszeit,
        Pm10,
        Pm25,
        No2,
        O3,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Anfangszeit => "anfangszeit",
                Field::Pm10 => "pm10",
                Field::Pm25 => "pm2_5",
                Field::No2 => "no2",
                Field::O3 => "o3",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12450/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Firmenmutationen nach Rechtsform, NOGA-Einteilung und Gemeinde (seit Februar 2016)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12460/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12460/</a>\n"]
#[doc = "<p>T\u{e4}gliche Meldungen aus dem Schweizerischen Handelsamtsblatt (SHAB).\u{a0}Eingetragen und im\u{a0}SHAB ver\u{f6}ffentlicht werden rechtlich verbindliche Tatsachen vorab bei privaten Rechtssubjekten.</p><p>Allgemeine Systematik der Wirtschaftszweige (NOGA)</p><ul><li>Die NOGA-Codes und -Labels stammen aus dem <a href=\"https://www.bfs.admin.ch/bfs/de/home/register/unternehmensregister/betriebs-unternehmensregister.html\" target=\"_blank\">Betriebs- und Unternehmensregister der Schweiz</a> und werden \u{fc}ber die entsprechende Schnittstelle (<a href=\"https://www.bfs.admin.ch/bfs/de/home/register/unternehmensregister/betriebs-unternehmensregister/burweb.html#-2080172010\" target=\"_blank\">BurWeb API</a>) abgefragt.</li><li>Angaben teilweise fehlend</li><li>Nach\u{a0}<a href=\"https://www.kubb-tool.bfs.admin.ch/de/search\" target=\"_blank\">NOGA-Code</a>\u{a0}suchen</li></ul>"]
#[cfg(feature = "bl12460")]
pub mod firmenmutationen_nach_rechtsform_noga_einteilung_und_gemeinde_seit_februar_2016 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Kategorie
        pub kategorie: Option<String>,
        /// Publikationsdatum im Schweizerischen Handelsamtsblatt
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub publikationsdatum_shab: Option<Date>,
        /// Journaldatum im Handelsregister BL
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub journaldatum_handelsregister: Option<Date>,
        /// Nummer im Schweizerischen Handelsamtsblatt
        pub id_shab: Option<i64>,
        /// BFS-Gemeindenummer des Firmensitzes zum Zeitpunkt der Mutation
        pub firmensitz_code: Option<String>,
        /// Gemeinde des Firmensitzes zum Zeitpunkt der Mutation
        pub firmensitz: Option<String>,
        /// Meldung
        pub meldung: Option<String>,
        /// Unternehmens-Identifikationsnummer
        pub uid: Option<String>,
        /// Name der Firma
        pub firmenname: Option<String>,
        /// Code der Rechtsform
        pub rechtsform_code: Option<String>,
        /// Rechtsform
        pub rechtsform: Option<String>,
        /// 6-stelliger NOGA-Code
        pub noga_code: Option<String>,
        /// NOGA-Art
        pub noga: Option<String>,
        /// 1-stelliger NOGA-Code
        pub noga_abschnitt_code: Option<String>,
        /// NOGA-Abschnitt
        pub noga_abschnitt: Option<String>,
        /// NOGA-Abteilung
        pub noga_abteilung: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Kategorie,
        PublikationsdatumShab,
        JournaldatumHandelsregister,
        IdShab,
        FirmensitzCode,
        Firmensitz,
        Meldung,
        Uid,
        Firmenname,
        RechtsformCode,
        Rechtsform,
        NogaCode,
        Noga,
        NogaAbschnittCode,
        NogaAbschnitt,
        NogaAbteilung,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Kategorie => "kategorie",
                Field::PublikationsdatumShab => "publikationsdatum_shab",
                Field::JournaldatumHandelsregister => "journaldatum_handelsregister",
                Field::IdShab => "id_shab",
                Field::FirmensitzCode => "firmensitz_code",
                Field::Firmensitz => "firmensitz",
                Field::Meldung => "meldung",
                Field::Uid => "uid",
                Field::Firmenname => "firmenname",
                Field::RechtsformCode => "rechtsform_code",
                Field::Rechtsform => "rechtsform",
                Field::NogaCode => "noga_code",
                Field::Noga => "noga",
                Field::NogaAbschnittCode => "noga_abschnitt_code",
                Field::NogaAbschnitt => "noga_abschnitt",
                Field::NogaAbteilung => "noga_abteilung",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12460/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Sitzverlegungen und Domizil\u{e4}nderungen von Firmen nach Rechtsform, NOGA-Einteilung und Gemeinde (seit Februar 2016)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12470/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12470/</a>\n"]
#[doc = "<p>T\u{e4}gliche Adress\u{e4}nderungen aus dem Schweizerischen Handelsamtsblatt (SHAB). Eingetragen und im\u{a0}SHAB ver\u{f6}ffentlicht werden rechtlich verbindliche Tatsachen vorab bei privaten Rechtssubjekten.</p><p>Allgemeine Systematik der Wirtschaftszweige (NOGA)</p><ul><li>Die NOGA-Codes und -Labels stammen aus dem <a href=\"https://www.bfs.admin.ch/bfs/de/home/register/unternehmensregister/betriebs-unternehmensregister.html\" target=\"_blank\">Betriebs- und Unternehmensregister der Schweiz</a> und werden \u{fc}ber die entsprechende Schnittstelle (<a href=\"https://www.bfs.admin.ch/bfs/de/home/register/unternehmensregister/betriebs-unternehmensregister/burweb.html\" target=\"_blank\">BurWeb API</a>) abgefragt.</li><li>Angaben teilweise fehlend</li><li>Nach\u{a0}<a href=\"https://www.kubb-tool.bfs.admin.ch/de/search\" target=\"_blank\">NOGA-Code</a>\u{a0}suchen</li></ul>"]
#[cfg(feature = "bl12470")]
pub mod sitzverlegungen_und_domizilaenderungen_von_firmen_nach_rechtsform_noga_einteilung_und_gemeinde_seit_februar_2016 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Kategorie
        pub kategorie: Option<String>,
        /// Publikationsdatum im Schweizerischen Handelsamtsblatt
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub publikationsdatum_shab: Option<Date>,
        /// Journaldatum im Handelsregister BL
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub journaldatum_handelsregister: Option<Date>,
        /// Nummer im Schweizerischen Handelsamtsblatt
        pub id_shab: Option<i64>,
        /// Aktuelle BFS-Gemeindenummer des Firmensitzes
        pub firmensitz_neu_code: Option<String>,
        /// Aktuelle Gemeinde des Firmensitzes
        pub firmensitz_neu: Option<String>,
        /// Aktueller Kanton
        pub firmensitz_neu_canton: Option<String>,
        /// Frühere BFS-Gemeindenummer des Firmensitzes
        pub firmensitz_bisher_code: Option<String>,
        /// Frühere Gemeinde des Firmensitzes
        pub firmensitz_bisher: Option<String>,
        /// Früherer Kanton
        pub firmensitz_bisher_canton: Option<String>,
        /// Meldung
        pub meldung: Option<String>,
        /// Unternehmens-Identifikationsnummer
        pub uid: Option<String>,
        /// Name der Firma
        pub firmenname: Option<String>,
        /// Code der Rechtsform
        pub rechtsform_code: Option<String>,
        /// Rechtsform
        pub rechtsform: Option<String>,
        /// 6-stelliger NOGA-Code
        pub noga_code: Option<String>,
        /// NOGA-Art
        pub noga: Option<String>,
        /// 1-stelliger NOGA-Code
        pub noga_abschnitt_code: Option<String>,
        /// NOGA-Abschnitt
        pub noga_abschnitt: Option<String>,
        /// NOGA-Abteilung
        pub noga_abteilung: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Kategorie,
        PublikationsdatumShab,
        JournaldatumHandelsregister,
        IdShab,
        FirmensitzNeuCode,
        FirmensitzNeu,
        FirmensitzNeuCanton,
        FirmensitzBisherCode,
        FirmensitzBisher,
        FirmensitzBisherCanton,
        Meldung,
        Uid,
        Firmenname,
        RechtsformCode,
        Rechtsform,
        NogaCode,
        Noga,
        NogaAbschnittCode,
        NogaAbschnitt,
        NogaAbteilung,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Kategorie => "kategorie",
                Field::PublikationsdatumShab => "publikationsdatum_shab",
                Field::JournaldatumHandelsregister => "journaldatum_handelsregister",
                Field::IdShab => "id_shab",
                Field::FirmensitzNeuCode => "firmensitz_neu_code",
                Field::FirmensitzNeu => "firmensitz_neu",
                Field::FirmensitzNeuCanton => "firmensitz_neu_canton",
                Field::FirmensitzBisherCode => "firmensitz_bisher_code",
                Field::FirmensitzBisher => "firmensitz_bisher",
                Field::FirmensitzBisherCanton => "firmensitz_bisher_canton",
                Field::Meldung => "meldung",
                Field::Uid => "uid",
                Field::Firmenname => "firmenname",
                Field::RechtsformCode => "rechtsform_code",
                Field::Rechtsform => "rechtsform",
                Field::NogaCode => "noga_code",
                Field::Noga => "noga",
                Field::NogaAbschnittCode => "noga_abschnitt_code",
                Field::NogaAbschnitt => "noga_abschnitt",
                Field::NogaAbteilung => "noga_abteilung",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12470/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Firmen nach Zweck, Rechtsform, NOGA-Einteilung und Standort"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12480/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12480/</a>\n"]
#[doc = "<p>Zentraler Firmenindex (Zefix). Tagesaktueller Auszug der im Handelsregister eingetragenen Rechtseinheiten.</p><p style=\"font-family: sans-serif;\">Die Daten werden t\u{e4}glich \u{fc}ber die <a href=\"https://www.zefix.admin.ch/ZefixPublicREST/swagger-ui/index.html\" target=\"_blank\">Zefix PublicREST API</a> abgefragt.</p><p style=\"font-family: sans-serif;\">Status</p><ul><li>Active = aktiv</li><li>Being cancelled = in Liquidation</li></ul><p style=\"font-family: sans-serif;\">Zweck</p><ul><li style=\"font-family: sans-serif;\">Angaben teilweise fehlend (z. B. bei Zweigniederlassungen)</li></ul><p style=\"font-family: sans-serif;\">Allgemeine Systematik der Wirtschaftszweige (NOGA)</p><ul><li>Die NOGA-Codes und -Labels stammen aus dem\u{a0}<a href=\"https://www.bfs.admin.ch/bfs/de/home/register/unternehmensregister/betriebs-unternehmensregister.html\" target=\"_blank\">Betriebs- und Unternehmensregister der Schweiz</a>\u{a0}und werden \u{fc}ber die entsprechende Schnittstelle (<a href=\"https://www.bfs.admin.ch/bfs/de/home/register/unternehmensregister/betriebs-unternehmensregister/burweb.html#-2080172010\" target=\"_blank\">BurWeb API</a>) abgefragt.</li><li>Angaben teilweise fehlend</li><li>Nach <a href=\"https://www.kubb-tool.bfs.admin.ch/de/search\" target=\"_blank\">NOGA-Code</a> suchen</li></ul><p>Geolokalisierungsmethoden</p><ul><li>Treffer im kGWR: Die Adresse der Firma ist im <a href=\"https://data.bl.ch/explore/dataset/12180\" target=\"_blank\">kantonalen Geb\u{e4}ude- und Wohnungsregister</a> erfasst.</li><li>Treffer im kGWR mit angepasster Adresse: Die Adresse der Firma ist mit unterschiedlicher Strassennummer, Strassennamen, Postleitzahl oder Gemeindename im <a href=\"https://data.bl.ch/explore/dataset/12180\" target=\"_blank\">kantonalen Geb\u{e4}ude- und Wohnungsregister</a> erfasst, konnte aber dennoch lokalisiert werden.</li><li>Kein Treffer im kGWR: Die Adresse der Firma konnte keinem Eintrag im\u{a0}<a href=\"https://data.bl.ch/explore/dataset/12180\" style=\"background-color: rgb(255, 255, 255); font-family: sans-serif; font-size: 14px; font-weight: 400;\" target=\"_blank\">kantonalen Geb\u{e4}ude- und Wohnungsregister</a>\u{a0}zugewiesen werden.<br></li></ul>"]
#[cfg(feature = "bl12480")]
pub mod firmen_nach_zweck_rechtsform_noga_einteilung_und_standort {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub firmensitz_code: Option<String>,
        pub firmensitz: Option<String>,
        pub uid: Option<String>,
        pub firmenname: Option<String>,
        pub zusatz: Option<String>,
        pub strassenbezeichnung: Option<String>,
        pub eingangsnummer_gebaeude: Option<String>,
        pub postleitzahl: Option<String>,
        pub ort: Option<String>,
        pub firmensitz_bezirk_nr: Option<String>,
        pub firmensitz_bezirk: Option<String>,
        pub status: Option<String>,
        pub zweck: Option<String>,
        pub rechtsform_code: Option<String>,
        pub rechtsform: Option<String>,
        pub noga_code: Option<String>,
        pub noga: Option<String>,
        pub noga_abschnitt_code: Option<String>,
        pub noga_abschnitt: Option<String>,
        pub noga_abteilung: Option<String>,
        pub kantonaler_auszug_link: Option<String>,
        pub egid: Option<i64>,
        pub e_eingangskoordinate: Option<f64>,
        pub n_eingangskoordinate: Option<f64>,
        pub koordinaten: Option<GeoPoint2d>,
        pub lokalisierungsmethode: Option<String>,
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub datum: Option<Date>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        FirmensitzCode,
        Firmensitz,
        Uid,
        Firmenname,
        Zusatz,
        Strassenbezeichnung,
        EingangsnummerGebaeude,
        Postleitzahl,
        Ort,
        FirmensitzBezirkNr,
        FirmensitzBezirk,
        Status,
        Zweck,
        RechtsformCode,
        Rechtsform,
        NogaCode,
        Noga,
        NogaAbschnittCode,
        NogaAbschnitt,
        NogaAbteilung,
        KantonalerAuszugLink,
        Egid,
        EEingangskoordinate,
        NEingangskoordinate,
        Lokalisierungsmethode,
        Datum,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::FirmensitzCode => "firmensitz_code",
                Field::Firmensitz => "firmensitz",
                Field::Uid => "uid",
                Field::Firmenname => "firmenname",
                Field::Zusatz => "zusatz",
                Field::Strassenbezeichnung => "strassenbezeichnung",
                Field::EingangsnummerGebaeude => "eingangsnummer_gebaeude",
                Field::Postleitzahl => "postleitzahl",
                Field::Ort => "ort",
                Field::FirmensitzBezirkNr => "firmensitz_bezirk_nr",
                Field::FirmensitzBezirk => "firmensitz_bezirk",
                Field::Status => "status",
                Field::Zweck => "zweck",
                Field::RechtsformCode => "rechtsform_code",
                Field::Rechtsform => "rechtsform",
                Field::NogaCode => "noga_code",
                Field::Noga => "noga",
                Field::NogaAbschnittCode => "noga_abschnitt_code",
                Field::NogaAbschnitt => "noga_abschnitt",
                Field::NogaAbteilung => "noga_abteilung",
                Field::KantonalerAuszugLink => "kantonaler_auszug_link",
                Field::Egid => "egid",
                Field::EEingangskoordinate => "e_eingangskoordinate",
                Field::NEingangskoordinate => "n_eingangskoordinate",
                Field::Lokalisierungsmethode => "lokalisierungsmethode",
                Field::Datum => "datum",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12480/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Wahlen Gemeindepr\u{e4}sidien 2024: Kandidierendenresultate"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12490/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12490/</a>\n"]
#[doc = "<p>Kommunale Wahlen vom 9. Juni 2024\u{a0}(offiziell kandidierende Personen)</p><p>Quellen: Landeskanzlei BL / Wahlb\u{fc}ros der Gemeinden / Websites der Gemeinden<br></p><p>Keine Angaben (...) zur Stimmenzahl bei stillen Wahlen</p><p>Teilweise fehlende Angaben (...) zu Kandidierenden, Jahrgang und Parteizugeh\u{f6}rigkeit</p>"]
#[cfg(feature = "bl12490")]
pub mod wahlen_gemeindepraesidien_2024_kandidierendenresultate {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Wahlbezeichnung
        pub wahlbezeichnung: Option<String>,
        /// BFS_Gemeindenummer
        pub bfs_gemeindenummer: Option<String>,
        /// Gemeinde
        pub gemeinde: Option<String>,
        /// Name
        pub name: Option<String>,
        /// Vorname
        pub vorname: Option<String>,
        /// Geschlecht
        pub geschlecht: Option<String>,
        /// Jahrgang
        pub jahrgang: Option<String>,
        /// Bisher
        pub bisher: Option<String>,
        /// Anzahl Stimmen
        pub anzahl_stimmen: Option<String>,
        /// Gewählt
        pub gewahlt: Option<String>,
        /// Parteibezeichnung
        pub parteibezeichnung: Option<String>,
        /// Stille Wahl
        pub stille_wahl: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Wahlbezeichnung,
        BfsGemeindenummer,
        Gemeinde,
        Name,
        Vorname,
        Geschlecht,
        Jahrgang,
        Bisher,
        AnzahlStimmen,
        Gewahlt,
        Parteibezeichnung,
        StilleWahl,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Wahlbezeichnung => "wahlbezeichnung",
                Field::BfsGemeindenummer => "bfs_gemeindenummer",
                Field::Gemeinde => "gemeinde",
                Field::Name => "name",
                Field::Vorname => "vorname",
                Field::Geschlecht => "geschlecht",
                Field::Jahrgang => "jahrgang",
                Field::Bisher => "bisher",
                Field::AnzahlStimmen => "anzahl_stimmen",
                Field::Gewahlt => "gewahlt",
                Field::Parteibezeichnung => "parteibezeichnung",
                Field::StilleWahl => "stille_wahl",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12490/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Luftqualit\u{e4}t Station Dornach (halbst\u{fc}ndliche Messdaten seit Januar 2020)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12500/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12500/</a>\n"]
#[doc = "<p class=\"\" style=\"\">Echtzeitdaten der Luftmessstation Dornach. Die Station wird gemeinsam mit dem Amt f\u{fc}r Umwelt Kanton Solothurn betrieben. Die Messwerte werden halbst\u{fc}ndlich ausgewiesen und st\u{fc}ndlich (jeweils 20 Minuten nach der vollen Stunde mit einer m\u{f6}glichen Latenz von 1 bis 4 Stunden) aktualisiert. Die ausgewiesenen Werte werden unbereinigt von der Messstation bezogen. Validierte Messwerte sind direkt vom Lufthygieneamt beider Basel zu beziehen.</p><p class=\"\" style=\"\">Aufgrund messtechnischer Ungenauigkeiten k\u{f6}nnen bei geringer Konzentration eines Schadstoffs auch Negativwerte auftreten (Nullpunktrauschen). Die Handhabung negativer Messwerte ist in der aktuell g\u{fc}ltigen\u{a0}<a href=\"https://www.bafu.admin.ch/bafu/de/home/themen/luft/publikationen-studien/publikationen/immissionsmessung-von-luftfremdstoffen.html\" style=\"background-color: rgb(255, 255, 255); font-family: sans-serif; font-size: 14px; font-weight: 400;\" target=\"_blank\">Imissionsmessempfehlung\u{a0}</a>2021 beschrieben.</p><p class=\"\" style=\"\">Ein Datensatz der Messwerte zwischen 1993 und 2019 kann <a href=\"https://fkd-sta-files.bl.ch/ogd/luftqualitaet/airmet_dornach_1993_2019.csv\" target=\"_blank\">mit diesem Link</a>\u{a0}bezogen werden.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Ausgewiesene Werte</span></p><ul><li>Anfangszeit: Zeitstempel des Beginns der halbst\u{fc}ndlichen Messung im Format %Y-%m-%dT%H:%M:%S</li><li>Lungeng\u{e4}ngiger Feinstaub PM10 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM10 in Mikrogramm pro Kubikmeter.</li><li>Lungeng\u{e4}ngiger Feinstaub PM2.5 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM2.5 in Mikrogramm pro Kubikmeter.</li><li>Stickstoffdioxid NO2 (\u{b5}g/m3): Gemessene Stickstoffdioxid-Konzentration in Mikrogramm pro Kubikmeter.</li><li>Ozon O3 (\u{b5}g/m3): Gemessene Ozon-Konzentration in Mikrogramm pro Kubikmeter.</li></ul><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Standortbeschreibung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Die Messstation befindet sich in einem typischen Wohnquartier. Sie gibt einen \u{dc}berblick \u{fc}ber die Luftschadstoffbelastung in der Agglomeration der Stadt Basel. Sie misst die Hintergrundbelastung wie sie typischerweise in Wohnquartieren anzutreffen ist, als Mix verschiedenster Schadstoffquellen.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Lage</span></p><p class=\"\" style=\"font-family: sans-serif;\">Kleinst\u{e4}dtisch/Vorst\u{e4}dtisch, Hintergrund</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Koordinaten</span></p><p class=\"\" style=\"font-family: sans-serif;\">2613144 / 1258911; 305 m \u{fc}. M.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Bebauung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Offene Bebauung</p>"]
#[cfg(feature = "bl12500")]
pub mod luftqualitaet_station_dornach_halbstuendliche_messdaten_seit_januar_2020 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Anfangszeit
        #[serde(with = "time::serde::iso8601::option")]
        pub anfangszeit: Option<OffsetDateTime>,
        /// PM10
        ///
        /// Lungengängiger Feinstaub PM10
        pub pm10: Option<f64>,
        /// PM2.5
        ///
        /// Lungengängiger Feinstaub PM2.5
        pub pm2_5: Option<f64>,
        /// NO2
        ///
        /// Stickstoffdioxid
        pub no2: Option<f64>,
        /// O3
        ///
        /// Ozon
        pub o3: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Anfangszeit,
        Pm10,
        Pm25,
        No2,
        O3,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Anfangszeit => "anfangszeit",
                Field::Pm10 => "pm10",
                Field::Pm25 => "pm2_5",
                Field::No2 => "no2",
                Field::O3 => "o3",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12500/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Luftqualit\u{e4}t Station A2 Hard (halbst\u{fc}ndliche Messdaten seit Januar 2020)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12510/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12510/</a>\n"]
#[doc = "<p class=\"\" style=\"\">Echtzeitdaten der Luftmessstation A2 Hard. Die Messwerte werden halbst\u{fc}ndlich ausgewiesen und st\u{fc}ndlich (jeweils 20 Minuten nach der vollen Stunde mit einer m\u{f6}glichen Latenz von 1 bis 4 Stunden) aktualisiert. Die ausgewiesenen Werte werden unbereinigt von der Messstation bezogen. Validierte Messwerte sind direkt vom Lufthygieneamt beider Basel zu beziehen.</p><p class=\"\" style=\"\">Aufgrund messtechnischer Ungenauigkeiten k\u{f6}nnen bei geringer Konzentration eines Schadstoffs auch Negativwerte auftreten (Nullpunktrauschen). Die Handhabung negativer Messwerte ist in der aktuell g\u{fc}ltigen <a href=\"https://www.bafu.admin.ch/bafu/de/home/themen/luft/publikationen-studien/publikationen/immissionsmessung-von-luftfremdstoffen.html\" target=\"_blank\">Imissionsmessempfehlung </a>2021 beschrieben.\u{a0}Ein Datensatz der Messwerte zwischen 2003 und 2019 kann\u{a0}<a href=\"https://fkd-sta-files.bl.ch/ogd/luftqualitaet/airmet_a2_hard_2003_2019.csv\" style=\"font-family: sans-serif; font-size: 14px; background-color: rgb(255, 255, 255); font-weight: 400;\" target=\"_blank\">mit diesem Link</a>\u{a0}bezogen werden.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Ausgewiesene Werte</span></p><ul><li>Anfangszeit: Zeitstempel des Beginns der halbst\u{fc}ndlichen Messung im Format %Y-%m-%dT%H:%M:%S</li><li>Stickstoffdioxid NO2 (\u{b5}g/m3): Gemessene Stickstoffdioxid-Konzentration in Mikrogramm pro Kubikmeter.</li><li>Lungeng\u{e4}ngiger Feinstaub PM10 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM10 in Mikrogramm pro Kubikmeter.</li><li>Lungeng\u{e4}ngiger Feinstaub PM2.5 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM2.5 in Mikrogramm pro Kubikmeter.</li></ul><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Standortbeschreibung</span></p><p class=\"\" style=\"\">Die Messstation liegt direkt an der A2 im Abschnitt Hardwald, einer der am st\u{e4}rksten befahrenen Autobahnabschnitte der Schweiz. Zus\u{e4}tzlich wird dieser Abschnitt stark vom Schwerverkehr frequentiert. Die Station A2 Hard gibt damit die Belastung wieder, welche direkt an stark befahrenen Hochleistungsstrassen auftreten. Sie ist Teil eines gesamtschweizerischen Monitoring-Programms zur \u{dc}berwachung des alpenquerenden G\u{fc}terverkehrs entlang der A2 und A13.</p><p class=\"\" style=\"\">Die Station A2 Hard befindet sich seit 14.6.2023 wegen Bauarbeiten vor\u{fc}bergehend auf der gegen\u{fc}berliegenden Fahrbahnseite in ca. 400 m Entfernung.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Lage</span></p><p class=\"\" style=\"font-family: sans-serif;\">Kleinst\u{e4}dtisch/Vorst\u{e4}dtisch, Verkehr</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Koordinaten</span></p><p class=\"\" style=\"\">2615839 / 1265282; 275 m \u{fc}. M.<br></p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Bebauung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Offene Bebauung</p>"]
#[cfg(feature = "bl12510")]
pub mod luftqualitaet_station_a2_hard_halbstuendliche_messdaten_seit_januar_2020 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Anfangszeit
        #[serde(with = "time::serde::iso8601::option")]
        pub anfangszeit: Option<OffsetDateTime>,
        /// PM10
        ///
        /// Lungengängiger Feinstaub PM10
        pub pm10: Option<f64>,
        /// PM2.5
        ///
        /// Lungengängiger Feinstaub PM2.5
        pub pm2_5: Option<f64>,
        /// NO2
        ///
        /// Stickstoffdioxid
        pub no2: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Anfangszeit,
        Pm10,
        Pm25,
        No2,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Anfangszeit => "anfangszeit",
                Field::Pm10 => "pm10",
                Field::Pm25 => "pm2_5",
                Field::No2 => "no2",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12510/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Volksinitiative vom 23. Januar 2020 \u{ab}Maximal 10 % des Einkommens f\u{fc}r die Krankenkassenpr\u{e4}mien (Pr\u{e4}mien-Entlastungs-Initiative)\u{bb}"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12520/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12520/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 9. Juni 2024<br></p>"]
#[cfg(feature = "bl12520")]
pub mod volksinitiative_vom_23_januar_2020_maximal_10_des_einkommens_fuer_die_krankenkassenpraemien_praemien_entlastungs_initiative {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12520/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Volksinitiative vom 10. M\u{e4}rz 2020 \u{ab}F\u{fc}r tiefere Pr\u{e4}mien \u{2013} Kostenbremse im Gesundheitswesen (Kostenbremse-Initiative)\u{bb}"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12530/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12530/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 9. Juni 2024<br></p>"]
#[cfg(feature = "bl12530")]
pub mod volksinitiative_vom_10_maerz_2020_fuer_tiefere_praemien_kostenbremse_im_gesundheitswesen_kostenbremse_initiative {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12530/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Volksinitiative vom 16. Dezember 2021 \u{ab}F\u{fc}r Freiheit und k\u{f6}rperliche Unversehrtheit\u{bb}"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12540/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12540/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 9. Juni 2024<br></p>"]
#[cfg(feature = "bl12540")]
pub mod volksinitiative_vom_16_dezember_2021_fuer_freiheit_und_koerperliche_unversehrtheit {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12540/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Bundesgesetz vom 29. September 2023 \u{fc}ber eine sichere Stromversorgung mit erneuerbaren Energien (\u{c4}nderung des Energiegesetzes und des Stromversorgungsgesetzes)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12550/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12550/</a>\n"]
#[doc = "<p>Eidgen\u{f6}ssische Abstimmung vom 9. Juni 2024<br></p>"]
#[cfg(feature = "bl12550")]
pub mod bundesgesetz_vom_29_september_2023_ueber_eine_sichere_stromversorgung_mit_erneuerbaren_energien_aenderung_des_energiegesetzes_und_des_stromversorgungsgesetzes {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12550/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# \u{c4}nderung des Energiegesetzes vom 19. Oktober 2023"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12560/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12560/</a>\n"]
#[doc = "<p>Kantonale Abstimmung vom 9. Juni 2024<br></p>"]
#[cfg(feature = "bl12560")]
pub mod aenderung_des_energiegesetzes_vom_19_oktober_2023 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        pub date: Option<String>,
        pub entity_id: Option<String>,
        pub name: Option<String>,
        pub eligible_voters: Option<i64>,
        pub empty: Option<i64>,
        pub expats: Option<i64>,
        pub invalid: Option<i64>,
        pub yeas: Option<i64>,
        pub nays: Option<i64>,
        /// title_de_CH
        pub title_de_ch: Option<String>,
        pub answer: Option<String>,
        pub ballot_answer: Option<String>,
        pub id: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Date,
        EntityId,
        Name,
        EligibleVoters,
        Empty,
        Expats,
        Invalid,
        Yeas,
        Nays,
        TitleDeCh,
        Answer,
        BallotAnswer,
        Id,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Date => "date",
                Field::EntityId => "entity_id",
                Field::Name => "name",
                Field::EligibleVoters => "eligible_voters",
                Field::Empty => "empty",
                Field::Expats => "expats",
                Field::Invalid => "invalid",
                Field::Yeas => "yeas",
                Field::Nays => "nays",
                Field::TitleDeCh => "title_de_ch",
                Field::Answer => "answer",
                Field::BallotAnswer => "ballot_answer",
                Field::Id => "id",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12560/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Publikationsarchiv Amt f\u{fc}r Daten und Statistik BL (seit 2000)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12570/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12570/</a>\n"]
#[doc = "<p>Digitales Publikationsverzeichnis des Amts f\u{fc}r Daten und Statistik (bis 2023: Statistisches Amt). Es umfasst das Statistische Jahrbuch, Baselland in Zahlen, Webartikel sowie weitere Berichte und Brosch\u{fc}ren im pdf-Format oder als Webseiteninhalt.</p>"]
#[cfg(feature = "bl12570")]
pub mod publikationsarchiv_amt_fuer_daten_und_statistik_bl_seit_2000 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Publikationsdatum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub publikationsdatum: Option<Date>,
        /// Publikationsreihe
        pub publikationsreihe: Option<String>,
        /// Themenbereich
        pub themenbereich: Option<String>,
        /// Titel
        pub titel: Option<String>,
        /// Untertitel
        pub untertitel: Option<String>,
        /// Link
        pub link: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Publikationsdatum,
        Publikationsreihe,
        Themenbereich,
        Titel,
        Untertitel,
        Link,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Publikationsdatum => "publikationsdatum",
                Field::Publikationsreihe => "publikationsreihe",
                Field::Themenbereich => "themenbereich",
                Field::Titel => "titel",
                Field::Untertitel => "untertitel",
                Field::Link => "link",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12570/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# Luftqualit\u{e4}t Station Sch\u{f6}nenbuch (halbst\u{fc}ndliche Messdaten Januar 2000 - April 2016)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12580/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12580/</a>\n"]
#[doc = "<p></p><p></p><p class=\"\">Historische Daten der Luftmessstation Sch\u{f6}nenbuch von Anfang 2002 bis April 2016. \u{c4}ltere Daten k\u{f6}nnen beim Lufthygieneamt beider Basel direkt bezogen werden. Die Messwerte sind halbst\u{fc}ndlich ausgewiesen und bereinigt. Seit 2016 ist die Station stillgelegt.</p><p class=\"\">Das Auftreten allf\u{e4}lliger Negativwerte stammt von messtechnischen Ungenauigkeiten. Diese Messunsicherheit ist bei der Interpretation der Zahlen entsprechend mit einzubeziehen.</p><p class=\"\"></p><p class=\"\">Die Zeitstempel entsprechen der Zeitzone Europe/Zurich obwohl sie im Zeitformat UTC angegeben sind. Allf\u{e4}llige Fragen zum Zeitformat beantwortet das Amt f\u{fc}r Lufthygiene beider Basel auf Anfrage.</p><p style=\"\"><span style=\"font-weight: 700;\">Ausgewiesene Werte</span><br></p><ul><li>Anfangszeit: Zeitstempel des Beginns der halbst\u{fc}ndlichen Messung im Format %Y-%m-%dT%H:%M:%S</li><li>Lungeng\u{e4}ngiger Feinstaub PM10 (\u{b5}g/m3): Lungeng\u{e4}ngiger Feinstaub PM10 in Mikrogramm pro Kubikmeter.</li><li>Stickstoffdioxid NO2 (\u{b5}g/m3): Gemessene Stickstoffdioxid-Konzentration in Mikrogramm pro Kubikmeter.</li><li>Ozon O3 (\u{b5}g/m3): Gemessene Ozon-Konzentration in Mikrogramm pro Kubikmeter.</li></ul><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Standortbeschreibung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Die Messstation befand sich an der Sandgrubenstrasse 25, am Dorfrand von Sch\u{f6}nenbuch. An diesem Standort wurde die l\u{e4}ndliche Hintergrundbelastung gemessen.\u{a0}</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Lage</span></p><p class=\"\" style=\"font-family: sans-serif;\">L\u{e4}ndlich (unterhalb von 1000m \u{fc}. M.), Hintergrund</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Koordinaten</span></p><p class=\"\" style=\"font-family: sans-serif;\">2604746 / 1264620; 385 m \u{fc}. M.</p><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Bebauung</span></p><p class=\"\" style=\"font-family: sans-serif;\">Offene Bebauung</p><p></p><p></p>"]
#[cfg(feature = "bl12580")]
pub mod luftqualitaet_station_schoenenbuch_halbstuendliche_messdaten_januar_2000_april_2016 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Anfangzeit
        #[serde(with = "time::serde::iso8601::option")]
        pub anfangzeit: Option<OffsetDateTime>,
        /// PM10
        ///
        /// Lungengängiger Feinstaub PM10
        pub pm10: Option<f64>,
        /// NO2
        ///
        /// Stickstoffdioxid
        pub no2: Option<f64>,
        /// O3
        ///
        /// Ozon
        pub o3: Option<f64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        Anfangzeit,
        Pm10,
        No2,
        O3,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::Anfangzeit => "anfangzeit",
                Field::Pm10 => "pm10",
                Field::No2 => "no2",
                Field::O3 => "o3",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12580/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}

#[doc = "# OGD-Portal: T\u{e4}gliche Nutzung nach Datensatz (seit Januar 2024)"]
#[doc = "\n<a href=\"https://data.bl.ch/explore/dataset/12610/\" target=\"_blank\">https://data.bl.ch/explore/dataset/12610/</a>\n"]
#[doc = "<p class=\"\" style=\"font-family: Roboto, sans-serif;\">Die Daten \u{fc}ber die Nutzung der Datens\u{e4}tze auf dem OGD-Portal BL (data.bl.ch) werden von der Fach- und Koordinationsstelle OGD BL erhoben und ver\u{f6}ffentlicht.</p><p class=\"\" style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">Spalten</span></p><ul style=\"font-family: Roboto, sans-serif;\"><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">date</span>:\u{a0}<span style=\"font-family: sans-serif;\">Enth\u{e4}lt den Tag, an dem die Nutzung gemessen wurde.</span></li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">dataset_title</span>: Der Titel des Datensatzes</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">dataset_id</span>: Die technische ID des Datensatzes.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">visitors</span>: Gibt die Anzahl der t\u{e4}glichen Besucher/innen des Datensatzes an. Die Erfassung der Besucher/innen erfolgt durch Z\u{e4}hlen der einzigartigen (unique) IP-Adressen, die am Erhebungstag Zugriffe verzeichneten. Die IP-Adresse repr\u{e4}sentiert die Netzwerkadresse des Ger\u{e4}ts, von dem aus der Zugriff auf das Portal erfolgte.</li><li style=\"font-family: Roboto, sans-serif;\"><span style=\"font-weight: bolder;\">interactions</span>: Umfasst alle Interaktionen mit einem beliebigen Datensatz auf data.bl.ch. Ein/e Besucher/in kann mehrere Interaktionen ausl\u{f6}sen. Zu den Interaktionen z\u{e4}hlen Klicks auf der Webseite (Durchsuchen von Datens\u{e4}tzen, Filtern, usw.) sowie API-Aufrufe (Herunterladen eines Datensatzes als JSON-Datei, usw.).</li></ul><p class=\"\" style=\"font-family: sans-serif;\"><span style=\"font-weight: bolder;\">Bemerkungen</span></p><ul><li>Nur Aufrufe von \u{f6}ffentlich zug\u{e4}nglichen Datens\u{e4}tzen werden ausgewiesen.</li><li>IP-Adressen sowie Interaktionen von Nutzenden mit einem Login des Kantons Basel-Landschaft \u{2013} insbesondere von Mitarbeitenden der Fach- und Koordinationsstelle OGD \u{2013} werden vor der Ver\u{f6}ffentlichung aus dem Datensatz entfernt und somit nicht ausgewiesen.</li><li>Aufrufe von Akteuren, welche durch den User-Agent header eindeutig als Bots erkennbar sind, werden ebenfalls nicht ausgewiesen.</li><li>Kombinationen von Datensatz und Datum, f\u{fc}r welche keine Nutzung passierte (Visitors == 0 &amp; Interactions == 0) sind nicht ausgewiesen.</li><li>Aufgrund von Synchronisationsproblemen k\u{f6}nnen Daten tageweise\u{a0}fehlen</li></ul>"]
#[cfg(feature = "bl12610")]
pub mod ogd_portal_taegliche_nutzung_nach_datensatz_seit_januar_2024 {
    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Record {
        /// Datum
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date"
        )]
        pub dataset_id: Option<Date>,
        /// Datensatztitel
        pub date: Option<String>,
        /// Datensatz ID
        pub dataset_title: Option<String>,
        /// Visitors
        ///
        /// Unique IPs
        pub visitors: Option<i64>,
        /// Interactions
        ///
        /// API calls auf Datensätze
        pub interactions: Option<i64>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Field {
        DatasetId,
        Date,
        DatasetTitle,
        Visitors,
        Interactions,
    }

    impl crate::common::Field for Field {
        fn name(self) -> &'static str {
            match self {
                Field::DatasetId => "dataset_id",
                Field::Date => "date",
                Field::DatasetTitle => "dataset_title",
                Field::Visitors => "visitors",
                Field::Interactions => "interactions",
            }
        }
    }

    pub async fn get(
        limit: u8,
        offset: u64,
        order: Order<Field>,
        filter: Option<Filter<Field>>,
    ) -> Result<Data<Record>, Box<dyn std::error::Error>> {
        let limit = if limit > 100 { 100 } else { limit };
        let filter = filter.map(|filter| filter.inner).unwrap_or(String::new());
        let url = format!("https://data.bl.ch/api/explore/v2.1/catalog/datasets/12610/records?limit={limit}&offset={offset}");
        let url =
            reqwest::Url::parse_with_params(&url, &[("order_by", order.inner), ("where", filter)])?;
        let response = reqwest::get(url).await?.text().await?;
        let data: Data<Record> = serde_json::from_str(&response)?;
        Ok(data)
    }
}
