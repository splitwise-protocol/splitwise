#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    pub name: String,
    pub members: Vec<Address>,
}

#[contract]
pub struct SplitwiseContract;

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

#[contractimpl]
impl SplitwiseContract {
    /// Creates a new group with the given name and adds the creator as the first member.
    /// Returns the assigned Group ID.
    pub fn create_group(env: Env, name: String, creator: Address) -> u32 {
        creator.require_auth();

        let mut counter: u32 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        counter += 1;
        
        // Ensure new vector is created
        let mut members = Vec::new(&env);
        members.push_back(creator);

        let group = Group {
            name,
            members,
        };

        env.storage().persistent().set(&counter, &group);
        env.storage().instance().set(&COUNTER_KEY, &counter);

        // Emit an event (optional but good practice)
        env.events()
            .publish((symbol_short!("group"), symbol_short!("created")), counter);

        counter
    }

    /// Allows a user to join an existing group.
    pub fn join_group(env: Env, group_id: u32, member: Address) {
        member.require_auth();

        // Retrieve the group from persistent storage
        let mut group: Group = env
            .storage()
            .persistent()
            .get(&group_id)
            .expect("Group not found");

        group.members.push_back(member.clone());

        // Save the updated group
        env.storage().persistent().set(&group_id, &group);

        env.events()
            .publish((symbol_short!("group"), symbol_short!("joined")), (group_id, member));
    }

    /// Helper to get group details.
    pub fn get_group(env: Env, group_id: u32) -> Group {
        env.storage()
            .persistent()
            .get(&group_id)
            .expect("Group not found")
    }
}

mod test;
