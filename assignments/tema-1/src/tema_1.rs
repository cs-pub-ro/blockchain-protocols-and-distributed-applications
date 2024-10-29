#![no_std]

use multiversx_sc::imports::*;
use multiversx_sc_modules::default_issue_callbacks;

mod card_properties;
use card_properties::CardProperties;

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000; // 100%

#[multiversx_sc::contract]
pub trait Tema1: default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[upgrade]
    fn upgrade(&self) {}

    // Endpoints

    #[only_owner]
    #[endpoint(issueNft)]
    #[payable("EGLD")]
    fn issue_nft(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer
    ) {
        require!(self.token_id().is_empty(), "NFT already issued");

        let issue_cost = self.call_value().egld_value();

        self.token_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            issue_cost.clone_value(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    #[only_owner]
    #[endpoint(createNftWithAttributes)]
    fn create_nft_with_attributes(
        &self,
        name: ManagedBuffer,
        class: u8,
        power: u8,
        rarity: u8,
    ) -> u64 {
        require!(!self.token_id().is_empty(), "NFT not issued");

        let nft_token_id = self.token_id().get_token_id();
        let card_details = match CardProperties::new(class, power, rarity) {
            Ok(details) => details,
            Err(_) => sc_panic!("Error creating card details"),
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let Err(err) = card_details.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();

        let nft_nonce = self.send().esdt_nft_create(
            &nft_token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &BigUint::from(ROYALTIES_MAX),
            attributes_hash,
            &card_details,
            &ManagedVec::new(),
        );

        nft_nonce
    }

    #[endpoint(exchangeNft)]
    #[payable("*")]
    fn exchange_nft(
        &self,
        nonce: u64
    ) {
        require!(!self.token_id().is_empty(), "NFT not issued");

        let nft_payment = self.call_value().single_esdt();
        let nft_data = self.token_id().get_all_token_data(nonce);
        let nft_student_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &nft_payment.token_identifier,
            nft_payment.token_nonce,
        );

        require!(nft_student_data.hash == nft_data.hash, "NFT data mismatch");

        self.send_nft_to_caller(nonce);
    }

    // Private helper functions

    fn send_nft_to_caller(
        &self,
        nonce: u64
    ) {
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            self.token_id().get_token_id_ref(),
            nonce,
            &BigUint::from(NFT_AMOUNT),
        );
    }

    // View functions

    #[view(getTokenId)]
    fn get_token_id(&self) -> TokenIdentifier {
        self.token_id().get_token_id()
    }

    #[view(getTokenData)]
    fn get_token_data(&self, token_nonce: u64) -> EsdtTokenData {
        self.token_id().get_all_token_data(token_nonce)
    }

    // Storage mappers

    #[view(tokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> NonFungibleTokenMapper;
}
