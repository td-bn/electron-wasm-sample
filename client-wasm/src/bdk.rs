use bdk::{
    bitcoin::{secp256k1::Secp256k1, util::bip32::DerivationPath, Network},
    database::MemoryDatabase,
    keys::{bip39::Mnemonic, DerivableKey, DescriptorKey, ExtendedKey},
    miniscript::Segwitv0,
    wallet,
};
use wasm_bindgen::prelude::*;

use std::str::FromStr;

#[wasm_bindgen]
pub struct Wollet {
    wallet: wallet::Wallet<MemoryDatabase>,
}

#[wasm_bindgen]
impl Wollet {
    #[wasm_bindgen(constructor)]
    pub fn new(mnemonic: String) -> Wollet {
        let (r, c) = get_descriptor(mnemonic);

        let wallet =
            wallet::Wallet::new(&r, Some(&c), Network::Regtest, MemoryDatabase::new()).unwrap();

        Wollet { wallet }
    }
}

// Generate new descriptor and return using (receive, change) tuple
fn get_descriptor(mnemonic: String) -> (String, String) {
    let secp = Secp256k1::new();

    let password = Some("password".to_string());

    let mnemonic = Mnemonic::from_str(&mnemonic).unwrap();

    let xkey: ExtendedKey = (mnemonic, password).into_extended_key().unwrap();

    let xprv = xkey.into_xprv(bdk::bitcoin::Network::Regtest).unwrap();

    let mut keys = Vec::new();

    for path in ["m/84h/1h/0h/0", "m/84h/1h/0h/1"] {
        let deriv_path = DerivationPath::from_str(path).unwrap();
        let derived_xprv = &xprv.derive_priv(&secp, &deriv_path).unwrap();

        let origin = (xprv.fingerprint(&secp), deriv_path);
        let derived_xprv_desc_key: DescriptorKey<Segwitv0> = derived_xprv
            .into_descriptor_key(Some(origin), DerivationPath::default())
            .unwrap();

        // Wrap the descriptor in a wpkh() string
        if let DescriptorKey::Secret(key, _, _) = derived_xprv_desc_key {
            keys.push(format!("wpkh({})", key.to_string()));
        }
    }
    (keys[0].clone(), keys[1].clone())
}
