mod sql_functions;

#[cfg(test)]
mod tests {
    use crate::sql_functions::*;

    #[async_std::test]
    async fn test_connect_through_port() {
        let result = connect_through_port().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_bit_column() {
        let result = bulk_insert_bit_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_float_real_column() {
        let result = bulk_insert_float_real_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_integer_column() {
        let result = bulk_insert_integer_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[cfg(feature="use_tds73")]
    #[async_std::test]
    async fn test_bulk_insert_decimal_column() {
        let result = bulk_insert_decimal_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[cfg(feature="use_tds73")]
    #[async_std::test]
    async fn test_insert_precision_decimal() {
        let result = insert_precision_decimal().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_char_column() {
        let result = bulk_insert_char_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_binary_column() {
        let result = bulk_insert_binary_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_uniqueidentifier_column() {
        let result = bulk_insert_uniqueidentifier_column().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_insert_xml(){
        let result = insert_xml().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_insert_money(){
        let result = insert_money().await;
        assert_eq!(result.is_ok(), true);
    }  

    #[async_std::test]
    async fn test_insert_text_image(){
        let result = insert_text_image().await;
        assert_eq!(result.is_ok(), true);
    }  

    #[async_std::test]
    async fn test_bulk_insert_datetime_tds72(){
        let result = bulk_insert_datetime_tds72().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_datetime2_tds72(){
        let result = bulk_insert_datetime2_tds72().await;
        assert_eq!(result.is_ok(), true);
    }

    #[cfg(feature="use_tds73")]
    #[async_std::test]
    async fn test_bulk_insert_datetimeoffset_tds73(){
        let result = bulk_insert_datetimeoffset_tds73().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_bulk_insert_smalldatetime_tds72(){
        let result = bulk_insert_smalldatetime_tds72().await;
        assert_eq!(result.is_ok(), true);
    }

    #[cfg(feature="use_tds73")]
    #[async_std::test]
    async fn test_insert_date(){
        let result = insert_date().await;
        assert_eq!(result.is_ok(), true);
    }

    #[cfg(feature="use_tds73")]
    #[async_std::test]
    async fn test_bulk_insert_time_tds73(){
        let result = bulk_insert_time_tds73().await;
        assert_eq!(result.is_ok(), true);
    }

    #[async_std::test]
    async fn test_insert_datetime_datetime2(){
        let result = insert_datetime_datetime2().await;
        assert_eq!(result.is_ok(), true);
    }

    #[cfg(feature = "use_tds73")]
    #[async_std::test]
    async fn test_bulk_insert_more_than_10_columns(){
        let result = bulk_insert_more_than_10_columns().await;
        assert_eq!(result.is_ok(), true);
    }
}
