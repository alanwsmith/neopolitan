use crate::spec::Spec;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Xob {
    pub spec: Spec,
    pub attributes: Option<Vec<(Option<String>, Option<String>)>>,
    // pub extras: Option<Vec<HashMap>>,
    pub children: Option<Vec<Xob>>,
}
