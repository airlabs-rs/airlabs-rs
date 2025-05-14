use super::*;

mod impls;

#[derive(Debug, Serialize, Deserialize)]
pub struct Airport {
    /// Public name. Available in the Free plan.
    pub name: String,
    /// Official IATA code. Available in the Free plan.
    pub iata_code: String,
    /// Official ICAO code. Available in the Free plan.
    pub icao_code: String,
    /// Geo Latitude. Available in the Free plan.
    pub lat: f64,
    /// Geo Longitude. Available in the Free plan.
    pub lng: f64,
    /// Airport Runway Elevation (feet)
    pub alt: u64,
    /// Airport metropolitan city name from Cities DB.
    pub city: String,
    /// Airport metropolitan 3 letter city code from Cities DB.
    pub city_code: String,
    /// United Nations location code.
    pub un_locode: String,
    /// Airport location timezone.
    pub timezone: String,
    /// ISO 2 country code from Countries DB. Available in the Free plan.
    pub country_code: String,
    /// Alternative names in different languages.
    pub names: BTreeMap<String, String>,
    /// Total airport runways.
    pub runways: u64,
    /// Total departures from airport per year.
    pub departures: u64,
    /// Total connections with another airports.
    pub connections: u64,
    /// The major airport in metropolitan area.
    pub is_major: bool,
    /// The airport provides international flights.
    pub is_international: u64,
    /// Airport official website.
    pub website: String,
    /// Airport official Facebook page.
    pub facebook: String,
    /// Airport official Twitter account.
    pub twitter: String,
    /// Airport official Instagram profile.
    pub instagram: String,
    /// Airport official Linkedin profile.
    pub linkedin: String,
    /// AirLabs generated uniq Airport ID, which you can use for SEO, etc.
    pub slug: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AirportsRequest {
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icao_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _fields: Option<String>,
}

impl AirportsRequest {
    pub fn new(key: &str) -> Self {
        let api_key = key.to_string();
        Self {
            api_key,
            ..default()
        }
    }
}

impl AirLabsRequest for AirportsRequest {
    fn url(&self, base: &str) -> String {
        format!("{base}/airports")
    }
}

#[cfg(test)]
mod tests {
    use codes_iso_639::part_1::LanguageCode;
    use serde_json as json;

    use super::*;

    const BODY: &str = r#"[{
  "name": "Paris Charles de Gaulle Airport",
  "iata_code": "CDG",
  "icao_code": "LFPG",
  "lat": 49.009592,
  "lng": 2.555675,
  "alt": 392,
  "city": "Paris",
  "city_code": "PAR",
  "un_locode": "FRCDG",
  "timezone": "Europe/Paris",
  "country_code": "FR",
  "phone": "+33170363950",
  "website": "http://www.aeroportsdeparis.fr/",
  "facebook": "facebook.com/parisaeroport",
  "instagram": "instagram.com/parisaeroport/",
  "linkedin": "linkedin.com/company/groupe-adp",
  "twitter": "twitter.com/parisaeroport",
  "names": {
      "sv": "Paris-Charles de Gaulle-flygplatsen",
      "pt": "Aeroporto de Paris-Charles de Gaulle",
      "nl": "Luchthaven Parijs-Charles de Gaulle",
      "ln": "Libándá lya Ndáko ya mpɛ́pɔ Paris-Charles-de-Gaulle",
      "hi": "चार्ल्स डि गॉल विमानक्षेत्र",
      "es": "Aeropuerto de París - Charles de Gaulle",
      "cs": "Letiště Charlese de Gaulla",
      "en": "Paris Charles de Gaulle Airport",
      "th": "ท่าอากาศยานนานาชาติปารีส-ชาร์ล เดอ โกล",
      "ru": "Париж — Шарль-де-Голль",
      "ja": "シャルル・ド・ゴール国際空港",
      "hu": "Párizs-Charles de Gaulle repülőtér",
      "is": "Paris-Charles de Gaulle-flugvöllur",
      "el": "Διεθνές Αεροδρόμιο Παρισιού Σαρλ ντε Γκωλ",
      "fr": "Aéroport Paris–Charles de Gaulle",
      "uk": "Міжнародний аеропорт імені Шарля де Голля",
      "fi": "Charles de Gaullen kansainvälinen lentoasema",
      "pl": "Port lotniczy Paryż-Roissy-Charles de Gaulle",
      "mr": "चार्ल्स दि गॉल आंतरराष्ट्रीय विमानतळ",
      "he": "נמל התעופה שארל דה גול",
      "ko": "파리 샤를 드 골 공항",
      "ar": "مطار باريس شارل دو غول الدولي",
      "wuu": "巴黎夏尔·戴高乐机场",
      "ta": "சார்லசு டிகால் வானூர்தி நிலையம்",
      "ro": "Aeroportul Internațional Charles de Gaulle",
      "no": "Charles de Gaulle internasjonale lufthavn",
      "jv": "Bandhar Udhara Paris-Charles de Gaulle",
      "id": "Bandar Udara Paris-Charles de Gaulle",
      "de": "Flughafen Paris-Charles-de-Gaulle",
      "tr": "Paris-Charles de Gaulle Havalimanı",
      "sk": "Letisko Charlesa de Gaulla",
      "pnb": "پیرس چارلس ڈیگال ہوائی اڈہ",
      "mk": "Аеродром Париз-Шарл де Гол",
      "it": "Aeroporto di Parigi Charles de Gaulle",
      "fa": "فرودگاه پاری-شارل-دو-گل",
      "vi": "Sân bay quốc tế Charles-de-Gaulle",
      "az": "Şarl De Qoll",
      "da": "Charles de Gaulle Lufthavn",
      "hr": "Zračna luka Charles de Gaulle",
      "hy": "Շարլ Դե Գոլ",
      "jp": "シャルル・ド・ゴール国際空港",
      "lv": "Šarla de Golla lidosta",
      "sr": "Aerodrom Šarl de Gol",
      "sl": "Letališče Charles de Gaulle",
      "ka": "ჩარლზ დე გოლის აეროპორტი",
      "ms": "Bandar Udara Paris-Charles de Gaulle",
      "tl": "Paris Charles de Gaulle"
  },
  "runways": 8,
  "departures": 186292,
  "connections": 411,
  "is_major": false,
  "is_international": 1,
  "slug": "charles-gaulle-cdg-lfpg-fr"
}]
"#;
    #[test]
    fn test_name() {
        let mut airports = json::from_str::<Vec<Airport>>(BODY).unwrap();
        assert_eq!(airports.len(), 1);
        let airport = airports.pop().unwrap();
        assert_eq!(airport.iata_code, "CDG");
        assert_eq!(
            airport.name_by_language("fr"),
            Some("Aéroport Paris–Charles de Gaulle")
        );
        assert_eq!(
            airport.name_by_language(LanguageCode::El),
            Some("Διεθνές Αεροδρόμιο Παρισιού Σαρλ ντε Γκωλ")
        );
        assert_eq!(
            airport.name_by_language(LanguageCode::He),
            Some("נמל התעופה שארל דה גול")
        );
    }
}
