// Copyright (c) Microsoft. All rights reserved.

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OwnerReference {
    api_version: Option<String>,
    block_owner_deletion: Option<bool>,
    controller: Option<bool>,
    kind: Option<String>,
    name: Option<String>,
    uid: Option<String>,
}

impl OwnerReference {
    pub fn new() -> OwnerReference {
        OwnerReference {
            api_version: None,
            block_owner_deletion: None,
            controller: None,
            kind: None,
            name: None,
            uid: None,
        }
    }

    pub fn block_owner_deletion(&self) -> Option<bool> {
        self.block_owner_deletion.clone()
    }

    pub fn with_block_owner_deletion(mut self, block_owner_deletion: bool) -> Self {
        self.block_owner_deletion = Some(block_owner_deletion);
        self
    }

    pub fn controller(&self) -> Option<bool> {
        self.controller.clone()
    }

    pub fn with_controller(mut self, controller: bool) -> Self {
        self.controller = Some(controller);
        self
    }

    pub fn api_version(&self) -> Option<&str> {
        self.api_version.as_ref().map(String::as_str)
    }

    pub fn with_api_version(mut self, api_version: String) -> Self {
        self.api_version = Some(api_version);
        self
    }

    pub fn kind(&self) -> Option<&str> {
        self.kind.as_ref().map(String::as_str)
    }

    pub fn with_kind(mut self, kind: String) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn uid(&self) -> Option<&str> {
        self.uid.as_ref().map(String::as_str)
    }

    pub fn with_uid(mut self, uid: String) -> Self {
        self.uid = Some(uid);
        self
    }
}
