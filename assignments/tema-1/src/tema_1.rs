#![no_std]

#[allow(unused_imports)]
use multiversx_sc::{derive_imports::*, imports::*};
use multiversx_sc_modules::default_issue_callbacks;

const NFT_AMOUNT: u32 = 1;

#[multiversx_sc::contract]
pub trait Tema1: default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(issueNft)]
    #[payable("EGLD")]
    fn issue_nft(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer
    ) {
        require!(self.token_id().is_empty(), "NFT already issued");

        let issue_cost = self.call_value().egld_value();

        self.token_id().issue(
            EsdtTokenType::NonFungible,
            issue_cost.clone_value(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    #[endpoint(createNft)]
    fn create_nft(
        &self,
        name: ManagedBuffer,
        attributes: ManagedBuffer
    ) -> EsdtTokenPayment {

        require!(!self.token_id().is_empty(), "NFT not issued");

        self.token_id().nft_create_named(
            BigUint::from(NFT_AMOUNT), 
            &name, 
            &attributes
        )
    }

    #[view(getTokenId)]
    fn get_token_id(&self) -> TokenIdentifier {
        self.token_id().get_token_id()
    }

    #[view(tokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> NonFungibleTokenMapper;
}