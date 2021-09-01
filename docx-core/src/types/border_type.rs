
//
// Please see p3813 <xsd:simpleType name="ST_Border">
//
use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderType {
    Nil,
    None,
    Single,
    Thick,
    Double,
    Dotted,
    Dashed,
    DotDash,
    DotDotDash,
    Triple,
    ThinThickSmallGap,
    ThickThinSmallGap,
    ThinThickThinSmallGap,
    ThinThickMediumGap,
    ThickThinMediumGap,
    ThinThickThinMediumGap,
    ThinThickLargeGap,
    ThickThinLargeGap,
    ThinThickThinLargeGap,
    Wave,
    DoubleWave,
    DashSmallGap,
    DashDotStroked,
    ThreeDEmboss,
    ThreeDEngrave,
    Outset,
    Inset,
    Apples,
    ArchedScallops,
    BabyPacifier,
    BabyRattle,
}

impl fmt::Display for BorderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BorderType::Nil => write!(f, "nil"),
            BorderType::None => write!(f, "none"),
            BorderType::Single => write!(f, "single"),
            BorderType::Thick => write!(f, "thick"),
            BorderType::Double => write!(f, "double"),
            BorderType::Dotted => write!(f, "dotted"),
            BorderType::Dashed => write!(f, "dashed"),
            BorderType::DotDash => write!(f, "dotDash"),
            BorderType::DotDotDash => write!(f, "dotDotDash"),