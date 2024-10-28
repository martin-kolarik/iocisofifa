use crate::{
    fifa, fifa_name, ioc, ioc_name, iso_alpha2,
    iso_alpha3::{self},
    iso_name,
    uppercase::uppercase,
    IocIsoFifa, Precedence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Country {
    ABW = 533,
    AFG = 4,
    AGO = 24,
    AIA = 660,
    AIN = 910, // IOC
    ALA = 248,
    ALB = 8,
    AND = 20,
    ANT = 525, // IOC, historical
    ARE = 784,
    ARG = 32,
    ARM = 51,
    ASM = 16,
    ATA = 10,
    ATF = 260,
    ATG = 28,
    AUS = 36,
    AUT = 40,
    AZE = 31,
    BDI = 108,
    BEL = 56,
    BEN = 204,
    BES = 535,
    BFA = 854,
    BGD = 50,
    BGR = 100,
    BHR = 48,
    BHS = 44,
    BIH = 70,
    BLM = 652,
    BLR = 112,
    BLZ = 84,
    BMU = 60,
    BOL = 68,
    BRA = 76,
    BRB = 52,
    BRN = 96,
    BTN = 64,
    BVT = 74,
    BWA = 72,
    CAF = 140,
    CAN = 124,
    CCK = 166,
    CIV = 384,
    CMR = 120,
    COD = 180,
    COG = 178,
    COK = 184,
    COL = 170,
    COM = 174,
    CPV = 132,
    CRI = 188,
    CUB = 192,
    CUW = 531,
    CXR = 162,
    CYM = 136,
    CYP = 196,
    CZE = 203,
    DEU = 276,
    DJI = 262,
    DMA = 212,
    DNK = 208,
    DOM = 214,
    DZA = 12,
    ECU = 218,
    EGY = 818,
    ENG = 920, // FIFA
    EOR = 911, // IOC
    ERI = 232,
    ESH = 732,
    ESP = 724,
    EST = 233,
    ETH = 231,
    FIN = 246,
    FJI = 242,
    FLK = 238,
    FRA = 250,
    FRO = 234,
    FSM = 583,
    GAB = 266,
    GBR = 826,
    GEO = 268,
    GGY = 831,
    GHA = 288,
    GIB = 292,
    GIN = 324,
    GLP = 312,
    GMB = 270,
    GNB = 624,
    GNQ = 226,
    GRC = 300,
    GRD = 308,
    GRL = 304,
    GTM = 320,
    GUF = 254,
    GUM = 316,
    GUY = 328,
    HKG = 344,
    HMD = 334,
    HND = 340,
    HRV = 191,
    HTI = 332,
    HUN = 348,
    CHE = 756,
    CHL = 152,
    CHN = 156,
    IDN = 360,
    IMN = 833,
    IND = 356,
    IOT = 86,
    IRL = 372,
    IRN = 364,
    IRQ = 368,
    ISL = 352,
    ISR = 376,
    ITA = 380,
    JAM = 388,
    JEY = 832,
    JOR = 400,
    JPN = 392,
    KAZ = 398,
    KEN = 404,
    KGZ = 417,
    KHM = 116,
    KIR = 296,
    KNA = 659,
    KOR = 410,
    KWT = 414,
    LAO = 418,
    LBN = 422,
    LBR = 430,
    LBY = 434,
    LCA = 662,
    LIE = 438,
    LKA = 144,
    LSO = 426,
    LTU = 440,
    LUX = 442,
    LVA = 428,
    MAC = 446,
    MAF = 663,
    MAR = 504,
    MCO = 492,
    MDA = 498,
    MDG = 450,
    MDV = 462,
    MEX = 484,
    MHL = 584,
    MKD = 807,
    MLI = 466,
    MLT = 470,
    MMR = 104,
    MNE = 499,
    MNG = 496,
    MNP = 580,
    MOZ = 508,
    MRT = 478,
    MSR = 500,
    MTQ = 474,
    MUS = 480,
    MWI = 454,
    MYS = 458,
    MYT = 175,
    NAC = 999, // private, Not a Code
    NAM = 516,
    NCL = 540,
    NER = 562,
    NEU = 912, // IOC
    NFK = 574,
    NGA = 566,
    NIC = 558,
    NIR = 921, // FIFA
    NIU = 570,
    NLD = 528,
    NOR = 578,
    NPA = 913, // IOC
    NPL = 524,
    NRU = 520,
    NZL = 554,
    OMN = 512,
    PAK = 586,
    PAN = 591,
    PCN = 612,
    PER = 604,
    PHL = 608,
    PLW = 585,
    PNG = 598,
    POL = 616,
    PRI = 630,
    PRK = 408,
    PRT = 620,
    PRY = 600,
    PSE = 275,
    PYF = 258,
    QAT = 634,
    REU = 638,
    ROC = 914, // IOC
    ROU = 642,
    RPC = 915, // IOC
    RPT = 916, // IOC
    RUS = 643,
    RWA = 646,
    SAU = 682,
    SCO = 922, // FIFA
    SDN = 729,
    SEN = 686,
    SGP = 702,
    SGS = 239,
    SHN = 654,
    SJM = 744,
    SLB = 90,
    SLE = 694,
    SLV = 222,
    SMR = 674,
    SOM = 706,
    SPM = 666,
    SRB = 688,
    SSD = 728,
    STP = 678,
    SUR = 740,
    SVK = 703,
    SVN = 705,
    SWE = 752,
    SWZ = 748,
    SXM = 534,
    SYC = 690,
    SYR = 760,
    TCA = 796,
    TCD = 148,
    TGO = 768,
    THA = 764,
    TJK = 762,
    TKL = 772,
    TKM = 795,
    TLS = 626,
    TON = 776,
    TTO = 780,
    TUN = 788,
    TUR = 792,
    TUV = 798,
    TWN = 158,
    TZA = 834,
    UGA = 800,
    UKR = 804,
    UMI = 581,
    URY = 858,
    USA = 840,
    UZB = 860,
    VAT = 336,
    VCT = 670,
    VEN = 862,
    VGB = 92,
    VIR = 850,
    VNM = 704,
    VRT = 917, // IOC
    VUT = 548,
    WAL = 923, // FIFA
    WLF = 876,
    WSM = 882,
    XKX = 900, // Kosovo
    YEM = 887,
    ZAF = 710,
    ZMB = 894,
    ZWE = 716,
}

impl IocIsoFifa for Country {
    fn from_numeric(numeric: u32) -> Option<Self> {
        let country = match numeric {
            4 => Self::AFG,
            248 => Self::ALA,
            8 => Self::ALB,
            12 => Self::DZA,
            16 => Self::ASM,
            20 => Self::AND,
            24 => Self::AGO,
            660 => Self::AIA,
            525 => Self::ANT,
            10 => Self::ATA,
            28 => Self::ATG,
            32 => Self::ARG,
            51 => Self::ARM,
            533 => Self::ABW,
            36 => Self::AUS,
            40 => Self::AUT,
            31 => Self::AZE,
            44 => Self::BHS,
            48 => Self::BHR,
            50 => Self::BGD,
            52 => Self::BRB,
            112 => Self::BLR,
            56 => Self::BEL,
            84 => Self::BLZ,
            204 => Self::BEN,
            60 => Self::BMU,
            64 => Self::BTN,
            68 => Self::BOL,
            535 => Self::BES,
            70 => Self::BIH,
            72 => Self::BWA,
            74 => Self::BVT,
            76 => Self::BRA,
            86 => Self::IOT,
            92 => Self::VGB,
            96 => Self::BRN,
            100 => Self::BGR,
            854 => Self::BFA,
            108 => Self::BDI,
            132 => Self::CPV,
            116 => Self::KHM,
            120 => Self::CMR,
            124 => Self::CAN,
            136 => Self::CYM,
            140 => Self::CAF,
            148 => Self::TCD,
            152 => Self::CHL,
            156 => Self::CHN,
            344 => Self::HKG,
            446 => Self::MAC,
            162 => Self::CXR,
            166 => Self::CCK,
            170 => Self::COL,
            174 => Self::COM,
            178 => Self::COG,
            184 => Self::COK,
            188 => Self::CRI,
            191 => Self::HRV,
            192 => Self::CUB,
            531 => Self::CUW,
            196 => Self::CYP,
            203 => Self::CZE,
            408 => Self::PRK,
            180 => Self::COD,
            208 => Self::DNK,
            262 => Self::DJI,
            212 => Self::DMA,
            214 => Self::DOM,
            218 => Self::ECU,
            818 => Self::EGY,
            222 => Self::SLV,
            226 => Self::GNQ,
            232 => Self::ERI,
            233 => Self::EST,
            748 => Self::SWZ,
            231 => Self::ETH,
            238 => Self::FLK,
            234 => Self::FRO,
            242 => Self::FJI,
            246 => Self::FIN,
            250 => Self::FRA,
            254 => Self::GUF,
            258 => Self::PYF,
            260 => Self::ATF,
            266 => Self::GAB,
            270 => Self::GMB,
            268 => Self::GEO,
            276 => Self::DEU,
            288 => Self::GHA,
            292 => Self::GIB,
            300 => Self::GRC,
            304 => Self::GRL,
            308 => Self::GRD,
            312 => Self::GLP,
            316 => Self::GUM,
            320 => Self::GTM,
            831 => Self::GGY,
            324 => Self::GIN,
            624 => Self::GNB,
            328 => Self::GUY,
            332 => Self::HTI,
            334 => Self::HMD,
            336 => Self::VAT,
            340 => Self::HND,
            348 => Self::HUN,
            352 => Self::ISL,
            356 => Self::IND,
            360 => Self::IDN,
            364 => Self::IRN,
            368 => Self::IRQ,
            372 => Self::IRL,
            833 => Self::IMN,
            376 => Self::ISR,
            380 => Self::ITA,
            384 => Self::CIV,
            388 => Self::JAM,
            392 => Self::JPN,
            832 => Self::JEY,
            400 => Self::JOR,
            398 => Self::KAZ,
            404 => Self::KEN,
            296 => Self::KIR,
            414 => Self::KWT,
            417 => Self::KGZ,
            418 => Self::LAO,
            428 => Self::LVA,
            422 => Self::LBN,
            426 => Self::LSO,
            430 => Self::LBR,
            434 => Self::LBY,
            438 => Self::LIE,
            440 => Self::LTU,
            442 => Self::LUX,
            450 => Self::MDG,
            454 => Self::MWI,
            458 => Self::MYS,
            462 => Self::MDV,
            466 => Self::MLI,
            470 => Self::MLT,
            584 => Self::MHL,
            474 => Self::MTQ,
            478 => Self::MRT,
            480 => Self::MUS,
            175 => Self::MYT,
            484 => Self::MEX,
            583 => Self::FSM,
            492 => Self::MCO,
            496 => Self::MNG,
            499 => Self::MNE,
            500 => Self::MSR,
            504 => Self::MAR,
            508 => Self::MOZ,
            104 => Self::MMR,
            516 => Self::NAM,
            520 => Self::NRU,
            524 => Self::NPL,
            528 => Self::NLD,
            540 => Self::NCL,
            554 => Self::NZL,
            558 => Self::NIC,
            562 => Self::NER,
            566 => Self::NGA,
            570 => Self::NIU,
            574 => Self::NFK,
            580 => Self::MNP,
            807 => Self::MKD,
            578 => Self::NOR,
            512 => Self::OMN,
            586 => Self::PAK,
            585 => Self::PLW,
            591 => Self::PAN,
            598 => Self::PNG,
            600 => Self::PRY,
            604 => Self::PER,
            608 => Self::PHL,
            612 => Self::PCN,
            616 => Self::POL,
            620 => Self::PRT,
            630 => Self::PRI,
            634 => Self::QAT,
            410 => Self::KOR,
            498 => Self::MDA,
            638 => Self::REU,
            642 => Self::ROU,
            643 => Self::RUS,
            646 => Self::RWA,
            652 => Self::BLM,
            654 => Self::SHN,
            659 => Self::KNA,
            662 => Self::LCA,
            663 => Self::MAF,
            666 => Self::SPM,
            670 => Self::VCT,
            882 => Self::WSM,
            674 => Self::SMR,
            678 => Self::STP,
            682 => Self::SAU,
            686 => Self::SEN,
            688 => Self::SRB,
            690 => Self::SYC,
            694 => Self::SLE,
            702 => Self::SGP,
            534 => Self::SXM,
            703 => Self::SVK,
            705 => Self::SVN,
            90 => Self::SLB,
            706 => Self::SOM,
            710 => Self::ZAF,
            239 => Self::SGS,
            728 => Self::SSD,
            724 => Self::ESP,
            144 => Self::LKA,
            275 => Self::PSE,
            729 => Self::SDN,
            740 => Self::SUR,
            744 => Self::SJM,
            752 => Self::SWE,
            756 => Self::CHE,
            760 => Self::SYR,
            158 => Self::TWN,
            762 => Self::TJK,
            764 => Self::THA,
            626 => Self::TLS,
            768 => Self::TGO,
            772 => Self::TKL,
            776 => Self::TON,
            780 => Self::TTO,
            788 => Self::TUN,
            792 => Self::TUR,
            795 => Self::TKM,
            796 => Self::TCA,
            798 => Self::TUV,
            800 => Self::UGA,
            804 => Self::UKR,
            784 => Self::ARE,
            826 => Self::GBR,
            834 => Self::TZA,
            581 => Self::UMI,
            840 => Self::USA,
            850 => Self::VIR,
            858 => Self::URY,
            860 => Self::UZB,
            548 => Self::VUT,
            862 => Self::VEN,
            704 => Self::VNM,
            876 => Self::WLF,
            732 => Self::ESH,
            887 => Self::YEM,
            894 => Self::ZMB,
            716 => Self::ZWE,
            900 => Self::XKX, // Kosovo
            910 => Self::AIN, // IOC
            911 => Self::EOR, // IOC
            912 => Self::NEU, // IOC
            913 => Self::NPA, // IOC
            914 => Self::ROC, // IOC
            915 => Self::RPC, // IOC
            916 => Self::RPT, // IOC
            917 => Self::VRT, // IOC
            920 => Self::ENG, // FIFA
            921 => Self::NIR, // FIFA
            922 => Self::SCO, // FIFA
            923 => Self::WAL, // FIFA
            999 => Self::NAC, // private, Not a Code
            _ => return None,
        };
        Some(country)
    }

    fn from_alpha2(alpha2: &str) -> Option<Self> {
        let mut buffer = [0u8; 2];
        let candidate = uppercase(alpha2, &mut buffer)?;
        iso_alpha2::code_to_country(candidate)
    }

    fn from_alpha3(alpha3: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(alpha3, &mut buffer)?;
        iso_alpha3::code_to_country(candidate)
    }

    fn from_iso_name(name: &str) -> Option<Self> {
        iso_name::name_to_country(name)
    }

    fn from_ioc(ioc: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(ioc, &mut buffer)?;
        ioc::code_to_country(candidate)
    }

    fn from_ioc_name(name: &str) -> Option<Self> {
        ioc_name::name_to_country(name)
    }

    fn from_fifa(fifa: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(fifa, &mut buffer)?;
        fifa::code_to_country(candidate)
    }

    fn from_code(code: &str, precedence: Precedence) -> Option<Self> {
        match code.len() {
            0..=1 | 4.. => None,
            2 => {
                let mut buffer = [0u8; 2];
                let code = uppercase(code, &mut buffer)?;
                Self::from_alpha2(code)
            }
            3 => {
                use Precedence::*;

                let mut buffer = [0u8; 3];
                let code = uppercase(code, &mut buffer)?;
                match precedence {
                    IsoFifaIoc | IsoIocFifa => iso_alpha3::code_to_country(code),
                    IocIsoFifa | IocFifaIso => ioc::code_to_country(code),
                    FifaIsoIoc | FifaIocIso => fifa::code_to_country(code),
                }
                .or_else(|| match precedence {
                    IocIsoFifa | FifaIsoIoc => iso_alpha3::code_to_country(code),
                    IsoIocFifa | FifaIocIso => ioc::code_to_country(code),
                    IsoFifaIoc | IocFifaIso => fifa::code_to_country(code),
                })
                .or_else(|| match precedence {
                    IocFifaIso | FifaIocIso => iso_alpha3::code_to_country(code),
                    IsoFifaIoc | FifaIsoIoc => ioc::code_to_country(code),
                    IsoIocFifa | IocIsoFifa => fifa::code_to_country(code),
                })
            }
        }
    }

    fn from_name(name: &str, precedence: Precedence) -> Option<Self> {
        use Precedence::*;

        match precedence {
            IsoFifaIoc | IsoIocFifa => iso_name::name_to_country(name),
            IocIsoFifa | IocFifaIso => ioc_name::name_to_country(name),
            FifaIsoIoc | FifaIocIso => fifa_name::name_to_country(name),
        }
        .or_else(|| match precedence {
            IocIsoFifa | FifaIsoIoc => iso_name::name_to_country(name),
            IsoIocFifa | FifaIocIso => ioc_name::name_to_country(name),
            IsoFifaIoc | IocFifaIso => fifa_name::name_to_country(name),
        })
        .or_else(|| match precedence {
            IocFifaIso | FifaIocIso => iso_name::name_to_country(name),
            IsoFifaIoc | FifaIsoIoc => ioc_name::name_to_country(name),
            IsoIocFifa | IocIsoFifa => fifa_name::name_to_country(name),
        })
    }

    #[cfg(feature = "std")]
    fn from_name_caseless(name: &str, precedence: Precedence) -> Option<Self> {
        use Precedence::*;

        match precedence {
            IsoFifaIoc | IsoIocFifa => iso_name::name_to_country_caseless(name),
            IocIsoFifa | IocFifaIso => ioc_name::name_to_country_caseless(name),
            FifaIsoIoc | FifaIocIso => fifa_name::name_to_country_caseless(name),
        }
        .or_else(|| match precedence {
            IocIsoFifa | FifaIsoIoc => iso_name::name_to_country_caseless(name),
            IsoIocFifa | FifaIocIso => ioc_name::name_to_country_caseless(name),
            IsoFifaIoc | IocFifaIso => fifa_name::name_to_country_caseless(name),
        })
        .or_else(|| match precedence {
            IocFifaIso | FifaIocIso => iso_name::name_to_country_caseless(name),
            IsoFifaIoc | FifaIsoIoc => ioc_name::name_to_country_caseless(name),
            IsoIocFifa | IocIsoFifa => fifa_name::name_to_country_caseless(name),
        })
    }

    fn numeric(&self) -> Option<u32> {
        Some(*self as u32)
    }

    fn alpha2(&self) -> Option<&'static str> {
        iso_alpha2::country_to_code(*self)
    }

    fn alpha3(&self) -> Option<&'static str> {
        iso_alpha3::country_to_code(*self)
    }

    fn iso_name(&self) -> Option<&'static str> {
        iso_name::country_to_name(*self)
    }

    fn ioc(&self) -> Option<&'static str> {
        ioc::country_to_code(*self)
    }

    fn ioc_name(&self) -> Option<&'static str> {
        ioc_name::country_to_name(*self)
    }

    fn fifa(&self) -> Option<&'static str> {
        fifa::country_to_code(*self)
    }

    fn code(&self, precedence: Precedence) -> Option<&'static str> {
        use Precedence::*;

        match precedence {
            IsoFifaIoc | IsoIocFifa => iso_alpha3::country_to_code(*self),
            IocIsoFifa | IocFifaIso => ioc::country_to_code(*self),
            FifaIsoIoc | FifaIocIso => fifa::country_to_code(*self),
        }
        .or_else(|| match precedence {
            IocIsoFifa | FifaIsoIoc => iso_alpha3::country_to_code(*self),
            IsoIocFifa | FifaIocIso => ioc::country_to_code(*self),
            IsoFifaIoc | IocFifaIso => fifa::country_to_code(*self),
        })
        .or_else(|| match precedence {
            IocFifaIso | FifaIocIso => iso_alpha3::country_to_code(*self),
            IsoFifaIoc | FifaIsoIoc => ioc::country_to_code(*self),
            IsoIocFifa | IocIsoFifa => fifa::country_to_code(*self),
        })
    }

    fn name(&self, precedence: Precedence) -> Option<&'static str> {
        use Precedence::*;

        match precedence {
            IsoFifaIoc | IsoIocFifa => iso_name::country_to_name(*self),
            IocIsoFifa | IocFifaIso => ioc_name::country_to_name(*self),
            FifaIsoIoc | FifaIocIso => fifa_name::country_to_name(*self),
        }
        .or_else(|| match precedence {
            IocIsoFifa | FifaIsoIoc => iso_name::country_to_name(*self),
            IsoIocFifa | FifaIocIso => ioc_name::country_to_name(*self),
            IsoFifaIoc | IocFifaIso => fifa_name::country_to_name(*self),
        })
        .or_else(|| match precedence {
            IocFifaIso | FifaIocIso => iso_name::country_to_name(*self),
            IsoFifaIoc | FifaIsoIoc => ioc_name::country_to_name(*self),
            IsoIocFifa | IocIsoFifa => fifa_name::country_to_name(*self),
        })
    }
}
