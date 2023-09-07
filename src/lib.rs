#[cfg(test)]
mod tests {
    use fuels::{
        accounts::predicate::Predicate,
        programs::script_calls::ScriptCallHandler,
        types::{
            output::Output,
            transaction_builders::{TransactionBuilder},
        },
    };

    
    use fuels::prelude::*;
    use itertools::chain;

    #[tokio::test]
    async fn test_name() -> Result<()> {
        let mut wallet = WalletUnlocked::new_random(None);

        let mut predicate_1 = Predicate::load_from("./a_predicate/out/debug/a_predicate.bin")?;

        let asset_id = AssetId::new([1; 32]);

        let coins_wallet = setup_single_asset_coins(wallet.address(), BASE_ASSET_ID, 2, 10000);
        let coins = setup_single_asset_coins(predicate_1.address(), asset_id, 1, 5);
        let coins = chain!(coins_wallet, coins).collect();

        let (provider, _) = setup_test_provider(coins, vec![], None, None).await;
        wallet.set_provider(provider.clone());
        predicate_1.set_provider(provider.clone());

        assert!(asset_id != BASE_ASSET_ID);
        let inputs = predicate_1.get_asset_inputs_for_amount(asset_id, 5).await?;
        let outputs = vec![Output::coin(wallet.address().into(), 5, asset_id)];

        ScriptCallHandler::<_, ()>::new(
            vec![],
            Default::default(),
            wallet,
            provider,
            LogDecoder::default(),
        )
        .with_inputs(inputs)
        .with_outputs(outputs)
        .tx_params(TxParameters::default().with_gas_price(1))
        .call()
        .await?;

        Ok(())
    }
}
