// canisters.rs
// use crate::canister::asset_proxy::AssetProxy;
use crate::canister::generated::asset_proxy::AssetProxy;
use crate::canister::provision::Provision;

// use crate::canister::provision::PROVISION_ID;
use dotenv_codegen::dotenv;
use ic_agent::AgentError;
// use crate::state::asset_manager::AssetManager;
use crate::state::auth::AuthService;
use candid::Encode;
use candid::Principal;
use ic_agent::Agent;
use leptos::*;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::rc::Rc;
#[derive(Clone)]
pub struct Canisters {
    pub auth_service: Rc<RefCell<AuthService>>,
    pub agent: Rc<Agent>,
    provision_principal: Principal,
}

impl Canisters {
    pub async fn new(auth_service: Rc<RefCell<AuthService>>) -> Result<Self, String> {
        let agent = {
            let mut auth_service_borrow = auth_service.borrow_mut();
            auth_service_borrow.get_agent().await?
        };
        let PROVISION_ID: Principal =
            Principal::from_text("6zfvq-kiaaa-aaaab-qacra-cai").expect(".");

        Ok(Self {
            auth_service,
            agent,
            provision_principal: PROVISION_ID,
        })
    }

    pub async fn provision_canister(&self) -> Provision<'_> {
        let agent_ref: &Agent = &self.agent;
        Provision(self.provision_principal, agent_ref)
    }
    //
    pub async fn asset_proxy_canister(&self, canister_id: Principal) -> AssetProxy<'_> {
        let agent_ref: &Agent = &self.agent;
        AssetProxy(canister_id, agent_ref)
    }

    pub async fn store_asset<T: candid::CandidType>(
        &self,
        asset_canister_id: Principal,
        upload_arguments: T,
    ) -> Result<(), AgentError> {
        self.agent
            .update(&asset_canister_id, "store")
            .with_arg(Encode!(&upload_arguments)?)
            .call_and_wait()
            .await
            .map(|_| ())
    }
    //
    // pub fn asset_manager(&self) -> AssetManager<'_> {
    //     dotenv::dotenv().ok();
    //     let asset_canister_id = Principal::from_text(dotenv!("ASSET_CANISTER_ID")).unwrap();
    //     let asset_proxy_canister_id =
    //         Principal::from_text(dotenv!("ASSET_PROXY_CANISTER_ID")).unwrap();
    //
    //     AssetManager::new(asset_proxy_canister_id, &self.agent)
    // }
}

impl PartialEq for Canisters {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.agent, &other.agent)
            && self.provision_principal == other.provision_principal
    }
}
