use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

/*
<xsd:enumeration value="nil"/>
<xsd:enumeration value="clear"/>
<xsd:enumeration value="solid"/>
<xsd:enumeration value="horzStripe"/>
<xsd:enumeration value="vertStripe"/>
<xsd:enumeration value="reverseDiagStripe"/>
<xsd:enumeration value="diagStripe"/>
<xsd:enumeration value="horzCross"/>
<xsd:enumeration value="diagCross"/>
<xsd:enumeration value="thinHorzStripe"/>
<xsd:enumeration value="thinVertStripe"/>
<xsd:enumeration value="thinReverseDiagStripe"/>
<xsd:enumeration value="thinDiagStripe"/>
<xsd:enumeration value="thinHorzCross"/>
<xsd:enumeration value="thinDiagCross"/>
<xsd:enumeration value="pct5"/>
<xsd:enumeration value="pct10"/>
<xsd:enumeration value="pct12"/>
<xsd:enumeration value="pct15"/>
<xsd:enumeration value="pct20"/>
<xsd:enumeration value="pct25"/>
<xsd:enumeration value="pct30"/>
<xsd:enumeration value="pct35"/>
<xsd:enumeration value="pct37"/>
<xsd:enumeration value="pct40"/>
<xsd:enumeration value="pct45"/>
<xsd:enumeration value="pct50"/>
<xsd:enumeration value="pct55"/>
<xsd:enumeration value="pct60"/>
<xsd:enumeration value="pct62"/>
<xsd:enumeration value="pct65"/>
<xsd:enumeration value="pct70"/>
<xsd:enumeration value="pct75"/>
<xsd:enumeration value="pct80"/>
<xsd:enumeration value="pct85"/>
<xsd:enumeration value="pct87"/>
<xsd:enumeration value="pct90"/>
<xsd:enumeration value="pct95"/>
*/
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ShdType {
    Nil,
    Clear,
    Solid,
    HorzStripe,
    VertStripe,
    ReverseDiagStripe,
    DiagStripe,
    HorzCross,
    DiagCross,
    ThinHorzStripe,
    ThinVertStripe,
    ThinReverseDiagStripe,
    ThinDiagStripe,
    ThinHorzCross,
    ThinDiagCross,
    Pct5,
    Pct10,
    Pct12,
    Pct15,
    Pct20,
    Pct25,
    Pc