// Copyright 2018 dgc.network
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate dgc_contract_sdk;

use dgc_contract_sdk::{WasmPtr, WasmPtrList, execute_smart_permission_entrypoint, WasmSdkError, Request};

fn has_permission(request: Request) -> Result<bool, WasmSdkError> {
    Ok(request
        .get_roles()
        .iter()
        .any(|x| x == "admin"))
}

#[no_mangle]
pub unsafe fn entrypoint(roles: WasmPtrList, org_id: WasmPtr, payload: WasmPtr, public_key: WasmPtr) -> i32 {
    execute_smart_permission_entrypoint(roles, org_id, public_key, payload, has_permission)
}
