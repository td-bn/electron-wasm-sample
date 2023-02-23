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
    //     blockchain: RpcBlockchain,
}

#[wasm_bindgen]
impl Wollet {
    #[wasm_bindgen(constructor)]
    pub fn new(mnemonic: String) -> Wollet {
        let (r, c) = get_descriptor(mnemonic);

        // Get deterministic wallet name from descriptor
        //         let wallet_name =
        //             wallet_name_from_descriptor(&r, Some(&c), Network::Regtest, &Secp256k1::new()).unwrap();

        let wallet =
            wallet::Wallet::new(&r, Some(&c), Network::Regtest, MemoryDatabase::new()).unwrap();

        // Get rpc config
        //       let rpc_config = rpc_config(&wallet_name);
        // Create a blockchain backend and initialize a wallet
        //       let blockchain = RpcBlockchain::from_config(&rpc_config).unwrap();

        Wollet { wallet }
    }
}

// type Wallet = wallet::Wallet<MemoryDatabase>;

// pub fn sync_wallet(wallet: Wallet, blockchain: RpcBlockchain) {
//     wallet
//         .sync(&blockchain, SyncOptions::default())
//         .expect("Failed to sync wallet");
// }

// pub fn send_transaction(wallet: Wallet, blockchain: RpcBlockchain, address: String, amount: u64) {
//     let to = Address::from_str(&address).unwrap();
//
//     let mut tx_builder = wallet.build_tx();
//     tx_builder.set_recipients(vec![(to.script_pubkey(), amount)]);
//     let (mut psbt, _) = tx_builder.finish().unwrap();
//
//     let signopts = SignOptions {
//         assume_height: None,
//         ..Default::default()
//     };
//     wallet.sign(&mut psbt, signopts).unwrap();
//
//     let tx = psbt.extract_tx();
//     blockchain.broadcast(&tx).unwrap();
// }

// fn start() {
//     let (wallet, wallet_name) = create_wallet();
//
//     // Get rpc config
//     let rpc_config = rpc_config(&wallet_name);
//
//     // Create a blockchain backend and initialize a wallet
//     let blockchain = RpcBlockchain::from_config(&rpc_config).unwrap();
//
//     let address = wallet.get_address(AddressIndex::New).unwrap().address;
//
//     // Create a RPC client
//     let core_rpc = start_rpc();
//     let core_address = core_rpc.get_new_address(None, None).unwrap();
//
//     // Send to BDK wallet
//     core_rpc
//         .send_to_address(
//             &address,
//             Amount::from_btc(10.00).unwrap(),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//         )
//         .unwrap();
//     core_rpc.generate_to_address(1, &core_address).unwrap();
//
//     // Sync wallet
//     wallet
//         .sync(&blockchain, SyncOptions::default())
//         .expect("Failed to sync wallet");
//
//     // Send to core_address
//     let mut tx_builder = wallet.build_tx();
//     tx_builder.set_recipients(vec![(core_address.script_pubkey(), 100000000)]);
//     let (mut psbt, _) = tx_builder.finish().unwrap();
//
//     let signopts = SignOptions {
//         assume_height: None,
//         ..Default::default()
//     };
//     wallet.sign(&mut psbt, signopts).unwrap();
//
//     let tx = psbt.extract_tx();
//     blockchain.broadcast(&tx).unwrap();
//     core_rpc.generate_to_address(1, &core_address).unwrap();
//
//     // Sync wallet
//     wallet
//         .sync(&blockchain, SyncOptions::default())
//         .expect("Failed to sync wallet");
//
//     // Fetch and display wallet balances
//     let core_balance = core_rpc.get_balance(None, None).unwrap();
//     let bdk_balance = wallet.get_balance().unwrap();
//     println!("core wallet balance: {:#?}", core_balance);
//     println!("BDK wallet balance: {:#?}", bdk_balance);
// }

// Get Rpc Config struct
// fn rpc_config(wallet_name: &str) -> RpcConfig {
//     let auth = Auth::UserPass {
//         username: "admin1".to_string(),
//         password: "123".to_string(),
//     };
//     let rpc_url = "http://127.0.0.1:18443".to_string();
//     RpcConfig {
//         url: rpc_url,
//         auth,
//         network: Network::Regtest,
//         wallet_name: wallet_name.to_string(),
//         sync_params: None,
//     }
// }

// Connects to RPC, returns a client handle that can be used to query
// fn start_rpc() -> Client {
//     let rpc_auth = rpc_auth::UserPass("admin1".to_string(), "123".to_string());
//     Client::new("http://localhost:18443/wallet/", rpc_auth).unwrap()
// }

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
