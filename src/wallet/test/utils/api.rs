use super::*;

pub(crate) fn test_blind_receive(wallet: &Wallet) -> ReceiveData {
    wallet
        .blind_receive(
            None,
            None,
            None,
            TRANSPORT_ENDPOINTS.clone(),
            MIN_CONFIRMATIONS,
        )
        .unwrap()
}

pub(crate) fn test_witness_receive(wallet: &Wallet) -> ReceiveData {
    wallet
        .witness_receive(
            None,
            None,
            None,
            TRANSPORT_ENDPOINTS.clone(),
            MIN_CONFIRMATIONS,
        )
        .unwrap()
}

pub(crate) fn test_create_utxos_default(wallet: &Wallet, online: &Online) -> u8 {
    _test_create_utxos(wallet, online, false, None, None, FEE_RATE)
}

pub(crate) fn test_create_utxos(
    wallet: &Wallet,
    online: &Online,
    up_to: bool,
    num: Option<u8>,
    size: Option<u32>,
    fee_rate: f32,
) -> u8 {
    _test_create_utxos(wallet, online, up_to, num, size, fee_rate)
}

pub(crate) fn test_create_utxos_begin_result(
    wallet: &Wallet,
    online: &Online,
    up_to: bool,
    num: Option<u8>,
    size: Option<u32>,
    fee_rate: f32,
) -> Result<String, Error> {
    wallet.create_utxos_begin(online.clone(), up_to, num, size, fee_rate)
}

pub(crate) fn _test_create_utxos(
    wallet: &Wallet,
    online: &Online,
    up_to: bool,
    num: Option<u8>,
    size: Option<u32>,
    fee_rate: f32,
) -> u8 {
    let delay = 200;
    let mut retries = 3;
    let mut num_utxos_created = 0;
    while retries > 0 {
        retries -= 1;
        let result = wallet.create_utxos(online.clone(), up_to, num, size, fee_rate);
        match result {
            Ok(_) => {
                num_utxos_created = result.unwrap();
                break;
            }
            Err(Error::InsufficientBitcoins {
                needed: _,
                available: _,
            }) => {
                std::thread::sleep(Duration::from_millis(delay));
                continue;
            }
            Err(error) => {
                panic!("error creating UTXOs for wallet: {error:?}");
            }
        }
    }
    if num_utxos_created == 0 {
        panic!("error creating UTXOs for wallet: insufficient bitcoins");
    }
    num_utxos_created
}

pub(crate) fn test_delete_transfers(
    wallet: &Wallet,
    batch_transfer_idx: Option<i32>,
    no_asset_only: bool,
) -> bool {
    test_delete_transfers_result(wallet, batch_transfer_idx, no_asset_only).unwrap()
}

pub(crate) fn test_delete_transfers_result(
    wallet: &Wallet,
    batch_transfer_idx: Option<i32>,
    no_asset_only: bool,
) -> Result<bool, Error> {
    wallet.delete_transfers(batch_transfer_idx, no_asset_only)
}

pub(crate) fn test_drain_to_result(
    wallet: &Wallet,
    online: &Online,
    address: &str,
    destroy_assets: bool,
) -> Result<String, Error> {
    wallet.drain_to(
        online.clone(),
        address.to_string(),
        destroy_assets,
        FEE_RATE,
    )
}

pub(crate) fn test_drain_to_begin_result(
    wallet: &Wallet,
    online: &Online,
    address: &str,
    destroy_assets: bool,
    fee_rate: f32,
) -> Result<String, Error> {
    wallet.drain_to_begin(
        online.clone(),
        address.to_string(),
        destroy_assets,
        fee_rate,
    )
}

pub(crate) fn test_drain_to_destroy(wallet: &Wallet, online: &Online, address: &str) -> String {
    wallet
        .drain_to(online.clone(), address.to_string(), true, FEE_RATE)
        .unwrap()
}

pub(crate) fn test_drain_to_keep(wallet: &Wallet, online: &Online, address: &str) -> String {
    wallet
        .drain_to(online.clone(), address.to_string(), false, FEE_RATE)
        .unwrap()
}

pub(crate) fn test_fail_transfers_all(wallet: &Wallet, online: &Online) -> bool {
    wallet.fail_transfers(online.clone(), None, false).unwrap()
}

pub(crate) fn test_fail_transfers_single(
    wallet: &Wallet,
    online: &Online,
    batch_transfer_idx: i32,
) -> bool {
    wallet
        .fail_transfers(online.clone(), Some(batch_transfer_idx), false)
        .unwrap()
}

pub(crate) fn test_get_address(wallet: &Wallet) -> String {
    wallet.get_address().unwrap()
}

pub(crate) fn test_get_asset_balance(wallet: &Wallet, asset_id: &str) -> Balance {
    test_get_asset_balance_result(wallet, asset_id).unwrap()
}

pub(crate) fn test_get_asset_balance_result(
    wallet: &Wallet,
    asset_id: &str,
) -> Result<Balance, Error> {
    wallet.get_asset_balance(asset_id.to_string())
}

pub(crate) fn test_get_asset_metadata(wallet: &Wallet, asset_id: &str) -> Metadata {
    test_get_asset_metadata_result(wallet, asset_id).unwrap()
}

pub(crate) fn test_get_asset_metadata_result(
    wallet: &Wallet,
    asset_id: &str,
) -> Result<Metadata, Error> {
    wallet.get_asset_metadata(asset_id.to_string())
}

pub(crate) fn test_get_btc_balance(wallet: &Wallet, online: &Online) -> BtcBalance {
    wallet.get_btc_balance(online.clone()).unwrap()
}

pub(crate) fn test_get_wallet_data(wallet: &Wallet) -> WalletData {
    wallet.get_wallet_data()
}

pub(crate) fn test_get_wallet_dir(wallet: &Wallet) -> PathBuf {
    wallet.get_wallet_dir()
}

pub(crate) fn test_go_online(
    wallet: &mut Wallet,
    skip_consistency_check: bool,
    electrum_url: Option<&str>,
) -> Online {
    test_go_online_result(wallet, skip_consistency_check, electrum_url).unwrap()
}

pub(crate) fn test_go_online_result(
    wallet: &mut Wallet,
    skip_consistency_check: bool,
    electrum_url: Option<&str>,
) -> Result<Online, Error> {
    let electrum = electrum_url.unwrap_or(ELECTRUM_URL).to_string();
    wallet.go_online(skip_consistency_check, electrum)
}

pub(crate) fn test_issue_asset_uda(
    wallet: &Wallet,
    online: &Online,
    details: Option<&str>,
    media_file_path: Option<&str>,
    attachments_file_paths: Vec<&str>,
) -> AssetUDA {
    test_issue_asset_uda_result(
        wallet,
        online,
        details,
        media_file_path,
        attachments_file_paths,
    )
    .unwrap()
}

pub(crate) fn test_issue_asset_uda_result(
    wallet: &Wallet,
    online: &Online,
    details: Option<&str>,
    media_file_path: Option<&str>,
    attachments_file_paths: Vec<&str>,
) -> Result<AssetUDA, Error> {
    wallet.issue_asset_uda(
        online.clone(),
        TICKER.to_string(),
        NAME.to_string(),
        details.map(|d| d.to_string()),
        PRECISION,
        media_file_path.map(|m| m.to_string()),
        attachments_file_paths
            .iter()
            .map(|a| a.to_string())
            .collect(),
    )
}

pub(crate) fn test_issue_asset_cfa(
    wallet: &Wallet,
    online: &Online,
    amounts: Option<&[u64]>,
    file_path: Option<String>,
) -> AssetCFA {
    test_issue_asset_cfa_result(wallet, online, amounts, file_path).unwrap()
}

pub(crate) fn test_issue_asset_cfa_result(
    wallet: &Wallet,
    online: &Online,
    amounts: Option<&[u64]>,
    file_path: Option<String>,
) -> Result<AssetCFA, Error> {
    let amounts = if let Some(a) = amounts {
        a.to_vec()
    } else {
        vec![AMOUNT]
    };
    wallet.issue_asset_cfa(
        online.clone(),
        NAME.to_string(),
        Some(DETAILS.to_string()),
        PRECISION,
        amounts,
        file_path,
    )
}

pub(crate) fn test_issue_asset_nia(
    wallet: &Wallet,
    online: &Online,
    amounts: Option<&[u64]>,
) -> AssetNIA {
    test_issue_asset_nia_result(wallet, online, amounts).unwrap()
}

pub(crate) fn test_issue_asset_nia_result(
    wallet: &Wallet,
    online: &Online,
    amounts: Option<&[u64]>,
) -> Result<AssetNIA, Error> {
    let amounts = if let Some(a) = amounts {
        a.to_vec()
    } else {
        vec![AMOUNT]
    };
    wallet.issue_asset_nia(
        online.clone(),
        TICKER.to_string(),
        NAME.to_string(),
        PRECISION,
        amounts,
    )
}

pub(crate) fn test_list_assets(wallet: &Wallet, filter_asset_schemas: &[AssetSchema]) -> Assets {
    wallet.list_assets(filter_asset_schemas.to_vec()).unwrap()
}

pub(crate) fn test_list_transactions(wallet: &Wallet, online: Option<&Online>) -> Vec<Transaction> {
    let online = online.cloned();
    wallet.list_transactions(online).unwrap()
}

pub(crate) fn test_list_transfers(wallet: &Wallet, asset_id: Option<&str>) -> Vec<Transfer> {
    test_list_transfers_result(wallet, asset_id).unwrap()
}

pub(crate) fn test_list_transfers_result(
    wallet: &Wallet,
    asset_id: Option<&str>,
) -> Result<Vec<Transfer>, Error> {
    let asset_id = asset_id.map(|a| a.to_string());
    wallet.list_transfers(asset_id)
}

pub(crate) fn test_list_unspents(
    wallet: &Wallet,
    online: Option<&Online>,
    settled_only: bool,
) -> Vec<Unspent> {
    let online = online.cloned();
    wallet.list_unspents(online, settled_only).unwrap()
}

pub(crate) fn test_list_unspents_vanilla(
    wallet: &Wallet,
    online: &Online,
    min_confirmations: Option<u8>,
) -> Vec<LocalUtxo> {
    let min_confirmations = min_confirmations.unwrap_or(MIN_CONFIRMATIONS);
    wallet
        .list_unspents_vanilla(online.clone(), min_confirmations)
        .unwrap()
}

pub(crate) fn test_refresh_all(wallet: &Wallet, online: &Online) -> bool {
    wallet.refresh(online.clone(), None, vec![]).unwrap()
}

pub(crate) fn test_refresh_asset(wallet: &Wallet, online: &Online, asset_id: &str) -> bool {
    wallet
        .refresh(online.clone(), Some(asset_id.to_string()), vec![])
        .unwrap()
}

pub(crate) fn test_save_new_asset(
    wallet: &Wallet,
    online: &Online,
    rcv_wallet: &Wallet,
    asset_id: &String,
    amount: u64,
) {
    let receive_data = test_witness_receive(rcv_wallet);
    let recipient_map = HashMap::from([(
        asset_id.clone(),
        vec![Recipient {
            amount,
            recipient_data: RecipientData::WitnessData {
                script_buf: ScriptBuf::from_hex(&receive_data.recipient_id).unwrap(),
                amount_sat: 1000,
                blinding: None,
            },
            transport_endpoints: TRANSPORT_ENDPOINTS.clone(),
        }],
    )]);
    let txid = test_send(wallet, online, &recipient_map);
    assert!(!txid.is_empty());

    let txid_dir = wallet._transfers_dir().join(txid);
    let asset_transfer_dir = txid_dir.join(asset_id);
    let consignment_path = asset_transfer_dir.join(CONSIGNMENT_FILE);

    let bindle = Bindle::<RgbTransfer>::load(consignment_path).unwrap();
    let consignment: RgbTransfer = bindle.unbindle();
    let mut contract = consignment.clone().into_contract();

    contract.bundles = none!();
    contract.terminals = none!();
    let minimal_contract_validated =
        match contract.validate(&mut rcv_wallet._blockchain_resolver().unwrap()) {
            Ok(consignment) => consignment,
            Err(consignment) => consignment,
        };

    let mut runtime = rcv_wallet._rgb_runtime().unwrap();
    runtime
        .import_contract(
            minimal_contract_validated.clone(),
            &mut rcv_wallet._blockchain_resolver().unwrap(),
        )
        .unwrap();
    let schema_id = minimal_contract_validated.schema_id().to_string();
    let asset_schema = AssetSchema::from_schema_id(schema_id).unwrap();
    rcv_wallet
        .save_new_asset(
            &mut runtime,
            &asset_schema,
            minimal_contract_validated.contract_id(),
        )
        .unwrap();
}

pub(crate) fn test_send(
    wallet: &Wallet,
    online: &Online,
    recipient_map: &HashMap<String, Vec<Recipient>>,
) -> String {
    test_send_result(wallet, online, recipient_map)
        .unwrap()
        .txid
}

pub(crate) fn test_send_result(
    wallet: &Wallet,
    online: &Online,
    recipient_map: &HashMap<String, Vec<Recipient>>,
) -> Result<SendResult, Error> {
    wallet.send(
        online.clone(),
        recipient_map.clone(),
        false,
        FEE_RATE,
        MIN_CONFIRMATIONS,
    )
}

pub(crate) fn test_send_begin_result(
    wallet: &Wallet,
    online: &Online,
    recipient_map: &HashMap<String, Vec<Recipient>>,
) -> Result<String, Error> {
    wallet.send_begin(
        online.clone(),
        recipient_map.clone(),
        false,
        FEE_RATE,
        MIN_CONFIRMATIONS,
    )
}

pub(crate) fn test_send_btc(
    wallet: &Wallet,
    online: &Online,
    address: &str,
    amount: u64,
) -> String {
    test_send_btc_result(wallet, online, address, amount).unwrap()
}

pub(crate) fn test_send_btc_result(
    wallet: &Wallet,
    online: &Online,
    address: &str,
    amount: u64,
) -> Result<String, Error> {
    wallet.send_btc(online.clone(), address.to_string(), amount, FEE_RATE)
}
