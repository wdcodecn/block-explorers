use crate::{block_number::BlockNumber, Client, EtherscanError, Response, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(missing_copy_implementations)]
pub struct BlockNumberByTimestamp {
    pub timestamp: u64,
    pub block_number: BlockNumber,
}

impl Client {
    /// Returns either (1) the oldest block since a particular timestamp occurred or (2) the newest
    /// block that occurred prior to that timestamp
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn foo(client: foundry_block_explorers::Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // The newest block that occurred prior to 1 January 2020
    /// let block_number_before = client.get_block_by_timestamp(1577836800, "before");
    /// // The oldest block that occurred after 1 January 2020
    /// let block_number_after = client.get_block_by_timestamp(1577836800, "after");
    /// # Ok(()) }
    /// ```
    pub async fn get_block_by_timestamp(
        &self,
        timestamp: u64,
        closest: &str,
    ) -> Result<BlockNumberByTimestamp> {
        let query = self.create_query(
            "block",
            "getblocknobytime",
            HashMap::from([("timestamp", timestamp.to_string()), ("closest", closest.to_string())]),
        );
        let response: Response<String> = self.get_json(&query).await?;

        match response.status.as_str() {
            "0" => Err(EtherscanError::BlockNumberByTimestampFailed),
            "1" => Ok(BlockNumberByTimestamp {
                timestamp,
                block_number: response.result.parse::<BlockNumber>().unwrap(),
            }),
            err => Err(EtherscanError::BadStatusCode(err.to_string())),
        }
    }

    /*
       &fromBlock=4993830
    &toBlock=4993832
    &address=0xe561479bebee0e606c19bb1973fc4761613e3c42
    &topic0=0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
    &topic0_1_opr=and
    &topic1=0x000000000000000000000000730e2065b9daee84c3003c05bf6d2b3a08e55667
    */
    pub async fn get_logs(
        &self,
        fromBlock: u64,
        toBlock: u64,
        address: &str,
        topic0: &str,
        topic1: &str,
        topic2: &str,
    ) -> Result<Vec<LogEntry>> {
        let query1 = self.create_query(
            "logs",
            "getLogs",
            HashMap::from([
                ("fromBlock", fromBlock.to_string()),
                ("toBlock", toBlock.to_string()),
                // ("address", address.to_string()), // 不过滤指定uniswap地址
                ("topic0", topic0.to_string()),
                ("topic1", topic1.to_string()),
                ("topic0_1_opr", "and".to_string()),
            ]),
        );
        // 失败了 添加重试
        let response1: Response<Vec<LogEntry>> = self.get_json(&query1).await?;

        let result1 = match response1.status.as_str() {
            "0" => {
                if response1.message == "No records found" {
                    Ok(vec![])
                } else {
                    Err(EtherscanError::BlockNumberByTimestampFailed)
                }
            }
            "1" => Ok(response1.result),
            err => Err(EtherscanError::BadStatusCode(err.to_string())),
        };

        println!("result1: {:?}", result1);

        let query2 = self.create_query(
            "logs",
            "getLogs",
            HashMap::from([
                ("fromBlock", fromBlock.to_string()),
                ("toBlock", toBlock.to_string()),
                // ("address", address.to_string()), // 不过滤指定uniswap地址
                ("topic0", topic0.to_string()),
                ("topic2", topic2.to_string()),
                ("topic0_2_opr", "and".to_string()),
            ]),
        );
        // 添加 tokio sleep

        sleep(Duration::from_millis(500)).await;
        println!("500 ms have elapsed");

        let response2: Response<Vec<LogEntry>> = self.get_json(&query2).await?;

        let result2 = match response2.status.as_str() {
            "0" => {
                if response2.message == "No records found" {
                    Ok(vec![])
                } else {
                    Err(EtherscanError::BlockNumberByTimestampFailed)
                }
            }

            "1" => Ok(response2.result),
            err => Err(EtherscanError::BadStatusCode(err.to_string())),
        };
        println!("result2: {:?}", result2);

        // 合并结果
        let mut result1: Vec<LogEntry> = result1.unwrap_or(vec![]);
        let mut result2: Vec<LogEntry> = result2.unwrap_or(vec![]);

        result1.extend(result2.clone());

        // 通过  block_number  log_index 排序 升序
        // if !&result1.is_empty() && !&result2.is_empty() {
        result1.sort_by(|a, b| {
            let a_block_number = a.block_number.parse::<u64>().unwrap();
            let b_block_number = b.block_number.parse::<u64>().unwrap();
            let a_log_index = a.log_index.parse::<u64>().unwrap();
            let b_log_index = b.log_index.parse::<u64>().unwrap();
            a_block_number.cmp(&b_block_number).then(a_log_index.cmp(&b_log_index))
        });
        // }
        Ok(result1)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub block_hash: String,
    pub time_stamp: String,
    pub gas_price: String,
    pub gas_used: String,
    pub log_index: String,
    pub transaction_hash: String,
    pub transaction_index: String,
}
