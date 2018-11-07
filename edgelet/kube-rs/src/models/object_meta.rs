// Copyright (c) Microsoft. All rights reserved.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::{Initializer, OwnerReference};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMeta {
    annotations: Option<HashMap<String, String>>,
    cluster_name: Option<String>,
    creation_timestamp: Option<DateTime<Utc>>,
    deletion_grace_period_seconds: Option<i64>,
    deletion_timestamp: Option<DateTime<Utc>>,
    finalizers: Option<Vec<String>>,
    generate_name: Option<String>,
    generation: Option<i64>,
    initializers: Option<Vec<Initializer>>,
    labels: Option<HashMap<String, String>>,
    name: Option<String>,
    namespace: Option<String>,
    owner_references: Option<Vec<OwnerReference>>,
    resource_version: Option<String>,
    self_link: Option<String>,
    uid: Option<String>,
}

impl ObjectMeta {
    pub fn new() -> ObjectMeta {
        ObjectMeta {
            annotations: None,
            cluster_name: None,
            creation_timestamp: None,
            deletion_grace_period_seconds: None,
            deletion_timestamp: None,
            finalizers: None,
            generate_name: None,
            generation: None,
            initializers: None,
            labels: None,
            name: None,
            namespace: None,
            owner_references: None,
            resource_version: None,
            self_link: None,
            uid: None,
        }
    }

    pub fn cluster_name(&self) -> Option<&str> {
        self.cluster_name.as_ref().map(String::as_str)
    }

    pub fn with_cluster_name(mut self, cluster_name: String) -> Self {
        self.cluster_name = Some(cluster_name);
        self
    }

    pub fn generate_name(&self) -> Option<&str> {
        self.generate_name.as_ref().map(String::as_str)
    }

    pub fn with_generate_name(mut self, generate_name: String) -> Self {
        self.generate_name = Some(generate_name);
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(String::as_str)
    }

    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }

    pub fn resource_version(&self) -> Option<&str> {
        self.resource_version.as_ref().map(String::as_str)
    }

    pub fn with_resource_version(mut self, resource_version: String) -> Self {
        self.resource_version = Some(resource_version);
        self
    }

    pub fn self_link(&self) -> Option<&str> {
        self.self_link.as_ref().map(String::as_str)
    }

    pub fn with_self_link(mut self, self_link: String) -> Self {
        self.self_link = Some(self_link);
        self
    }

    pub fn uid(&self) -> Option<&str> {
        self.uid.as_ref().map(String::as_str)
    }

    pub fn with_uid(mut self, uid: String) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn annotations(&self) -> Option<&HashMap<String, String>> {
        self.annotations.as_ref()
    }

    pub fn with_annotations(mut self, annotations: HashMap<String, String>) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn creation_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.creation_timestamp.as_ref()
    }

    pub fn with_creation_timestamp(mut self, creation_timestamp: DateTime<Utc>) -> Self {
        self.creation_timestamp = Some(creation_timestamp);
        self
    }

    pub fn deletion_grace_period_seconds(&self) -> Option<&i64> {
        self.deletion_grace_period_seconds.as_ref()
    }

    pub fn with_deletion_grace_period_seconds(
        mut self,
        deletion_grace_period_seconds: i64,
    ) -> Self {
        self.deletion_grace_period_seconds = Some(deletion_grace_period_seconds);
        self
    }

    pub fn deletion_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.deletion_timestamp.as_ref()
    }

    pub fn with_deletion_timestamp(mut self, deletion_timestamp: DateTime<Utc>) -> Self {
        self.deletion_timestamp = Some(deletion_timestamp);
        self
    }

    pub fn finalizers(&self) -> Option<&Vec<String>> {
        self.finalizers.as_ref()
    }

    pub fn with_finalizers(mut self, finalizers: Vec<String>) -> Self {
        self.finalizers = Some(finalizers);
        self
    }

    pub fn generation(&self) -> Option<&i64> {
        self.generation.as_ref()
    }

    pub fn with_generation(mut self, generation: i64) -> Self {
        self.generation = Some(generation);
        self
    }

    pub fn initializers(&self) -> Option<&Vec<Initializer>> {
        self.initializers.as_ref()
    }

    pub fn with_initializers(mut self, initializers: Vec<Initializer>) -> Self {
        self.initializers = Some(initializers);
        self
    }

    pub fn labels(&self) -> Option<&HashMap<String, String>> {
        self.labels.as_ref()
    }

    pub fn with_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn owner_references(&self) -> Option<&Vec<OwnerReference>> {
        self.owner_references.as_ref()
    }

    pub fn with_owner_references(mut self, owner_references: Vec<OwnerReference>) -> Self {
        self.owner_references = Some(owner_references);
        self
    }
}
