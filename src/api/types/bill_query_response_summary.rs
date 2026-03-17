pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillQueryResponseSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    #[serde(rename = "total2approval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_2_approval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalactive: Option<i64>,
    /// Total amount of bills in response.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(rename = "totalamount2approval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamount_2_approval: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountactive: Option<f64>,
    /// The total amount of approved bills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountapproved: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountcancel: Option<f64>,
    /// The total amount of disapproved bills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountdisapproved: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountintransit: Option<f64>,
    /// The total amount of bills that are overdue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountoverdue: Option<f64>,
    /// The total amount of paid bills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountpaid: Option<f64>,
    #[serde(rename = "totalamountsent2approval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalamountsent_2_approval: Option<f64>,
    /// The total number of bills that were approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalapproved: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalcancel: Option<i64>,
    /// The number of bills that were disapproved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totaldisapproved: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalintransit: Option<i64>,
    /// The number of bills that are overdue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totaloverdue: Option<i64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<Totalpages>,
    /// The total number of paid bills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalpaid: Option<i64>,
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<Totalrecords>,
    #[serde(rename = "totalsent2approval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalsent_2_approval: Option<i64>,
}
