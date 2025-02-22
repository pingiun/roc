use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::colors::{from_hsb, RgbaTup};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum HighlightStyle {
    Operator, // =+-<>...
    String,
    FunctionName,
    FunctionArgName,
    Type,
    Bracket,
    Number,
    PackageRelated, // app, packages, imports, exposes, provides...
    Value,
    RecordField,
    Import,
    Provides,
    Blank,
    Comment,
    DocsComment,
    UppercaseIdent,
    LowercaseIdent, // TODO we probably don't want all lowercase identifiers to have the same color?
    Keyword,        // if, else, when...
}

pub fn default_highlight_map() -> HashMap<HighlightStyle, RgbaTup> {
    use HighlightStyle::*;

    let almost_white = from_hsb(258, 5, 95);

    let mut highlight_map = HashMap::new();
    [
        (Operator, from_hsb(185, 50, 75)),
        (String, from_hsb(346, 65, 97)),
        (FunctionName, almost_white),
        (FunctionArgName, from_hsb(225, 50, 100)),
        (Type, almost_white),
        (Bracket, from_hsb(347, 80, 100)),
        (Number, from_hsb(225, 50, 100)),
        (PackageRelated, almost_white),
        (Value, almost_white),
        (RecordField, from_hsb(258, 50, 90)),
        (Import, from_hsb(225, 50, 100)),
        (Provides, from_hsb(225, 50, 100)),
        (Blank, from_hsb(258, 50, 90)),
        (Comment, from_hsb(258, 50, 90)),     // TODO check color
        (DocsComment, from_hsb(258, 50, 90)), // TODO check color
        (UppercaseIdent, almost_white),
        (LowercaseIdent, from_hsb(225, 50, 100)),
        (Keyword, almost_white),
    ]
    .iter()
    .for_each(|tup| {
        highlight_map.insert(tup.0, tup.1);
    });

    highlight_map
}
