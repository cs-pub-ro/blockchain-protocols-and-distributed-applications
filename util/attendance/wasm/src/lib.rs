// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           11
// Async Callback (empty):               1
// Total number of exported functions:  14

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    attendance
    (
        init => init
        upgrade => upgrade
        registerStudent => register_student
        registerAttendance => register_attendance
        registerSecretKey => register_secret_key
        insertAdmin => insert_admin
        getStudents => students
        getAttendance => attendance
        getSecretKey => secret_key
        isAdmin => is_admin
        addAdmin => add_admin
        removeAdmin => remove_admin
        getAdmins => admins
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
