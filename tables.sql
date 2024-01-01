create table dbo.random_bit
(
    a_bit_column bit
)

create table dbo.random_float
(
    a_float_column float, -- Stores 15 digits
    a_float_precision_column float(3) -- Stores 7 digits
)

create table dbo.random_integer
(
    a_tinyint_column tinyint,
    a_smallint_column smallint,
    a_int_column int,
    a_bigint_column bigint
)

create table dbo.random_money
(
    a_smallmoney_column smallmoney,
    a_money_column money
)

create table dbo.random_decimal
(
    a_numeric_column numeric, -- defaults to a precision of 18 (includes decimal digits)
    a_decimal_column decimal, -- defaults to a precision of 18 (includes decimal digits)
)

create table dbo.random_precision_decimal
(
    a_numeric_precision_column numeric(10,6),
    a_decimal_precision_column decimal(5,4)
)

create table dbo.random_string
(
    a_char_column char(21),
    a_nchar_column nchar(14),
    a_varchar_column varchar(16),
    a_varchar_max_column varchar(max),
    a_nvarchar_column nvarchar(15),
    a_nvarchar_max_column nvarchar(max)
)

create table dbo.random_binary
(
    a_binary_column binary,
    a_binary_specific_column binary(700),
    a_varbinary_column varbinary,
    a_varbinary_specific_column varbinary(2900),
    a_varbinary_max_column varbinary(max),
)

create table dbo.random_text_image
(
    a_text_column text,
    a_ntext_column ntext,
    a_image_column image
)

create table dbo.random_guid
(
    a_guid_column uniqueidentifier
)

create table dbo.random_xml
(
    a_xml_column xml
)

create table dbo.random_datetime
(
    a_datetime_column datetime
)

create table dbo.random_datetime2
(
    a_datetime_column datetime
)

create table dbo.random_datetimeoffset
(
    a_datetimeoffset_column datetimeoffset
)

create table dbo.random_smalldatetime
(
    a_smalldatetime_column smalldatetime
)

create table dbo.random_date
(
    a_date_column date
)

create table dbo.random_time
(
    a_time_column time
)

create table dbo.random_datetime_datetime2
(
    a_datetime_column datetime,
    a_datetime2_column datetime2
)

create table dbo.random_data_several_columns
(
    mycolumn1 bit,
    mycolumn2 float,
    mycolumn3 tinyint,
    mycolumn4 smallint,
    mycolumn5 int,
    mycolumn6 bigint,
    mycolumn7 numeric,
    mycolumn8 decimal,
    mycolumn9 char(21),
    mycolumn10 nchar(43),
    mycolumn11 varchar(55),
    mycolumn12 nvarchar(70),
    mycolumn13 binary(21),
    mycolumn14 varbinary(27),
    mycolumn15 uniqueidentifier,
    register_date datetimeoffset
)