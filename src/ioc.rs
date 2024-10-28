enum IsoIoc {
    Same(Country),
    Different(Country, &'static str),
    #[allow(dead_code)]
    Iso(Country),
}

use IsoIoc::*;

use crate::{country::Country, iso_alpha3};

// full ISO 3166-1 mapping
const AFG: IsoIoc = Same(Country::AFG);
const ALA: IsoIoc = Iso(Country::ALA);
const ALB: IsoIoc = Same(Country::ALB);
const DZA: IsoIoc = Different(Country::DZA, "ALG");
const ASM: IsoIoc = Different(Country::ASM, "ASA");
const AND: IsoIoc = Same(Country::AND);
const AGO: IsoIoc = Different(Country::AGO, "ANG");
const AHO: IsoIoc = Different(Country::ANT, "AHO");
const AIA: IsoIoc = Same(Country::AIA);
const ATA: IsoIoc = Same(Country::ATA);
const ATG: IsoIoc = Different(Country::ATG, "ANT");
const ARG: IsoIoc = Same(Country::ARG);
const ARM: IsoIoc = Same(Country::ARM);
const ABW: IsoIoc = Different(Country::ABW, "ARU");
const AUS: IsoIoc = Same(Country::AUS);
const AUT: IsoIoc = Same(Country::AUT);
const AZE: IsoIoc = Same(Country::AZE);
const BHS: IsoIoc = Different(Country::BHS, "BAH");
const BHR: IsoIoc = Different(Country::BHR, "BRN");
const BGD: IsoIoc = Different(Country::BGD, "BAN");
const BRB: IsoIoc = Different(Country::BRB, "BAR");
const BLR: IsoIoc = Same(Country::BLR);
const BEL: IsoIoc = Same(Country::BEL);
const BLZ: IsoIoc = Different(Country::BLZ, "BIZ");
const BEN: IsoIoc = Same(Country::BEN);
const BMU: IsoIoc = Different(Country::BMU, "BER");
const BTN: IsoIoc = Different(Country::BTN, "BHU");
const BOL: IsoIoc = Same(Country::BOL);
const BES: IsoIoc = Different(Country::BES, "AHO");
const BIH: IsoIoc = Same(Country::BIH);
const BWA: IsoIoc = Different(Country::BWA, "BOT");
const BVT: IsoIoc = Same(Country::BVT);
const BRA: IsoIoc = Same(Country::BRA);
const IOT: IsoIoc = Iso(Country::IOT);
const VGB: IsoIoc = Different(Country::VGB, "IVB");
const BRN: IsoIoc = Different(Country::BRN, "BRU");
const BGR: IsoIoc = Different(Country::BGR, "BUL");
const BFA: IsoIoc = Different(Country::BFA, "BUR");
const BDI: IsoIoc = Same(Country::BDI);
const CPV: IsoIoc = Same(Country::CPV);
const KHM: IsoIoc = Different(Country::KHM, "CAM");
const CMR: IsoIoc = Same(Country::CMR);
const CAN: IsoIoc = Same(Country::CAN);
const CYM: IsoIoc = Different(Country::CYM, "CAY");
const CAF: IsoIoc = Same(Country::CAF);
const TCD: IsoIoc = Different(Country::TCD, "CHA");
const CHL: IsoIoc = Different(Country::CHL, "CHI");
const CHN: IsoIoc = Same(Country::CHN);
const HKG: IsoIoc = Same(Country::HKG);
const MAC: IsoIoc = Same(Country::MAC);
const CXR: IsoIoc = Iso(Country::CXR);
const CCK: IsoIoc = Iso(Country::CCK);
const COL: IsoIoc = Same(Country::COL);
const COM: IsoIoc = Same(Country::COM);
const COG: IsoIoc = Different(Country::COG, "CGO");
const COK: IsoIoc = Same(Country::COK);
const CRI: IsoIoc = Different(Country::CRI, "CRC");
const HRV: IsoIoc = Different(Country::HRV, "CRO");
const CUB: IsoIoc = Same(Country::CUB);
const CUW: IsoIoc = Iso(Country::CUW);
const CYP: IsoIoc = Same(Country::CYP);
const CZE: IsoIoc = Same(Country::CZE);
const PRK: IsoIoc = Same(Country::PRK);
const COD: IsoIoc = Same(Country::COD);
const DNK: IsoIoc = Different(Country::DNK, "DEN");
const DJI: IsoIoc = Same(Country::DJI);
const DMA: IsoIoc = Same(Country::DMA);
const DOM: IsoIoc = Same(Country::DOM);
const ECU: IsoIoc = Same(Country::ECU);
const EGY: IsoIoc = Same(Country::EGY);
const SLV: IsoIoc = Different(Country::SLV, "ESA");
const GNQ: IsoIoc = Different(Country::GNQ, "GEQ");
const ERI: IsoIoc = Same(Country::ERI);
const EST: IsoIoc = Same(Country::EST);
const SWZ: IsoIoc = Same(Country::SWZ);
const ETH: IsoIoc = Same(Country::ETH);
const FLK: IsoIoc = Same(Country::FLK);
const FRO: IsoIoc = Different(Country::FRO, "FAR");
const FJI: IsoIoc = Different(Country::FJI, "FIJ");
const FIN: IsoIoc = Same(Country::FIN);
const FRA: IsoIoc = Same(Country::FRA);
const GUF: IsoIoc = Different(Country::GUF, "FGU");
const PYF: IsoIoc = Different(Country::PYF, "FPO");
const ATF: IsoIoc = Iso(Country::ATF);
const GAB: IsoIoc = Same(Country::GAB);
const GMB: IsoIoc = Different(Country::GMB, "GAM");
const GEO: IsoIoc = Same(Country::GEO);
const DEU: IsoIoc = Different(Country::DEU, "GER");
const GHA: IsoIoc = Same(Country::GHA);
const GIB: IsoIoc = Different(Country::GIB, "GIB");
const GRC: IsoIoc = Different(Country::GRC, "GRE");
const GRL: IsoIoc = Different(Country::GRL, "GRL");
const GRD: IsoIoc = Different(Country::GRD, "GRN");
const GLP: IsoIoc = Different(Country::GLP, "GUD");
const GUM: IsoIoc = Same(Country::GUM);
const GTM: IsoIoc = Different(Country::GTM, "GUA");
const GGY: IsoIoc = Iso(Country::GGY);
const GIN: IsoIoc = Different(Country::GIN, "GUI");
const GNB: IsoIoc = Different(Country::GNB, "GBS");
const GUY: IsoIoc = Different(Country::GUY, "GUY");
const HTI: IsoIoc = Different(Country::HTI, "HAI");
const HMD: IsoIoc = Same(Country::HMD);
const VAT: IsoIoc = Iso(Country::VAT);
const HND: IsoIoc = Different(Country::HND, "HON");
const HUN: IsoIoc = Same(Country::HUN);
const ISL: IsoIoc = Same(Country::ISL);
const IND: IsoIoc = Same(Country::IND);
const IDN: IsoIoc = Different(Country::IDN, "INA");
const IRN: IsoIoc = Different(Country::IRN, "IRI");
const IRQ: IsoIoc = Same(Country::IRQ);
const IRL: IsoIoc = Same(Country::IRL);
const IMN: IsoIoc = Iso(Country::IMN);
const ISR: IsoIoc = Same(Country::ISR);
const ITA: IsoIoc = Same(Country::ITA);
const CIV: IsoIoc = Same(Country::CIV);
const JAM: IsoIoc = Same(Country::JAM);
const JPN: IsoIoc = Same(Country::JPN);
const JEY: IsoIoc = Iso(Country::JEY);
const JOR: IsoIoc = Same(Country::JOR);
const KAZ: IsoIoc = Same(Country::KAZ);
const KEN: IsoIoc = Same(Country::KEN);
const KIR: IsoIoc = Same(Country::KIR);
const KWT: IsoIoc = Different(Country::KWT, "KUW");
const KGZ: IsoIoc = Same(Country::KGZ);
const LAO: IsoIoc = Same(Country::LAO);
const LVA: IsoIoc = Different(Country::LVA, "LAT");
const LBN: IsoIoc = Different(Country::LBN, "LIB");
const LSO: IsoIoc = Different(Country::LSO, "LES");
const LBR: IsoIoc = Same(Country::LBR);
const LBY: IsoIoc = Different(Country::LBY, "LBA");
const LIE: IsoIoc = Same(Country::LIE);
const LTU: IsoIoc = Same(Country::LTU);
const LUX: IsoIoc = Same(Country::LUX);
const MDG: IsoIoc = Different(Country::MDG, "MAD");
const MWI: IsoIoc = Different(Country::MWI, "MAW");
const MYS: IsoIoc = Different(Country::MYS, "MAS");
const MDV: IsoIoc = Same(Country::MDV);
const MLI: IsoIoc = Same(Country::MLI);
const MLT: IsoIoc = Same(Country::MLT);
const MHL: IsoIoc = Different(Country::MHL, "MSH");
const MTQ: IsoIoc = Different(Country::MTQ, "MRT");
const MRT: IsoIoc = Different(Country::MRT, "MTN");
const MUS: IsoIoc = Different(Country::MUS, "MRI");
const MYT: IsoIoc = Different(Country::MYT, "MAY");
const MEX: IsoIoc = Same(Country::MEX);
const FSM: IsoIoc = Same(Country::FSM);
const MCO: IsoIoc = Different(Country::MCO, "MON");
const MNG: IsoIoc = Different(Country::MNG, "MGL");
const MNE: IsoIoc = Different(Country::MNE, "MGO");
const MSR: IsoIoc = Different(Country::MSR, "MNT");
const MAR: IsoIoc = Same(Country::MAR);
const MOZ: IsoIoc = Same(Country::MOZ);
const MMR: IsoIoc = Different(Country::MMR, "MYA");
const NAM: IsoIoc = Same(Country::NAM);
const NRU: IsoIoc = Same(Country::NRU);
const NPL: IsoIoc = Different(Country::NPL, "NEP");
const NLD: IsoIoc = Different(Country::NLD, "NED");
const NCL: IsoIoc = Different(Country::NCL, "NCD");
const NZL: IsoIoc = Same(Country::NZL);
const NIC: IsoIoc = Different(Country::NIC, "NCA");
const NER: IsoIoc = Different(Country::NER, "NIG");
const NGA: IsoIoc = Different(Country::NGA, "NGR");
const NIU: IsoIoc = Same(Country::NIU);
const NFK: IsoIoc = Different(Country::NFK, "NFI");
const MNP: IsoIoc = Different(Country::MNP, "NMA");
const MKD: IsoIoc = Same(Country::MKD);
const NOR: IsoIoc = Same(Country::NOR);
const OMN: IsoIoc = Different(Country::OMN, "OMA");
const PAK: IsoIoc = Same(Country::PAK);
const PLW: IsoIoc = Same(Country::PLW);
const PAN: IsoIoc = Same(Country::PAN);
const PNG: IsoIoc = Same(Country::PNG);
const PRY: IsoIoc = Different(Country::PRY, "PAR");
const PER: IsoIoc = Same(Country::PER);
const PHL: IsoIoc = Different(Country::PHL, "PHI");
const PCN: IsoIoc = Same(Country::PCN);
const POL: IsoIoc = Same(Country::POL);
const PRT: IsoIoc = Different(Country::PRT, "POR");
const PRI: IsoIoc = Different(Country::PRI, "PUR");
const QAT: IsoIoc = Different(Country::QAT, "QAT");
const KOR: IsoIoc = Same(Country::KOR);
const MDA: IsoIoc = Same(Country::MDA);
const REU: IsoIoc = Same(Country::REU);
const ROU: IsoIoc = Same(Country::ROU);
const RUS: IsoIoc = Same(Country::RUS);
const RWA: IsoIoc = Same(Country::RWA);
const BLM: IsoIoc = Iso(Country::BLM);
const SHN: IsoIoc = Different(Country::SHN, "HEL");
const KNA: IsoIoc = Different(Country::KNA, "SKN");
const LCA: IsoIoc = Same(Country::LCA);
const MAF: IsoIoc = Iso(Country::MAF);
const SPM: IsoIoc = Same(Country::SPM);
const VCT: IsoIoc = Different(Country::VCT, "VIN");
const WSM: IsoIoc = Different(Country::WSM, "SAM");
const SMR: IsoIoc = Same(Country::SMR);
const STP: IsoIoc = Same(Country::STP);
const SAU: IsoIoc = Different(Country::SAU, "KSA");
const SEN: IsoIoc = Same(Country::SEN);
const SRB: IsoIoc = Same(Country::SRB);
const SYC: IsoIoc = Same(Country::SYC);
const SLE: IsoIoc = Same(Country::SLE);
const SGP: IsoIoc = Different(Country::SGP, "SIN");
const SXM: IsoIoc = Iso(Country::SXM);
const SVK: IsoIoc = Same(Country::SVK);
const SVN: IsoIoc = Different(Country::SVN, "SLO");
const SLB: IsoIoc = Different(Country::SLB, "SOL");
const SOM: IsoIoc = Same(Country::SOM);
const ZAF: IsoIoc = Different(Country::ZAF, "RSA");
const SGS: IsoIoc = Same(Country::SGS);
const SSD: IsoIoc = Iso(Country::SSD);
const ESP: IsoIoc = Same(Country::ESP);
const LKA: IsoIoc = Different(Country::LKA, "SRI");
const PSE: IsoIoc = Different(Country::PSE, "PLE");
const SDN: IsoIoc = Different(Country::SDN, "SUD");
const SUR: IsoIoc = Same(Country::SUR);
const SJM: IsoIoc = Iso(Country::SJM);
const SWE: IsoIoc = Same(Country::SWE);
const CHE: IsoIoc = Different(Country::CHE, "SUI");
const SYR: IsoIoc = Same(Country::SYR);
const TWN: IsoIoc = Different(Country::TWN, "TPE");
const TJK: IsoIoc = Different(Country::TJK, "TJK");
const THA: IsoIoc = Same(Country::THA);
const TLS: IsoIoc = Same(Country::TLS);
const TGO: IsoIoc = Different(Country::TGO, "TOG");
const TKL: IsoIoc = Same(Country::TKL);
const TON: IsoIoc = Different(Country::TON, "TGA");
const TTO: IsoIoc = Same(Country::TTO);
const TUN: IsoIoc = Same(Country::TUN);
const TUR: IsoIoc = Same(Country::TUR);
const TKM: IsoIoc = Same(Country::TKM);
const TCA: IsoIoc = Different(Country::TCA, "TKS");
const TUV: IsoIoc = Same(Country::TUV);
const UGA: IsoIoc = Same(Country::UGA);
const UKR: IsoIoc = Same(Country::UKR);
const ARE: IsoIoc = Different(Country::ARE, "UAE");
const GBR: IsoIoc = Same(Country::GBR);
const TZA: IsoIoc = Different(Country::TZA, "TAN");
const UMI: IsoIoc = Iso(Country::UMI);
const USA: IsoIoc = Same(Country::USA);
const VIR: IsoIoc = Different(Country::VIR, "ISV");
const URY: IsoIoc = Different(Country::URY, "URU");
const UZB: IsoIoc = Same(Country::UZB);
const VUT: IsoIoc = Different(Country::VUT, "VAN");
const VEN: IsoIoc = Same(Country::VEN);
const VNM: IsoIoc = Different(Country::VNM, "VIE");
const WLF: IsoIoc = Different(Country::WLF, "WAF");
const ESH: IsoIoc = Same(Country::ESH);
const YEM: IsoIoc = Same(Country::YEM);
const YUG: IsoIoc = Same(Country::YUG);
const ZMB: IsoIoc = Different(Country::ZMB, "ZAM");
const ZWE: IsoIoc = Different(Country::ZWE, "ZIM");

// IOC special codes
pub const KOS: &str = "KOS";
const KOS_IOC: IsoIoc = Different(Country::XKX, KOS);

pub const AIN: &str = "AIN";
const AIN_IOC: IsoIoc = Different(Country::AIN, AIN);

pub const EOR: &str = "EOR";
const EOR_IOC: IsoIoc = Different(Country::EOR, EOR);

pub const NPA: &str = "NPA";
const NPA_IOC: IsoIoc = Different(Country::NPA, NPA);

pub const ROC: &str = "ROC";
const ROC_IOC: IsoIoc = Different(Country::ROC, ROC);

pub const RPC: &str = "RPC";
const RPC_IOC: IsoIoc = Different(Country::RPC, RPC);

pub const RPT: &str = "RPT";
const RPT_IOC: IsoIoc = Different(Country::RPT, RPT);

const COUNTRY_IOC: [&'static IsoIoc; 258] = [
    &AFG, &ALA, &ALB, &DZA, &ASM, &AND, &AGO, &AHO, &AIA, &ATA, &ATG, &ARG, &ARM, &ABW, &AUS, &AUT,
    &AZE, &BHS, &BHR, &BGD, &BRB, &BLR, &BEL, &BLZ, &BEN, &BMU, &BTN, &BOL, &BES, &BIH, &BWA, &BVT,
    &BRA, &IOT, &VGB, &BRN, &BGR, &BFA, &BDI, &CPV, &KHM, &CMR, &CAN, &CYM, &CAF, &TCD, &CHL, &CHN,
    &HKG, &MAC, &CXR, &CCK, &COL, &COM, &COG, &COK, &CRI, &HRV, &CUB, &CUW, &CYP, &CZE, &PRK, &COD,
    &DNK, &DJI, &DMA, &DOM, &ECU, &EGY, &SLV, &GNQ, &ERI, &EST, &SWZ, &ETH, &FLK, &FRO, &FJI, &FIN,
    &FRA, &GUF, &PYF, &ATF, &GAB, &GMB, &GEO, &DEU, &GHA, &GIB, &GRC, &GRL, &GRD, &GLP, &GUM, &GTM,
    &GGY, &GIN, &GNB, &GUY, &HTI, &HMD, &VAT, &HND, &HUN, &ISL, &IND, &IDN, &IRN, &IRQ, &IRL, &IMN,
    &ISR, &ITA, &CIV, &JAM, &JPN, &JEY, &JOR, &KAZ, &KEN, &KIR, &KWT, &KGZ, &LAO, &LVA, &LBN, &LSO,
    &LBR, &LBY, &LIE, &LTU, &LUX, &MDG, &MWI, &MYS, &MDV, &MLI, &MLT, &MHL, &MTQ, &MRT, &MUS, &MYT,
    &MEX, &FSM, &MCO, &MNG, &MNE, &MSR, &MAR, &MOZ, &MMR, &NAM, &NRU, &NPL, &NLD, &NCL, &NZL, &NIC,
    &NER, &NGA, &NIU, &NFK, &MNP, &MKD, &NOR, &OMN, &PAK, &PLW, &PAN, &PNG, &PRY, &PER, &PHL, &PCN,
    &POL, &PRT, &PRI, &QAT, &KOR, &MDA, &REU, &ROU, &RUS, &RWA, &BLM, &SHN, &KNA, &LCA, &MAF, &SPM,
    &VCT, &WSM, &SMR, &STP, &SAU, &SEN, &SRB, &SYC, &SLE, &SGP, &SXM, &SVK, &SVN, &SLB, &SOM, &ZAF,
    &SGS, &SSD, &ESP, &LKA, &PSE, &SDN, &SUR, &SJM, &SWE, &CHE, &SYR, &TWN, &TJK, &THA, &TLS, &TGO,
    &TKL, &TON, &TTO, &TUN, &TUR, &TKM, &TCA, &TUV, &UGA, &UKR, &ARE, &GBR, &TZA, &UMI, &USA, &VIR,
    &URY, &UZB, &VUT, &VEN, &VNM, &WLF, &ESH, &YEM, &YUG, &ZMB, &ZWE, &AIN_IOC, &EOR_IOC, &NPA_IOC,
    &ROC_IOC, &RPC_IOC, &RPT_IOC, &KOS_IOC,
];

pub fn code_to_country(code: &str) -> Option<Country> {
    COUNTRY_IOC.iter().find_map(|iso_ioc| match iso_ioc {
        Same(country) if iso_alpha3::country_to_code(*country) == Some(code) => Some(*country),
        Different(country, candidate) if *candidate == code => Some(*country),
        _ => None,
    })
}

pub fn country_to_code(country: Country) -> Option<&'static str> {
    COUNTRY_IOC.iter().find_map(|iso_ioc| match iso_ioc {
        Same(candidate) if *candidate == country => iso_alpha3::country_to_code(country),
        Different(candidate, ioc) if *candidate == country => Some(*ioc),
        _ => None,
    })
}
