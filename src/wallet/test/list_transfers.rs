use super::*;
use serial_test::parallel;

#[test]
#[parallel]
fn success() {
    initialize();

    let amount: u64 = 66;

    // wallets
    let (wallet, online) = get_funded_wallet!();
    let (rcv_wallet, rcv_online) = get_funded_wallet!();

    // issue NIA asset
    let asset = test_issue_asset_nia(&wallet, &online, None);

    // single transfer (issuance)
    let bak_info_before = wallet.database.get_backup_info().unwrap().unwrap();
    let transfer_list = test_list_transfers(&wallet, Some(&asset.asset_id));
    let bak_info_after = wallet.database.get_backup_info().unwrap().unwrap();
    assert_eq!(
        bak_info_after.last_operation_timestamp,
        bak_info_before.last_operation_timestamp
    );
    assert_eq!(transfer_list.len(), 1);
    let transfer = transfer_list.first().unwrap();
    assert_eq!(transfer.amount, AMOUNT);
    assert_eq!(transfer.status, TransferStatus::Settled);

    // new wallet
    let (wallet, online) = get_funded_wallet!();

    // issue CFA asset
    let asset = test_issue_asset_cfa(&wallet, &online, None, None);

    // single transfer (issuance)
    let transfer_list = test_list_transfers(&wallet, Some(&asset.asset_id));
    assert_eq!(transfer_list.len(), 1);
    let transfer = transfer_list.first().unwrap();
    assert_eq!(transfer.amount, AMOUNT);
    assert_eq!(transfer.status, TransferStatus::Settled);

    // send
    let receive_data_1 = test_blind_receive(&rcv_wallet);
    let receive_data_2 = test_witness_receive(&rcv_wallet);
    let recipient_map = HashMap::from([(
        asset.asset_id.clone(),
        vec![
            Recipient {
                amount,
                recipient_data: RecipientData::BlindedUTXO(
                    SecretSeal::from_str(&receive_data_1.recipient_id).unwrap(),
                ),
                transport_endpoints: TRANSPORT_ENDPOINTS.clone(),
            },
            Recipient {
                amount: amount * 2,
                recipient_data: RecipientData::WitnessData {
                    script_buf: ScriptBuf::from_hex(&receive_data_2.recipient_id).unwrap(),
                    amount_sat: 1000,
                    blinding: None,
                },
                transport_endpoints: TRANSPORT_ENDPOINTS.clone(),
            },
        ],
    )]);
    let txid = test_send(&wallet, &online, &recipient_map);
    assert!(!txid.is_empty());

    // multiple transfers (sender)
    let transfer_list = test_list_transfers(&wallet, Some(&asset.asset_id));
    assert_eq!(transfer_list.len(), 3);
    let transfer_send_1 = transfer_list
        .iter()
        .find(|t| {
            t.kind == TransferKind::Send
                && t.recipient_id == Some(receive_data_1.recipient_id.clone())
        })
        .unwrap();
    let transfer_send_2 = transfer_list
        .iter()
        .find(|t| {
            t.kind == TransferKind::Send
                && t.recipient_id == Some(receive_data_2.recipient_id.clone())
        })
        .unwrap();
    assert_eq!(transfer_send_1.amount, amount);
    assert_eq!(transfer_send_2.amount, amount * 2);
    assert_eq!(transfer_send_1.status, TransferStatus::WaitingCounterparty);
    assert_eq!(transfer_send_2.status, TransferStatus::WaitingCounterparty);
    assert_eq!(transfer_send_1.txid, Some(txid.clone()));
    assert_eq!(transfer_send_2.txid, Some(txid.clone()));

    // refresh once, so the asset appears on the receiver side
    test_refresh_all(&rcv_wallet, &rcv_online);
    test_refresh_all(&wallet, &online);

    // multiple transfers (receiver)
    let transfer_list_rcv = test_list_transfers(&rcv_wallet, Some(&asset.asset_id));
    assert_eq!(transfer_list_rcv.len(), 2);
    let transfer_recv_blind = transfer_list_rcv
        .iter()
        .find(|t| t.kind == TransferKind::ReceiveBlind)
        .unwrap();
    let transfer_recv_witness = transfer_list_rcv
        .iter()
        .find(|t| t.kind == TransferKind::ReceiveWitness)
        .unwrap();
    assert_eq!(transfer_recv_blind.amount, amount);
    assert_eq!(transfer_recv_witness.amount, amount * 2);
    assert_eq!(
        transfer_recv_blind.status,
        TransferStatus::WaitingConfirmations
    );
    assert_eq!(
        transfer_recv_witness.status,
        TransferStatus::WaitingConfirmations
    );
    assert_eq!(transfer_recv_blind.txid, Some(txid.clone()));
    assert_eq!(transfer_recv_witness.txid, Some(txid.clone()));

    // refresh a second time to settle the transfers
    mine(false);
    test_refresh_all(&rcv_wallet, &rcv_online);
    test_refresh_all(&wallet, &online);

    // check all transfers are now in status Settled
    let transfer_list = test_list_transfers(&wallet, Some(&asset.asset_id));
    let transfer_list_rcv = test_list_transfers(&rcv_wallet, Some(&asset.asset_id));
    assert!(transfer_list
        .iter()
        .all(|t| t.status == TransferStatus::Settled));
    assert!(transfer_list_rcv
        .iter()
        .all(|t| t.status == TransferStatus::Settled));
}

#[test]
#[parallel]
fn fail() {
    let wallet = get_test_wallet(false, None);

    // asset not found
    let result = test_list_transfers_result(&wallet, Some("rgb1inexistent"));
    assert!(matches!(result, Err(Error::AssetNotFound { asset_id: _ })));
}
