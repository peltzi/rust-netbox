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
pub struct WritablePowerPortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "device_type")]
    pub device_type: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Maximum power draw (watts)
    #[serde(rename = "maximum_draw", skip_serializing_if = "Option::is_none")]
    pub maximum_draw: Option<i32>,
    /// Allocated power draw (watts)
    #[serde(rename = "allocated_draw", skip_serializing_if = "Option::is_none")]
    pub allocated_draw: Option<i32>,
}

impl WritablePowerPortTemplate {
    pub fn new(device_type: i32, name: String) -> WritablePowerPortTemplate {
        WritablePowerPortTemplate {
            id: None,
            device_type,
            name,
            _type: None,
            maximum_draw: None,
            allocated_draw: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "iec-60320-c6")]
    Iec60320C6,
    #[serde(rename = "iec-60320-c8")]
    Iec60320C8,
    #[serde(rename = "iec-60320-c14")]
    Iec60320C14,
    #[serde(rename = "iec-60320-c16")]
    Iec60320C16,
    #[serde(rename = "iec-60320-c20")]
    Iec60320C20,
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
    #[serde(rename = "nema-5-15p")]
    Nema515p,
    #[serde(rename = "nema-5-20p")]
    Nema520p,
    #[serde(rename = "nema-5-30p")]
    Nema530p,
    #[serde(rename = "nema-5-50p")]
    Nema550p,
    #[serde(rename = "nema-6-15p")]
    Nema615p,
    #[serde(rename = "nema-6-20p")]
    Nema620p,
    #[serde(rename = "nema-6-30p")]
    Nema630p,
    #[serde(rename = "nema-6-50p")]
    Nema650p,
    #[serde(rename = "nema-l5-15p")]
    NemaL515p,
    #[serde(rename = "nema-l5-20p")]
    NemaL520p,
    #[serde(rename = "nema-l5-30p")]
    NemaL530p,
    #[serde(rename = "nema-l5-50p")]
    NemaL550p,
    #[serde(rename = "nema-l6-20p")]
    NemaL620p,
    #[serde(rename = "nema-l6-30p")]
    NemaL630p,
    #[serde(rename = "nema-l6-50p")]
    NemaL650p,
    #[serde(rename = "cs6361c")]
    Cs6361c,
    #[serde(rename = "cs6365c")]
    Cs6365c,
    #[serde(rename = "cs8165c")]
    Cs8165c,
    #[serde(rename = "cs8265c")]
    Cs8265c,
    #[serde(rename = "cs8365c")]
    Cs8365c,
    #[serde(rename = "cs8465c")]
    Cs8465c,
    #[serde(rename = "ita-e")]
    ItaE,
    #[serde(rename = "ita-f")]
    ItaF,
    #[serde(rename = "ita-ef")]
    ItaEf,
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
