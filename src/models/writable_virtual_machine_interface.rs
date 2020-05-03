/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 2.7
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableVirtualMachineInterface {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "virtual_machine", skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<i32>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    #[serde(rename = "mac_address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "untagged_vlan", skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<i32>,
    #[serde(rename = "tagged_vlans", skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl WritableVirtualMachineInterface {
    pub fn new(name: String, _type: Type) -> WritableVirtualMachineInterface {
        WritableVirtualMachineInterface {
            id: None,
            virtual_machine: None,
            name,
            _type,
            enabled: None,
            mtu: None,
            mac_address: None,
            description: None,
            mode: None,
            untagged_vlan: None,
            tagged_vlans: None,
            tags: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
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
    #[serde(rename = "128gfc-sfp28")]
    _128gfcSfp28,
    #[serde(rename = "inifiband-sdr")]
    InifibandSdr,
    #[serde(rename = "inifiband-ddr")]
    InifibandDdr,
    #[serde(rename = "inifiband-qdr")]
    InifibandQdr,
    #[serde(rename = "inifiband-fdr10")]
    InifibandFdr10,
    #[serde(rename = "inifiband-fdr")]
    InifibandFdr,
    #[serde(rename = "inifiband-edr")]
    InifibandEdr,
    #[serde(rename = "inifiband-hdr")]
    InifibandHdr,
    #[serde(rename = "inifiband-ndr")]
    InifibandNdr,
    #[serde(rename = "inifiband-xdr")]
    InifibandXdr,
    #[serde(rename = "t1")]
    T1,
    #[serde(rename = "e1")]
    E1,
    #[serde(rename = "t3")]
    T3,
    #[serde(rename = "e3")]
    E3,
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
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "tagged")]
    Tagged,
    #[serde(rename = "tagged-all")]
    TaggedAll,
}
