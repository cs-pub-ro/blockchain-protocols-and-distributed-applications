use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/tema-1.mxsc.json", tema_1::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/tema_1.scen.json");
}
