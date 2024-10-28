use crate::country::Country;

enum IsoIso {
    Both(Country, &'static str),
    #[allow(dead_code)]
    Iso3(Country),
}

use IsoIso::*;

const AFG: IsoIso = Both(Country::AFG, "AF");
const ALA: IsoIso = Both(Country::ALA, "AX");
const ALB: IsoIso = Both(Country::ALB, "AL");
const DZA: IsoIso = Both(Country::DZA, "DZ");
const ASM: IsoIso = Both(Country::ASM, "AS");
const AND: IsoIso = Both(Country::AND, "AD");
const AGO: IsoIso = Both(Country::AGO, "AO");
const AIA: IsoIso = Both(Country::AIA, "AI");
const ATA: IsoIso = Both(Country::ATA, "AQ");
const ATG: IsoIso = Both(Country::ATG, "AG");
const ARG: IsoIso = Both(Country::ARG, "AR");
const ARM: IsoIso = Both(Country::ARM, "AM");
const ABW: IsoIso = Both(Country::ABW, "AW");
const AUS: IsoIso = Both(Country::AUS, "AU");
const AUT: IsoIso = Both(Country::AUT, "AT");
const AZE: IsoIso = Both(Country::AZE, "AZ");
const BHS: IsoIso = Both(Country::BHS, "BS");
const BHR: IsoIso = Both(Country::BHR, "BH");
const BGD: IsoIso = Both(Country::BGD, "BD");
const BRB: IsoIso = Both(Country::BRB, "BB");
const BLR: IsoIso = Both(Country::BLR, "BY");
const BEL: IsoIso = Both(Country::BEL, "BE");
const BLZ: IsoIso = Both(Country::BLZ, "BZ");
const BEN: IsoIso = Both(Country::BEN, "BJ");
const BMU: IsoIso = Both(Country::BMU, "BM");
const BTN: IsoIso = Both(Country::BTN, "BT");
const BOL: IsoIso = Both(Country::BOL, "BO");
const BES: IsoIso = Both(Country::BES, "BQ");
const BIH: IsoIso = Both(Country::BIH, "BA");
const BWA: IsoIso = Both(Country::BWA, "BW");
const BVT: IsoIso = Both(Country::BVT, "BV");
const BRA: IsoIso = Both(Country::BRA, "BR");
const IOT: IsoIso = Both(Country::IOT, "IO");
const VGB: IsoIso = Both(Country::VGB, "VG");
const BRN: IsoIso = Both(Country::BRN, "BN");
const BGR: IsoIso = Both(Country::BGR, "BG");
const BFA: IsoIso = Both(Country::BFA, "BF");
const BDI: IsoIso = Both(Country::BDI, "BI");
const CPV: IsoIso = Both(Country::CPV, "CV");
const KHM: IsoIso = Both(Country::KHM, "KH");
const CMR: IsoIso = Both(Country::CMR, "CM");
const CAN: IsoIso = Both(Country::CAN, "CA");
const CYM: IsoIso = Both(Country::CYM, "KY");
const CAF: IsoIso = Both(Country::CAF, "CF");
const TCD: IsoIso = Both(Country::TCD, "TD");
const CHL: IsoIso = Both(Country::CHL, "CL");
const CHN: IsoIso = Both(Country::CHN, "CN");
const HKG: IsoIso = Both(Country::HKG, "HK");
const MAC: IsoIso = Both(Country::MAC, "MO");
const CXR: IsoIso = Both(Country::CXR, "CX");
const CCK: IsoIso = Both(Country::CCK, "CC");
const COL: IsoIso = Both(Country::COL, "CO");
const COM: IsoIso = Both(Country::COM, "KM");
const COG: IsoIso = Both(Country::COG, "CG");
const COK: IsoIso = Both(Country::COK, "CK");
const CRI: IsoIso = Both(Country::CRI, "CR");
const HRV: IsoIso = Both(Country::HRV, "HR");
const CUB: IsoIso = Both(Country::CUB, "CU");
const CUW: IsoIso = Both(Country::CUW, "CW");
const CYP: IsoIso = Both(Country::CYP, "CY");
const CZE: IsoIso = Both(Country::CZE, "CZ");
const PRK: IsoIso = Both(Country::PRK, "KP");
const COD: IsoIso = Both(Country::COD, "CD");
const DNK: IsoIso = Both(Country::DNK, "DK");
const DJI: IsoIso = Both(Country::DJI, "DJ");
const DMA: IsoIso = Both(Country::DMA, "DM");
const DOM: IsoIso = Both(Country::DOM, "DO");
const ECU: IsoIso = Both(Country::ECU, "EC");
const EGY: IsoIso = Both(Country::EGY, "EG");
const SLV: IsoIso = Both(Country::SLV, "SV");
const GNQ: IsoIso = Both(Country::GNQ, "GQ");
const ERI: IsoIso = Both(Country::ERI, "ER");
const EST: IsoIso = Both(Country::EST, "EE");
const SWZ: IsoIso = Both(Country::SWZ, "SZ");
const ETH: IsoIso = Both(Country::ETH, "ET");
const FLK: IsoIso = Both(Country::FLK, "FK");
const FRO: IsoIso = Both(Country::FRO, "FO");
const FJI: IsoIso = Both(Country::FJI, "FJ");
const FIN: IsoIso = Both(Country::FIN, "FI");
const FRA: IsoIso = Both(Country::FRA, "FR");
const GUF: IsoIso = Both(Country::GUF, "GF");
const PYF: IsoIso = Both(Country::PYF, "PF");
const ATF: IsoIso = Both(Country::ATF, "TF");
const GAB: IsoIso = Both(Country::GAB, "GA");
const GMB: IsoIso = Both(Country::GMB, "GM");
const GEO: IsoIso = Both(Country::GEO, "GE");
const DEU: IsoIso = Both(Country::DEU, "DE");
const GHA: IsoIso = Both(Country::GHA, "GH");
const GIB: IsoIso = Both(Country::GIB, "GI");
const GRC: IsoIso = Both(Country::GRC, "GR");
const GRL: IsoIso = Both(Country::GRL, "GL");
const GRD: IsoIso = Both(Country::GRD, "GD");
const GLP: IsoIso = Both(Country::GLP, "GP");
const GUM: IsoIso = Both(Country::GUM, "GU");
const GTM: IsoIso = Both(Country::GTM, "GT");
const GGY: IsoIso = Both(Country::GGY, "GG");
const GIN: IsoIso = Both(Country::GIN, "GN");
const GNB: IsoIso = Both(Country::GNB, "GW");
const GUY: IsoIso = Both(Country::GUY, "GY");
const HTI: IsoIso = Both(Country::HTI, "HT");
const HMD: IsoIso = Both(Country::HMD, "HM");
const VAT: IsoIso = Both(Country::VAT, "VA");
const HND: IsoIso = Both(Country::HND, "HN");
const HUN: IsoIso = Both(Country::HUN, "HU");
const ISL: IsoIso = Both(Country::ISL, "IS");
const IND: IsoIso = Both(Country::IND, "IN");
const IDN: IsoIso = Both(Country::IDN, "ID");
const IRN: IsoIso = Both(Country::IRN, "IR");
const IRQ: IsoIso = Both(Country::IRQ, "IQ");
const IRL: IsoIso = Both(Country::IRL, "IE");
const IMN: IsoIso = Both(Country::IMN, "IM");
const ISR: IsoIso = Both(Country::ISR, "IL");
const ITA: IsoIso = Both(Country::ITA, "IT");
const CIV: IsoIso = Both(Country::CIV, "CI");
const JAM: IsoIso = Both(Country::JAM, "JM");
const JPN: IsoIso = Both(Country::JPN, "JP");
const JEY: IsoIso = Both(Country::JEY, "JE");
const JOR: IsoIso = Both(Country::JOR, "JO");
const KAZ: IsoIso = Both(Country::KAZ, "KZ");
const KEN: IsoIso = Both(Country::KEN, "KE");
const KIR: IsoIso = Both(Country::KIR, "KI");
const KWT: IsoIso = Both(Country::KWT, "KW");
const KGZ: IsoIso = Both(Country::KGZ, "KG");
const LAO: IsoIso = Both(Country::LAO, "LA");
const LVA: IsoIso = Both(Country::LVA, "LV");
const LBN: IsoIso = Both(Country::LBN, "LB");
const LSO: IsoIso = Both(Country::LSO, "LS");
const LBR: IsoIso = Both(Country::LBR, "LR");
const LBY: IsoIso = Both(Country::LBY, "LY");
const LIE: IsoIso = Both(Country::LIE, "LI");
const LTU: IsoIso = Both(Country::LTU, "LT");
const LUX: IsoIso = Both(Country::LUX, "LU");
const MDG: IsoIso = Both(Country::MDG, "MG");
const MWI: IsoIso = Both(Country::MWI, "MW");
const MYS: IsoIso = Both(Country::MYS, "MY");
const MDV: IsoIso = Both(Country::MDV, "MV");
const MLI: IsoIso = Both(Country::MLI, "ML");
const MLT: IsoIso = Both(Country::MLT, "MT");
const MHL: IsoIso = Both(Country::MHL, "MH");
const MTQ: IsoIso = Both(Country::MTQ, "MQ");
const MRT: IsoIso = Both(Country::MRT, "MR");
const MUS: IsoIso = Both(Country::MUS, "MU");
const MYT: IsoIso = Both(Country::MYT, "YT");
const MEX: IsoIso = Both(Country::MEX, "MX");
const FSM: IsoIso = Both(Country::FSM, "FM");
const MCO: IsoIso = Both(Country::MCO, "MC");
const MNG: IsoIso = Both(Country::MNG, "MN");
const MNE: IsoIso = Both(Country::MNE, "ME");
const MSR: IsoIso = Both(Country::MSR, "MS");
const MAR: IsoIso = Both(Country::MAR, "MA");
const MOZ: IsoIso = Both(Country::MOZ, "MZ");
const MMR: IsoIso = Both(Country::MMR, "MM");
const NAM: IsoIso = Iso3(Country::NAM);
const NRU: IsoIso = Both(Country::NRU, "NR");
const NPL: IsoIso = Both(Country::NPL, "NP");
const NLD: IsoIso = Both(Country::NLD, "NL");
const NCL: IsoIso = Both(Country::NCL, "NC");
const NZL: IsoIso = Both(Country::NZL, "NZ");
const NIC: IsoIso = Both(Country::NIC, "NI");
const NER: IsoIso = Both(Country::NER, "NE");
const NGA: IsoIso = Both(Country::NGA, "NG");
const NIU: IsoIso = Both(Country::NIU, "NU");
const NFK: IsoIso = Both(Country::NFK, "NF");
const MNP: IsoIso = Both(Country::MNP, "MP");
const MKD: IsoIso = Both(Country::MKD, "MK");
const NOR: IsoIso = Both(Country::NOR, "NO");
const OMN: IsoIso = Both(Country::OMN, "OM");
const PAK: IsoIso = Both(Country::PAK, "PK");
const PLW: IsoIso = Both(Country::PLW, "PW");
const PAN: IsoIso = Both(Country::PAN, "PA");
const PNG: IsoIso = Both(Country::PNG, "PG");
const PRY: IsoIso = Both(Country::PRY, "PY");
const PER: IsoIso = Both(Country::PER, "PE");
const PHL: IsoIso = Both(Country::PHL, "PH");
const PCN: IsoIso = Both(Country::PCN, "PN");
const POL: IsoIso = Both(Country::POL, "PL");
const PRT: IsoIso = Both(Country::PRT, "PT");
const PRI: IsoIso = Both(Country::PRI, "PR");
const QAT: IsoIso = Both(Country::QAT, "QA");
const KOR: IsoIso = Both(Country::KOR, "KR");
const MDA: IsoIso = Both(Country::MDA, "MD");
const REU: IsoIso = Both(Country::REU, "RE");
const ROU: IsoIso = Both(Country::ROU, "RO");
const RUS: IsoIso = Both(Country::RUS, "RU");
const RWA: IsoIso = Both(Country::RWA, "RW");
const BLM: IsoIso = Both(Country::BLM, "BL");
const SHN: IsoIso = Both(Country::SHN, "SH");
const KNA: IsoIso = Both(Country::KNA, "KN");
const LCA: IsoIso = Both(Country::LCA, "LC");
const MAF: IsoIso = Both(Country::MAF, "MF");
const SPM: IsoIso = Both(Country::SPM, "PM");
const VCT: IsoIso = Both(Country::VCT, "VC");
const WSM: IsoIso = Both(Country::WSM, "WS");
const SMR: IsoIso = Both(Country::SMR, "SM");
const STP: IsoIso = Both(Country::STP, "ST");
const SAU: IsoIso = Both(Country::SAU, "SA");
const SEN: IsoIso = Both(Country::SEN, "SN");
const SRB: IsoIso = Both(Country::SRB, "RS");
const SYC: IsoIso = Both(Country::SYC, "SC");
const SLE: IsoIso = Both(Country::SLE, "SL");
const SGP: IsoIso = Both(Country::SGP, "SG");
const SXM: IsoIso = Both(Country::SXM, "SX");
const SVK: IsoIso = Both(Country::SVK, "SK");
const SVN: IsoIso = Both(Country::SVN, "SI");
const SLB: IsoIso = Both(Country::SLB, "SB");
const SOM: IsoIso = Both(Country::SOM, "SO");
const ZAF: IsoIso = Both(Country::ZAF, "ZA");
const SGS: IsoIso = Both(Country::SGS, "GS");
const SSD: IsoIso = Both(Country::SSD, "SS");
const ESP: IsoIso = Both(Country::ESP, "ES");
const LKA: IsoIso = Both(Country::LKA, "LK");
const PSE: IsoIso = Both(Country::PSE, "PS");
const SDN: IsoIso = Both(Country::SDN, "SD");
const SUR: IsoIso = Both(Country::SUR, "SR");
const SJM: IsoIso = Both(Country::SJM, "SJ");
const SWE: IsoIso = Both(Country::SWE, "SE");
const CHE: IsoIso = Both(Country::CHE, "CH");
const SYR: IsoIso = Both(Country::SYR, "SY");
const TWN: IsoIso = Both(Country::TWN, "TW");
const TJK: IsoIso = Both(Country::TJK, "TJ");
const THA: IsoIso = Both(Country::THA, "TH");
const TLS: IsoIso = Both(Country::TLS, "TL");
const TGO: IsoIso = Both(Country::TGO, "TG");
const TKL: IsoIso = Both(Country::TKL, "TK");
const TON: IsoIso = Both(Country::TON, "TO");
const TTO: IsoIso = Both(Country::TTO, "TT");
const TUN: IsoIso = Both(Country::TUN, "TN");
const TUR: IsoIso = Both(Country::TUR, "TR");
const TKM: IsoIso = Both(Country::TKM, "TM");
const TCA: IsoIso = Both(Country::TCA, "TC");
const TUV: IsoIso = Both(Country::TUV, "TV");
const UGA: IsoIso = Both(Country::UGA, "UG");
const UKR: IsoIso = Both(Country::UKR, "UA");
const ARE: IsoIso = Both(Country::ARE, "AE");
const GBR: IsoIso = Both(Country::GBR, "GB");
const TZA: IsoIso = Both(Country::TZA, "TZ");
const UMI: IsoIso = Both(Country::UMI, "UM");
const USA: IsoIso = Both(Country::USA, "US");
const VIR: IsoIso = Both(Country::VIR, "VI");
const URY: IsoIso = Both(Country::URY, "UY");
const UZB: IsoIso = Both(Country::UZB, "UZ");
const VUT: IsoIso = Both(Country::VUT, "VU");
const VEN: IsoIso = Both(Country::VEN, "VE");
const VNM: IsoIso = Both(Country::VNM, "VN");
const WLF: IsoIso = Both(Country::WLF, "WF");
const ESH: IsoIso = Both(Country::ESH, "EH");
const YEM: IsoIso = Both(Country::YEM, "YE");
const ZMB: IsoIso = Both(Country::ZMB, "ZM");
const ZWE: IsoIso = Both(Country::ZWE, "ZW");

// Kosovo
const XKX: IsoIso = Both(Country::XKX, "XK");

const COUNTRY_ALPHA2: [&IsoIso; 250] = [
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

pub fn code_to_country(code: &str) -> Option<Country> {
    COUNTRY_ALPHA2.iter().find_map(|iso_iso| match iso_iso {
        Both(country, candidate) if *candidate == code => Some(*country),
        _ => None,
    })
}

pub fn country_to_code(country: Country) -> Option<&'static str> {
    COUNTRY_ALPHA2.iter().find_map(|iso_iso| match iso_iso {
        Both(candidate, alpha2) if *candidate == country => Some(*alpha2),
        _ => None,
    })
}
