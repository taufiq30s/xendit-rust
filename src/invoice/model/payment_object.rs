use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bank {
    bank_code: String,
    collection_type: String,
    transfer_amount: f64,           // Will be deprected soon
    bank_branch: String,
    account_holder_name: String,
    identity_amount: Option<f64>,
}
impl Bank {
    pub fn get_bank_code(&self) -> &str {
        &self.bank_code
    }
    pub fn get_collection_type(&self) -> &str {
        &self.collection_type
    }
    pub fn get_transfer_amount(&self) -> f64 {
        self.transfer_amount
    }
    pub fn get_bank_branch(&self) -> &str {
        &self.bank_branch
    }
    pub fn get_account_holder_name(&self) -> &str {
        &self.account_holder_name
    }
    pub fn get_identity_amount(&self) -> Option<f64> {
        self.identity_amount
    }
}

#[derive(Deserialize)]
pub struct RetailOutlet {
    retail_outlet_name: String,
    transfer_amount: Option<f64>        // Will be depracted
}
impl RetailOutlet {
    pub fn get_retail_outlet_name(&self) -> &str {
        &self.retail_outlet_name
    }
    pub fn get_transfer_amount(&self) -> Option<f64> {
        self.transfer_amount
    }
}

#[derive(Deserialize)]
pub struct Ewallet {
    ewallet_type: String
}
impl Ewallet {
    pub fn get_ewallet_type(&self) -> &str {
        &self.ewallet_type
    }
}

#[derive(Deserialize)]
pub struct QRCode {
    qr_code_type: String
}
impl QRCode {
    pub fn get_qr_code_type(&self) -> &str {
        &self.qr_code_type
    }
}

#[derive(Deserialize)]
pub struct DirectDebit {
    direct_debit_type: String
}
impl DirectDebit {
    pub fn get_direct_debit_type(&self) -> &str {
        &self.direct_debit_type
    }
}

#[derive(Deserialize)]
pub struct Paylater {
    paylater_type: String
}
impl Paylater {
    pub fn get_paylater_type(&self) -> &str {
        &self.paylater_type
    }
}

#[derive(Deserialize)]
pub struct PaymentDetails {
    receipt_id: Option<String>,
    source: Option<String>
}
impl PaymentDetails {
    pub fn get_receipt_id(&self) -> Option<&str> {
        self.receipt_id.as_deref()
    }
    pub fn get_source(&self) -> Option<&str> {
        self.source.as_deref()
    }
}