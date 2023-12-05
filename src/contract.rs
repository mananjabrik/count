use crate::msg::{AdminsListResp, GreetResp, InstantiateMessage, QueryMsg};
use crate::state::ADMINS;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    dep: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMessage,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| dep.api.addr_validate(&addr))
        .collect();

    ADMINS.save(dep.storage, &admins?)?;

    Ok(Response::new())
}

pub fn query(dep: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_json_binary(&query::greet()?),
        AdminsList {} => to_json_binary(&query::admin_lists(dep)?),
    }
}
mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp: GreetResp = GreetResp {
            message: "Hello world".to_owned(),
        };

        Ok(resp)
    }

    pub fn admin_lists(deps: Deps) -> StdResult<AdminsListResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp = AdminsListResp { admins };
        Ok(resp)
    }
}

#[allow(dead_code)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn great_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMessage {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                },
                &[],
                "Contract 2",
                None,
            )
            .unwrap();

        // let resp: GreetResp = app
        //     .wrap()
        //     .query_wasm_smart(addr, &QueryMsg::Greet {})
        //     .unwrap();

        // assert_eq!(
        //     resp,
        //     GreetResp {
        //         message: "Hello world".to_owned()
        //     }
        // );

        let resp_admin: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();
        assert_eq!(
            resp_admin,
            AdminsListResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")]
            }
        );
    }
}
