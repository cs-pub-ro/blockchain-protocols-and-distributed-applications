use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/attendance.mxsc.json", attendance::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/attendance.scen.json");
}
