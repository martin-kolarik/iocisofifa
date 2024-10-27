use crate::{
    fifa, ioc, ioc_name, iso_alpha2,
    iso_alpha3::{self},
    iso_name,
    uppercase::uppercase,
    IocIsoFifa,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Country {
    AFG = 4,
    ALA = 248,
    ALB = 8,
    DZA = 12,
    ASM = 16,
    AND = 20,
    AGO = 24,
    AIA = 660,
    ATA = 10,
    ATG = 28,
    ARG = 32,
    ARM = 51,
    ABW = 533,
    AUS = 36,
    AUT = 40,
    AZE = 31,
    BHS = 44,
    BHR = 48,
    BGD = 50,
    BRB = 52,
    BLR = 112,
    BEL = 56,
    BLZ = 84,
    BEN = 204,
    BMU = 60,
    BTN = 64,
    BOL = 68,
    BES = 535,
    BIH = 70,
    BWA = 72,
    BVT = 74,
    BRA = 76,
    IOT = 86,
    VGB = 92,
    BRN = 96,
    BGR = 100,
    BFA = 854,
    BDI = 108,
    CPV = 132,
    KHM = 116,
    CMR = 120,
    CAN = 124,
    CYM = 136,
    CAF = 140,
    TCD = 148,
    CHL = 152,
    CHN = 156,
    HKG = 344,
    MAC = 446,
    CXR = 162,
    CCK = 166,
    COL = 170,
    COM = 174,
    COG = 178,
    COK = 184,
    CRI = 188,
    HRV = 191,
    CUB = 192,
    CUW = 531,
    CYP = 196,
    CZE = 203,
    PRK = 408,
    COD = 180,
    DNK = 208,
    DJI = 262,
    DMA = 212,
    DOM = 214,
    ECU = 218,
    EGY = 818,
    SLV = 222,
    GNQ = 226,
    ERI = 232,
    EST = 233,
    SWZ = 748,
    ETH = 231,
    FLK = 238,
    FRO = 234,
    FJI = 242,
    FIN = 246,
    FRA = 250,
    GUF = 254,
    PYF = 258,
    ATF = 260,
    GAB = 266,
    GMB = 270,
    GEO = 268,
    DEU = 276,
    GHA = 288,
    GIB = 292,
    GRC = 300,
    GRL = 304,
    GRD = 308,
    GLP = 312,
    GUM = 316,
    GTM = 320,
    GGY = 831,
    GIN = 324,
    GNB = 624,
    GUY = 328,
    HTI = 332,
    HMD = 334,
    VAT = 336,
    HND = 340,
    HUN = 348,
    ISL = 352,
    IND = 356,
    IDN = 360,
    IRN = 364,
    IRQ = 368,
    IRL = 372,
    IMN = 833,
    ISR = 376,
    ITA = 380,
    CIV = 384,
    JAM = 388,
    JPN = 392,
    JEY = 832,
    JOR = 400,
    KAZ = 398,
    KEN = 404,
    KIR = 296,
    KWT = 414,
    KGZ = 417,
    LAO = 418,
    LVA = 428,
    LBN = 422,
    LSO = 426,
    LBR = 430,
    LBY = 434,
    LIE = 438,
    LTU = 440,
    LUX = 442,
    MDG = 450,
    MWI = 454,
    MYS = 458,
    MDV = 462,
    MLI = 466,
    MLT = 470,
    MHL = 584,
    MTQ = 474,
    MRT = 478,
    MUS = 480,
    MYT = 175,
    MEX = 484,
    FSM = 583,
    MCO = 492,
    MNG = 496,
    MNE = 499,
    MSR = 500,
    MAR = 504,
    MOZ = 508,
    MMR = 104,
    NAM = 516,
    NRU = 520,
    NPL = 524,
    NLD = 528,
    NCL = 540,
    NZL = 554,
    NIC = 558,
    NER = 562,
    NGA = 566,
    NIU = 570,
    NFK = 574,
    MNP = 580,
    MKD = 807,
    NOR = 578,
    OMN = 512,
    PAK = 586,
    PLW = 585,
    PAN = 591,
    PNG = 598,
    PRY = 600,
    PER = 604,
    PHL = 608,
    PCN = 612,
    POL = 616,
    PRT = 620,
    PRI = 630,
    QAT = 634,
    KOR = 410,
    MDA = 498,
    REU = 638,
    ROU = 642,
    RUS = 643,
    RWA = 646,
    BLM = 652,
    SHN = 654,
    KNA = 659,
    LCA = 662,
    MAF = 663,
    SPM = 666,
    VCT = 670,
    WSM = 882,
    SMR = 674,
    STP = 678,
    SAU = 682,
    SEN = 686,
    SRB = 688,
    SYC = 690,
    SLE = 694,
    SGP = 702,
    SXM = 534,
    SVK = 703,
    SVN = 705,
    SLB = 90,
    SOM = 706,
    ZAF = 710,
    SGS = 239,
    SSD = 728,
    ESP = 724,
    LKA = 144,
    PSE = 275,
    SDN = 729,
    SUR = 740,
    SJM = 744,
    SWE = 752,
    CHE = 756,
    SYR = 760,
    TWN = 158,
    TJK = 762,
    THA = 764,
    TLS = 626,
    TGO = 768,
    TKL = 772,
    TON = 776,
    TTO = 780,
    TUN = 788,
    TUR = 792,
    TKM = 795,
    TCA = 796,
    TUV = 798,
    UGA = 800,
    UKR = 804,
    ARE = 784,
    GBR = 826,
    TZA = 834,
    UMI = 581,
    USA = 840,
    VIR = 850,
    URY = 858,
    UZB = 860,
    VUT = 548,
    VEN = 862,
    VNM = 704,
    WLF = 876,
    ESH = 732,
    YEM = 887,
    ZMB = 894,
    ZWE = 716,

    // generic extension
    XKX = 900, // Kosovo

    // extension for IOC
    AIN = 910,
    EOR = 911,
    NEU = 912,
    NPA = 913,
    ROC = 914,
    RPC = 915,
    RPT = 916,
    VRT = 917,

    // extension for FIFA
    ENG = 920,
    NIR = 921,
    SCO = 922,
    WAL = 923,
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
            900 => Self::XKX,
            910 => Self::AIN,
            911 => Self::EOR,
            912 => Self::NEU,
            913 => Self::NPA,
            914 => Self::ROC,
            915 => Self::RPC,
            916 => Self::RPT,
            917 => Self::VRT,
            920 => Self::ENG,
            921 => Self::NIR,
            922 => Self::SCO,
            923 => Self::WAL,
            _ => return None,
        };
        Some(country)
    }

    fn from_alpha2(alpha2: &str) -> Option<Self> {
        let mut buffer = [0u8; 2];
        let candidate = uppercase(alpha2, &mut buffer)?;
        iso_alpha2::alpha2_to_country(candidate)
    }

    fn from_alpha3(alpha3: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(alpha3, &mut buffer)?;
        iso_alpha3::alpha3_to_country(candidate)
    }

    fn from_iso_name(name: &str) -> Option<Self> {
        iso_name::name_to_country(name)
    }

    fn from_ioc(ioc: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(ioc, &mut buffer)?;
        ioc::ioc_to_country(candidate)
    }

    fn from_ioc_name(name: &str) -> Option<Self> {
        ioc_name::name_to_country(name)
    }

    fn from_fifa(fifa: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(fifa, &mut buffer)?;
        fifa::fifa_to_country(candidate)
    }

    fn from_alpha3_ioc(code: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        iso_alpha3::alpha3_to_country(candidate).or_else(|| ioc::ioc_to_country(candidate))
    }

    fn from_alpha3_fifa(code: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        iso_alpha3::alpha3_to_country(candidate).or_else(|| fifa::fifa_to_country(candidate))
    }

    fn from_iso_ioc_name(name: &str) -> Option<Self> {
        iso_name::name_to_country(name).or_else(|| ioc_name::name_to_country(name))
    }

    fn from_ioc_alpha3(code: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        ioc::ioc_to_country(candidate).or_else(|| iso_alpha3::alpha3_to_country(candidate))
    }

    fn from_ioc_iso_name(name: &str) -> Option<Self> {
        ioc_name::name_to_country(name).or_else(|| iso_name::name_to_country(name))
    }

    fn from_fifa_alpha3(code: &str) -> Option<Self> {
        let mut buffer = [0u8; 3];
        let candidate = uppercase(code, &mut buffer)?;
        fifa::fifa_to_country(candidate).or_else(|| iso_alpha3::alpha3_to_country(candidate))
    }

    fn numeric(&self) -> Option<u32> {
        Some(*self as u32)
    }

    fn alpha2(&self) -> Option<&'static str> {
        iso_alpha2::country_to_alpha2(*self)
    }

    fn alpha3(&self) -> Option<&'static str> {
        iso_alpha3::country_to_alpha3(*self)
    }

    fn iso_name(&self) -> Option<&'static str> {
        iso_name::country_to_name(*self)
    }

    fn ioc(&self) -> Option<&'static str> {
        ioc::country_to_ioc(*self)
    }

    fn ioc_name(&self) -> Option<&'static str> {
        ioc_name::country_to_name(*self)
    }

    fn fifa(&self) -> Option<&'static str> {
        fifa::country_to_fifa(*self)
    }

    fn alpha3_ioc(&self) -> Option<&'static str> {
        iso_alpha3::country_to_alpha3(*self).or_else(|| ioc::country_to_ioc(*self))
    }

    fn iso_ioc_name(&self) -> Option<&'static str> {
        iso_name::country_to_name(*self).or_else(|| ioc_name::country_to_name(*self))
    }

    fn alpha3_fifa(&self) -> Option<&'static str> {
        iso_alpha3::country_to_alpha3(*self).or_else(|| fifa::country_to_fifa(*self))
    }

    fn ioc_alpha3(&self) -> Option<&'static str> {
        ioc::country_to_ioc(*self).or_else(|| iso_alpha3::country_to_alpha3(*self))
    }

    fn ioc_iso_name(&self) -> Option<&'static str> {
        ioc_name::country_to_name(*self).or_else(|| iso_name::country_to_name(*self))
    }

    fn fifa_alpha3(&self) -> Option<&'static str> {
        fifa::country_to_fifa(*self).or_else(|| iso_alpha3::country_to_alpha3(*self))
    }
}
