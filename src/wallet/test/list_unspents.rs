use super::*;
use serial_test::parallel;

#[test]
#[parallel]
fn success() {
    initialize();

    let amount: u64 = 66;

    // wallets
    let (wallet, online) = get_empty_wallet!();
    let (rcv_wallet, rcv_online) = get_funded_wallet!();

    // no unspents
    let bak_info_before = wallet.database.get_backup_info().unwrap();
    assert!(bak_info_before.is_none());
    let unspent_list_settled = test_list_unspents(&wallet, None, true);
    let bak_info_after = wallet.database.get_backup_info().unwrap();
    assert!(bak_info_after.is_none());
    assert_eq!(unspent_list_settled.len(), 0);
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    assert_eq!(unspent_list_all.len(), 0);

    fund_wallet(test_get_address(&wallet));
    mine(false);

    // one unspent, no RGB allocations
    let unspent_list_settled = test_list_unspents(&wallet, Some(&online), true);
    assert_eq!(unspent_list_settled.len(), 1);
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    assert_eq!(unspent_list_all.len(), 1);
    assert!(unspent_list_all.iter().all(|u| !u.utxo.colorable));

    test_create_utxos_default(&wallet, &online);

    // multiple unspents, one settled RGB allocation
    let asset = test_issue_asset_nia(&wallet, &online, None);
    let unspent_list_settled = test_list_unspents(&wallet, None, true);
    assert_eq!(unspent_list_settled.len(), UTXO_NUM as usize + 1);
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    assert_eq!(unspent_list_all.len(), UTXO_NUM as usize + 1);
    assert_eq!(
        unspent_list_all.iter().filter(|u| u.utxo.colorable).count(),
        UTXO_NUM as usize
    );
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| !u.utxo.colorable)
            .count(),
        1
    );
    let mut settled_allocations = vec![];
    unspent_list_settled
        .iter()
        .for_each(|u| settled_allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(settled_allocations.len(), 1);
    assert!(settled_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT && a.settled));

    // multiple unspents, one failed blind, not listed
    let receive_data_fail = test_blind_receive(&rcv_wallet);
    test_fail_transfers_single(
        &rcv_wallet,
        &rcv_online,
        receive_data_fail.batch_transfer_idx,
    );
    show_unspent_colorings(&rcv_wallet, "after blind fail");
    let unspent_list_all = test_list_unspents(&rcv_wallet, None, false);
    let mut allocations = vec![];
    unspent_list_all
        .iter()
        .for_each(|u| allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(allocations.len(), 0);
    // one failed send, not listed
    let receive_data = test_blind_receive(&rcv_wallet);
    let recipient_map = HashMap::from([(
        asset.asset_id.clone(),
        vec![Recipient {
            amount,
            recipient_data: RecipientData::BlindedUTXO(
                SecretSeal::from_str(&receive_data.recipient_id).unwrap(),
            ),
            transport_endpoints: TRANSPORT_ENDPOINTS.clone(),
        }],
    )]);
    let send_result = test_send_result(&wallet, &online, &recipient_map).unwrap();
    let txid = send_result.txid;
    assert!(!txid.is_empty());
    test_fail_transfers_single(&wallet, &online, send_result.batch_transfer_idx);
    show_unspent_colorings(&wallet, "after send fail");
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| u.utxo.colorable && u.utxo.exists)
            .count(),
        UTXO_NUM as usize
    );
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| u.utxo.colorable && !u.utxo.exists)
            .count(),
        1
    );
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| !u.utxo.colorable)
            .count(),
        1
    );
    let mut allocations = vec![];
    unspent_list_all
        .iter()
        .for_each(|u| allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(allocations.len(), 1);
    assert!(allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT && a.settled));

    // new wallets
    let (wallet, online) = get_funded_wallet!();
    let (rcv_wallet, rcv_online) = get_funded_wallet!();

    // issue + send some asset
    let asset = test_issue_asset_nia(&wallet, &online, None);
    let receive_data = test_blind_receive(&rcv_wallet);
    let recipient_map = HashMap::from([(
        asset.asset_id.clone(),
        vec![Recipient {
            amount,
            recipient_data: RecipientData::BlindedUTXO(
                SecretSeal::from_str(&receive_data.recipient_id).unwrap(),
            ),
            transport_endpoints: TRANSPORT_ENDPOINTS.clone(),
        }],
    )]);
    let txid = test_send(&wallet, &online, &recipient_map);
    assert!(!txid.is_empty());
    show_unspent_colorings(&rcv_wallet, "receiver after send - WaitingCounterparty");
    show_unspent_colorings(&wallet, "sender after send - WaitingCounterparty");
    // check receiver lists no settled allocations
    let rcv_unspent_list = test_list_unspents(&rcv_wallet, None, true);
    assert!(!rcv_unspent_list
        .iter()
        .any(|u| !u.rgb_allocations.is_empty()));
    // check receiver lists one pending blind
    let rcv_unspent_list_all = test_list_unspents(&rcv_wallet, None, false);
    let mut allocations = vec![];
    rcv_unspent_list_all
        .iter()
        .for_each(|u| allocations.extend(u.rgb_allocations.clone()));
    assert!(!allocations.iter().any(|a| a.settled));
    assert_eq!(allocations.iter().filter(|a| !a.settled).count(), 1);
    // check sender lists one settled issue
    let unspent_list_settled = test_list_unspents(&wallet, None, true);
    let mut settled_allocations = vec![];
    unspent_list_settled
        .iter()
        .for_each(|u| settled_allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(settled_allocations.len(), 1);
    assert!(settled_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT && a.settled));
    // check sender lists one pending change (exists = false) + 1 settled issue
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| u.utxo.colorable && u.utxo.exists)
            .count(),
        UTXO_NUM as usize
    );
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| u.utxo.colorable && !u.utxo.exists)
            .count(),
        1
    );
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| !u.utxo.colorable)
            .count(),
        1
    );
    let mut pending_allocations = vec![];
    let mut settled_allocations = vec![];
    unspent_list_all
        .iter()
        .for_each(|u| pending_allocations.extend(u.rgb_allocations.iter().filter(|a| !a.settled)));
    assert_eq!(pending_allocations.len(), 1);
    assert!(pending_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT - amount));
    unspent_list_all
        .iter()
        .for_each(|u| settled_allocations.extend(u.rgb_allocations.iter().filter(|a| a.settled)));
    assert_eq!(settled_allocations.len(), 1);
    assert!(settled_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT));

    stop_mining();

    // transfer progresses to status WaitingConfirmations
    test_refresh_all(&rcv_wallet, &rcv_online);
    test_refresh_asset(&wallet, &online, &asset.asset_id);
    show_unspent_colorings(&rcv_wallet, "receiver after send - WaitingConfirmations");
    show_unspent_colorings(&wallet, "sender after send - WaitingConfirmations");
    // check receiver lists no settled allocations
    let rcv_unspent_list = test_list_unspents(&rcv_wallet, None, true);
    assert!(!rcv_unspent_list
        .iter()
        .any(|u| !u.rgb_allocations.is_empty()));
    // check receiver lists one pending blind
    let rcv_unspent_list_all = test_list_unspents(&rcv_wallet, None, false);
    let mut allocations = vec![];
    rcv_unspent_list_all
        .iter()
        .for_each(|u| allocations.extend(u.rgb_allocations.clone()));
    assert!(!allocations.iter().any(|a| a.settled));
    assert_eq!(allocations.iter().filter(|a| !a.settled).count(), 1);
    assert!(allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == amount));
    // check sender lists one settled issue
    let unspent_list_settled = test_list_unspents(&wallet, None, true);
    let mut settled_allocations = vec![];
    unspent_list_settled
        .iter()
        .for_each(|u| settled_allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(settled_allocations.len(), 1);
    assert!(settled_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT && a.settled));
    // check sender lists one pending change (exists = true)
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    let mut pending_allocations = vec![];
    unspent_list_all
        .iter()
        .for_each(|u| pending_allocations.extend(u.rgb_allocations.iter().filter(|a| !a.settled)));
    assert_eq!(pending_allocations.len(), 1);
    assert!(pending_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == AMOUNT - amount));
    assert_eq!(
        unspent_list_all
            .iter()
            .filter(|u| u.utxo.colorable && !u.utxo.exists)
            .count(),
        0
    );

    // transfer progresses to status Settled
    mine(true);
    rcv_wallet.refresh(rcv_online, None, vec![]).unwrap();
    test_refresh_asset(&wallet, &online, &asset.asset_id);
    show_unspent_colorings(&rcv_wallet, "receiver after send - Settled");
    show_unspent_colorings(&wallet, "sender after send - Settled");
    // check receiver lists one settled allocation
    let rcv_unspent_list = test_list_unspents(&rcv_wallet, None, true);
    let mut settled_allocations = vec![];
    rcv_unspent_list
        .iter()
        .for_each(|u| settled_allocations.extend(u.rgb_allocations.clone()));
    assert!(settled_allocations.iter().all(|a| a.settled));
    assert_eq!(settled_allocations.len(), 1);
    assert!(settled_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone()) && a.amount == amount));
    // check receiver lists no pending allocations
    let rcv_unspent_list_all = test_list_unspents(&rcv_wallet, None, false);
    let mut allocations = vec![];
    rcv_unspent_list_all
        .iter()
        .for_each(|u| allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(allocations, settled_allocations);
    // check sender lists one settled change
    let unspent_list_settled = test_list_unspents(&wallet, None, true);
    let mut settled_allocations = vec![];
    unspent_list_settled
        .iter()
        .for_each(|u| settled_allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(settled_allocations.len(), 1);
    assert!(settled_allocations
        .iter()
        .all(|a| a.asset_id == Some(asset.asset_id.clone())
            && a.amount == AMOUNT - amount
            && a.settled));
    // check sender lists no pending allocations
    let unspent_list_all = test_list_unspents(&wallet, None, false);
    let mut allocations = vec![];
    unspent_list_all
        .iter()
        .for_each(|u| allocations.extend(u.rgb_allocations.clone()));
    assert_eq!(allocations, settled_allocations);
}
