use super::{IntoBytes, FromBytes};
use super::types::*;

pub struct BodyResResultRows {
    pub metadata: RowsMetadata,
    pub rows_count: i32,
    /// From spec: it is composed of <row_1>...<row_m> where m is <rows_count>.
    /// Each <row_i> is composed of <value_1>...<value_n> where n is
    /// <columns_count> and where <value_j> is a [bytes] representing the value
    /// returned for the jth column of the ith row.
    pub rows_content: Vec<Vec<u8>>
}

pub struct RowsMetadata {
    pub flags: Vec<RowsMetadataFlag>,
    pub columns_count: i32,
    pub paging_state: Option<Vec<u8>>,
    // In fact by specification Vec should have only two elements representing the
    // (unique) keyspace name and table name the columns belong to
    pub global_table_space: Option<Vec<String>>,
    pub col_specs: Vec<ColSpec>,
}

const GLOBAL_TABLE_SPACE: i32 = 0x0001;
const HAS_MORE_PAGES: i32 = 0x0002;
const NO_METADATA: i32 = 0x0004;

pub enum RowsMetadataFlag {
    GlobalTableSpace,
    HasMorePages,
    NoMetadata
}

impl RowsMetadataFlag {
    /// Shows if provided flag contains GlobalTableSpace rows metadata flag
    pub fn has_global_table_space(flag: i32) -> bool {
        return (flag & GLOBAL_TABLE_SPACE) != 0;
    }

    /// Sets GlobalTableSpace rows metadata flag
    pub fn set_global_table_space(flag: i32) -> i32 {
        return flag | GLOBAL_TABLE_SPACE;
    }

    /// Shows if provided flag contains HasMorePages rows metadata flag
    pub fn has_has_more_pages(flag: i32) -> bool {
        return (flag & HAS_MORE_PAGES) != 0;
    }

    /// Sets HasMorePages rows metadata flag
    pub fn set_has_more_pages(flag: i32) -> i32 {
        return flag | HAS_MORE_PAGES;
    }

    /// Shows if provided flag contains NoMetadata rows metadata flag
    pub fn has_no_metadata(flag: i32) -> bool {
        return (flag & NO_METADATA) != 0;
    }

    /// Sets NoMetadata rows metadata flag
    pub fn set_no_metadata(flag: i32) -> i32 {
        return flag | NO_METADATA;
    }
}

impl IntoBytes for RowsMetadataFlag {
    fn into_cbytes(&self) -> Vec<u8> {
        return match *self {
            RowsMetadataFlag::GlobalTableSpace => to_int(GLOBAL_TABLE_SPACE as i64),
            RowsMetadataFlag::HasMorePages => to_int(HAS_MORE_PAGES as i64),
            RowsMetadataFlag::NoMetadata => to_int(NO_METADATA as i64)
        };
    }
}

impl FromBytes for RowsMetadataFlag {
    fn from_bytes(bytes: Vec<u8>) -> RowsMetadataFlag {
        return match from_bytes(bytes.clone()) as i32 {
            GLOBAL_TABLE_SPACE => RowsMetadataFlag::GlobalTableSpace,
            HAS_MORE_PAGES => RowsMetadataFlag::HasMorePages,
            NO_METADATA => RowsMetadataFlag::NoMetadata,
            _ => {
                error!("Unexpected Cassandra rows metadata flag: {:?}", bytes);
                panic!("Unexpected Cassandra rows metadata flag: {:?}", bytes);
            }
        };
    }
}

pub struct ColSpec {
    /// The initial <ksname> is a [string] and is only present
    /// if the Global_tables_spec flag is not set
    pub ksname: Option<String>,
    /// The initial <tablename> is a [string] and is present
    /// if the Global_tables_spec flag is not set
    pub tablename: Option<String>,
    /// Column name
    pub name: String,
    /// Column type defined in spec in 4.2.5.2
    pub col_type: ColType
}

pub enum ColType {
    Custom,
    Ascii,
    Bigint,
    Blob,
    Boolean,
    Cunter,
    Decimal,
    Double,
    Float,
    Int,
    Timestamp,
    Uuid,
    Varchar,
    Varint,
    Timeuuid,
    Inet,
    Date,
    Time,
    Smallint,
    Tinyint,
    List,
    Map,
    Set,
    Udt,
    Tuple
}

impl FromBytes for ColType {
    fn from_bytes(bytes: Vec<u8>) -> ColType {
        return match from_bytes(bytes.clone()) {
            0x0000 => ColType::Custom,
            0x0001 => ColType::Ascii,
            0x0002 => ColType::Bigint,
            0x0003 => ColType::Blob,
            0x0004 => ColType::Boolean,
            0x0005 => ColType::Cunter,
            0x0006 => ColType::Decimal,
            0x0007 => ColType::Double,
            0x0008 => ColType::Float,
            0x0009 => ColType::Int,
            0x000B => ColType::Timestamp,
            0x000C => ColType::Uuid,
            0x000D => ColType::Varchar,
            0x000E => ColType::Varint,
            0x000F => ColType::Timeuuid,
            0x0010 => ColType::Inet,
            0x0011 => ColType::Date,
            0x0012 => ColType::Time,
            0x0013 => ColType::Smallint,
            0x0014 => ColType::Tinyint,
            0x0020 => ColType::List,
            0x0021 => ColType::Map,
            0x0022 => ColType::Set,
            0x0030 => ColType::Udt,
            0x0031 => ColType::Tuple,
            _ => {
                error!("Unexpected Cassandra column type: {:?}", bytes);
                panic!("Unexpected Cassandra column type: {:?}", bytes);
            }
        };
    }
}