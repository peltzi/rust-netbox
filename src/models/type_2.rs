/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Type2 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type2 {
    pub fn new(label: Label, value: Value) -> Type2 {
        Type2 { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Virtual")]
    _Virtual,
    #[serde(rename = "Link Aggregation Group (LAG)")]
    LinkAggregationGroupLAG,
    #[serde(rename = "100BASE-TX (10/100ME)")]
    _100BASETX10100ME,
    #[serde(rename = "1000BASE-T (1GE)")]
    _1000BASET1GE,
    #[serde(rename = "2.5GBASE-T (2.5GE)")]
    _25GBASET25GE,
    #[serde(rename = "5GBASE-T (5GE)")]
    _5GBASET5GE,
    #[serde(rename = "10GBASE-T (10GE)")]
    _10GBASET10GE,
    #[serde(rename = "10GBASE-CX4 (10GE)")]
    _10GBASECX410GE,
    #[serde(rename = "GBIC (1GE)")]
    GBIC1GE,
    #[serde(rename = "SFP (1GE)")]
    SFP1GE,
    #[serde(rename = "SFP+ (10GE)")]
    SFP10GE,
    #[serde(rename = "XFP (10GE)")]
    XFP10GE,
    #[serde(rename = "XENPAK (10GE)")]
    XENPAK10GE,
    #[serde(rename = "X2 (10GE)")]
    X210GE,
    #[serde(rename = "SFP28 (25GE)")]
    SFP2825GE,
    #[serde(rename = "SFP56 (50GE)")]
    SFP5650GE,
    #[serde(rename = "QSFP+ (40GE)")]
    QSFP40GE,
    #[serde(rename = "QSFP28 (50GE)")]
    QSFP2850GE,
    #[serde(rename = "CFP (100GE)")]
    CFP100GE,
    #[serde(rename = "CFP2 (100GE)")]
    CFP2100GE,
    #[serde(rename = "CFP2 (200GE)")]
    CFP2200GE,
    #[serde(rename = "CFP4 (100GE)")]
    CFP4100GE,
    #[serde(rename = "Cisco CPAK (100GE)")]
    CiscoCPAK100GE,
    #[serde(rename = "QSFP28 (100GE)")]
    QSFP28100GE,
    #[serde(rename = "QSFP56 (200GE)")]
    QSFP56200GE,
    #[serde(rename = "QSFP-DD (400GE)")]
    QSFPDD400GE,
    #[serde(rename = "OSFP (400GE)")]
    OSFP400GE,
    #[serde(rename = "IEEE 802.11a")]
    IEEE80211a,
    #[serde(rename = "IEEE 802.11b/g")]
    IEEE80211bG,
    #[serde(rename = "IEEE 802.11n")]
    IEEE80211n,
    #[serde(rename = "IEEE 802.11ac")]
    IEEE80211ac,
    #[serde(rename = "IEEE 802.11ad")]
    IEEE80211ad,
    #[serde(rename = "IEEE 802.11ax")]
    IEEE80211ax,
    #[serde(rename = "IEEE 802.15.1 (Bluetooth)")]
    IEEE802151Bluetooth,
    #[serde(rename = "GSM")]
    GSM,
    #[serde(rename = "CDMA")]
    CDMA,
    #[serde(rename = "LTE")]
    LTE,
    #[serde(rename = "OC-3/STM-1")]
    OC3STM1,
    #[serde(rename = "OC-12/STM-4")]
    OC12STM4,
    #[serde(rename = "OC-48/STM-16")]
    OC48STM16,
    #[serde(rename = "OC-192/STM-64")]
    OC192STM64,
    #[serde(rename = "OC-768/STM-256")]
    OC768STM256,
    #[serde(rename = "OC-1920/STM-640")]
    OC1920STM640,
    #[serde(rename = "OC-3840/STM-1234")]
    OC3840STM1234,
    #[serde(rename = "SFP (1GFC)")]
    SFP1GFC,
    #[serde(rename = "SFP (2GFC)")]
    SFP2GFC,
    #[serde(rename = "SFP (4GFC)")]
    SFP4GFC,
    #[serde(rename = "SFP+ (8GFC)")]
    SFP8GFC,
    #[serde(rename = "SFP+ (16GFC)")]
    SFP16GFC,
    #[serde(rename = "SFP28 (32GFC)")]
    SFP2832GFC,
    #[serde(rename = "QSFP+ (64GFC)")]
    QSFP64GFC,
    #[serde(rename = "QSFP28 (128GFC)")]
    QSFP28128GFC,
    #[serde(rename = "SDR (2 Gbps)")]
    SDR2Gbps,
    #[serde(rename = "DDR (4 Gbps)")]
    DDR4Gbps,
    #[serde(rename = "QDR (8 Gbps)")]
    QDR8Gbps,
    #[serde(rename = "FDR10 (10 Gbps)")]
    FDR1010Gbps,
    #[serde(rename = "FDR (13.5 Gbps)")]
    FDR135Gbps,
    #[serde(rename = "EDR (25 Gbps)")]
    EDR25Gbps,
    #[serde(rename = "HDR (50 Gbps)")]
    HDR50Gbps,
    #[serde(rename = "NDR (100 Gbps)")]
    NDR100Gbps,
    #[serde(rename = "XDR (250 Gbps)")]
    XDR250Gbps,
    #[serde(rename = "T1 (1.544 Mbps)")]
    T11544Mbps,
    #[serde(rename = "E1 (2.048 Mbps)")]
    E12048Mbps,
    #[serde(rename = "T3 (45 Mbps)")]
    T345Mbps,
    #[serde(rename = "E3 (34 Mbps)")]
    E334Mbps,
    #[serde(rename = "xDSL")]
    XDSL,
    #[serde(rename = "Cisco StackWise")]
    CiscoStackWise,
    #[serde(rename = "Cisco StackWise Plus")]
    CiscoStackWisePlus,
    #[serde(rename = "Cisco FlexStack")]
    CiscoFlexStack,
    #[serde(rename = "Cisco FlexStack Plus")]
    CiscoFlexStackPlus,
    #[serde(rename = "Juniper VCP")]
    JuniperVCP,
    #[serde(rename = "Extreme SummitStack")]
    ExtremeSummitStack,
    #[serde(rename = "Extreme SummitStack-128")]
    ExtremeSummitStack128,
    #[serde(rename = "Extreme SummitStack-256")]
    ExtremeSummitStack256,
    #[serde(rename = "Extreme SummitStack-512")]
    ExtremeSummitStack512,
    #[serde(rename = "Other")]
    Other,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "virtual")]
    _Virtual,
    #[serde(rename = "lag")]
    Lag,
    #[serde(rename = "100base-tx")]
    _100baseTx,
    #[serde(rename = "1000base-t")]
    _1000baseT,
    #[serde(rename = "2.5gbase-t")]
    _25gbaseT,
    #[serde(rename = "5gbase-t")]
    _5gbaseT,
    #[serde(rename = "10gbase-t")]
    _10gbaseT,
    #[serde(rename = "10gbase-cx4")]
    _10gbaseCx4,
    #[serde(rename = "1000base-x-gbic")]
    _1000baseXGbic,
    #[serde(rename = "1000base-x-sfp")]
    _1000baseXSfp,
    #[serde(rename = "10gbase-x-sfpp")]
    _10gbaseXSfpp,
    #[serde(rename = "10gbase-x-xfp")]
    _10gbaseXXfp,
    #[serde(rename = "10gbase-x-xenpak")]
    _10gbaseXXenpak,
    #[serde(rename = "10gbase-x-x2")]
    _10gbaseXX2,
    #[serde(rename = "25gbase-x-sfp28")]
    _25gbaseXSfp28,
    #[serde(rename = "50gbase-x-sfp56")]
    _50gbaseXSfp56,
    #[serde(rename = "40gbase-x-qsfpp")]
    _40gbaseXQsfpp,
    #[serde(rename = "50gbase-x-sfp28")]
    _50gbaseXSfp28,
    #[serde(rename = "100gbase-x-cfp")]
    _100gbaseXCfp,
    #[serde(rename = "100gbase-x-cfp2")]
    _100gbaseXCfp2,
    #[serde(rename = "200gbase-x-cfp2")]
    _200gbaseXCfp2,
    #[serde(rename = "100gbase-x-cfp4")]
    _100gbaseXCfp4,
    #[serde(rename = "100gbase-x-cpak")]
    _100gbaseXCpak,
    #[serde(rename = "100gbase-x-qsfp28")]
    _100gbaseXQsfp28,
    #[serde(rename = "200gbase-x-qsfp56")]
    _200gbaseXQsfp56,
    #[serde(rename = "400gbase-x-qsfpdd")]
    _400gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-osfp")]
    _400gbaseXOsfp,
    #[serde(rename = "ieee802.11a")]
    Ieee80211a,
    #[serde(rename = "ieee802.11g")]
    Ieee80211g,
    #[serde(rename = "ieee802.11n")]
    Ieee80211n,
    #[serde(rename = "ieee802.11ac")]
    Ieee80211ac,
    #[serde(rename = "ieee802.11ad")]
    Ieee80211ad,
    #[serde(rename = "ieee802.11ax")]
    Ieee80211ax,
    #[serde(rename = "ieee802.15.1")]
    Ieee802151,
    #[serde(rename = "gsm")]
    Gsm,
    #[serde(rename = "cdma")]
    Cdma,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "sonet-oc3")]
    SonetOc3,
    #[serde(rename = "sonet-oc12")]
    SonetOc12,
    #[serde(rename = "sonet-oc48")]
    SonetOc48,
    #[serde(rename = "sonet-oc192")]
    SonetOc192,
    #[serde(rename = "sonet-oc768")]
    SonetOc768,
    #[serde(rename = "sonet-oc1920")]
    SonetOc1920,
    #[serde(rename = "sonet-oc3840")]
    SonetOc3840,
    #[serde(rename = "1gfc-sfp")]
    _1gfcSfp,
    #[serde(rename = "2gfc-sfp")]
    _2gfcSfp,
    #[serde(rename = "4gfc-sfp")]
    _4gfcSfp,
    #[serde(rename = "8gfc-sfpp")]
    _8gfcSfpp,
    #[serde(rename = "16gfc-sfpp")]
    _16gfcSfpp,
    #[serde(rename = "32gfc-sfp28")]
    _32gfcSfp28,
    #[serde(rename = "64gfc-qsfpp")]
    _64gfcQsfpp,
    #[serde(rename = "128gfc-sfp28")]
    _128gfcSfp28,
    #[serde(rename = "infiniband-sdr")]
    InfinibandSdr,
    #[serde(rename = "infiniband-ddr")]
    InfinibandDdr,
    #[serde(rename = "infiniband-qdr")]
    InfinibandQdr,
    #[serde(rename = "infiniband-fdr10")]
    InfinibandFdr10,
    #[serde(rename = "infiniband-fdr")]
    InfinibandFdr,
    #[serde(rename = "infiniband-edr")]
    InfinibandEdr,
    #[serde(rename = "infiniband-hdr")]
    InfinibandHdr,
    #[serde(rename = "infiniband-ndr")]
    InfinibandNdr,
    #[serde(rename = "infiniband-xdr")]
    InfinibandXdr,
    #[serde(rename = "t1")]
    T1,
    #[serde(rename = "e1")]
    E1,
    #[serde(rename = "t3")]
    T3,
    #[serde(rename = "e3")]
    E3,
    #[serde(rename = "xdsl")]
    Xdsl,
    #[serde(rename = "cisco-stackwise")]
    CiscoStackwise,
    #[serde(rename = "cisco-stackwise-plus")]
    CiscoStackwisePlus,
    #[serde(rename = "cisco-flexstack")]
    CiscoFlexstack,
    #[serde(rename = "cisco-flexstack-plus")]
    CiscoFlexstackPlus,
    #[serde(rename = "juniper-vcp")]
    JuniperVcp,
    #[serde(rename = "extreme-summitstack")]
    ExtremeSummitstack,
    #[serde(rename = "extreme-summitstack-128")]
    ExtremeSummitstack128,
    #[serde(rename = "extreme-summitstack-256")]
    ExtremeSummitstack256,
    #[serde(rename = "extreme-summitstack-512")]
    ExtremeSummitstack512,
    #[serde(rename = "other")]
    Other,
}
