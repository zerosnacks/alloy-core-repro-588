//! This example demonstrates how to deploy a contract using Alloy.

use alloy::{
    network::EthereumSigner,
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::client::RpcClient,
    signers::wallet::LocalWallet,
    sol,
};
use eyre::Result;

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    // NOTE: it is currently not possible to add documentation that would satisfy the `missing_docs` lint.
    // #[allow(missing_docs)]
    // solc v0.8.24; solc a.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="608080604052346100155760d2908161001a8239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c9081633fb5c1cb1460865781638381f58a14606f575063d09de08a146039575f80fd5b34606b575f366003190112606b575f545f1981146057576001015f55005b634e487b7160e01b5f52601160045260245ffd5b5f80fd5b34606b575f366003190112606b576020905f548152f35b34606b576020366003190112606b576004355f5500fea2646970667358221220bdecd3c1dd631eb40587cafcd6e8297479db76db6a328e18ad1ea5b340852e3864736f6c63430008180033")]
    contract Counter {
        uint256 public number;

        function setNumber(uint256 newNumber) public {
            number = newNumber;
        }

        function increment() public {
            number++;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().try_spawn()?;

    // Set up wallet
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // Create a provider with a signer.
    let http = anvil.endpoint().parse()?;
    let provider = ProviderBuilder::new()
        .signer(EthereumSigner::from(wallet))
        .on_client(RpcClient::new_http(http));

    println!("Anvil running at `{}`", anvil.endpoint());

    // Get the base fee for the block.
    let base_fee = provider.get_gas_price().await?;

    // Deploy the contract.
    let contract_builder = Counter::deploy_builder(&provider);
    let estimate = contract_builder.estimate_gas().await?;
    let contract_address = contract_builder
        .gas(estimate)
        .gas_price(base_fee)
        .nonce(0)
        .deploy()
        .await?;

    println!("Deployed contract at address: {:?}", contract_address);

    Ok(())
}
