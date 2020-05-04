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
pub struct Type5 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type5 {
    pub fn new(label: Label, value: Value) -> Type5 {
        Type5 { label, value }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "C5")]
    C5,
    #[serde(rename = "C7")]
    C7,
    #[serde(rename = "C13")]
    C13,
    #[serde(rename = "C15")]
    C15,
    #[serde(rename = "C19")]
    C19,
    #[serde(rename = "P+N+E 4H")]
    PNE_4H,
    #[serde(rename = "P+N+E 6H")]
    PNE_6H,
    #[serde(rename = "P+N+E 9H")]
    PNE_9H,
    #[serde(rename = "2P+E 4H")]
    _2PE_4H,
    #[serde(rename = "2P+E 6H")]
    _2PE_6H,
    #[serde(rename = "2P+E 9H")]
    _2PE_9H,
    #[serde(rename = "3P+E 4H")]
    _3PE_4H,
    #[serde(rename = "3P+E 6H")]
    _3PE_6H,
    #[serde(rename = "3P+E 9H")]
    _3PE_9H,
    #[serde(rename = "3P+N+E 4H")]
    _3PNE_4H,
    #[serde(rename = "3P+N+E 6H")]
    _3PNE_6H,
    #[serde(rename = "3P+N+E 9H")]
    _3PNE_9H,
    #[serde(rename = "NEMA 5-15R")]
    NEMA_515R,
    #[serde(rename = "NEMA 5-20R")]
    NEMA_520R,
    #[serde(rename = "NEMA 5-30R")]
    NEMA_530R,
    #[serde(rename = "NEMA 5-50R")]
    NEMA_550R,
    #[serde(rename = "NEMA 6-15R")]
    NEMA_615R,
    #[serde(rename = "NEMA 6-20R")]
    NEMA_620R,
    #[serde(rename = "NEMA 6-30R")]
    NEMA_630R,
    #[serde(rename = "NEMA 6-50R")]
    NEMA_650R,
    #[serde(rename = "NEMA L5-15R")]
    NEMA_L515R,
    #[serde(rename = "NEMA L5-20R")]
    NEMA_L520R,
    #[serde(rename = "NEMA L5-30R")]
    NEMA_L530R,
    #[serde(rename = "NEMA L6-15R")]
    NEMA_L615R,
    #[serde(rename = "NEMA L6-20R")]
    NEMA_L620R,
    #[serde(rename = "NEMA L6-30R")]
    NEMA_L630R,
    #[serde(rename = "NEMA L6-50R")]
    NEMA_L650R,
    #[serde(rename = "CS6360C")]
    CS6360C,
    #[serde(rename = "CS6364C")]
    CS6364C,
    #[serde(rename = "CS8164C")]
    CS8164C,
    #[serde(rename = "CS8264C")]
    CS8264C,
    #[serde(rename = "CS8364C")]
    CS8364C,
    #[serde(rename = "CS8464C")]
    CS8464C,
    #[serde(rename = "ITA Type E (CEE7/5)")]
    ITA_Type_E__CEE75,
    #[serde(rename = "ITA Type F (CEE7/3)")]
    ITA_Type_F__CEE73,
    #[serde(rename = "ITA Type G (BS 1363)")]
    ITA_Type_G__BS_1363,
    #[serde(rename = "ITA Type H")]
    ITA_Type_H,
    #[serde(rename = "ITA Type I")]
    ITA_Type_I,
    #[serde(rename = "ITA Type J")]
    ITA_Type_J,
    #[serde(rename = "ITA Type K")]
    ITA_Type_K,
    #[serde(rename = "ITA Type L (CEI 23-50)")]
    ITA_Type_L__CEI_2350,
    #[serde(rename = "ITA Type M (BS 546)")]
    ITA_Type_M__BS_546,
    #[serde(rename = "ITA Type N")]
    ITA_Type_N,
    #[serde(rename = "ITA Type O")]
    ITA_Type_O,
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "iec-60320-c5")]
    Iec60320C5,
    #[serde(rename = "iec-60320-c7")]
    Iec60320C7,
    #[serde(rename = "iec-60320-c13")]
    Iec60320C13,
    #[serde(rename = "iec-60320-c15")]
    Iec60320C15,
    #[serde(rename = "iec-60320-c19")]
    Iec60320C19,
    #[serde(rename = "iec-60309-p-n-e-4h")]
    Iec60309PNE4h,
    #[serde(rename = "iec-60309-p-n-e-6h")]
    Iec60309PNE6h,
    #[serde(rename = "iec-60309-p-n-e-9h")]
    Iec60309PNE9h,
    #[serde(rename = "iec-60309-2p-e-4h")]
    Iec603092pE4h,
    #[serde(rename = "iec-60309-2p-e-6h")]
    Iec603092pE6h,
    #[serde(rename = "iec-60309-2p-e-9h")]
    Iec603092pE9h,
    #[serde(rename = "iec-60309-3p-e-4h")]
    Iec603093pE4h,
    #[serde(rename = "iec-60309-3p-e-6h")]
    Iec603093pE6h,
    #[serde(rename = "iec-60309-3p-e-9h")]
    Iec603093pE9h,
    #[serde(rename = "iec-60309-3p-n-e-4h")]
    Iec603093pNE4h,
    #[serde(rename = "iec-60309-3p-n-e-6h")]
    Iec603093pNE6h,
    #[serde(rename = "iec-60309-3p-n-e-9h")]
    Iec603093pNE9h,
    #[serde(rename = "nema-5-15r")]
    Nema515r,
    #[serde(rename = "nema-5-20r")]
    Nema520r,
    #[serde(rename = "nema-5-30r")]
    Nema530r,
    #[serde(rename = "nema-5-50r")]
    Nema550r,
    #[serde(rename = "nema-6-15r")]
    Nema615r,
    #[serde(rename = "nema-6-20r")]
    Nema620r,
    #[serde(rename = "nema-6-30r")]
    Nema630r,
    #[serde(rename = "nema-6-50r")]
    Nema650r,
    #[serde(rename = "nema-l5-15r")]
    NemaL515r,
    #[serde(rename = "nema-l5-20r")]
    NemaL520r,
    #[serde(rename = "nema-l5-30r")]
    NemaL530r,
    #[serde(rename = "nema-l5-50r")]
    NemaL550r,
    #[serde(rename = "nema-l6-20r")]
    NemaL620r,
    #[serde(rename = "nema-l6-30r")]
    NemaL630r,
    #[serde(rename = "nema-l6-50r")]
    NemaL650r,
    #[serde(rename = "CS6360C")]
    CS6360C,
    #[serde(rename = "CS6364C")]
    CS6364C,
    #[serde(rename = "CS8164C")]
    CS8164C,
    #[serde(rename = "CS8264C")]
    CS8264C,
    #[serde(rename = "CS8364C")]
    CS8364C,
    #[serde(rename = "CS8464C")]
    CS8464C,
    #[serde(rename = "ita-e")]
    ItaE,
    #[serde(rename = "ita-f")]
    ItaF,
    #[serde(rename = "ita-g")]
    ItaG,
    #[serde(rename = "ita-h")]
    ItaH,
    #[serde(rename = "ita-i")]
    ItaI,
    #[serde(rename = "ita-j")]
    ItaJ,
    #[serde(rename = "ita-k")]
    ItaK,
    #[serde(rename = "ita-l")]
    ItaL,
    #[serde(rename = "ita-m")]
    ItaM,
    #[serde(rename = "ita-n")]
    ItaN,
    #[serde(rename = "ita-o")]
    ItaO,
}