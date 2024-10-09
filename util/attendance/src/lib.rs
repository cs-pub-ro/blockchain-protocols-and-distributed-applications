#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc::{
    codec,
    proxy_imports::{NestedDecode, NestedEncode},
};
use multiversx_sc::{
    derive::type_abi,
    proxy_imports::{TopDecode, TopEncode},
};

#[type_abi]
#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode)]
pub struct StudentStruct<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub name: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait Attendance {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(registerStudent)]
    fn register_student(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            self.students().contains(&caller),
            "Student is already registered"
        );

        self.students().insert(caller);
    }

    #[endpoint(registerAttendance)]
    fn register_attendance(&self) {
        let caller = self.blockchain().get_caller();
        self.require_caller_registered(&caller);

        self.attendance(&caller)
            .update(|attendance| *attendance += BigUint::from(1u64));
    }

    fn require_caller_registered(&self, caller: &ManagedAddress) {
        require!(
            self.students().contains(caller),
            "Caller is not registered inside the students list"
        );
    }

    #[view(getStudents)]
    #[storage_mapper("students")]
    fn students(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getAttendance)]
    #[storage_mapper("attendance")]
    fn attendance(&self, student_address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
