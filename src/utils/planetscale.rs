use crate::utils::env_var::get_env_var;
use crate::utils::schema::{Network, PsGetBlockTxid, PsGetExtremeBlock};
use anyhow::Error;
use planetscale_driver::{query, PSConnection};
use serde_json::Value;

async fn ps_init() -> PSConnection {
    let host = get_env_var("DATABASE_HOST").unwrap();
    let username = get_env_var("DATABASE_USERNAME").unwrap();
    let password = get_env_var("DATABASE_PASSWORD").unwrap();

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    conn
}

pub async fn ps_archive_block(
    network_block_id: &u64,
    wvm_calldata_txid: &str,
) -> Result<(), Error> {
    // format to the table VAR(66) limitation
    let wvm_calldata_txid = wvm_calldata_txid.trim_matches('"');
    let conn = ps_init().await;

    let res =
        query("INSERT INTO GOATWeaveVMArchiver(NetworkBlockId, WeaveVMArchiveTxid) VALUES($0, \"$1\")")
            .bind(network_block_id)
            .bind(wvm_calldata_txid)
            .execute(&conn)
            .await;

    match res {
        Ok(result) => {
            println!("Insert operation was successful: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            println!("Error occurred during insert operation: {:?}", e);
            Err(e)
        }
    }
}

pub async fn ps_get_latest_block_id() -> u64 {
    let network = Network::config();
    let conn = ps_init().await;

    let latest_archived: u64 =
        query("SELECT MAX(NetworkBlockId) AS LatestNetworkBlockId FROM GOATWeaveVMArchiver;")
            .fetch_scalar(&conn)
            .await
            .unwrap_or(network.start_block);
    // return latest archived block in planetscale + 1
    // so the process can start archiving from latest_archived + 1
    latest_archived + 1
}

pub async fn ps_get_archived_block_txid(id: u64) -> Value {
    let conn = ps_init().await;

    let query_formatted = format!(
        "SELECT WeaveVMArchiveTxid FROM GOATWeaveVMArchiver WHERE NetworkBlockId = {}",
        id
    );
    let txid: PsGetBlockTxid = query(&query_formatted).fetch_one(&conn).await.unwrap();

    let res = serde_json::json!(txid);
    res
}

pub async fn ps_get_blocks_extremes(extreme: &str) -> Value {
    let conn = ps_init().await;

    let query_type = match extreme {
        "first" => "ASC",
        "last" => "DESC",
        _ => panic!("invalid extreme value. Use 'first' or 'last'."),
    };

    let query_formatted = format!(
        "SELECT NetworkBlockId FROM GOATWeaveVMArchiver ORDER BY NetworkBlockId {} LIMIT 1;",
        query_type
    );

    let query: PsGetExtremeBlock = query(&query_formatted).fetch_one(&conn).await.unwrap();

    let res = serde_json::json!(query);
    res
}
