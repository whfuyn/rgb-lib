use super::*;
use serial_test::parallel;

#[test]
#[parallel]
fn success() {
    let test_data_dir_str = get_test_data_dir_string();
    let test_data_dir = PathBuf::from(test_data_dir_str.clone());
    fs::create_dir_all(test_data_dir).unwrap();

    // test manual values
    let keys = generate_keys(BitcoinNetwork::Signet);
    let wallet_1 = Wallet::new(WalletData {
        data_dir: test_data_dir_str.clone(),
        bitcoin_network: BitcoinNetwork::Signet,
        database_type: DatabaseType::Sqlite,
        max_allocations_per_utxo: 1,
        pubkey: keys.account_xpub.clone(),
        mnemonic: Some(keys.mnemonic.clone()),
        vanilla_keychain: Some(2),
    })
    .unwrap();

    let wallet_1_data = test_get_wallet_data(&wallet_1);
    assert_eq!(wallet_1_data.data_dir, test_data_dir_str);
    assert_eq!(
        wallet_1.get_wallet_dir().parent().unwrap(),
        fs::canonicalize(wallet_1_data.data_dir).unwrap(),
    );
    assert_eq!(wallet_1_data.bitcoin_network, BitcoinNetwork::Signet);
    assert!(matches!(wallet_1_data.database_type, DatabaseType::Sqlite));
    assert_eq!(wallet_1_data.pubkey, keys.account_xpub);
    assert_eq!(wallet_1_data.max_allocations_per_utxo, 1);
    assert_eq!(wallet_1_data.mnemonic.unwrap(), keys.mnemonic);
    assert_eq!(wallet_1_data.vanilla_keychain.unwrap(), 2);

    // test default values
    let wallet_2 = Wallet::new(WalletData {
        data_dir: test_data_dir_str.clone(),
        bitcoin_network: BitcoinNetwork::Regtest,
        database_type: DatabaseType::Sqlite,
        max_allocations_per_utxo: 5,
        pubkey: keys.account_xpub.clone(),
        mnemonic: None,
        vanilla_keychain: None,
    })
    .unwrap();
    let wallet_2_data = test_get_wallet_data(&wallet_2);
    assert_eq!(wallet_2_data.data_dir, test_data_dir_str);
    assert_eq!(wallet_2_data.bitcoin_network, BitcoinNetwork::Regtest);
    assert!(matches!(wallet_2_data.database_type, DatabaseType::Sqlite));
    assert_eq!(
        wallet_2_data.max_allocations_per_utxo,
        MAX_ALLOCATIONS_PER_UTXO
    );
    assert!(wallet_2_data.mnemonic.is_none());
    assert!(wallet_2_data.vanilla_keychain.is_none());
}
