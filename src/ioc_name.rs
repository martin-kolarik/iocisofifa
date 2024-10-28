use crate::country::Country;

type CountryName = (Country, &'static str);

const AFG: CountryName = (Country::AFG, "Afghanistan");
const ALB: CountryName = (Country::ALB, "Albania");
const DZA: CountryName = (Country::DZA, "Algeria");
const ASM: CountryName = (Country::ASM, "American Samoa");
const AND: CountryName = (Country::AND, "Andorra");
const AGO: CountryName = (Country::AGO, "Angola");
const AIA: CountryName = (Country::AIA, "Anguilla");
const ATA: CountryName = (Country::ATA, "Antarctica");
const ATG: CountryName = (Country::ATG, "Antigua and Barbuda");
const ARG: CountryName = (Country::ARG, "Argentina");
const ARM: CountryName = (Country::ARM, "Armenia");
const ABW: CountryName = (Country::ABW, "Aruba");
const AUS: CountryName = (Country::AUS, "Australia");
const AUT: CountryName = (Country::AUT, "Austria");
const AZE: CountryName = (Country::AZE, "Azerbaijan");
const BHS: CountryName = (Country::BHS, "Bahamas");
const BHR: CountryName = (Country::BHR, "Bahrain");
const BGD: CountryName = (Country::BGD, "Bangladesh");
const BRB: CountryName = (Country::BRB, "Barbados");
const BLR: CountryName = (Country::BLR, "Belarus");
const BEL: CountryName = (Country::BEL, "Belgium");
const BLZ: CountryName = (Country::BLZ, "Belize");
const BEN: CountryName = (Country::BEN, "Benin");
const BMU: CountryName = (Country::BMU, "Bermuda");
const BTN: CountryName = (Country::BTN, "Bhutan");
const BOL: CountryName = (Country::BOL, "Bolivia");
const BES: CountryName = (Country::BES, "Bonaire, Sint Eustatius and Saba");
const BIH: CountryName = (Country::BIH, "Bosnia and Herzegovina");
const BWA: CountryName = (Country::BWA, "Botswana");
const BVT: CountryName = (Country::BVT, "Bouvet Island");
const BRA: CountryName = (Country::BRA, "Brazil");
const BRN: CountryName = (Country::BRN, "Brunei Darussalam");
const BGR: CountryName = (Country::BGR, "Bulgaria");
const BFA: CountryName = (Country::BFA, "Burkina Faso");
const BDI: CountryName = (Country::BDI, "Burundi");
const KHM: CountryName = (Country::KHM, "Cambodia");
const CMR: CountryName = (Country::CMR, "Cameroon");
const CAN: CountryName = (Country::CAN, "Canada");
const CPV: CountryName = (Country::CPV, "Cape Verde");
const CYM: CountryName = (Country::CYM, "Cayman Islands");
const CAF: CountryName = (Country::CAF, "Central African Republic");
const TCD: CountryName = (Country::TCD, "Chad");
const CHL: CountryName = (Country::CHL, "Chile");
const CHN: CountryName = (Country::CHN, "China, People's Republic of");
const TWN: CountryName = (Country::TWN, "Chinese Taipei");
const CXR: CountryName = (Country::CXR, "Christmas Island");
const CCK: CountryName = (Country::CCK, "Cocos (Keeling) Islands");
const COL: CountryName = (Country::COL, "Colombia");
const COM: CountryName = (Country::COM, "Comoros");
const COG: CountryName = (Country::COG, "Congo, Democratic Republic of");
const COK: CountryName = (Country::COK, "Cook Islands");
const CRI: CountryName = (Country::CRI, "Costa Rica");
const CIV: CountryName = (Country::CIV, "Côte d'Ivoire");
const HRV: CountryName = (Country::HRV, "Croatia");
const CUB: CountryName = (Country::CUB, "Cuba");
const CYP: CountryName = (Country::CYP, "Cyprus");
const CZE: CountryName = (Country::CZE, "Czechia");
const DNK: CountryName = (Country::DNK, "Denmark");
const DJI: CountryName = (Country::DJI, "Djibouti");
const DMA: CountryName = (Country::DMA, "Dominica");
const DOM: CountryName = (Country::DOM, "Dominican Republic");
const ECU: CountryName = (Country::ECU, "Ecuador");
const EGY: CountryName = (Country::EGY, "EGYPT");
const SLV: CountryName = (Country::SLV, "El Salvador");
const ENG: CountryName = (Country::ENG, "England");
const GNQ: CountryName = (Country::GNQ, "Equatorial Guinea");
const ERI: CountryName = (Country::ERI, "Eritrea");
const EST: CountryName = (Country::EST, "Estonia");
const ETH: CountryName = (Country::ETH, "Ethiopia");
const FLK: CountryName = (Country::FLK, "Falkland Islands");
const FRO: CountryName = (Country::FRO, "Faroe Islands");
const FJI: CountryName = (Country::FJI, "Fiji");
const FIN: CountryName = (Country::FIN, "Finland");
const FRA: CountryName = (Country::FRA, "France");
const GUF: CountryName = (Country::GUF, "French Guiana");
const PYF: CountryName = (Country::PYF, "French Polynesia");
const GAB: CountryName = (Country::GAB, "Gabon");
const GMB: CountryName = (Country::GMB, "Gambia");
const GEO: CountryName = (Country::GEO, "Georgia");
const DEU: CountryName = (Country::DEU, "Germany");
const GHA: CountryName = (Country::GHA, "Ghana");
const GIB: CountryName = (Country::GIB, "Gibraltar");
const GRC: CountryName = (Country::GRC, "Greece");
const GRL: CountryName = (Country::GRL, "Greenland");
const GRD: CountryName = (Country::GRD, "Grenada");
const GLP: CountryName = (Country::GLP, "Guadeloupe");
const GUM: CountryName = (Country::GUM, "Guam");
const GTM: CountryName = (Country::GTM, "Guatemala");
const GIN: CountryName = (Country::GIN, "Guinea");
const GNB: CountryName = (Country::GNB, "Guinea-Bissau");
const GUY: CountryName = (Country::GUY, "Guyana");
const HTI: CountryName = (Country::HTI, "Haiti");
const HMD: CountryName = (Country::HMD, "Heard Island and McDonald Islands");
const HND: CountryName = (Country::HND, "Honduras");
const HKG: CountryName = (Country::HKG, "Hong Kong");
const HUN: CountryName = (Country::HUN, "Hungary");
const ISL: CountryName = (Country::ISL, "Iceland");
const IND: CountryName = (Country::IND, "India");
const IDN: CountryName = (Country::IDN, "Indonesia");
const IRN: CountryName = (Country::IRN, "Iran, Islamic Republic of");
const IRQ: CountryName = (Country::IRQ, "Iraq");
const IRL: CountryName = (Country::IRL, "Ireland, Republic of");
const ISR: CountryName = (Country::ISR, "Israel");
const ITA: CountryName = (Country::ITA, "Italy");
const JAM: CountryName = (Country::JAM, "Jamaica");
const JPN: CountryName = (Country::JPN, "Japan");
const JOR: CountryName = (Country::JOR, "Jordan");
const KAZ: CountryName = (Country::KAZ, "Kazakhstan");
const KEN: CountryName = (Country::KEN, "Kenya");
const KIR: CountryName = (Country::KIR, "Kiribati");
const KOR: CountryName = (Country::KOR, "Korea");
const PRK: CountryName = (Country::PRK, "Korea, Democratic People's Republic of");
const KWT: CountryName = (Country::KWT, "Kuwait");
const KGZ: CountryName = (Country::KGZ, "Kyrgyzstan");
const LAO: CountryName = (Country::LAO, "Lao People's Democratic Republic");
const LVA: CountryName = (Country::LVA, "Latvia");
const LBN: CountryName = (Country::LBN, "Lebanon");
const LSO: CountryName = (Country::LSO, "Lesotho");
const LBR: CountryName = (Country::LBR, "Liberia");
const LBY: CountryName = (Country::LBY, "Libyan Arab Jamahiriya");
const LIE: CountryName = (Country::LIE, "Liechtenstein");
const LTU: CountryName = (Country::LTU, "Lithuania");
const LUX: CountryName = (Country::LUX, "Luxembourg");
const MAC: CountryName = (Country::MAC, "Macau");
const MKD: CountryName = (Country::MKD, "Macedonia");
const MDG: CountryName = (Country::MDG, "Madagascar");
const MWI: CountryName = (Country::MWI, "Malawi");
const MYS: CountryName = (Country::MYS, "Malaysia");
const MDV: CountryName = (Country::MDV, "Maldives");
const MLI: CountryName = (Country::MLI, "Mali");
const MLT: CountryName = (Country::MLT, "Malta");
const MHL: CountryName = (Country::MHL, "Marshall Islands");
const MTQ: CountryName = (Country::MTQ, "Martinique");
const MRT: CountryName = (Country::MRT, "Mauritania");
const MUS: CountryName = (Country::MUS, "Mauritius");
const MYT: CountryName = (Country::MYT, "Mayotte");
const MEX: CountryName = (Country::MEX, "Mexico");
const FSM: CountryName = (Country::FSM, "Micronesia, Federated States of");
const MDA: CountryName = (Country::MDA, "Moldova");
const MCO: CountryName = (Country::MCO, "Monaco");
const MNG: CountryName = (Country::MNG, "Mongolia");
const MNE: CountryName = (Country::MNE, "Montenegro");
const MSR: CountryName = (Country::MSR, "Montserrat");
const MAR: CountryName = (Country::MAR, "Morocco");
const MOZ: CountryName = (Country::MOZ, "Mozambique");
const MMR: CountryName = (Country::MMR, "Myanmar");
const NAM: CountryName = (Country::NAM, "Namibia");
const NRU: CountryName = (Country::NRU, "Nauru");
const NPL: CountryName = (Country::NPL, "Nepal");
const NLD: CountryName = (Country::NLD, "Netherlands");
const NEU: CountryName = (Country::NEU, "NEUTRAL");
const NCL: CountryName = (Country::NCL, "New Caledonia");
const NZL: CountryName = (Country::NZL, "New Zealand");
const NIC: CountryName = (Country::NIC, "Nicaragua");
const NER: CountryName = (Country::NER, "Niger");
const NGA: CountryName = (Country::NGA, "Nigeria");
const NIU: CountryName = (Country::NIU, "Niue");
const NFK: CountryName = (Country::NFK, "Norfolk Island");
const NIR: CountryName = (Country::NIR, "Northern Ireland");
const MNP: CountryName = (Country::MNP, "Northern Mariana Islands");
const NOR: CountryName = (Country::NOR, "Norway");
const OMN: CountryName = (Country::OMN, "Oman");
const PAK: CountryName = (Country::PAK, "Pakistan");
const PLW: CountryName = (Country::PLW, "Palau");
const PAN: CountryName = (Country::PAN, "Panama");
const PNG: CountryName = (Country::PNG, "Papua New Guinea");
const PRY: CountryName = (Country::PRY, "Paraguay");
const PER: CountryName = (Country::PER, "Peru");
const PHL: CountryName = (Country::PHL, "Philippines");
const PCN: CountryName = (Country::PCN, "Pitcairn");
const POL: CountryName = (Country::POL, "Poland");
const PRT: CountryName = (Country::PRT, "Portugal");
const PRI: CountryName = (Country::PRI, "Puerto Rico");
const QAT: CountryName = (Country::QAT, "Qatar");
const XKX1: CountryName = (Country::XKX, "Republic of Kosovo");
const XKX2: CountryName = (Country::XKX, "Kosovo");
const REU: CountryName = (Country::REU, "Réunion");
const ROU: CountryName = (Country::ROU, "Romania");
const RUS: CountryName = (Country::RUS, "Russian Federation");
const RWA: CountryName = (Country::RWA, "Rwanda");
const SHN: CountryName = (Country::SHN, "Saint Helena");
const KNA: CountryName = (Country::KNA, "Saint Kitts and Nevis");
const LCA: CountryName = (Country::LCA, "Saint Lucia");
const SPM: CountryName = (Country::SPM, "Saint Pierre and Miquelon");
const VCT: CountryName = (Country::VCT, "Saint Vincent and the Grenadines");
const WSM: CountryName = (Country::WSM, "Samoa");
const SMR: CountryName = (Country::SMR, "San Marino");
const SAU: CountryName = (Country::SAU, "Saudi Arabia");
const SCO: CountryName = (Country::SCO, "Scotland");
const SEN: CountryName = (Country::SEN, "Senegal");
const SRB: CountryName = (Country::SRB, "Serbia");
const SYC: CountryName = (Country::SYC, "Seychelles");
const SLE: CountryName = (Country::SLE, "Sierra Leone");
const SGP: CountryName = (Country::SGP, "Singapore");
const SVK: CountryName = (Country::SVK, "Slovakia");
const SVN: CountryName = (Country::SVN, "Slovenia");
const STP: CountryName = (Country::STP, "So Tomé and Príncipe");
const SLB: CountryName = (Country::SLB, "Solomon Islands");
const SOM: CountryName = (Country::SOM, "Somalia");
const ZAF: CountryName = (Country::ZAF, "South Africa");
const SGS: CountryName = (Country::SGS, "South Georgia and the South Sandwich Islands");
const ESP: CountryName = (Country::ESP, "Spain");
const LKA: CountryName = (Country::LKA, "Sri Lanka");
const SDN: CountryName = (Country::SDN, "Sudan");
const SUR: CountryName = (Country::SUR, "Suriname");
const SJM: CountryName = (Country::SJM, "Svalbard and Jan Mayen");
const SWE: CountryName = (Country::SWE, "Sweden");
const CHE: CountryName = (Country::CHE, "Switzerland");
const SYR: CountryName = (Country::SYR, "Syrian Arab Republic");
const TJK: CountryName = (Country::TJK, "Tajikistan");
const TZA: CountryName = (Country::TZA, "Tanzania, United Republic Of");
const THA: CountryName = (Country::THA, "Thailand");
const TLS: CountryName = (Country::TLS, "Timor-Leste");
const TGO: CountryName = (Country::TGO, "Togo");
const TKL: CountryName = (Country::TKL, "Tokelau");
const TON: CountryName = (Country::TON, "Tonga");
const TTO: CountryName = (Country::TTO, "Trinidad and Tobago");
const TUN: CountryName = (Country::TUN, "Tunisia");
const TUR: CountryName = (Country::TUR, "Türkiye");
const TKM: CountryName = (Country::TKM, "Turkmenistan");
const TCA: CountryName = (Country::TCA, "Turks and Caicos Islands");
const TUV: CountryName = (Country::TUV, "Tuvalu");
const UGA: CountryName = (Country::UGA, "Uganda");
const UKR: CountryName = (Country::UKR, "Ukraine");
const ARE: CountryName = (Country::ARE, "United Arab Emirates");
const GBR: CountryName = (Country::GBR, "United Kingdom");
const USA: CountryName = (Country::USA, "United States");
const URY: CountryName = (Country::URY, "Uruguay");
const UZB: CountryName = (Country::UZB, "Uzbekistan");
const VUT: CountryName = (Country::VUT, "Vanuatu");
const VAT: CountryName = (Country::VAT, "Vatican City State");
const VEN: CountryName = (Country::VEN, "Venezuela");
const VNM: CountryName = (Country::VNM, "Vietnam");
const VGB: CountryName = (Country::VGB, "Virgin Islands, British");
const VIR: CountryName = (Country::VIR, "Virgin Islands, U.S.");
const VRT: CountryName = (Country::VRT, "Virtual");
const WAL: CountryName = (Country::WAL, "Wales");
const WLF: CountryName = (Country::WLF, "Wallis and Futuna");
const ESH: CountryName = (Country::ESH, "Western Sahara");
const YEM: CountryName = (Country::YEM, "Yemen");
const ZMB: CountryName = (Country::ZMB, "Zambia");
const ZWE: CountryName = (Country::ZWE, "Zimbabwe");

// Special codes
const AIN: CountryName = (Country::AIN, "Individual Neutral Athletes");
const EOR: CountryName = (Country::EOR, "Refugee Olympic Team");
const NPA: CountryName = (Country::NPA, "Neutral Paralympic Athletes");
const ROC: CountryName = (Country::ROC, "Russian Olympic Committee");
const RPC: CountryName = (Country::RPC, "Russian Paralympic Committee");
const RPT: CountryName = (Country::RPT, "Refugee Paralympic Team");

const COUNTRY_NAME: [&CountryName; 248] = [
    &AFG, &ALB, &DZA, &ASM, &AND, &AGO, &AIA, &ATA, &ATG, &ARG, &ARM, &ABW, &AUS, &AUT, &AZE, &BHS,
    &BHR, &BGD, &BRB, &BLR, &BEL, &BLZ, &BEN, &BMU, &BTN, &BOL, &BES, &BIH, &BWA, &BVT, &BRA, &BRN,
    &BGR, &BFA, &BDI, &KHM, &CMR, &CAN, &CPV, &CYM, &CAF, &TCD, &CHL, &CHN, &TWN, &CXR, &CCK, &COL,
    &COM, &COG, &COK, &CRI, &CIV, &HRV, &CUB, &CYP, &CZE, &DNK, &DJI, &DMA, &DOM, &ECU, &EGY, &SLV,
    &ENG, &GNQ, &ERI, &EST, &ETH, &FLK, &FRO, &FJI, &FIN, &FRA, &GUF, &PYF, &GAB, &GMB, &GEO, &DEU,
    &GHA, &GIB, &GRC, &GRL, &GRD, &GLP, &GUM, &GTM, &GIN, &GNB, &GUY, &HTI, &HMD, &HND, &HKG, &HUN,
    &ISL, &IND, &IDN, &IRN, &IRQ, &IRL, &ISR, &ITA, &JAM, &JPN, &JOR, &KAZ, &KEN, &KIR, &KOR, &PRK,
    &KWT, &KGZ, &LAO, &LVA, &LBN, &LSO, &LBR, &LBY, &LIE, &LTU, &LUX, &MAC, &MKD, &MDG, &MWI, &MYS,
    &MDV, &MLI, &MLT, &MHL, &MTQ, &MRT, &MUS, &MYT, &MEX, &FSM, &MDA, &MCO, &MNG, &MNE, &MSR, &MAR,
    &MOZ, &MMR, &NAM, &NRU, &NPL, &NLD, &NEU, &NCL, &NZL, &NIC, &NER, &NGA, &NIU, &NFK, &NIR, &MNP,
    &NOR, &OMN, &PAK, &PLW, &PAN, &PNG, &PRY, &PER, &PHL, &PCN, &POL, &PRT, &PRI, &QAT, &XKX1,
    &XKX2, &REU, &ROU, &RUS, &RWA, &SHN, &KNA, &LCA, &SPM, &VCT, &WSM, &SMR, &SAU, &SCO, &SEN,
    &SRB, &SYC, &SLE, &SGP, &SVK, &SVN, &STP, &SLB, &SOM, &ZAF, &SGS, &ESP, &LKA, &SDN, &SUR, &SJM,
    &SWE, &CHE, &SYR, &TJK, &TZA, &THA, &TLS, &TGO, &TKL, &TON, &TTO, &TUN, &TUR, &TKM, &TCA, &TUV,
    &UGA, &UKR, &ARE, &GBR, &USA, &URY, &UZB, &VUT, &VAT, &VEN, &VNM, &VGB, &VIR, &VRT, &WAL, &WLF,
    &ESH, &YEM, &ZMB, &ZWE, &AIN, &EOR, &NPA, &ROC, &RPC, &RPT,
];

pub fn name_to_country(name: &str) -> Option<Country> {
    COUNTRY_NAME
        .iter()
        .find_map(|country_name| match country_name {
            (country, candidate) if *candidate == name => Some(*country),
            _ => None,
        })
}

#[cfg(feature = "std")]
pub fn name_to_country_caseless(name: &str) -> Option<Country> {
    let name = name.to_uppercase();
    COUNTRY_NAME
        .iter()
        .find_map(|country_name| match country_name {
            (country, candidate) if candidate.to_uppercase() == name => Some(*country),
            _ => None,
        })
}

pub fn country_to_name(country: Country) -> Option<&'static str> {
    COUNTRY_NAME
        .iter()
        .find_map(|country_name| match country_name {
            (candidate, name) if *candidate == country => Some(*name),
            _ => None,
        })
}
