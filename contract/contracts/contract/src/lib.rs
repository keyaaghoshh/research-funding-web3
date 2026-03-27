#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

#[contract]
pub struct ResearchFunding;

#[contractimpl]
impl ResearchFunding {

    // Create a research project with required funding
    pub fn create_project(env: Env, project_id: Symbol, owner: Address, goal: i128) {
        owner.require_auth();

        let mut projects: Map<Symbol, (Address, i128, i128)> =
            env.storage().instance().get(&symbol_short!("PROJECTS")).unwrap_or(Map::new(&env));

        // (owner, goal, raised)
        projects.set(project_id.clone(), (owner, goal, 0));

        env.storage().instance().set(&symbol_short!("PROJECTS"), &projects);
    }

    // Fund a project
    pub fn fund_project(env: Env, project_id: Symbol, amount: i128) {
        let mut projects: Map<Symbol, (Address, i128, i128)> =
            env.storage().instance().get(&symbol_short!("PROJECTS")).unwrap();

        let (owner, goal, raised) = projects.get(project_id.clone()).unwrap();

        let new_amount = raised + amount;

        projects.set(project_id, (owner, goal, new_amount));

        env.storage().instance().set(&symbol_short!("PROJECTS"), &projects);
    }

    // Get project details
    pub fn get_project(env: Env, project_id: Symbol) -> (Address, i128, i128) {
        let projects: Map<Symbol, (Address, i128, i128)> =
            env.storage().instance().get(&symbol_short!("PROJECTS")).unwrap();

        projects.get(project_id).unwrap()
    }
}