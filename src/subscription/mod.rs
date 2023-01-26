use cosmwasm_std::Addr;

pub struct Subscription {
    pub admin: Addr,
    pub lp: Addr,
    pub commitment_denom: String,
    pub initial_commitment: Option<u64>,
}

impl Subscription {
    pub fn new(
        admin: Addr,
        lp: Addr,
        commitment_denom: String,
        initial_commitment: Option<u64>,
    ) -> Self {
        Subscription {
            admin,
            lp,
            commitment_denom,
            initial_commitment,
        }
    }
}
