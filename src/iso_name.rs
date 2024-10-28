use crate::country::Country;

type CountryName = (Country, &'static str);

const AFG: CountryName = (Country::AFG, "Afghanistan");
const ALA: CountryName = (Country::ALA, "Åland Islands");
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
const BOL: CountryName = (Country::BOL, "Bolivia (Plurinational State of)");
const BES: CountryName = (Country::BES, "Bonaire, Sint Eustatius and Saba");
const BIH: CountryName = (Country::BIH, "Bosnia and Herzegovina");
const BWA: CountryName = (Country::BWA, "Botswana");
const BVT: CountryName = (Country::BVT, "Bouvet Island");
const BRA: CountryName = (Country::BRA, "Brazil");
const IOT: CountryName = (Country::IOT, "British Indian Ocean Territory");
const VGB: CountryName = (Country::VGB, "British Virgin Islands");
const BRN: CountryName = (Country::BRN, "Brunei Darussalam");
const BGR: CountryName = (Country::BGR, "Bulgaria");
const BFA: CountryName = (Country::BFA, "Burkina Faso");
const BDI: CountryName = (Country::BDI, "Burundi");
const CPV: CountryName = (Country::CPV, "Cabo Verde");
const KHM: CountryName = (Country::KHM, "Cambodia");
const CMR: CountryName = (Country::CMR, "Cameroon");
const CAN: CountryName = (Country::CAN, "Canada");
const CYM: CountryName = (Country::CYM, "Cayman Islands");
const CAF: CountryName = (Country::CAF, "Central African Republic");
const TCD: CountryName = (Country::TCD, "Chad");
const CHL: CountryName = (Country::CHL, "Chile");
const CHN: CountryName = (Country::CHN, "China");
const HKG: CountryName = (
    Country::HKG,
    "China, Hong Kong Special Administrative Region",
);
const MAC: CountryName = (Country::MAC, "China, Macao Special Administrative Region");
const CXR: CountryName = (Country::CXR, "Christmas Island");
const CCK: CountryName = (Country::CCK, "Cocos (Keeling) Islands");
const COL: CountryName = (Country::COL, "Colombia");
const COM: CountryName = (Country::COM, "Comoros");
const COG: CountryName = (Country::COG, "Congo");
const COK: CountryName = (Country::COK, "Cook Islands");
const CRI: CountryName = (Country::CRI, "Costa Rica");
const HRV: CountryName = (Country::HRV, "Croatia");
const CUB: CountryName = (Country::CUB, "Cuba");
const CUW: CountryName = (Country::CUW, "Curaçao");
const CYP: CountryName = (Country::CYP, "Cyprus");
const CZE: CountryName = (Country::CZE, "Czechia");
const PRK: CountryName = (Country::PRK, "Democratic People's Republic of Korea");
const COD: CountryName = (Country::COD, "Democratic Republic of the Congo");
const DNK: CountryName = (Country::DNK, "Denmark");
const DJI: CountryName = (Country::DJI, "Djibouti");
const DMA: CountryName = (Country::DMA, "Dominica");
const DOM: CountryName = (Country::DOM, "Dominican Republic");
const ECU: CountryName = (Country::ECU, "Ecuador");
const EGY: CountryName = (Country::EGY, "Egypt");
const SLV: CountryName = (Country::SLV, "El Salvador");
const GNQ: CountryName = (Country::GNQ, "Equatorial Guinea");
const ERI: CountryName = (Country::ERI, "Eritrea");
const EST: CountryName = (Country::EST, "Estonia");
const SWZ: CountryName = (Country::SWZ, "Eswatini");
const ETH: CountryName = (Country::ETH, "Ethiopia");
const FLK: CountryName = (Country::FLK, "Falkland Islands");
const FRO: CountryName = (Country::FRO, "Faroe Islands");
const FJI: CountryName = (Country::FJI, "Fiji");
const FIN: CountryName = (Country::FIN, "Finland");
const FRA: CountryName = (Country::FRA, "France");
const GUF: CountryName = (Country::GUF, "French Guiana");
const PYF: CountryName = (Country::PYF, "French Polynesia");
const ATF: CountryName = (Country::ATF, "French Southern Territories");
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
const GGY: CountryName = (Country::GGY, "Guernsey");
const GIN: CountryName = (Country::GIN, "Guinea");
const GNB: CountryName = (Country::GNB, "Guinea-Bissau");
const GUY: CountryName = (Country::GUY, "Guyana");
const HTI: CountryName = (Country::HTI, "Haiti");
const HMD: CountryName = (Country::HMD, "Heard Island and McDonald Islands");
const VAT: CountryName = (Country::VAT, "Holy See");
const HND: CountryName = (Country::HND, "Honduras");
const HUN: CountryName = (Country::HUN, "Hungary");
const ISL: CountryName = (Country::ISL, "Iceland");
const IND: CountryName = (Country::IND, "India");
const IDN: CountryName = (Country::IDN, "Indonesia");
const IRN: CountryName = (Country::IRN, "Iran (Islamic Republic of)");
const IRQ: CountryName = (Country::IRQ, "Iraq");
const IRL: CountryName = (Country::IRL, "Ireland");
const IMN: CountryName = (Country::IMN, "Isle of Man");
const ISR: CountryName = (Country::ISR, "Israel");
const ITA: CountryName = (Country::ITA, "Italy");
const CIV: CountryName = (Country::CIV, "Ivory Coast");
const JAM: CountryName = (Country::JAM, "Jamaica");
const JPN: CountryName = (Country::JPN, "Japan");
const JEY: CountryName = (Country::JEY, "Jersey");
const JOR: CountryName = (Country::JOR, "Jordan");
const KAZ: CountryName = (Country::KAZ, "Kazakhstan");
const KEN: CountryName = (Country::KEN, "Kenya");
const KIR: CountryName = (Country::KIR, "Kiribati");
const KWT: CountryName = (Country::KWT, "Kuwait");
const KGZ: CountryName = (Country::KGZ, "Kyrgyzstan");
const LAO: CountryName = (Country::LAO, "Lao People's Democratic Republic");
const LVA: CountryName = (Country::LVA, "Latvia");
const LBN: CountryName = (Country::LBN, "Lebanon");
const LSO: CountryName = (Country::LSO, "Lesotho");
const LBR: CountryName = (Country::LBR, "Liberia");
const LBY: CountryName = (Country::LBY, "Libya");
const LIE: CountryName = (Country::LIE, "Liechtenstein");
const LTU: CountryName = (Country::LTU, "Lithuania");
const LUX: CountryName = (Country::LUX, "Luxembourg");
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
const FSM: CountryName = (Country::FSM, "Micronesia (Federated States of)");
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
const NCL: CountryName = (Country::NCL, "New Caledonia");
const NZL: CountryName = (Country::NZL, "New Zealand");
const NIC: CountryName = (Country::NIC, "Nicaragua");
const NER: CountryName = (Country::NER, "Niger");
const NGA: CountryName = (Country::NGA, "Nigeria");
const NIU: CountryName = (Country::NIU, "Niue");
const NFK: CountryName = (Country::NFK, "Norfolk Island");
const MNP: CountryName = (Country::MNP, "Northern Mariana Islands");
const MKD: CountryName = (Country::MKD, "North Macedonia");
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
const KOR: CountryName = (Country::KOR, "Republic of Korea");
const MDA: CountryName = (Country::MDA, "Republic of Moldova");
const REU: CountryName = (Country::REU, "Réunion");
const ROU: CountryName = (Country::ROU, "Romania");
const RUS: CountryName = (Country::RUS, "Russian Federation");
const RWA: CountryName = (Country::RWA, "Rwanda");
const BLM: CountryName = (Country::BLM, "Saint Barthélemy");
const SHN: CountryName = (Country::SHN, "Saint Helena");
const KNA: CountryName = (Country::KNA, "Saint Kitts and Nevis");
const LCA: CountryName = (Country::LCA, "Saint Lucia");
const MAF: CountryName = (Country::MAF, "Saint Martin (French Part)");
const SPM: CountryName = (Country::SPM, "Saint Pierre and Miquelon");
const VCT: CountryName = (Country::VCT, "Saint Vincent and the Grenadines");
const WSM: CountryName = (Country::WSM, "Samoa");
const SMR: CountryName = (Country::SMR, "San Marino");
const STP: CountryName = (Country::STP, "Sao Tome and Principe");
const SAU: CountryName = (Country::SAU, "Saudi Arabia");
const SEN: CountryName = (Country::SEN, "Senegal");
const SRB: CountryName = (Country::SRB, "Serbia");
const SYC: CountryName = (Country::SYC, "Seychelles");
const SLE: CountryName = (Country::SLE, "Sierra Leone");
const SGP: CountryName = (Country::SGP, "Singapore");
const SXM: CountryName = (Country::SXM, "Sint Maarten (Dutch part)");
const SVK: CountryName = (Country::SVK, "Slovakia");
const SVN: CountryName = (Country::SVN, "Slovenia");
const SLB: CountryName = (Country::SLB, "Solomon Islands");
const SOM: CountryName = (Country::SOM, "Somalia");
const ZAF: CountryName = (Country::ZAF, "South Africa");
const SGS: CountryName = (Country::SGS, "South Georgia and the South Sandwich Islands");
const SSD: CountryName = (Country::SSD, "South Sudan");
const ESP: CountryName = (Country::ESP, "Spain");
const LKA: CountryName = (Country::LKA, "Sri Lanka");
const PSE: CountryName = (Country::PSE, "State of Palestine");
const SDN: CountryName = (Country::SDN, "Sudan");
const SUR: CountryName = (Country::SUR, "Suriname");
const SJM: CountryName = (Country::SJM, "Svalbard and Jan Mayen Islands");
const SWE: CountryName = (Country::SWE, "Sweden");
const CHE: CountryName = (Country::CHE, "Switzerland");
const SYR: CountryName = (Country::SYR, "Syrian Arab Republic");
const TWN: CountryName = (Country::TWN, "Taiwan");
const TJK: CountryName = (Country::TJK, "Tajikistan");
const THA: CountryName = (Country::THA, "Thailand");
const TLS: CountryName = (Country::TLS, "Timor-Leste");
const TGO: CountryName = (Country::TGO, "Togo");
const TKL: CountryName = (Country::TKL, "Tokelau");
const TON: CountryName = (Country::TON, "Tonga");
const TTO: CountryName = (Country::TTO, "Trinidad and Tobago");
const TUN: CountryName = (Country::TUN, "Tunisia");
const TUR: CountryName = (Country::TUR, "Turkey");
const TKM: CountryName = (Country::TKM, "Turkmenistan");
const TCA: CountryName = (Country::TCA, "Turks and Caicos Islands");
const TUV: CountryName = (Country::TUV, "Tuvalu");
const UGA: CountryName = (Country::UGA, "Uganda");
const UKR: CountryName = (Country::UKR, "Ukraine");
const ARE: CountryName = (Country::ARE, "United Arab Emirates");
const GBR: CountryName = (
    Country::GBR,
    "United Kingdom of Great Britain and Northern Ireland",
);
const TZA: CountryName = (Country::TZA, "United Republic of Tanzania");
const UMI: CountryName = (Country::UMI, "United States Minor Outlying Islands");
const USA: CountryName = (Country::USA, "United States of America");
const VIR: CountryName = (Country::VIR, "United States Virgin Islands");
const URY: CountryName = (Country::URY, "Uruguay");
const UZB: CountryName = (Country::UZB, "Uzbekistan");
const VUT: CountryName = (Country::VUT, "Vanuatu");
const VEN: CountryName = (Country::VEN, "Venezuela (Bolivarian Republic of)");
const VNM: CountryName = (Country::VNM, "Viet Nam");
const WLF: CountryName = (Country::WLF, "Wallis and Futuna Islands");
const ESH: CountryName = (Country::ESH, "Western Sahara");
const YEM: CountryName = (Country::YEM, "Yemen");
const ZMB: CountryName = (Country::ZMB, "Zambia");
const ZWE: CountryName = (Country::ZWE, "Zimbabwe");

// Kosovo
const XKX: CountryName = (Country::XKX, "Kosovo");

const COUNTRY_NAME: [&CountryName; 250] = [
    &AFG, &ALA, &ALB, &DZA, &ASM, &AND, &AGO, &AIA, &ATA, &ATG, &ARG, &ARM, &ABW, &AUS, &AUT, &AZE,
    &BHS, &BHR, &BGD, &BRB, &BLR, &BEL, &BLZ, &BEN, &BMU, &BTN, &BOL, &BES, &BIH, &BWA, &BVT, &BRA,
    &IOT, &VGB, &BRN, &BGR, &BFA, &BDI, &CPV, &KHM, &CMR, &CAN, &CYM, &CAF, &TCD, &CHL, &CHN, &HKG,
    &MAC, &CXR, &CCK, &COL, &COM, &COG, &COK, &CRI, &HRV, &CUB, &CUW, &CYP, &CZE, &PRK, &COD, &DNK,
    &DJI, &DMA, &DOM, &ECU, &EGY, &SLV, &GNQ, &ERI, &EST, &SWZ, &ETH, &FLK, &FRO, &FJI, &FIN, &FRA,
    &GUF, &PYF, &ATF, &GAB, &GMB, &GEO, &DEU, &GHA, &GIB, &GRC, &GRL, &GRD, &GLP, &GUM, &GTM, &GGY,
    &GIN, &GNB, &GUY, &HTI, &HMD, &VAT, &HND, &HUN, &ISL, &IND, &IDN, &IRN, &IRQ, &IRL, &IMN, &ISR,
    &ITA, &CIV, &JAM, &JPN, &JEY, &JOR, &KAZ, &KEN, &KIR, &KWT, &KGZ, &LAO, &LVA, &LBN, &LSO, &LBR,
    &LBY, &LIE, &LTU, &LUX, &MDG, &MWI, &MYS, &MDV, &MLI, &MLT, &MHL, &MTQ, &MRT, &MUS, &MYT, &MEX,
    &FSM, &MCO, &MNG, &MNE, &MSR, &MAR, &MOZ, &MMR, &NAM, &NRU, &NPL, &NLD, &NCL, &NZL, &NIC, &NER,
    &NGA, &NIU, &NFK, &MNP, &MKD, &NOR, &OMN, &PAK, &PLW, &PAN, &PNG, &PRY, &PER, &PHL, &PCN, &POL,
    &PRT, &PRI, &QAT, &KOR, &MDA, &REU, &ROU, &RUS, &RWA, &BLM, &SHN, &KNA, &LCA, &MAF, &SPM, &VCT,
    &WSM, &SMR, &STP, &SAU, &SEN, &SRB, &SYC, &SLE, &SGP, &SXM, &SVK, &SVN, &SLB, &SOM, &ZAF, &SGS,
    &SSD, &ESP, &LKA, &PSE, &SDN, &SUR, &SJM, &SWE, &CHE, &SYR, &TWN, &TJK, &THA, &TLS, &TGO, &TKL,
    &TON, &TTO, &TUN, &TUR, &TKM, &TCA, &TUV, &UGA, &UKR, &ARE, &GBR, &TZA, &UMI, &USA, &VIR, &URY,
    &UZB, &VUT, &VEN, &VNM, &WLF, &ESH, &YEM, &ZMB, &ZWE, &XKX,
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
