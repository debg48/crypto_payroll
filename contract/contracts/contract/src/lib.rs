#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct Employee {
    pub salary: i128,
}

#[contract]
pub struct CryptoPayroll;

#[contractimpl]
impl CryptoPayroll {

    // Initialize employer
    pub fn init(env: Env, employer: Address) {
        employer.require_auth();
        env.storage().instance().set(&symbol_short!("EMPLOYER"), &employer);
    }

    // Add employee with salary
    pub fn add_employee(env: Env, employee: Address, salary: i128) {
        let employer: Address = env.storage().instance()
            .get(&symbol_short!("EMPLOYER"))
            .unwrap();

        employer.require_auth();

        let mut employees: Map<Address, Employee> =
            env.storage().instance()
                .get(&symbol_short!("EMPLOYEES"))
                .unwrap_or(Map::new(&env));

        employees.set(employee.clone(), Employee { salary });

        env.storage().instance().set(&symbol_short!("EMPLOYEES"), &employees);
    }

    // Pay salary to employee
    pub fn pay_salary(env: Env, employee: Address) {
        let employer: Address = env.storage().instance()
            .get(&symbol_short!("EMPLOYER"))
            .unwrap();

        employer.require_auth();

        let employees: Map<Address, Employee> =
            env.storage().instance()
                .get(&symbol_short!("EMPLOYEES"))
                .unwrap();

        let emp = employees.get(employee.clone()).unwrap();

        // Transfer native token (XLM)
        let token = soroban_sdk::token::Client::new(
            &env,
            &env.current_contract_address(),
        );

        token.transfer(
            &env.current_contract_address(),
            &employee,
            &emp.salary,
        );
    }

    // View salary
    pub fn get_salary(env: Env, employee: Address) -> i128 {
        let employees: Map<Address, Employee> =
            env.storage().instance()
                .get(&symbol_short!("EMPLOYEES"))
                .unwrap();

        let emp = employees.get(employee).unwrap();
        emp.salary
    }
}