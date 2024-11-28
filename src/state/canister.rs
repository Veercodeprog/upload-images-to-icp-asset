use crate::canister::asset_proxy::Asset_Proxy;
use crate::canister::provision::Provision;
use crate::canister::PROVISION_ID;
use crate::utils::ic::AgentWrapper;
use candid::Principal;
use ic_agent::Agent;
use ic_auth_client::AuthClient;
use ic_cdk::api::management_canister::main::canister_status;
use leptos::leptos_dom::logging;
use leptos::{expect_context, use_context, ReadSignal, SignalGet};
use log::logger;

#[derive(Clone)]
pub struct Canisters {
    agent: AgentWrapper,
    provision_principal: Principal,
}

impl Default for Canisters {
    fn default() -> Self {
        Self {
            agent: AgentWrapper::build(),
            provision_principal: PROVISION_ID,
        }
    }
}

impl Canisters {
    pub fn agent() -> Agent {
        AgentWrapper::refresh_agent()
    }

    pub async fn provision_canister(&self) -> Provision<'_> {
        let agent = self.agent.get_agent().await;
        Provision(self.provision_principal, &agent)
    }

    pub async fn asset_proxy_canister<'a>(&self, canister_id: Principal) -> Asset_Proxy<'a> {
        let agent = self.agent.get_agent().await;
        Asset_Proxy(canister_id, agent)
    }
}
