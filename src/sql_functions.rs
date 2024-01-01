use anyhow::Ok;
use async_std::net::TcpStream;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use once_cell::sync::Lazy;
use std::{env, borrow::Cow};
use tiberius::{
    Client, ColumnData,Config, IntoRow, 
    numeric::{BigDecimal, BigInt},
    ToSql, TokenRow,
};

static SQL_AUTH_CONN_STR_PORT: Lazy<String> = Lazy::new(|| {
    env::var("SQL_AUTH_CONN_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost\\sql2022d,22828;database=DestinationDB;user=developer;password=developer;TrustServerCertificate=true".to_owned()
    })
});

/// Connect to a SQL Server instance using the hostname and port number.
pub async fn connect_through_port() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    // Create a `TCPStream` from the `async-std` library with
    // a address that contains the hostname/IP and port number.
    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    // Connect to SQL Server
    let client = Client::connect(config, tcp).await?;
    println!("Successfully connected to server.");

    client.close().await?;

    Ok(())
}

pub async fn bulk_insert_bit_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_bit").await?;

    println!("Start loading data");

    for i in 0..100000 {
        let bit = match i % 2 {
            0 => Some(false),
            _ => Some(true),
        };
        let new_row = (bit).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_float_real_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_float").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        // This data type of mapped to a `float` column
        let default_float_number = Some(i as f64);

        // This data type of mapped to a `float(n)` column
        // where `n` is the number of digits.
        let precision_float_number = Some(i as f32);

        // The `real` data type of SQL Server is not supported yet

        let new_row = (default_float_number, precision_float_number).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_integer_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_integer").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        let tinyint_number = Some(i as u8);
        let smallint_number = Some(i as i16);
        let int_number = Some(i as i32);
        let bigint_number = Some(i as i64);

        let new_row = (tinyint_number, smallint_number, int_number, bigint_number).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

#[cfg(feature = "use_tds73")]
pub async fn bulk_insert_decimal_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_decimal").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        let numeric = BigDecimal::from(i);
        let decimal = BigDecimal::from(i);

        let numeric_number = Some(numeric);
        let decimal_number = Some(decimal);

        // `numeric(p,s)` and `decimal(p,s)` with specific precision
        // and scale are not supported yet

        let new_row = (numeric_number, decimal_number).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_char_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_string").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        let char = Cow::from(format!("Hard disk : {0}", i));
        let nchar = Cow::from(format!("CD 💿: {0}", i));
        let varchar = Cow::from("Floppy: ".to_string() + i.to_string().as_str());
        let varchar_max = Cow::from(format!("Floppy {0} is no longer used nowadays.", i));
        let nvarchar = Cow::from(format!("SSD 🗄️: {0}", i));
        let nvarchar_max = Cow::from(format!(
            "Solid State Drive {0} is faster than hard disk 💽 {1}",
            i, i
        ));

        let char_string = Some(char);
        let nchar_string = Some(nchar);
        let varchar_string = Some(varchar);
        let varchar_max_string = Some(varchar_max);
        let nvarchar_string = Some(nvarchar);
        let nvarchar_max_string = Some(nvarchar_max);

        let new_row = (
            char_string,
            nchar_string,
            varchar_string,
            varchar_max_string,
            nvarchar_string,
            nvarchar_max_string,
        )
            .into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_binary_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_binary").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        let binary: &[u8] = &[10];
        let binary_specific: &[u8] = &[10, 20];
        let varbinary = "T".as_bytes().to_owned();
        let varbinary_specific = format!("There is varbinary data in the row: {0}", i)
            .as_bytes()
            .to_owned();
        let varbinary_max = (i as i32).to_le_bytes().to_vec();

        let binary_data = Some(binary);
        let binary_specific_data = Some(binary_specific);
        let varbinary_data = Some(varbinary);
        let varbinary_specific_data = Some(varbinary_specific);
        let varbinary_max_data = Some(varbinary_max);

        let new_row = (
            binary_data,
            binary_specific_data,
            varbinary_data,
            varbinary_specific_data,
            varbinary_max_data,
        )
            .into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_uniqueidentifier_column() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_guid").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        let seed = 0xbf3ba35b44de49dc86eaf1b5c69bcc8e ^ i;
        let new_uuid = tiberius::Uuid::from_u128(seed);

        // You can also use other functions like such as `parse_str` like this:
        // ```
        // tiberius::Uuid::parse_str("ad93ed82e692479785198ff01e32d9ae").unwrap();
        // ```
        // or `from_u128` like this:
        // ```
        // let number = 561de8c7a4e611ee8728c4b53ac415ddu128;
        // tiberius::Uuid::from_u128(number);
        // ```

        let uuid_value = Some(new_uuid);

        let new_row = (uuid_value).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn insert_money() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let result = client
        .execute(
            "INSERT INTO random_money (a_smallmoney_column, a_money_column) VALUES (@P1, @P2)",
            &[&1316i32, &701321588.2505f64],
        )
        .await?;

    println!("Rows affected: {}", result.total());
    client.close().await?;

    Ok(())
}

pub async fn insert_text_image() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let text = Cow::from(format!("There is text data in the row"));
    let ntext = Cow::from(format!("There is ntext data in the row"));
    let i: &[u8] = &[1, 5];

    let result = client
        .execute(
            "INSERT INTO random_text_image (a_text_column, a_ntext_column, a_image_column)
         VALUES (@P1, @P2, @P3)",
            &[&text, &ntext, &i],
        )
        .await?;

    println!("Rows affected: {}", result.total());
    client.close().await?;

    Ok(())
}

pub async fn insert_xml() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let xml = tiberius::xml::XmlData::new(
        r#"<calculator>
    <hex base="16">163</hex>
    <dec base="10">355</dec>
    <oct base="8">543</oct>
    <bin base="2">0001 0110 0011</bin>
</calculator>"#,
    );

    let result = client
        .execute(
            "INSERT INTO random_xml (a_xml_column) VALUES (@P1)",
            &[&xml],
        )
        .await?;

    println!("Rows affected: {}", result.total());
    client.close().await?;

    Ok(())
}

pub async fn bulk_insert_datetime_tds72() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_datetime").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        // Create a date struct using the Chrono library
        let date = NaiveDate::from_ymd_opt(2022, 8, 1);
        let time = NaiveTime::from_num_seconds_from_midnight_opt(i % 60, i);

        let mut new_date: Option<NaiveDateTime> = None;
        if let Some(dt) = date {
            if let Some(tm) = time {
                new_date = Some(NaiveDateTime::new(dt, tm));
            }
        }

        let new_row = (new_date).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_datetime2_tds72() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_datetime2").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        // Create a date struct using the Chrono library
        let date = NaiveDate::from_ymd_opt(2022, 8, 1);
        let time = NaiveTime::from_num_seconds_from_midnight_opt(i % 60, 0);

        let mut new_date: Option<NaiveDateTime> = None;
        if let Some(dt) = date {
            if let Some(tm) = time {
                new_date = Some(NaiveDateTime::new(dt, tm));
            }
        }

        let new_row = (new_date).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

#[cfg(feature = "use_tds73")]
pub async fn bulk_insert_datetimeoffset_tds73() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_datetimeoffset").await?;

    println!("Start loading data");

    for _ in 0..1000000 {
        let dt = chrono::Utc::now();

        // Get components
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();

        let dt_new = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive_utc, offset);

        let new_row = (Some(dt_new)).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn bulk_insert_smalldatetime_tds72() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_smalldatetime").await?;

    println!("Start loading data");

    for i in 0..10000 {
        // Create a date struct using the Chrono library
        let date = NaiveDate::from_ymd_opt(2023, 10, 1);
        let time = NaiveTime::from_num_seconds_from_midnight_opt(i % 60, 0);

        let mut new_date: Option<NaiveDateTime> = None;
        if let Some(dt) = date {
            if let Some(tm) = time {
                new_date = Some(NaiveDateTime::new(dt, tm));
            }
        }

        let new_row = (new_date).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

#[cfg(feature = "use_tds73")]
pub async fn insert_date() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let date = NaiveDate::from_ymd_opt(2023, 06, 30);

    let result = client
        .execute(
            "INSERT INTO random_date (a_date_column) VALUES (@P1)",
            &[&date.unwrap()],
        )
        .await?;

    println!("Rows affected: {}", result.total());
    client.close().await?;

    Ok(())
}

#[cfg(feature = "use_tds73")]
pub async fn bulk_insert_time_tds73() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_time").await?;

    println!("Start loading data");

    for i in 0..1000000 {
        let time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(i % 60, 0);

        let new_row = (time).into_row();

        result.send(new_row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn insert_datetime_datetime2() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let date = NaiveDateTime::from_timestamp_millis(126000);
    let reg_date = NaiveDateTime::from_timestamp_millis(231688);

    let result = client
        .execute(
            "INSERT INTO random_datetime_datetime2(a_datetime_column, a_datetime2_column)
         VALUES (@P1, @P2)",
            &[&date.unwrap(), &reg_date.unwrap()],
        )
        .await?;

    println!("Rows affected: {}", result.total());
    client.close().await?;

    Ok(())
}

#[cfg(feature = "use_tds73")]
pub async fn bulk_insert_more_than_10_columns() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;
    let mut result = client.bulk_insert("random_data_several_columns").await?;

    println!("Start loading data");

    let mut temporal_i: i32 = 0;
    let decimal_value = BigDecimal::from(29540577);
    let numeric_value = BigDecimal::from(30024);

    let dt = chrono::Utc::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset().clone();

    let today = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive_utc, offset);

    for i in 0..1000000 {
        let mut row = TokenRow::with_capacity(15);
        temporal_i = i.clone();
        row.push(ColumnData::Bit(Some(temporal_i % 3 == 0)));
        row.push(ColumnData::F64(Some((temporal_i + 1) as f64)));
        row.push(ColumnData::U8(Some(temporal_i as u8)));
        row.push(ColumnData::I16(Some(
            (temporal_i - temporal_i + 65535) as i16,
        )));
        row.push(ColumnData::I32(Some((temporal_i + 4) as i32)));
        row.push(ColumnData::I64(Some((temporal_i + 5) as i64)));

        row.push(numeric_value.to_sql());
        row.push(decimal_value.to_sql());

        let a_char_value = format!("{0} strawberry", i.to_owned());
        row.push(ColumnData::String(Some(Cow::from(a_char_value))));

        let a_nchar_value = format!("{0} blueberry 🫐", i.to_owned());
        row.push(ColumnData::String(Some(Cow::from(a_nchar_value))));

        let a_varchar_value = format!("{0} kiwi", i.to_owned());
        row.push(ColumnData::String(Some(Cow::from(a_varchar_value))));

        let a_nvarchar_value = format!("{0} tangerine 🍊", i.to_owned());
        row.push(ColumnData::String(Some(Cow::from(a_nvarchar_value))));

        row.push(ColumnData::Binary(Some(Cow::from(
            (temporal_i as i32).to_le_bytes().to_vec(),
        ))));
        row.push(ColumnData::Binary(Some(Cow::from(
            (temporal_i as i32).to_le_bytes().to_vec(),
        ))));
        row.push(ColumnData::Guid(Some(tiberius::Uuid::from_u128(
            temporal_i as u128,
        ))));

        row.push(today.to_sql());

        result.send(row).await?;
    }

    let res = result.finalize().await?;
    println!("Result: {:?}", res);

    Ok(())
}


#[cfg(feature = "use_tds73")]
pub async fn insert_precision_decimal()->anyhow::Result<()> {
    use std::str::FromStr;

    let config = Config::from_ado_string(&SQL_AUTH_CONN_STR_PORT)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;

    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    // Parse a decimal value with a with 6 decimal digits (scale)
    // This equals to value: 1.690601
    let numeric=BigDecimal::new( BigInt::parse_bytes(b"1690601", 10).unwrap(), 6);
    
    // Parse a hexadecimal value with 4 decimal digits (scale)
    // This equals to value: 4.3981
    let decimal=BigDecimal::new( BigInt::parse_bytes(b"ABCD", 16).unwrap(), 4);
    
    let result = client.execute(
        "INSERT INTO random_precision_decimal (a_numeric_precision_column, a_decimal_precision_column ) VALUES (@P1, @P2)", 
        &[&numeric, &decimal],
    ).await?;
    
    
    println!("Rows affected: {}",result.total());
    client.close().await?;

    Ok(())
}