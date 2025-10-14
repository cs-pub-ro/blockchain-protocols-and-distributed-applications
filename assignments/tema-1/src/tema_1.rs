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
        rarity: u8,
        power: u8,
    ) {
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

        self.send().esdt_nft_create(
            &nft_token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &BigUint::from(ROYALTIES_MAX),
            attributes_hash,
            &card_details,
            &ManagedVec::new(),
        );

        let mut token_data = EsdtTokenData::default();
        token_data.token_type = EsdtTokenType::NonFungible;
        token_data.amount = BigUint::from(NFT_AMOUNT);
        token_data.frozen = false;
        token_data.hash = attributes_hash.clone();
        token_data.name = name;
        token_data.attributes = serialized_attributes;
        token_data.creator = self.blockchain().get_owner_address();
        token_data.royalties = BigUint::from(ROYALTIES_MAX);
        token_data.uris = ManagedVec::new();

        self.nft_supply().push(&token_data);
        self.cards_properties().push(&card_details);
    }

    #[endpoint(getYourNftCardProperties)]
    fn get_your_nft_card_properties(&self) -> CardProperties {
        require!(!self.token_id().is_empty(), "NFT not issued");

        let student_address = self.blockchain().get_caller();
        require!(!self.student_address().contains(&student_address), "Congratulations! You already finished the homework!");

        let mut rand_source = RandomnessSource::new();
        let cards_properties_len = self.cards_properties().len();
        let rand_index = rand_source.next_usize_in_range(1, cards_properties_len + 1);

        let rand_card_properties = self.cards_properties().get(rand_index);
        self.students_cards(student_address).set(&rand_card_properties);

        rand_card_properties
    }

    #[endpoint(exchangeNft)]
    #[payable("*")]
    fn exchange_nft(
        &self,
        nonce: u64
    ) {
        require!(!self.token_id().is_empty(), "NFT not issued");
        require!(nonce > 0, "NFTs can't have a nonce of 0");
        require!(nonce <= self.nft_supply().len() as u64, "NFT not found");

        let student_address = self.blockchain().get_caller();
        require!(!self.student_address().contains(&student_address), "Congratulations! You already finished the homework!");

        let nft_payment = self.call_value().single_esdt();
        let nft_student_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &nft_payment.token_identifier,
            nft_payment.token_nonce,
        );

        let is_empty = self.students_cards(student_address.clone()).is_empty();
        require!(!is_empty, "You need to get your card first");

        let student_card_attributes = self.students_cards(student_address.clone()).get();
        let payment_card_attributes = nft_student_data.try_decode_attributes::<CardProperties>().unwrap();
        require!(payment_card_attributes == student_card_attributes, "Card attributes do not match with those returned by getYourNftCardProperties");

        let nft_data = self.token_id().get_all_token_data(nonce);
        let nft_data_attributes = nft_data.try_decode_attributes::<CardProperties>().unwrap();
        require!(payment_card_attributes == nft_data_attributes, "Card attributes do not match with the one you want to exchange");

        let cards_properties_len = self.cards_properties().len();
        let mut index_to_remove: usize = cards_properties_len + 1;
        for (index, card) in self.cards_properties().iter().enumerate() {
            if card == nft_data_attributes {
                index_to_remove = index;
                break;
            }
        }

        require!(index_to_remove < cards_properties_len, "We didn't find the card you want to exchange");

        self.cards_properties().swap_remove(index_to_remove + 1);
        self.nft_supply().set(nonce as usize, &nft_student_data);
        self.student_address().insert(student_address);
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

    #[view(nftSupply)]
    #[storage_mapper("nftSupply")]
    fn nft_supply(&self) -> VecMapper<EsdtTokenData>;

    #[view(cardsProperties)]
    #[storage_mapper("cardsProperties")]
    fn cards_properties(&self) -> VecMapper<CardProperties>;

    #[view(studentsCards)]
    #[storage_mapper("studentsCards")]
    fn students_cards(&self, student_address: ManagedAddress) -> SingleValueMapper<CardProperties>;

    #[view(studentsAddresses)]
    #[storage_mapper("studentsAddresses")]
    fn student_address(&self) -> UnorderedSetMapper<ManagedAddress>;

}
