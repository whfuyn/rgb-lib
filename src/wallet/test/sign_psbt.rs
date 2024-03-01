use super::*;
use serial_test::parallel;

#[test]
#[parallel]
fn success() {
    initialize();
    let (wallet, online) = get_funded_wallet!();
    let address = test_get_address(&wallet);

    let unsigned_psbt_str = wallet
        .send_btc_begin(online, address, AMOUNT, FEE_RATE)
        .unwrap();

    // no SignOptions
    let signed_psbt = wallet.sign_psbt(unsigned_psbt_str.clone(), None).unwrap();
    assert!(BdkPsbt::from_str(&signed_psbt).is_ok());

    // with SignOptions
    let opts = SignOptions::default();
    let signed_psbt = wallet
        .sign_psbt(unsigned_psbt_str.clone(), Some(opts))
        .unwrap();
    assert!(BdkPsbt::from_str(&signed_psbt).is_ok());
}

#[test]
#[parallel]
fn fail() {
    initialize();
    let (wallet, _online) = get_funded_wallet!();

    let result = wallet.sign_psbt("rgb1invalid".to_string(), None);
    assert!(matches!(result, Err(Error::InvalidPsbt { details: _ })));
}
