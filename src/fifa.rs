enum IsoFifa {
    Same(Country),
    Different(Country, &'static str),
    #[allow(dead_code)]
    Iso(Country),
}

use IsoFifa::*;

use crate::{country::Country, iso_alpha3};

// full ISO 3166-1 mapping
const AFG: IsoFifa = Same(Country::AFG);
const ALA: IsoFifa = Different(Country::ALA, "ALD");
const ALB: IsoFifa = Same(Country::ALB);
const DZA: IsoFifa = Different(Country::DZA, "ALG");
const ASM: IsoFifa = Different(Country::ASM, "ASA");
const AND: IsoFifa = Same(Country::AND);
const AGO: IsoFifa = Different(Country::AGO, "ANG");
const AIA: IsoFifa = Same(Country::AIA);
const ATA: IsoFifa = Different(Country::ATA, "ROS");
const ATG: IsoFifa = Same(Country::ATG);
const ARG: IsoFifa = Same(Country::ARG);
const ARM: IsoFifa = Same(Country::ARM);
const ABW: IsoFifa = Different(Country::ABW, "ARU");
const AUS: IsoFifa = Same(Country::AUS);
const AUT: IsoFifa = Same(Country::AUT);
const AZE: IsoFifa = Same(Country::AZE);
const BHS: IsoFifa = Different(Country::BHS, "BAH");
const BHR: IsoFifa = Same(Country::BHR);
const BGD: IsoFifa = Different(Country::BGD, "BAN");
const BRB: IsoFifa = Same(Country::BRB);
const BLR: IsoFifa = Same(Country::BLR);
const BEL: IsoFifa = Same(Country::BEL);
const BLZ: IsoFifa = Same(Country::BLZ);
const BEN: IsoFifa = Same(Country::BEN);
const BMU: IsoFifa = Different(Country::BMU, "BER");
const BTN: IsoFifa = Different(Country::BTN, "BHU");
const BOL: IsoFifa = Same(Country::BOL);
const BES: IsoFifa = Different(Country::BES, "ANT");
const BIH: IsoFifa = Same(Country::BIH);
const BWA: IsoFifa = Different(Country::BWA, "BOT");
const BVT: IsoFifa = Iso(Country::BVT);
const BRA: IsoFifa = Same(Country::BRA);
const IOT: IsoFifa = Iso(Country::IOT);
const VGB: IsoFifa = Same(Country::VGB);
const BRN: IsoFifa = Different(Country::BRN, "BRU");
const BGR: IsoFifa = Different(Country::BGR, "BUL");
const BFA: IsoFifa = Same(Country::BFA);
const BDI: IsoFifa = Same(Country::BDI);
const CPV: IsoFifa = Same(Country::CPV);
const KHM: IsoFifa = Different(Country::KHM, "CAM");
const CMR: IsoFifa = Same(Country::CMR);
const CAN: IsoFifa = Same(Country::CAN);
const CYM: IsoFifa = Different(Country::CYM, "CAY");
const CAF: IsoFifa = Different(Country::CAF, "CTA");
const TCD: IsoFifa = Different(Country::TCD, "CHA");
const CHL: IsoFifa = Different(Country::CHL, "CHI");
const CHN: IsoFifa = Same(Country::CHN);
const HKG: IsoFifa = Same(Country::HKG);
const MAC: IsoFifa = Same(Country::MAC);
const CXR: IsoFifa = Same(Country::CXR);
const CCK: IsoFifa = Same(Country::CCK);
const COL: IsoFifa = Same(Country::COL);
const COM: IsoFifa = Same(Country::COM);
const COG: IsoFifa = Different(Country::COG, "CGO");
const COK: IsoFifa = Same(Country::COK);
const CRI: IsoFifa = Different(Country::CRI, "CRC");
const HRV: IsoFifa = Different(Country::HRV, "CRO");
const CUB: IsoFifa = Same(Country::CUB);
const CUW: IsoFifa = Iso(Country::CUW);
const CYP: IsoFifa = Same(Country::CYP);
const CZE: IsoFifa = Same(Country::CZE);
const PRK: IsoFifa = Same(Country::PRK);
const COD: IsoFifa = Same(Country::COD);
const DNK: IsoFifa = Different(Country::DNK, "DEN");
const DJI: IsoFifa = Same(Country::DJI);
const DMA: IsoFifa = Same(Country::DMA);
const DOM: IsoFifa = Same(Country::DOM);
const ECU: IsoFifa = Same(Country::ECU);
const EGY: IsoFifa = Same(Country::EGY);
const SLV: IsoFifa = Same(Country::SLV);
const GNQ: IsoFifa = Different(Country::GNQ, "EQG");
const ERI: IsoFifa = Same(Country::ERI);
const EST: IsoFifa = Same(Country::EST);
const SWZ: IsoFifa = Same(Country::SWZ);
const ETH: IsoFifa = Same(Country::ETH);
const FLK: IsoFifa = Same(Country::FLK);
const FRO: IsoFifa = Same(Country::FRO);
const FJI: IsoFifa = Different(Country::FJI, "FIJ");
const FIN: IsoFifa = Same(Country::FIN);
const FRA: IsoFifa = Same(Country::FRA);
const GUF: IsoFifa = Same(Country::GUF);
const PYF: IsoFifa = Different(Country::PYF, "TAH");
const ATF: IsoFifa = Iso(Country::ATF);
const GAB: IsoFifa = Same(Country::GAB);
const GMB: IsoFifa = Different(Country::GMB, "GAM");
const GEO: IsoFifa = Same(Country::GEO);
const DEU: IsoFifa = Different(Country::DEU, "GER");
const GHA: IsoFifa = Same(Country::GHA);
const GIB: IsoFifa = Different(Country::GIB, "GBZ");
const GRC: IsoFifa = Different(Country::GRC, "GRE");
const GRL: IsoFifa = Same(Country::GRL);
const GRD: IsoFifa = Different(Country::GRD, "GRN");
const GLP: IsoFifa = Different(Country::GLP, "GLP");
const GUM: IsoFifa = Same(Country::GUM);
const GTM: IsoFifa = Different(Country::GTM, "GUA");
const GGY: IsoFifa = Different(Country::GGY, "GBG");
const GIN: IsoFifa = Different(Country::GIN, "GUI");
const GNB: IsoFifa = Same(Country::GNB);
const GUY: IsoFifa = Different(Country::GUY, "GUY");
const HTI: IsoFifa = Different(Country::HTI, "HAI");
const HMD: IsoFifa = Iso(Country::HMD);
const VAT: IsoFifa = Same(Country::VAT);
const HND: IsoFifa = Different(Country::HND, "HON");
const HUN: IsoFifa = Same(Country::HUN);
const ISL: IsoFifa = Different(Country::ISL, "ISL");
const IND: IsoFifa = Same(Country::IND);
const IDN: IsoFifa = Same(Country::IDN);
const IRN: IsoFifa = Same(Country::IRN);
const IRQ: IsoFifa = Same(Country::IRQ);
const IRL: IsoFifa = Same(Country::IRL);
const IMN: IsoFifa = Different(Country::IMN, "GBM");
const ISR: IsoFifa = Same(Country::ISR);
const ITA: IsoFifa = Same(Country::ITA);
const CIV: IsoFifa = Same(Country::CIV);
const JAM: IsoFifa = Same(Country::JAM);
const JPN: IsoFifa = Same(Country::JPN);
const JEY: IsoFifa = Different(Country::JEY, "GBJ");
const JOR: IsoFifa = Same(Country::JOR);
const KAZ: IsoFifa = Same(Country::KAZ);
const KEN: IsoFifa = Same(Country::KEN);
const KIR: IsoFifa = Same(Country::KIR);
const KWT: IsoFifa = Different(Country::KWT, "KUW");
const KGZ: IsoFifa = Same(Country::KGZ);
const LAO: IsoFifa = Same(Country::LAO);
const LVA: IsoFifa = Same(Country::LVA);
const LBN: IsoFifa = Different(Country::LBN, "LIB");
const LSO: IsoFifa = Different(Country::LSO, "LES");
const LBR: IsoFifa = Same(Country::LBR);
const LBY: IsoFifa = Same(Country::LBY);
const LIE: IsoFifa = Same(Country::LIE);
const LTU: IsoFifa = Same(Country::LTU);
const LUX: IsoFifa = Same(Country::LUX);
const MDG: IsoFifa = Different(Country::MDG, "MAD");
const MWI: IsoFifa = Same(Country::MWI);
const MYS: IsoFifa = Different(Country::MYS, "MAS");
const MDV: IsoFifa = Same(Country::MDV);
const MLI: IsoFifa = Same(Country::MLI);
const MLT: IsoFifa = Same(Country::MLT);
const MHL: IsoFifa = Same(Country::MHL);
const MTQ: IsoFifa = Same(Country::MTQ);
const MRT: IsoFifa = Different(Country::MRT, "MTN");
const MUS: IsoFifa = Different(Country::MUS, "MRI");
const MYT: IsoFifa = Same(Country::MYT);
const MEX: IsoFifa = Same(Country::MEX);
const FSM: IsoFifa = Same(Country::FSM);
const MCO: IsoFifa = Different(Country::MCO, "MON");
const MNG: IsoFifa = Same(Country::MNG);
const MNE: IsoFifa = Same(Country::MNE);
const MSR: IsoFifa = Same(Country::MSR);
const MAR: IsoFifa = Same(Country::MAR);
const MOZ: IsoFifa = Same(Country::MOZ);
const MMR: IsoFifa = Different(Country::MMR, "MYA");
const NAM: IsoFifa = Same(Country::NAM);
const NRU: IsoFifa = Same(Country::NRU);
const NPL: IsoFifa = Different(Country::NPL, "NEP");
const NLD: IsoFifa = Different(Country::NLD, "NED");
const NCL: IsoFifa = Same(Country::NCL);
const NZL: IsoFifa = Same(Country::NZL);
const NIC: IsoFifa = Different(Country::NIC, "NCA");
const NER: IsoFifa = Different(Country::NER, "NIG");
const NGA: IsoFifa = Same(Country::NGA);
const NIU: IsoFifa = Same(Country::NIU);
const NFK: IsoFifa = Same(Country::NFK);
const MNP: IsoFifa = Different(Country::MNP, "NMI");
const MKD: IsoFifa = Same(Country::MKD);
const NOR: IsoFifa = Same(Country::NOR);
const OMN: IsoFifa = Different(Country::OMN, "OMA");
const PAK: IsoFifa = Same(Country::PAK);
const PLW: IsoFifa = Same(Country::PLW);
const PAN: IsoFifa = Same(Country::PAN);
const PNG: IsoFifa = Same(Country::PNG);
const PRY: IsoFifa = Different(Country::PRY, "PAR");
const PER: IsoFifa = Same(Country::PER);
const PHL: IsoFifa = Different(Country::PHL, "PHI");
const PCN: IsoFifa = Same(Country::PCN);
const POL: IsoFifa = Same(Country::POL);
const PRT: IsoFifa = Different(Country::PRT, "POR");
const PRI: IsoFifa = Different(Country::PRI, "PUR");
const QAT: IsoFifa = Same(Country::QAT);
const KOR: IsoFifa = Same(Country::KOR);
const MDA: IsoFifa = Same(Country::MDA);
const REU: IsoFifa = Same(Country::REU);
const ROU: IsoFifa = Same(Country::ROU);
const RUS: IsoFifa = Same(Country::RUS);
const RWA: IsoFifa = Same(Country::RWA);
const BLM: IsoFifa = Iso(Country::BLM);
const SHN: IsoFifa = Same(Country::SHN);
const KNA: IsoFifa = Different(Country::KNA, "SKN");
const LCA: IsoFifa = Same(Country::LCA);
const MAF: IsoFifa = Iso(Country::MAF);
const SPM: IsoFifa = Same(Country::SPM);
const VCT: IsoFifa = Different(Country::VCT, "VIN");
const WSM: IsoFifa = Different(Country::WSM, "SAM");
const SMR: IsoFifa = Same(Country::SMR);
const STP: IsoFifa = Same(Country::STP);
const SAU: IsoFifa = Different(Country::SAU, "KSA");
const SEN: IsoFifa = Same(Country::SEN);
const SRB: IsoFifa = Same(Country::SRB);
const SYC: IsoFifa = Different(Country::SYC, "SEY");
const SLE: IsoFifa = Same(Country::SLE);
const SGP: IsoFifa = Different(Country::SGP, "SIN");
const SXM: IsoFifa = Iso(Country::SXM);
const SVK: IsoFifa = Same(Country::SVK);
const SVN: IsoFifa = Same(Country::SVN);
const SLB: IsoFifa = Different(Country::SLB, "SOL");
const SOM: IsoFifa = Same(Country::SOM);
const ZAF: IsoFifa = Different(Country::ZAF, "RSA");
const SGS: IsoFifa = Iso(Country::SGS);
const SSD: IsoFifa = Iso(Country::SSD);
const ESP: IsoFifa = Same(Country::ESP);
const LKA: IsoFifa = Different(Country::LKA, "SRI");
const PSE: IsoFifa = Different(Country::PSE, "PLE");
const SDN: IsoFifa = Different(Country::SDN, "SUD");
const SUR: IsoFifa = Same(Country::SUR);
const SJM: IsoFifa = Iso(Country::SJM);
const SWE: IsoFifa = Same(Country::SWE);
const CHE: IsoFifa = Different(Country::CHE, "SUI");
const SYR: IsoFifa = Same(Country::SYR);
const TWN: IsoFifa = Different(Country::TWN, "TPE");
const TJK: IsoFifa = Same(Country::TJK);
const THA: IsoFifa = Same(Country::THA);
const TLS: IsoFifa = Same(Country::TLS);
const TGO: IsoFifa = Different(Country::TGO, "TOG");
const TKL: IsoFifa = Same(Country::TKL);
const TON: IsoFifa = Different(Country::TON, "TGA");
const TTO: IsoFifa = Different(Country::TTO, "TRI");
const TUN: IsoFifa = Same(Country::TUN);
const TUR: IsoFifa = Same(Country::TUR);
const TKM: IsoFifa = Same(Country::TKM);
const TCA: IsoFifa = Same(Country::TCA);
const TUV: IsoFifa = Same(Country::TUV);
const UGA: IsoFifa = Same(Country::UGA);
const UKR: IsoFifa = Same(Country::UKR);
const ARE: IsoFifa = Different(Country::ARE, "UAE");
const GBR: IsoFifa = Iso(Country::GBR);
const TZA: IsoFifa = Different(Country::TZA, "TAN");
const UMI: IsoFifa = Iso(Country::UMI);
const USA: IsoFifa = Same(Country::USA);
const VIR: IsoFifa = Same(Country::VIR);
const URY: IsoFifa = Different(Country::URY, "URU");
const UZB: IsoFifa = Same(Country::UZB);
const VUT: IsoFifa = Different(Country::VUT, "VAN");
const VEN: IsoFifa = Same(Country::VEN);
const VNM: IsoFifa = Different(Country::VNM, "VIE");
const WLF: IsoFifa = Same(Country::WLF);
const ESH: IsoFifa = Different(Country::ESH, "SAH");
const YEM: IsoFifa = Same(Country::YEM);
const ZMB: IsoFifa = Different(Country::ZMB, "ZAM");
const ZWE: IsoFifa = Different(Country::ZWE, "ZIM");

// FIFA special codes
pub const KOS: &str = "KOS";
const KOS_FIFA: IsoFifa = Different(Country::XKX, KOS);

pub const ENG: &str = "ENG";
const ENG_FIFA: IsoFifa = Different(Country::ENG, ENG);

pub const NIR: &str = "NIR";
const NIR_FIFA: IsoFifa = Different(Country::NIR, NIR);

pub const SCO: &str = "SCO";
const SCO_FIFA: IsoFifa = Different(Country::SCO, SCO);

pub const WAL: &str = "WAL";
const WAL_FIFA: IsoFifa = Different(Country::WAL, WAL);

const COUNTRY_FIFA: [&'static IsoFifa; 254] = [
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
    &UZB, &VUT, &VEN, &VNM, &WLF, &ESH, &YEM, &ZMB, &ZWE, &KOS_FIFA, &ENG_FIFA, &NIR_FIFA,
    &SCO_FIFA, &WAL_FIFA,
];

pub fn code_to_country(code: &str) -> Option<Country> {
    COUNTRY_FIFA.iter().find_map(|iso_ioc| match iso_ioc {
        Same(country) if iso_alpha3::country_to_code(*country) == Some(code) => Some(*country),
        Different(country, candidate) if *candidate == code => Some(*country),
        _ => None,
    })
}

pub fn country_to_code(country: Country) -> Option<&'static str> {
    COUNTRY_FIFA.iter().find_map(|iso_ioc| match iso_ioc {
        Same(candidate) if *candidate == country => iso_alpha3::country_to_code(country),
        Different(candidate, ioc) if *candidate == country => Some(*ioc),
        _ => None,
    })
}
