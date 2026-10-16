use soroban_sdk::{contractevent, Address, Env};

#[contractevent(topics = ["vote"], data_format = "vec")]
struct VoteEvent {
    #[topic]
    voter: Address,
    #[topic]
    option: u32,
    shares: i128,
}

pub fn vote(e: &Env, voter: Address, option: u32, shares: i128) {
    VoteEvent {
        voter,
        option,
        shares,
    }
    .publish(e);
}

#[contractevent(topics = ["vote_changed"], data_format = "vec")]
struct VoteChangedEvent {
    #[topic]
    voter: Address,
    #[topic]
    previous_option: u32,
    #[topic]
    option: u32,
    shares: i128,
}

pub fn vote_changed(e: &Env, voter: Address, previous_option: u32, option: u32, shares: i128) {
    VoteChangedEvent {
        voter,
        previous_option,
        option,
        shares,
    }
    .publish(e);
}

#[cfg(test)]
pub(crate) fn vote_changed_xdr(
    e: &Env,
    contract: &Address,
    voter: Address,
    previous_option: u32,
    option: u32,
    shares: i128,
) -> soroban_sdk::xdr::ContractEvent {
    use soroban_sdk::Event;

    VoteChangedEvent {
        voter,
        previous_option,
        option,
        shares,
    }
    .to_xdr(e, contract)
}
