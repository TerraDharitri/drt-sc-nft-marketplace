numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait CommonUtilFunctions: numbat_wasm_modules::pause::PauseModule {
    fn get_nft_info(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> DcdtTokenData<Self::Api> {
        self.blockchain().get_dcdt_token_data(
            &self.blockchain().get_sc_address(),
            nft_type,
            nft_nonce,
        )
    }
}
