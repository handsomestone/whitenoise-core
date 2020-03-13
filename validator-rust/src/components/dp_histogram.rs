use crate::errors::*;


use std::collections::HashMap;

use crate::{proto, base};
use crate::hashmap;
use crate::components::{Component, Accuracy, Expandable, Report};


use crate::base::{NodeProperties, Value, ValueProperties};
use crate::utilities::json::{JSONRelease};



impl Component for proto::DpHistogram {
    // modify min, max, n, categories, is_public, non-null, etc. based on the arguments and component
    fn propagate_property(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        _public_arguments: &HashMap<String, Value>,
        properties: &base::NodeProperties,
    ) -> Result<ValueProperties> {
        Err("DPCount is ethereal, and has no property propagation".into())
    }

    fn get_names(
        &self,
        _properties: &NodeProperties,
    ) -> Result<Vec<String>> {
        Err("get_names not implemented".into())
    }
}


impl Expandable for proto::DpHistogram {
    fn expand_component(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        component: &proto::Component,
        _properties: &base::NodeProperties,
        component_id: u32,
        maximum_id: u32,
    ) -> Result<(u32, HashMap<u32, proto::Component>)> {
        let mut current_id = maximum_id.clone();
        let mut graph_expansion: HashMap<u32, proto::Component> = HashMap::new();

        let data_id = component.arguments.get("data")
            .ok_or::<Error>("data is a required argument to DPHistogram".into())?;
        let edges_id = component.arguments.get("edges")
            .ok_or::<Error>("edges is a required argument to DPHistogram".into())?;
        let null_id = component.arguments.get("null")
            .ok_or::<Error>("null is a required argument to DPHistogram".into())?;
        let inclusive_left_id = component.arguments.get("inclusive_left")
            .ok_or::<Error>("inclusive_left is a required argument to DPHistogram".into())?;
        let count_min_id = component.arguments.get("count_min")
            .ok_or::<Error>("count_min is a required argument to DPHistogram".into())?;
        let count_max_id = component.arguments.get("count_max")
            .ok_or::<Error>("count_max is a required argument to DPHistogram".into())?;
        // TODO: also handle categorical case, which doesn't require binning
        // bin
        current_id += 1;
        let id_bin = current_id.clone();
        graph_expansion.insert(id_bin, proto::Component {
            arguments: hashmap![
                "data".to_owned() => *data_id,
                "edges".to_owned() => *edges_id,
                "null".to_owned() => *null_id,
                "inclusive_left".to_owned() => *inclusive_left_id
            ],
            variant: Some(proto::component::Variant::from(proto::Bin {
                side: self.side.clone()
            })),
            omit: true,
            batch: component.batch,
        });

        // dp_count
        graph_expansion.insert(component_id, proto::Component {
            arguments: hashmap![
                "data".to_owned() => id_bin,
                "count_min".to_owned() => *count_min_id,
                "count_max".to_owned() => *count_max_id
            ],
            variant: Some(proto::component::Variant::from(proto::DpCount {
                privacy_usage: self.privacy_usage.clone(),
                implementation: self.implementation.clone()
            })),
            omit: false,
            batch: component.batch,
        });

        Ok((current_id, graph_expansion))
    }
}

impl Accuracy for proto::DpHistogram {
    fn accuracy_to_privacy_usage(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        _properties: &base::NodeProperties,
        _accuracy: &proto::Accuracy,
    ) -> Option<proto::PrivacyUsage> {
        None
    }

    fn privacy_usage_to_accuracy(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        _property: &base::NodeProperties,
    ) -> Option<f64> {
        None
    }
}

impl Report for proto::DpHistogram {
    fn summarize(
        &self,
        _node_id: &u32,
        _component: &proto::Component,
        _public_arguments: &HashMap<String, Value>,
        _properties: &NodeProperties,
        _release: &Value
    ) -> Result<Option<Vec<JSONRelease>>> {
        Ok(None)
    }
}
