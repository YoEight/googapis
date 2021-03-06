/// A managed metastore service that serves metadata queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Immutable. The relative resource name of the metastore service, of the form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The time when the metastore service was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the metastore service was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// User-defined labels for the metastore service.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Immutable. The relative resource name of the VPC network on which the instance can be
    /// accessed. The network must belong to the same project as the metastore
    /// instance. It is specified in the following form:
    ///
    /// "projects/{project_id}/global/networks/{network_id}".
    #[prost(string, tag = "7")]
    pub network: std::string::String,
    /// Output only. The URI of the endpoint used to access the metastore service.
    #[prost(string, tag = "8")]
    pub endpoint_uri: std::string::String,
    /// The TCP port at which the metastore service is reached. Default: 9083.
    #[prost(int32, tag = "9")]
    pub port: i32,
    /// Output only. The current state of the metastore service.
    #[prost(enumeration = "service::State", tag = "10")]
    pub state: i32,
    /// Output only. Additional information about the current state of the metastore service, if
    /// available.
    #[prost(string, tag = "11")]
    pub state_message: std::string::String,
    /// Output only. A Cloud Storage URI (starting with `gs://`) that specifies where artifacts
    /// related to the metastore service are stored.
    #[prost(string, tag = "12")]
    pub artifact_gcs_uri: std::string::String,
    /// The tier of the service.
    #[prost(enumeration = "service::Tier", tag = "13")]
    pub tier: i32,
    /// Configuration properties specific to the underlying metastore service
    /// technology (the software that serves metastore queries).
    #[prost(oneof = "service::MetastoreConfig", tags = "5")]
    pub metastore_config: ::std::option::Option<service::MetastoreConfig>,
}
pub mod service {
    /// The current state of the metastore service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the metastore service is unknown.
        Unspecified = 0,
        /// The metastore service is in the process of being created.
        Creating = 1,
        /// The metastore service is running and ready to serve queries.
        Active = 2,
        /// The metastore service is entering suspension. Its query-serving
        /// availability may cease unexpectedly.
        Suspending = 3,
        /// The metastore service is suspended and unable to serve queries.
        Suspended = 4,
        /// The metastore service is being updated. It remains usable but cannot
        /// accept additional update requests or be deleted at this time.
        Updating = 5,
        /// The metastore service is undergoing deletion. It cannot be used.
        Deleting = 6,
        /// The metastore service has encountered an error and cannot be used. The
        /// metastore service should be deleted.
        Error = 7,
    }
    /// Available service tiers.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tier {
        /// The tier is not set.
        Unspecified = 0,
        /// The enterprise tier combines a powerful metastore serving layer with a
        /// highly scalable data storage layer.
        Enterprise = 3,
    }
    /// Configuration properties specific to the underlying metastore service
    /// technology (the software that serves metastore queries).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MetastoreConfig {
        /// Configuration information specific to running Hive metastore
        /// software as the metastore service.
        #[prost(message, tag = "5")]
        HiveMetastoreConfig(super::HiveMetastoreConfig),
    }
}
/// Specifies configuration information specific to running Hive metastore
/// software as the metastore service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HiveMetastoreConfig {
    /// Immutable. The Hive metastore schema version.
    #[prost(string, tag = "1")]
    pub version: std::string::String,
    /// A mapping of Hive metastore configuration key-value pairs to apply to the
    /// Hive metastore (configured in `hive-site.xml`). The mappings
    /// override system defaults (some keys cannot be overridden).
    #[prost(map = "string, string", tag = "2")]
    pub config_overrides: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Information used to configure the Hive metastore service as a service
    /// principal in a Kerberos realm.
    #[prost(message, optional, tag = "3")]
    pub kerberos_config: ::std::option::Option<KerberosConfig>,
}
/// Configuration information for a Kerberos principal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KerberosConfig {
    /// A Kerberos keytab file that can be used to authenticate a service principal
    /// with a Kerberos Key Distribution Center (KDC).
    #[prost(message, optional, tag = "1")]
    pub keytab: ::std::option::Option<Secret>,
    /// A Kerberos principal that exists in the both the keytab the KDC
    /// to authenticate as. A typical principal is of the form
    /// "primary/instance@REALM", but there is no exact format.
    #[prost(string, tag = "2")]
    pub principal: std::string::String,
    /// A Cloud Storage URI that specifies the path to a
    /// krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf,
    /// although the file does not need to be named krb5.conf explicitly.
    #[prost(string, tag = "3")]
    pub krb5_config_gcs_uri: std::string::String,
}
/// A securely stored value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    #[prost(oneof = "secret::Value", tags = "2")]
    pub value: ::std::option::Option<secret::Value>,
}
pub mod secret {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The relative resource name of a Secret Manager secret version, in the
        /// following form:
        ///
        /// "projects/{project_id}/secrets/{secret_id}/versions/{version_id}".
        #[prost(string, tag = "2")]
        CloudSecret(std::string::String),
    }
}
/// A metastore resource that imports metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataImport {
    /// Immutable. The relative resource name of the metadata import, of the form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The description of the metadata import.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Output only. The time when the metadata import was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the metadata import was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the metadata import.
    #[prost(enumeration = "metadata_import::State", tag = "5")]
    pub state: i32,
    /// The metadata to be imported.
    #[prost(oneof = "metadata_import::Metadata", tags = "6")]
    pub metadata: ::std::option::Option<metadata_import::Metadata>,
}
pub mod metadata_import {
    /// A specification of the location of and metadata about a database dump from
    /// a relational database management system.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatabaseDump {
        /// The type of the database.
        #[prost(enumeration = "database_dump::DatabaseType", tag = "1")]
        pub database_type: i32,
        /// A Cloud Storage object URI that specifies the source from which to
        /// import metadata. It must begin with `gs://`.
        #[prost(string, tag = "2")]
        pub gcs_uri: std::string::String,
        /// The name of the source database.
        #[prost(string, tag = "3")]
        pub source_database: std::string::String,
    }
    pub mod database_dump {
        /// The type of the database.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum DatabaseType {
            /// The type of the source database is unknown.
            Unspecified = 0,
            /// The type of the source database is MySQL.
            Mysql = 1,
        }
    }
    /// The current state of the metadata import.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the metadata import is unknown.
        Unspecified = 0,
        /// The metadata import is running.
        Running = 1,
        /// The metadata import completed successfully.
        Succeeded = 2,
        /// The metadata import is being updated.
        Updating = 3,
        /// The metadata import failed, and attempted metadata changes were rolled
        /// back.
        Failed = 4,
    }
    /// The metadata to be imported.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Immutable. A database dump from a pre-existing metastore's database.
        #[prost(message, tag = "6")]
        DatabaseDump(DatabaseDump),
    }
}
/// Request message for [DataprocMetastore.ListServices][google.cloud.metastore.v1alpha.DataprocMetastore.ListServices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. The relative resource name of the location of metastore services to
    /// list, in the following form:
    ///
    /// "projects/{project_id}/locations/{location_id}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of services to return. The response may contain less
    /// than the maximum number. If unspecified, no more than 500 services are
    /// returned. The maximum value is 1000; values above 1000 are changed to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous [DataprocMetastore.ListServices][google.cloud.metastore.v1alpha.DataprocMetastore.ListServices]
    /// call. Provide this token to retrieve the subsequent page.
    ///
    /// To retrieve the first page, supply an empty page token.
    ///
    /// When paginating, other parameters provided to
    /// [DataprocMetastore.ListServices][google.cloud.metastore.v1alpha.DataprocMetastore.ListServices] must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify the ordering of results as described in
    /// [Sorting Order](/apis/design/design_patterns#sorting_order). If not
    /// specified, the results will be sorted in the default order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for [DataprocMetastore.ListServices][google.cloud.metastore.v1alpha.DataprocMetastore.ListServices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The services in the specified location.
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this
    /// field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for [DataprocMetastore.GetService][google.cloud.metastore.v1alpha.DataprocMetastore.GetService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The relative resource name of the metastore service to retrieve, in the
    /// following form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [DataprocMetastore.CreateService][google.cloud.metastore.v1alpha.DataprocMetastore.CreateService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. The relative resource name of the location in which to create a metastore
    /// service, in the following form:
    ///
    /// "projects/{project_id}/locations/{location_id}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the metastore service, which is used as the final
    /// component of the metastore service's name.
    ///
    /// This value must be between 1 and 64 characters long, begin with a
    /// letter, end with a letter or number, and consist of alpha-numeric
    /// ASCII characters or hyphens.
    #[prost(string, tag = "2")]
    pub service_id: std::string::String,
    /// Required. The Metastore service to create. The `name` field is
    /// ignored. The ID of the created metastore service must be provided in
    /// the request's `service_id` field.
    #[prost(message, optional, tag = "3")]
    pub service: ::std::option::Option<Service>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the
    /// request if it has completed. The server will ignore subsequent requests
    /// that provide a duplicate request ID for at least 60 minutes after the first
    /// request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier#Format)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for [DataprocMetastore.UpdateService][google.cloud.metastore.v1alpha.DataprocMetastore.UpdateService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. A field mask used to specify the fields to be overwritten in the
    /// metastore service resource by the update.
    /// Fields specified in the `update_mask` are relative to the resource (not
    /// to the full request). A field is overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. The metastore service to update. The server only merges fields
    /// in the service if they are specified in `update_mask`.
    ///
    /// The metastore service's `name` field is used to identify the metastore
    /// service to be updated.
    #[prost(message, optional, tag = "2")]
    pub service: ::std::option::Option<Service>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the
    /// request if it has completed. The server will ignore subsequent requests
    /// that provide a duplicate request ID for at least 60 minutes after the first
    /// request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier#Format)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Request message for [DataprocMetastore.DeleteService][google.cloud.metastore.v1alpha.DataprocMetastore.DeleteService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The relative resource name of the metastore service to delete, in the
    /// following form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the
    /// request if it has completed. The server will ignore subsequent requests
    /// that provide a duplicate request ID for at least 60 minutes after the first
    /// request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier#Format)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
}
/// Request message for [DataprocMetastore.ListMetadataImports][google.cloud.metastore.v1alpha.DataprocMetastore.ListMetadataImports].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataImportsRequest {
    /// Required. The relative resource name of the service whose metadata imports to
    /// list, in the following form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}/metadataImports".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of imports to return. The response may contain less
    /// than the maximum number. If unspecified, no more than 500 imports are
    /// returned. The maximum value is 1000; values above 1000 are changed to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous [DataprocMetastore.ListServices][google.cloud.metastore.v1alpha.DataprocMetastore.ListServices]
    /// call. Provide this token to retrieve the subsequent page.
    ///
    /// To retrieve the first page, supply an empty page token.
    ///
    /// When paginating, other parameters provided to
    /// [DataprocMetastore.ListServices][google.cloud.metastore.v1alpha.DataprocMetastore.ListServices] must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify the ordering of results as described in
    /// [Sorting Order](/apis/design/design_patterns#sorting_order). If not
    /// specified, the results will be sorted in the default order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for [DataprocMetastore.ListMetadataImports][google.cloud.metastore.v1alpha.DataprocMetastore.ListMetadataImports].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataImportsResponse {
    /// The imports in the specified service.
    #[prost(message, repeated, tag = "1")]
    pub metadata_imports: ::std::vec::Vec<MetadataImport>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this
    /// field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for [DataprocMetastore.GetMetadataImport][google.cloud.metastore.v1alpha.DataprocMetastore.GetMetadataImport].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetadataImportRequest {
    /// Required. The relative resource name of the metadata import to retrieve, in the
    /// following form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}/metadataImports/{import_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [DataprocMetastore.CreateMetadataImport][google.cloud.metastore.v1alpha.DataprocMetastore.CreateMetadataImport].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataImportRequest {
    /// Required. The relative resource name of the service in which to create a metastore
    /// import, in the following form:
    ///
    /// "projects/{project_id}/locations/{location_id}/services/{service_id}"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the metadata import, which is used as the final component of the
    /// metadata import's name.
    ///
    /// This value must be between 1 and 64 characters long, begin with a letter,
    /// end with a letter or number, and consist of alpha-numeric ASCII characters
    /// or hyphens.
    #[prost(string, tag = "2")]
    pub metadata_import_id: std::string::String,
    /// Required. The metadata import to create. The `name` field is ignored. The ID of the
    /// created metadata import must be provided in the request's
    /// `metadata_import_id` field.
    #[prost(message, optional, tag = "3")]
    pub metadata_import: ::std::option::Option<MetadataImport>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the
    /// request if it has completed. The server will ignore subsequent requests
    /// that provide a duplicate request ID for at least 60 minutes after the first
    /// request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier#Format)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for [DataprocMetastore.UpdateMetadataImport][google.cloud.metastore.v1alpha.DataprocMetastore.UpdateMetadataImport].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataImportRequest {
    /// Required. A field mask used to specify the fields to be overwritten in the
    /// metadata import resource by the update.
    /// Fields specified in the `update_mask` are relative to the resource (not
    /// to the full request). A field is overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. The metadata import to update. The server only merges fields
    /// in the import if they are specified in `update_mask`.
    ///
    /// The metadata import's `name` field is used to identify the metastore
    /// import to be updated.
    #[prost(message, optional, tag = "2")]
    pub metadata_import: ::std::option::Option<MetadataImport>,
    /// Optional. A request ID. Specify a unique request ID to allow the server to ignore the
    /// request if it has completed. The server will ignore subsequent requests
    /// that provide a duplicate request ID for at least 60 minutes after the first
    /// request.
    ///
    /// For example, if an initial request times out, followed by another request
    /// with the same request ID, the server ignores the second request to prevent
    /// the creation of duplicate commitments.
    ///
    /// The request ID must be a valid
    /// [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier#Format)
    /// A zero UUID (00000000-0000-0000-0000-000000000000) is not supported.
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: std::string::String,
    /// Output only. Identifies whether the caller has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: std::string::String,
}
/// Metadata about the service in a location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// The versions of Hive Metastore that can be used when creating a new
    /// metastore service in this location. The server guarantees that exactly one
    /// `HiveMetastoreVersion` in the list will set `is_default`.
    #[prost(message, repeated, tag = "1")]
    pub supported_hive_metastore_versions: ::std::vec::Vec<location_metadata::HiveMetastoreVersion>,
}
pub mod location_metadata {
    /// A specification of a supported version of the Hive Metastore software.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HiveMetastoreVersion {
        /// The semantic version of the Hive Metastore software.
        #[prost(string, tag = "1")]
        pub version: std::string::String,
        /// Whether `version` will be chosen by the server if a metastore service is
        /// created with a `HiveMetastoreConfig` that omits the `version`.
        #[prost(bool, tag = "2")]
        pub is_default: bool,
    }
}
#[doc = r" Generated client implementations."]
pub mod dataproc_metastore_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Configures and manages metastore services."]
    #[doc = " Metastore services are fully managed, highly available, auto-scaled,"]
    #[doc = " auto-healing, OSS-native deployments of technical metadata management"]
    #[doc = " software. Each metastore service exposes a network endpoint through which"]
    #[doc = " metadata queries are served. Metadata queries can originate from a variety"]
    #[doc = " of sources, including Apache Hive, Apache Presto, and Apache Spark."]
    #[doc = ""]
    #[doc = " The Dataproc Metastore API defines the following resource model:"]
    #[doc = ""]
    #[doc = " * The service works with a collection of Google Cloud projects, named:"]
    #[doc = " `/projects/*`"]
    #[doc = " * Each project has a collection of available locations, named: `/locations/*`"]
    #[doc = "   (a location must refer to a Google Cloud `region`)"]
    #[doc = " * Each location has a collection of services, named: `/services/*`"]
    #[doc = " * Dataproc Metastore services are resources with names of the form:"]
    #[doc = ""]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/services/{service_id}`."]
    pub struct DataprocMetastoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataprocMetastoreClient<T>
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
        #[doc = " Lists services in a project and location."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a single service."]
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a metastore service in a project and location."]
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/CreateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single service."]
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/UpdateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single service."]
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists imports in a service."]
        pub async fn list_metadata_imports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataImportsRequest>,
        ) -> Result<tonic::Response<super::ListMetadataImportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/ListMetadataImports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single import."]
        pub async fn get_metadata_import(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetadataImportRequest>,
        ) -> Result<tonic::Response<super::MetadataImport>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/GetMetadataImport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new MetadataImport in a given project and location."]
        pub async fn create_metadata_import(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetadataImportRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/CreateMetadataImport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a single import."]
        #[doc = " Only the description field of MetadataImport is supported to be updated."]
        pub async fn update_metadata_import(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMetadataImportRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.metastore.v1alpha.DataprocMetastore/UpdateMetadataImport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DataprocMetastoreClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DataprocMetastoreClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DataprocMetastoreClient {{ ... }}")
        }
    }
}
