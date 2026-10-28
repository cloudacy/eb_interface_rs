use rust_decimal::Decimal;

use crate::{
    decimal::CloneAndRescale,
    xml::{ToXml, XmlElement},
};

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub enum TaxCategory {
    #[default]
    S,
    AA,
    O,
    D,
    E,
    F,
    G,
    I,
    J,
    K,
    AE,
    Z,
}

impl std::fmt::Display for TaxCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TaxCategory::S => "S",
                TaxCategory::AA => "AA",
                TaxCategory::O => "O",
                TaxCategory::D => "D",
                TaxCategory::E => "E",
                TaxCategory::F => "F",
                TaxCategory::G => "G",
                TaxCategory::I => "I",
                TaxCategory::J => "J",
                TaxCategory::K => "K",
                TaxCategory::AE => "AE",
                TaxCategory::Z => "Z",
            }
        )
    }
}

#[derive(Default)]
pub struct TaxItem {
    pub(crate) tax_percent: Decimal,
    pub(crate) tax_category: TaxCategory,
}

impl TaxItem {
    pub fn new(tax_percent: Decimal, tax_category: TaxCategory) -> Self {
        TaxItem {
            tax_percent,
            tax_category,
        }
    }

    pub(crate) fn with_taxable_amount(&self, taxable_amount: Decimal) -> TaxItemWithTaxableAmount {
        TaxItemWithTaxableAmount {
            tax_percent: self.tax_percent,
            tax_category: self.tax_category,
            taxable_amount,
        }
    }
}

pub(crate) struct TaxItemWithTaxableAmount {
    tax_percent: Decimal,
    tax_category: TaxCategory,
    taxable_amount: Decimal,
}

impl TaxItemWithTaxableAmount {
    pub(crate) fn taxable_amount(&self) -> Decimal {
        self.taxable_amount
    }

    pub(crate) fn tax_amount(&self) -> Decimal {
        (self.taxable_amount * (self.tax_percent / Decimal::ONE_HUNDRED)).clone_with_scale(2)
    }
}

impl ToXml for TaxItemWithTaxableAmount {
    fn to_xml(&self) -> String {
        XmlElement::new("TaxItem")
            .with_text_element(
                "TaxableAmount",
                self.taxable_amount.clone_with_scale(2).to_string(),
            )
            .with_element(
                &XmlElement::new("TaxPercent")
                    .with_attr("TaxCategoryCode", self.tax_category.to_string())
                    .with_text(self.tax_percent.to_string()),
            )
            .with_text_element("TaxAmount", self.tax_amount().to_string())
            .to_xml()
    }
}
