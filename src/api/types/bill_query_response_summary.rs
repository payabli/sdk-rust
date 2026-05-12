pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl BillQueryResponseSummary {
    pub fn builder() -> BillQueryResponseSummaryBuilder {
        <BillQueryResponseSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillQueryResponseSummaryBuilder {
    pageidentifier: Option<PageIdentifier>,
    page_size: Option<Pagesize>,
    total_2_approval: Option<i64>,
    totalactive: Option<i64>,
    total_amount: Option<f64>,
    totalamount_2_approval: Option<f64>,
    totalamountactive: Option<f64>,
    totalamountapproved: Option<f64>,
    totalamountcancel: Option<f64>,
    totalamountdisapproved: Option<f64>,
    totalamountintransit: Option<f64>,
    totalamountoverdue: Option<f64>,
    totalamountpaid: Option<f64>,
    totalamountsent_2_approval: Option<f64>,
    totalapproved: Option<i64>,
    totalcancel: Option<i64>,
    totaldisapproved: Option<i64>,
    totalintransit: Option<i64>,
    totaloverdue: Option<i64>,
    total_pages: Option<Totalpages>,
    totalpaid: Option<i64>,
    total_records: Option<Totalrecords>,
    totalsent_2_approval: Option<i64>,
}

impl BillQueryResponseSummaryBuilder {
    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_2_approval(mut self, value: i64) -> Self {
        self.total_2_approval = Some(value);
        self
    }

    pub fn totalactive(mut self, value: i64) -> Self {
        self.totalactive = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn totalamount_2_approval(mut self, value: f64) -> Self {
        self.totalamount_2_approval = Some(value);
        self
    }

    pub fn totalamountactive(mut self, value: f64) -> Self {
        self.totalamountactive = Some(value);
        self
    }

    pub fn totalamountapproved(mut self, value: f64) -> Self {
        self.totalamountapproved = Some(value);
        self
    }

    pub fn totalamountcancel(mut self, value: f64) -> Self {
        self.totalamountcancel = Some(value);
        self
    }

    pub fn totalamountdisapproved(mut self, value: f64) -> Self {
        self.totalamountdisapproved = Some(value);
        self
    }

    pub fn totalamountintransit(mut self, value: f64) -> Self {
        self.totalamountintransit = Some(value);
        self
    }

    pub fn totalamountoverdue(mut self, value: f64) -> Self {
        self.totalamountoverdue = Some(value);
        self
    }

    pub fn totalamountpaid(mut self, value: f64) -> Self {
        self.totalamountpaid = Some(value);
        self
    }

    pub fn totalamountsent_2_approval(mut self, value: f64) -> Self {
        self.totalamountsent_2_approval = Some(value);
        self
    }

    pub fn totalapproved(mut self, value: i64) -> Self {
        self.totalapproved = Some(value);
        self
    }

    pub fn totalcancel(mut self, value: i64) -> Self {
        self.totalcancel = Some(value);
        self
    }

    pub fn totaldisapproved(mut self, value: i64) -> Self {
        self.totaldisapproved = Some(value);
        self
    }

    pub fn totalintransit(mut self, value: i64) -> Self {
        self.totalintransit = Some(value);
        self
    }

    pub fn totaloverdue(mut self, value: i64) -> Self {
        self.totaloverdue = Some(value);
        self
    }

    pub fn total_pages(mut self, value: Totalpages) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn totalpaid(mut self, value: i64) -> Self {
        self.totalpaid = Some(value);
        self
    }

    pub fn total_records(mut self, value: Totalrecords) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn totalsent_2_approval(mut self, value: i64) -> Self {
        self.totalsent_2_approval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillQueryResponseSummary`].
    pub fn build(self) -> Result<BillQueryResponseSummary, BuildError> {
        Ok(BillQueryResponseSummary {
            pageidentifier: self.pageidentifier,
            page_size: self.page_size,
            total_2_approval: self.total_2_approval,
            totalactive: self.totalactive,
            total_amount: self.total_amount,
            totalamount_2_approval: self.totalamount_2_approval,
            totalamountactive: self.totalamountactive,
            totalamountapproved: self.totalamountapproved,
            totalamountcancel: self.totalamountcancel,
            totalamountdisapproved: self.totalamountdisapproved,
            totalamountintransit: self.totalamountintransit,
            totalamountoverdue: self.totalamountoverdue,
            totalamountpaid: self.totalamountpaid,
            totalamountsent_2_approval: self.totalamountsent_2_approval,
            totalapproved: self.totalapproved,
            totalcancel: self.totalcancel,
            totaldisapproved: self.totaldisapproved,
            totalintransit: self.totalintransit,
            totaloverdue: self.totaloverdue,
            total_pages: self.total_pages,
            totalpaid: self.totalpaid,
            total_records: self.total_records,
            totalsent_2_approval: self.totalsent_2_approval,
        })
    }
}
