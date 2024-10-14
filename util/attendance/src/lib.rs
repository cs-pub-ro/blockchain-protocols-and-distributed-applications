#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait Attendance: multiversx_sc_modules::only_admin::OnlyAdminModule {
    #[init]
    fn init(&self, admins: MultiValueEncoded<ManagedAddress>) {
        self.admins().extend(admins);
    }

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
            .update(|attendance| *attendance += 1);
    }

    #[only_admin]
    #[endpoint(registerSecretKey)]
    fn register_secret_key(&self, secret_key: ManagedBuffer) {
        let current_epoch = self.blockchain().get_block_epoch();
        let secret_key_mapper = self.secret_key(&secret_key);

        require!(
            secret_key_mapper.is_empty(),
            "This secret key was already registered"
        );

        secret_key_mapper.set(current_epoch);
    }

    #[only_admin]
    #[endpoint(insertAdmin)]
    fn insert_admin(&self, admin: ManagedAddress) {
        require!(
            !self.admins().contains(&admin),
            "Admin has already been registered"
        );

        self.add_admin(admin);
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
    fn attendance(&self, student_address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getSecretKey)]
    #[storage_mapper("secretKey")]
    fn secret_key(&self, secret_key: &ManagedBuffer) -> SingleValueMapper<u64>;
}
