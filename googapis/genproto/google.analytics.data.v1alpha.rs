/// A contiguous set of days: startDate, startDate + 1, ..., endDate. Requests
/// are allowed up to 4 date ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot
    /// be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also
    /// accepted, and in that case, the date is inferred based on the property's
    /// reporting time zone.
    #[prost(string, tag = "1")]
    pub start_date: std::string::String,
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot
    /// be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is
    /// also accepted, and in that case, the date is inferred based on the
    /// property's reporting time zone.
    #[prost(string, tag = "2")]
    pub end_date: std::string::String,
    /// Assigns a name to this date range. The dimension `dateRange` is valued to
    /// this name in a report response. If set, cannot begin with `date_range_` or
    /// `RESERVED_`. If not set, date ranges are named by their zero based index in
    /// the request: `date_range_0`, `date_range_1`, etc.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The unique identifier of the property whose events are tracked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// A Google Analytics GA4 property id. To learn more, see [where to find your
    /// Property
    /// ID](https://developers.google.com/analytics/trusted-testing/analytics-data/property-id).
    #[prost(string, tag = "1")]
    pub property_id: std::string::String,
}
/// Dimensions are attributes of your data. For example, the dimension city
/// indicates the city from which an event originates. Dimension values in report
/// responses are strings; for example, city could be "Paris" or "New York".
/// Requests are allowed up to 8 dimensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    /// The name of the dimension. See the [API
    /// Dimensions](https://developers.google.com/analytics/trusted-testing/analytics-data/api-schema#dimensions)
    /// for the list of dimension names.
    ///
    /// If `dimensionExpression` is specified, `name` can be any string that you
    /// would like. For example if a `dimensionExpression` concatenates `country`
    /// and `city`, you could call that dimension `countryAndCity`.
    ///
    /// Dimensions are referenced by `name` in `dimensionFilter`, `orderBys`,
    /// `dimensionExpression`, and `pivots`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// One dimension can be the result of an expression of multiple dimensions.
    /// For example, dimension "country, city": concatenate(country, ", ", city).
    #[prost(message, optional, tag = "2")]
    pub dimension_expression: ::std::option::Option<DimensionExpression>,
}
/// Used to express a dimension which is the result of a formula of multiple
/// dimensions. Example usages:
/// 1) lower_case(dimension)
/// 2) concatenate(dimension1, symbol, dimension2).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionExpression {
    /// Specify one type of dimension expression for `DimensionExpression`.
    #[prost(oneof = "dimension_expression::OneExpression", tags = "4, 5, 6")]
    pub one_expression: ::std::option::Option<dimension_expression::OneExpression>,
}
pub mod dimension_expression {
    /// Used to convert a dimension value to a single case.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaseExpression {
        /// Name of a dimension. The name must refer back to a name in dimensions
        /// field of the request.
        #[prost(string, tag = "1")]
        pub dimension_name: std::string::String,
    }
    /// Used to combine dimension values to a single dimension.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConcatenateExpression {
        /// Names of dimensions. The names must refer back to names in the dimensions
        /// field of the request.
        #[prost(string, repeated, tag = "1")]
        pub dimension_names: ::std::vec::Vec<std::string::String>,
        /// The delimiter placed between dimension names.
        ///
        /// Delimiters are often single characters such as "|" or "," but can be
        /// longer strings. If a dimension value contains the delimiter, both will be
        /// present in response with no distinction. For example if dimension 1 value
        /// = "US,FR", dimension 2 value = "JP", and delimiter = ",", then the
        /// response will contain "US,FR,JP".
        #[prost(string, tag = "2")]
        pub delimiter: std::string::String,
    }
    /// Specify one type of dimension expression for `DimensionExpression`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneExpression {
        /// Used to convert a dimension value to lower case.
        #[prost(message, tag = "4")]
        LowerCase(CaseExpression),
        /// Used to convert a dimension value to upper case.
        #[prost(message, tag = "5")]
        UpperCase(CaseExpression),
        /// Used to combine dimension values to a single dimension.
        /// For example, dimension "country, city": concatenate(country, ", ", city).
        #[prost(message, tag = "6")]
        Concatenate(ConcatenateExpression),
    }
}
/// The quantitative measurements of a report. For example, the metric
/// `eventCount` is the total number of events. Requests are allowed up to 10
/// metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// The name of the metric. See the [API
    /// Metrics](https://developers.google.com/analytics/trusted-testing/analytics-data/api-schema#metrics)
    /// for the list of metric names.
    ///
    /// If `expression` is specified, `name` can be any string that you would like.
    /// For example if `expression` is `screenPageViews/sessions`, you could call
    /// that metric's name = `viewsPerSession`.
    ///
    /// Metrics are referenced by `name` in `metricFilter`, `orderBys`, and metric
    /// `expression`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A mathematical expression for derived metrics. For example, the metric
    /// Event count per user is `eventCount/totalUsers`.
    #[prost(string, tag = "2")]
    pub expression: std::string::String,
    /// Indicates if a metric is invisible in the report response. If a metric is
    /// invisible, the metric will not produce a column in the response, but can be
    /// used in `metricFilter`, `orderBys`, or a metric `expression`.
    #[prost(bool, tag = "3")]
    pub invisible: bool,
}
/// To express dimension or metric filters.
/// The fields in the same FilterExpression need to be either all dimensions or
/// all metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExpression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[prost(oneof = "filter_expression::Expr", tags = "1, 2, 3, 4")]
    pub expr: ::std::option::Option<filter_expression::Expr>,
}
pub mod filter_expression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// The FilterExpressions in and_group have an AND relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::FilterExpressionList),
        /// The FilterExpressions in or_group have an OR relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::FilterExpressionList),
        /// The FilterExpression is NOT of not_expression.
        #[prost(message, tag = "3")]
        NotExpression(Box<super::FilterExpression>),
        /// A primitive filter.
        /// All fields in filter in same FilterExpression needs to be either all
        /// dimensions or metrics.
        #[prost(message, tag = "4")]
        Filter(super::Filter),
    }
}
/// A list of filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExpressionList {
    /// A list of filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::std::vec::Vec<FilterExpression>,
}
/// An expression to filter dimension or metric values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The dimension name or metric name. Must be a name defined in dimensions
    /// or metrics.
    #[prost(string, tag = "1")]
    pub field_name: std::string::String,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "filter::OneFilter", tags = "2, 3, 4, 5, 6")]
    pub one_filter: ::std::option::Option<filter::OneFilter>,
}
pub mod filter {
    /// The filter for string
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// The match type for this filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// The string value used for the matching.
        #[prost(string, tag = "2")]
        pub value: std::string::String,
        /// If true, the string value is case sensitive.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
    pub mod string_filter {
        /// The match type of a string filter
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum MatchType {
            /// Unspecified
            Unspecified = 0,
            /// Exact match of the string value.
            Exact = 1,
            /// Begins with the string value.
            BeginsWith = 2,
            /// Ends with the string value.
            EndsWith = 3,
            /// Contains the string value.
            Contains = 4,
            /// Full regular expression match with the string value.
            FullRegexp = 5,
            /// Partial regular expression match with the string value.
            PartialRegexp = 6,
        }
    }
    /// The result needs to be in a list of string values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InListFilter {
        /// The list of string values.
        /// Must be non-empty.
        #[prost(string, repeated, tag = "1")]
        pub values: ::std::vec::Vec<std::string::String>,
        /// If true, the string value is case sensitive.
        #[prost(bool, tag = "2")]
        pub case_sensitive: bool,
    }
    /// Filters for numeric or date values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NumericFilter {
        /// The operation type for this filter.
        #[prost(enumeration = "numeric_filter::Operation", tag = "1")]
        pub operation: i32,
        /// A numeric value or a date value.
        #[prost(message, optional, tag = "2")]
        pub value: ::std::option::Option<super::NumericValue>,
    }
    pub mod numeric_filter {
        /// The operation applied to a numeric filter
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Operation {
            /// Unspecified.
            Unspecified = 0,
            /// Equal
            Equal = 1,
            /// Less than
            LessThan = 2,
            /// Less than or equal
            LessThanOrEqual = 3,
            /// Greater than
            GreaterThan = 4,
            /// Greater than or equal
            GreaterThanOrEqual = 5,
        }
    }
    /// To express that the result needs to be between two numbers (inclusive).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BetweenFilter {
        /// Begins with this number.
        #[prost(message, optional, tag = "1")]
        pub from_value: ::std::option::Option<super::NumericValue>,
        /// Ends with this number.
        #[prost(message, optional, tag = "2")]
        pub to_value: ::std::option::Option<super::NumericValue>,
    }
    /// Specify one type of filter for `Filter`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// A filter for null values. If True, a null dimension value is matched by
        /// this filter. Null filter is commonly used inside a NOT filter
        /// expression. For example, a NOT expression of a null filter removes rows
        /// when a dimension is null.
        #[prost(bool, tag = "2")]
        NullFilter(bool),
        /// Strings related filter.
        #[prost(message, tag = "3")]
        StringFilter(StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "4")]
        InListFilter(InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "5")]
        NumericFilter(NumericFilter),
        /// A filter for two values.
        #[prost(message, tag = "6")]
        BetweenFilter(BetweenFilter),
    }
}
/// The sort options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBy {
    /// If true, sorts by descending order.
    #[prost(bool, tag = "4")]
    pub desc: bool,
    /// Specify one type of order by for `OrderBy`.
    #[prost(oneof = "order_by::OneOrderBy", tags = "1, 2, 3")]
    pub one_order_by: ::std::option::Option<order_by::OneOrderBy>,
}
pub mod order_by {
    /// Sorts by metric values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricOrderBy {
        /// A metric name in the request to order by.
        #[prost(string, tag = "1")]
        pub metric_name: std::string::String,
    }
    /// Sorts by dimension values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionOrderBy {
        /// A dimension name in the request to order by.
        #[prost(string, tag = "1")]
        pub dimension_name: std::string::String,
        /// Controls the rule for dimension value ordering.
        #[prost(enumeration = "dimension_order_by::OrderType", tag = "2")]
        pub order_type: i32,
    }
    pub mod dimension_order_by {
        /// Rule to order the string dimension values by.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum OrderType {
            /// Unspecified.
            Unspecified = 0,
            /// Alphanumeric sort by Unicode code point. For example, "2" < "A" < "X" <
            /// "b" < "z".
            Alphanumeric = 1,
            /// Case insensitive alphanumeric sort by lower case Unicode code point.
            /// For example, "2" < "A" < "b" < "X" < "z".
            CaseInsensitiveAlphanumeric = 2,
            /// Dimension values are converted to numbers before sorting. For example
            /// in NUMERIC sort, "25" < "100", and in `ALPHANUMERIC` sort, "100" <
            /// "25". Non-numeric dimension values all have equal ordering value below
            /// all numeric values.
            Numeric = 3,
        }
    }
    /// Sorts by a pivot column group.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PivotOrderBy {
        /// In the response to order by, order rows by this column. Must be a metric
        /// name from the request.
        #[prost(string, tag = "1")]
        pub metric_name: std::string::String,
        /// Used to select a dimension name and value pivot. If multiple pivot
        /// selections are given, the sort occurs on rows where all pivot selection
        /// dimension name and value pairs match the row's dimension name and value
        /// pair.
        #[prost(message, repeated, tag = "2")]
        pub pivot_selections: ::std::vec::Vec<pivot_order_by::PivotSelection>,
    }
    pub mod pivot_order_by {
        /// A pair of dimension names and values. Rows with this dimension pivot pair
        /// are ordered by the metric's value.
        ///
        /// For example if pivots = {{"browser", "Chrome"}} and
        /// metric_name = "Sessions",
        /// then the rows will be sorted based on Sessions in Chrome.
        ///
        ///     ---------|----------|----------------|----------|----------------
        ///              |  Chrome  |    Chrome      |  Safari  |     Safari
        ///     ---------|----------|----------------|----------|----------------
        ///      Country | Sessions | Pages/Sessions | Sessions | Pages/Sessions
        ///     ---------|----------|----------------|----------|----------------
        ///         US   |    2     |       2        |     3    |        1
        ///     ---------|----------|----------------|----------|----------------
        ///       Canada |    3     |       1        |     4    |        1
        ///     ---------|----------|----------------|----------|----------------
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PivotSelection {
            /// Must be a dimension name from the request.
            #[prost(string, tag = "1")]
            pub dimension_name: std::string::String,
            /// Order by only when the named dimension is this value.
            #[prost(string, tag = "2")]
            pub dimension_value: std::string::String,
        }
    }
    /// Specify one type of order by for `OrderBy`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneOrderBy {
        /// Sorts results by a metric's values.
        #[prost(message, tag = "1")]
        Metric(MetricOrderBy),
        /// Sorts results by a dimension's values.
        #[prost(message, tag = "2")]
        Dimension(DimensionOrderBy),
        /// Sorts results by a metric's values within a pivot column group.
        #[prost(message, tag = "3")]
        Pivot(PivotOrderBy),
    }
}
/// Describes the visible dimension columns and rows in the report response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pivot {
    /// Dimension names for visible columns in the report response. Including
    /// "dateRange" produces a date range column; for each row in the response,
    /// dimension values in the date range column will indicate the corresponding
    /// date range from the request.
    #[prost(string, repeated, tag = "1")]
    pub field_names: ::std::vec::Vec<std::string::String>,
    /// Specifies how dimensions are ordered in the pivot. In the first Pivot, the
    /// OrderBys determine Row and PivotDimensionHeader ordering; in subsequent
    /// Pivots, the OrderBys determine only PivotDimensionHeader ordering.
    /// Dimensions specified in these OrderBys must be a subset of
    /// Pivot.field_names.
    #[prost(message, repeated, tag = "2")]
    pub order_bys: ::std::vec::Vec<OrderBy>,
    /// The row count of the start row. The first row is counted as row 0.
    #[prost(int64, tag = "3")]
    pub offset: i64,
    /// The number of rows to return in this pivot. If unspecified, 10 rows are
    /// returned. If -1, all rows are returned.
    #[prost(int64, tag = "4")]
    pub limit: i64,
    /// Aggregate the metrics by dimensions in this pivot using the specified
    /// metric_aggregations.
    #[prost(enumeration = "MetricAggregation", repeated, tag = "5")]
    pub metric_aggregations: ::std::vec::Vec<i32>,
}
/// Specification for a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortSpec {
    /// The definition for the cohorts.
    #[prost(message, repeated, tag = "1")]
    pub cohorts: ::std::vec::Vec<Cohort>,
    /// The data ranges of cohorts.
    #[prost(message, optional, tag = "2")]
    pub cohorts_range: ::std::option::Option<CohortsRange>,
    /// Settings of a cohort report.
    #[prost(message, optional, tag = "3")]
    pub cohort_report_settings: ::std::option::Option<CohortReportSettings>,
}
/// Defines a cohort. A cohort is a group of users who share a common
/// characteristic. For example, all users with the same acquisition date
/// belong to the same cohort.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cohort {
    /// Assigns a name to this cohort. The dimension `cohort` is valued to this
    /// name in a report response. If set, cannot begin with `cohort_` or
    /// `RESERVED_`. If not set, cohorts are named by their zero based index
    /// `cohort_0`, `cohort_1`, etc.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The dimension used by cohort. Only supports `firstTouchDate` for retention
    /// report.
    #[prost(string, tag = "2")]
    pub dimension: std::string::String,
    /// The cohort selects users whose first visit date is between start date
    /// and end date defined in the `dateRange`. In a cohort request, this
    /// `dateRange` is required and the `dateRanges` in the `RunReportRequest` or
    /// `RunPivotReportRequest` must be unspecified.
    ///
    /// The date range should be aligned with the cohort's granularity. If
    /// CohortsRange uses daily granularity, the date range can be aligned to any
    /// day. If CohortsRange uses weekly granularity, the date range should be
    /// aligned to the week boundary, starting at Sunday and ending Saturday. If
    /// CohortsRange uses monthly granularity, the date range should be aligned to
    /// the month, starting at the first and ending on the last day of the month.
    #[prost(message, optional, tag = "3")]
    pub date_range: ::std::option::Option<DateRange>,
}
/// Settings of a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortReportSettings {
    /// If true, accumulates the result from first visit day to the end day. Not
    /// supported in `RunReportRequest`.
    #[prost(bool, tag = "1")]
    pub accumulate: bool,
}
/// Describes date range for a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortsRange {
    /// Reporting date range for each cohort is calculated based on these three
    /// fields.
    #[prost(enumeration = "cohorts_range::Granularity", tag = "1")]
    pub granularity: i32,
    /// For daily cohorts, this will be the start day offset.
    /// For weekly cohorts, this will be the week offset.
    #[prost(int32, tag = "2")]
    pub start_offset: i32,
    /// For daily cohorts, this will be the end day offset.
    /// For weekly cohorts, this will be the week offset.
    #[prost(int32, tag = "3")]
    pub end_offset: i32,
}
pub mod cohorts_range {
    /// Reporting granularity for the cohorts.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Granularity {
        /// Unspecified.
        Unspecified = 0,
        /// Daily
        Daily = 1,
        /// Weekly
        Weekly = 2,
        /// Monthly
        Monthly = 3,
    }
}
/// Response's metadata carrying additional information about the report content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetaData {
    /// If true, indicates some buckets of dimension combinations are rolled into
    /// "(other)" row. This can happen for high cardinality reports.
    #[prost(bool, tag = "3")]
    pub data_loss_from_other_row: bool,
}
/// Describes a dimension column in the report. Dimensions requested in a report
/// produce column entries within rows and DimensionHeaders. However, dimensions
/// used exclusively within filters or expressions do not produce columns in a
/// report; correspondingly, those dimensions do not produce headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionHeader {
    /// The dimension's name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Describes a metric column in the report. Visible metrics requested in a
/// report produce column entries within rows and MetricHeaders. However,
/// metrics used exclusively within filters or expressions do not produce columns
/// in a report; correspondingly, those metrics do not produce headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricHeader {
    /// The metric's name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The metric's data type.
    #[prost(enumeration = "MetricType", tag = "2")]
    pub r#type: i32,
}
/// Dimensions' values in a single pivot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PivotHeader {
    /// The size is the same as the cardinality of the corresponding dimension
    /// combinations.
    #[prost(message, repeated, tag = "1")]
    pub pivot_dimension_headers: ::std::vec::Vec<PivotDimensionHeader>,
    /// The cardinality of the pivot as if offset = 0 and limit = -1. The total
    /// number of rows for this pivot's fields regardless of how the parameters
    /// offset and limit are specified in the request.
    #[prost(int32, tag = "2")]
    pub row_count: i32,
}
/// Summarizes dimension values from a row for this pivot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PivotDimensionHeader {
    /// Values of multiple dimensions in a pivot.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::std::vec::Vec<DimensionValue>,
}
/// Report data for each row.
/// For example if RunReportRequest contains:
///
/// ```none
/// "dimensions": [
///   {
///     "name": "eventName"
///   },
///   {
///     "name": "countryId"
///   }
/// ],
/// "metrics": [
///   {
///     "name": "eventCount"
///   }
/// ]
/// ```
///
/// One row with 'in_app_purchase' as the eventName, 'JP' as the countryId, and
/// 15 as the eventCount, would be:
///
/// ```none
/// "dimensionValues": [
///   {
///     "value": "in_app_purchase"
///   },
///   {
///     "value": "JP"
///   }
/// ],
/// "metricValues": [
///   {
///     "value": "15"
///   }
/// ]
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// List of requested dimension values. In a PivotReport, dimension_values
    /// are only listed for dimensions included in a pivot.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::std::vec::Vec<DimensionValue>,
    /// List of requested visible metric values.
    #[prost(message, repeated, tag = "2")]
    pub metric_values: ::std::vec::Vec<MetricValue>,
}
/// The value of a dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionValue {
    /// One kind of dimension value
    #[prost(oneof = "dimension_value::OneValue", tags = "1")]
    pub one_value: ::std::option::Option<dimension_value::OneValue>,
}
pub mod dimension_value {
    /// One kind of dimension value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Value as a string if the dimension type is a string.
        #[prost(string, tag = "1")]
        Value(std::string::String),
    }
}
/// The value of a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValue {
    /// One of metric value
    #[prost(oneof = "metric_value::OneValue", tags = "4")]
    pub one_value: ::std::option::Option<metric_value::OneValue>,
}
pub mod metric_value {
    /// One of metric value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Measurement value. See MetricHeader for type.
        #[prost(string, tag = "4")]
        Value(std::string::String),
    }
}
/// To represent a number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericValue {
    /// One of a numeric value
    #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
    pub one_value: ::std::option::Option<numeric_value::OneValue>,
}
pub mod numeric_value {
    /// One of a numeric value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Integer value
        #[prost(int64, tag = "1")]
        Int64Value(i64),
        /// Double value
        #[prost(double, tag = "2")]
        DoubleValue(f64),
    }
}
/// Current state of all quotas for this Analytics Property. If any quota for a
/// property is exhausted, all requests to that property will return Resource
/// Exhausted errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyQuota {
    /// Standard Analytics Properties can use up to 25,000 tokens per day;
    /// Analytics 360 Properties can use 250,000 tokens per day. Most requests
    /// consume fewer than 10 tokens.
    #[prost(message, optional, tag = "1")]
    pub tokens_per_day: ::std::option::Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 5,000 tokens per day; Analytics
    /// 360 Properties can use 50,000 tokens per day. An API request consumes a
    /// single number of tokens, and that number is deducted from both the hourly
    /// and daily quotas.
    #[prost(message, optional, tag = "2")]
    pub tokens_per_hour: ::std::option::Option<QuotaStatus>,
    /// Standard Analytics Properties can send up to 10 concurrent requests;
    /// Analytics 360 Properties can use up to 50 concurrent requests.
    #[prost(message, optional, tag = "3")]
    pub concurrent_requests: ::std::option::Option<QuotaStatus>,
    /// Standard Analytics Properties and cloud project pairs can have up to 10
    /// server errors per hour; Analytics 360 Properties and cloud project pairs
    /// can have up to 50 server errors per hour.
    #[prost(message, optional, tag = "4")]
    pub server_errors_per_project_per_hour: ::std::option::Option<QuotaStatus>,
}
/// Current state for a particular quota group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaStatus {
    /// Quota consumed by this request.
    #[prost(int32, tag = "1")]
    pub consumed: i32,
    /// Quota remaining after this request.
    #[prost(int32, tag = "2")]
    pub remaining: i32,
}
/// Explains a dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionMetadata {
    /// This dimension's name. Useable in [Dimension](#Dimension)'s `name`. For
    /// example, `eventName`.
    #[prost(string, tag = "1")]
    pub api_name: std::string::String,
    /// This dimension's name within the Google Analytics user interface. For
    /// example, `Event name`.
    #[prost(string, tag = "2")]
    pub ui_name: std::string::String,
    /// Description of how this dimension is used and calculated.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Still usable but deprecated names for this dimension. If populated, this
    /// dimension is available by either `apiName` or one of `deprecatedApiNames`
    /// for a period of time. After the deprecation period, the dimension will be
    /// available only by `apiName`.
    #[prost(string, repeated, tag = "4")]
    pub deprecated_api_names: ::std::vec::Vec<std::string::String>,
}
/// Explains a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricMetadata {
    /// A metric name. Useable in [Metric](#Metric)'s `name`. For example,
    /// `eventCount`.
    #[prost(string, tag = "1")]
    pub api_name: std::string::String,
    /// This metric's name within the Google Analytics user interface. For example,
    /// `Event count`.
    #[prost(string, tag = "2")]
    pub ui_name: std::string::String,
    /// Description of how this metric is used and calculated.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Still usable but deprecated names for this metric. If populated, this
    /// metric is available by either `apiName` or one of `deprecatedApiNames`
    /// for a period of time. After the deprecation period, the metric will be
    /// available only by `apiName`.
    #[prost(string, repeated, tag = "4")]
    pub deprecated_api_names: ::std::vec::Vec<std::string::String>,
    /// The type of this metric.
    #[prost(enumeration = "MetricType", tag = "5")]
    pub r#type: i32,
    /// The mathematical expression for this derived metric. Can be used in
    /// [Metric](#Metric)'s `expression` field for equivalent reports. Most metrics
    /// are not expressions, and for non-expressions, this field is empty.
    #[prost(string, tag = "6")]
    pub expression: std::string::String,
}
/// Represents aggregation of metrics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricAggregation {
    /// Unspecified operator.
    Unspecified = 0,
    /// SUM operator.
    Total = 1,
    /// Minimum operator.
    Minimum = 5,
    /// Maximum operator.
    Maximum = 6,
    /// Count operator.
    Count = 4,
}
/// A metric's value type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    /// Unspecified type.
    Unspecified = 0,
    /// Integer type.
    TypeInteger = 1,
    /// Floating point type.
    TypeFloat = 2,
    /// A duration of seconds; a special floating point type.
    TypeSeconds = 4,
    /// A duration in milliseconds; a special floating point type.
    TypeMilliseconds = 5,
    /// A duration in minutes; a special floating point type.
    TypeMinutes = 6,
    /// A duration in hours; a special floating point type.
    TypeHours = 7,
    /// A custom metric of standard type; a special floating point type.
    TypeStandard = 8,
    /// An amount of money; a special floating point type.
    TypeCurrency = 9,
    /// A length in feet; a special floating point type.
    TypeFeet = 10,
    /// A length in miles; a special floating point type.
    TypeMiles = 11,
    /// A length in meters; a special floating point type.
    TypeMeters = 12,
    /// A length in kilometers; a special floating point type.
    TypeKilometers = 13,
}
/// The dimensions and metrics currently accepted in reporting methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Resource name of this metadata.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// The dimension descriptions.
    #[prost(message, repeated, tag = "1")]
    pub dimensions: ::std::vec::Vec<DimensionMetadata>,
    /// The metric descriptions.
    #[prost(message, repeated, tag = "2")]
    pub metrics: ::std::vec::Vec<MetricMetadata>,
}
/// The request to generate a report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunReportRequest {
    /// A property whose events are tracked. Within a batch request, this entity
    /// should either be unspecified or consistent with the batch-level entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// The dimensions requested and displayed.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::std::vec::Vec<Dimension>,
    /// The metrics requested and displayed.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::std::vec::Vec<Metric>,
    /// Date ranges of data to read. If multiple date ranges are requested, each
    /// response row will contain a zero based date range index. If two date
    /// ranges overlap, the event data for the overlapping days is included in the
    /// response rows for both date ranges. In a cohort request, this `dateRanges`
    /// must be unspecified.
    #[prost(message, repeated, tag = "4")]
    pub date_ranges: ::std::vec::Vec<DateRange>,
    /// The row count of the start row. The first row is counted as row 0.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](basics#pagination).
    #[prost(int64, tag = "5")]
    pub offset: i64,
    /// The number of rows to return. If unspecified, 10 rows are returned. If
    /// -1, all rows are returned.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](basics#pagination).
    #[prost(int64, tag = "6")]
    pub limit: i64,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows
    /// where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[prost(enumeration = "MetricAggregation", repeated, tag = "7")]
    pub metric_aggregations: ::std::vec::Vec<i32>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in
    /// this filter. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "8")]
    pub dimension_filter: ::std::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Metrics must be requested to be used in this filter.
    /// Dimensions cannot be used in this filter.
    #[prost(message, optional, tag = "9")]
    pub metric_filter: ::std::option::Option<FilterExpression>,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "10")]
    pub order_bys: ::std::vec::Vec<OrderBy>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY".
    /// If the field is empty, the report uses the entity's default currency.
    #[prost(string, tag = "11")]
    pub currency_code: std::string::String,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the 'cohort' dimension must be present.
    #[prost(message, optional, tag = "12")]
    pub cohort_spec: ::std::option::Option<CohortSpec>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be
    /// returned. If true, these rows will be returned if they are not separately
    /// removed by a filter.
    #[prost(bool, tag = "13")]
    pub keep_empty_rows: bool,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[prost(bool, tag = "14")]
    pub return_property_quota: bool,
}
/// The response report table corresponding to a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of
    /// DimensionHeaders matches the dimensions present in rows.
    #[prost(message, repeated, tag = "11")]
    pub dimension_headers: ::std::vec::Vec<DimensionHeader>,
    /// Describes metric columns. The number of MetricHeaders and ordering of
    /// MetricHeaders matches the metrics present in rows.
    #[prost(message, repeated, tag = "1")]
    pub metric_headers: ::std::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "2")]
    pub rows: ::std::vec::Vec<Row>,
    /// If requested, the totaled values of metrics.
    #[prost(message, repeated, tag = "8")]
    pub totals: ::std::vec::Vec<Row>,
    /// If requested, the maximum values of metrics.
    #[prost(message, repeated, tag = "9")]
    pub maximums: ::std::vec::Vec<Row>,
    /// If requested, the minimum values of metrics.
    #[prost(message, repeated, tag = "10")]
    pub minimums: ::std::vec::Vec<Row>,
    /// The total number of rows in the query result, regardless of the number of
    /// rows returned in the response. For example if a query returns 175 rows and
    /// includes limit = 50 in the API request, the response will contain row_count
    /// = 175 but only 50 rows.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](basics#pagination).
    #[prost(int32, tag = "12")]
    pub row_count: i32,
    /// Metadata for the report.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::std::option::Option<ResponseMetaData>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "7")]
    pub property_quota: ::std::option::Option<PropertyQuota>,
}
/// The request to generate a pivot report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPivotReportRequest {
    /// A property whose events are tracked. Within a batch request, this entity
    /// should either be unspecified or consistent with the batch-level entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// The dimensions requested. All defined dimensions must be used by one of the
    /// following: dimension_expression, dimension_filter, pivots, order_bys.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::std::vec::Vec<Dimension>,
    /// The metrics requested, at least one metric needs to be specified. All
    /// defined metrics must be used by one of the following: metric_expression,
    /// metric_filter, order_bys.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::std::vec::Vec<Metric>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in
    /// this filter. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "4")]
    pub dimension_filter: ::std::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Metrics must be requested to be used in this filter.
    /// Dimensions cannot be used in this filter.
    #[prost(message, optional, tag = "5")]
    pub metric_filter: ::std::option::Option<FilterExpression>,
    /// Describes the visual format of the report's dimensions in columns or rows.
    /// The union of the fieldNames (dimension names) in all pivots must be a
    /// subset of dimension names defined in Dimensions. No two pivots can share a
    /// dimension. A dimension is only visible if it appears in a pivot.
    #[prost(message, repeated, tag = "6")]
    pub pivots: ::std::vec::Vec<Pivot>,
    /// The date range to retrieve event data for the report. If multiple date
    /// ranges are specified, event data from each date range is used in the
    /// report. A special dimension with field name "dateRange" can be included in
    /// a Pivot's field names; if included, the report compares between date
    /// ranges. In a cohort request, this `dateRanges` must be unspecified.
    #[prost(message, repeated, tag = "7")]
    pub date_ranges: ::std::vec::Vec<DateRange>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY".
    /// If the field is empty, the report uses the entity's default currency.
    #[prost(string, tag = "8")]
    pub currency_code: std::string::String,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the 'cohort' dimension must be present.
    #[prost(message, optional, tag = "9")]
    pub cohort_spec: ::std::option::Option<CohortSpec>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be
    /// returned. If true, these rows will be returned if they are not separately
    /// removed by a filter.
    #[prost(bool, tag = "10")]
    pub keep_empty_rows: bool,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[prost(bool, tag = "11")]
    pub return_property_quota: bool,
}
/// The response pivot report table corresponding to a pivot request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPivotReportResponse {
    /// Summarizes the columns and rows created by a pivot. Each pivot in the
    /// request produces one header in the response. If we have a request like
    /// this:
    ///
    ///     "pivots": [{
    ///       "fieldNames": ["country",
    ///         "city"]
    ///     },
    ///     {
    ///       "fieldNames": "eventName"
    ///     }]
    ///
    /// We will have the following `pivotHeaders` in the response:
    ///
    ///     "pivotHeaders" : [{
    ///       "dimensionHeaders": [{
    ///         "dimensionValues": [
    ///            { "value": "United Kingdom" },
    ///            { "value": "London" }
    ///          ]
    ///       },
    ///       {
    ///         "dimensionValues": [
    ///         { "value": "Japan" },
    ///         { "value": "Osaka" }
    ///         ]
    ///       }]
    ///     },
    ///     {
    ///       "dimensionHeaders": [{
    ///         "dimensionValues": [{ "value": "session_start" }]
    ///       },
    ///       {
    ///         "dimensionValues": [{ "value": "scroll" }]
    ///       }]
    ///     }]
    #[prost(message, repeated, tag = "1")]
    pub pivot_headers: ::std::vec::Vec<PivotHeader>,
    /// Describes dimension columns. The number of DimensionHeaders and ordering of
    /// DimensionHeaders matches the dimensions present in rows.
    #[prost(message, repeated, tag = "7")]
    pub dimension_headers: ::std::vec::Vec<DimensionHeader>,
    /// Describes metric columns. The number of MetricHeaders and ordering of
    /// MetricHeaders matches the metrics present in rows.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::std::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::std::vec::Vec<Row>,
    /// Aggregation of metric values. Can be totals, minimums, or maximums. The
    /// returned aggregations are controlled by the metric_aggregations in the
    /// pivot. The type of aggregation returned in each row is shown by the
    /// dimension_values which are set to "RESERVED_<MetricAggregation>".
    #[prost(message, repeated, tag = "4")]
    pub aggregates: ::std::vec::Vec<Row>,
    /// Metadata for the report.
    #[prost(message, optional, tag = "5")]
    pub metadata: ::std::option::Option<ResponseMetaData>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "6")]
    pub property_quota: ::std::option::Option<PropertyQuota>,
}
/// The batch request containing multiple report requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunReportsRequest {
    /// A property whose events are tracked. This entity must be specified for the
    /// batch. The entity within RunReportRequest may either be unspecified or
    /// consistent with this entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// Individual requests. Each request has a separate report response. Each
    /// batch request is allowed up to 5 requests.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<RunReportRequest>,
}
/// The batch response containing multiple reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunReportsResponse {
    /// Individual responses. Each response has a separate report request.
    #[prost(message, repeated, tag = "1")]
    pub reports: ::std::vec::Vec<RunReportResponse>,
}
/// The batch request containing multiple pivot report requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunPivotReportsRequest {
    /// A property whose events are tracked. This entity must be specified for the
    /// batch. The entity within RunPivotReportRequest may either be unspecified or
    /// consistent with this entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// Individual requests. Each request has a separate pivot report response.
    /// Each batch request is allowed up to 5 requests.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<RunPivotReportRequest>,
}
/// The batch response containing multiple pivot reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunPivotReportsResponse {
    /// Individual responses. Each response has a separate pivot report request.
    #[prost(message, repeated, tag = "1")]
    pub pivot_reports: ::std::vec::Vec<RunPivotReportResponse>,
}
/// Request for a property's dimension and metric metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetadataRequest {
    /// Required. The resource name of the metadata to retrieve. This name field is
    /// specified in the URL path and not URL parameters. Property is a numeric
    /// Google Analytics GA4 Property identifier. To learn more, see [where to find
    /// your Property
    /// ID](https://developers.google.com/analytics/trusted-testing/analytics-data/property-id).
    ///
    /// Example: properties/1234/metadata
    ///
    /// Set the Property ID to 0 for dimensions and metrics common to all
    /// properties. In this special mode, this method will not return custom
    /// dimensions and metrics.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request to generate a realtime report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunRealtimeReportRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked.
    /// Specified in the URL path and not the body. To learn more, see [where to
    /// find your Property
    /// ID](https://developers.google.com/analytics/trusted-testing/analytics-data/property-id).
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: std::string::String,
    /// The dimensions requested and displayed.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::std::vec::Vec<Dimension>,
    /// The metrics requested and displayed.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::std::vec::Vec<Metric>,
    /// The number of rows to return. If unspecified, 10 rows are returned. If
    /// -1, all rows are returned.
    #[prost(int64, tag = "4")]
    pub limit: i64,
    /// The filter clause of dimensions. Dimensions must be requested to be used in
    /// this filter. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "5")]
    pub dimension_filter: ::std::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Metrics must be requested to be used in this filter.
    /// Dimensions cannot be used in this filter.
    #[prost(message, optional, tag = "6")]
    pub metric_filter: ::std::option::Option<FilterExpression>,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows
    /// where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[prost(enumeration = "MetricAggregation", repeated, tag = "7")]
    pub metric_aggregations: ::std::vec::Vec<i32>,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "8")]
    pub order_bys: ::std::vec::Vec<OrderBy>,
    /// Toggles whether to return the current state of this Analytics Property's
    /// Realtime quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[prost(bool, tag = "9")]
    pub return_property_quota: bool,
}
/// The response realtime report table corresponding to a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunRealtimeReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of
    /// DimensionHeaders matches the dimensions present in rows.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::std::vec::Vec<DimensionHeader>,
    /// Describes metric columns. The number of MetricHeaders and ordering of
    /// MetricHeaders matches the metrics present in rows.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::std::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::std::vec::Vec<Row>,
    /// If requested, the totaled values of metrics.
    #[prost(message, repeated, tag = "4")]
    pub totals: ::std::vec::Vec<Row>,
    /// If requested, the maximum values of metrics.
    #[prost(message, repeated, tag = "5")]
    pub maximums: ::std::vec::Vec<Row>,
    /// If requested, the minimum values of metrics.
    #[prost(message, repeated, tag = "6")]
    pub minimums: ::std::vec::Vec<Row>,
    /// The total number of rows in the query result, regardless of the number of
    /// rows returned in the response. For example if a query returns 175 rows and
    /// includes limit = 50 in the API request, the response will contain row_count
    /// = 175 but only 50 rows.
    #[prost(int32, tag = "7")]
    pub row_count: i32,
    /// This Analytics Property's Realtime quota state including this request.
    #[prost(message, optional, tag = "8")]
    pub property_quota: ::std::option::Option<PropertyQuota>,
}
#[doc = r" Generated client implementations."]
pub mod alpha_analytics_data_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Google Analytics reporting data service."]
    pub struct AlphaAnalyticsDataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AlphaAnalyticsDataClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Returns a customized report of your Google Analytics event data. Reports"]
        #[doc = " contain statistics derived from data collected by the Google Analytics"]
        #[doc = " tracking code. The data returned from the API is as a table with columns"]
        #[doc = " for the requested dimensions and metrics. Metrics are individual"]
        #[doc = " measurements of user activity on your property, such as active users or"]
        #[doc = " event count. Dimensions break down metrics across some common criteria,"]
        #[doc = " such as country or event name."]
        pub async fn run_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunReportRequest>,
        ) -> Result<tonic::Response<super::RunReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a customized pivot report of your Google Analytics event data."]
        #[doc = " Pivot reports are more advanced and expressive formats than regular"]
        #[doc = " reports. In a pivot report, dimensions are only visible if they are"]
        #[doc = " included in a pivot. Multiple pivots can be specified to further dissect"]
        #[doc = " your data."]
        pub async fn run_pivot_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunPivotReportRequest>,
        ) -> Result<tonic::Response<super::RunPivotReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunPivotReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns multiple reports in a batch. All reports must be for the same"]
        #[doc = " Entity."]
        pub async fn batch_run_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchRunReportsRequest>,
        ) -> Result<tonic::Response<super::BatchRunReportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/BatchRunReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns multiple pivot reports in a batch. All reports must be for the same"]
        #[doc = " Entity."]
        pub async fn batch_run_pivot_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchRunPivotReportsRequest>,
        ) -> Result<tonic::Response<super::BatchRunPivotReportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/BatchRunPivotReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns metadata for dimensions and metrics available in reporting methods."]
        #[doc = " Used to explore the dimensions and metrics. In this method, a Google"]
        #[doc = " Analytics GA4 Property Identifier is specified in the request, and"]
        #[doc = " the metadata response includes Custom dimensions and metrics as well as"]
        #[doc = " Universal metadata."]
        #[doc = ""]
        #[doc = " For example if a custom metric with parameter name `levels_unlocked` is"]
        #[doc = " registered to a property, the Metadata response will contain"]
        #[doc = " `customEvent:levels_unlocked`. Universal metadata are dimensions and"]
        #[doc = " metrics applicable to any property such as `country` and `totalUsers`."]
        pub async fn get_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetadataRequest>,
        ) -> Result<tonic::Response<super::Metadata>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/GetMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " The Google Analytics Realtime API returns a customized report of realtime"]
        #[doc = " event data for your property. These reports show events and usage from the"]
        #[doc = " last 30 minutes."]
        pub async fn run_realtime_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunRealtimeReportRequest>,
        ) -> Result<tonic::Response<super::RunRealtimeReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunRealtimeReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AlphaAnalyticsDataClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AlphaAnalyticsDataClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AlphaAnalyticsDataClient {{ ... }}")
        }
    }
}
