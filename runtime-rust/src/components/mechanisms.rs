use whitenoise_validator::errors::*;

use crate::NodeArguments;
use whitenoise_validator::base::{Array, ReleaseNode};
use whitenoise_validator::utilities::{get_argument, broadcast_privacy_usage, broadcast_ndarray, get_epsilon, get_delta};
use crate::components::Evaluable;
use crate::utilities;
use whitenoise_validator::proto;
use ndarray;

impl Evaluable for proto::LaplaceMechanism {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<ReleaseNode> {
        let mut data = match get_argument(&arguments, "data")?.array()? {
            Array::F64(data) => data.clone(),
            Array::I64(data) => data.mapv(|v| v as f64),
            _ => return Err("data must be numeric".into())
        };
//        println!("data: {:?}", data);

        let sensitivity = get_argument(&arguments, "sensitivity")?.array()?.f64()?;
//        println!("sensitivity: {:?}", sensitivity);

        let usages = broadcast_privacy_usage(&self.privacy_usage, sensitivity.len())?;

        let epsilon = ndarray::Array::from_shape_vec(
            data.shape(), usages.iter().map(get_epsilon).collect::<Result<Vec<f64>>>()?)?;
//        println!("epsilon: {:?}", epsilon);

        data.gencolumns_mut().into_iter()
            .zip(sensitivity.gencolumns().into_iter().zip(epsilon.gencolumns().into_iter()))
            .map(|(mut data_column, (sensitivity, epsilon))| data_column.iter_mut()
                .zip(sensitivity.iter().zip(epsilon.iter()))
                .map(|(v, (sens, eps))| {
                    *v += utilities::mechanisms::laplace_mechanism(&eps, &sens)?;
                    Ok(())
                })
                .collect::<Result<()>>())
            .collect::<Result<()>>()?;

        Ok(ReleaseNode {
            value: data.into(),
            privacy_usages: Some(usages),
            public: true
        })
    }
}

impl Evaluable for proto::GaussianMechanism {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<ReleaseNode> {
        let mut data = match get_argument(&arguments, "data")?.array()? {
            Array::F64(data) => data.clone(),
            Array::I64(data) => data.mapv(|v| v as f64),
            _ => return Err("data must be numeric".into())
        };
//        println!("data: {:?}", data.shape());

        let sensitivity = get_argument(&arguments, "sensitivity")?.array()?.f64()?;
//        println!("sensitivity: {:?}", sensitivity.shape());

        let usages = broadcast_privacy_usage(&self.privacy_usage, sensitivity.len())?;

        let epsilon = ndarray::Array::from_shape_vec(
            data.shape(), usages.iter().map(get_epsilon).collect::<Result<Vec<f64>>>()?)?;
//        println!("epsilon: {:?}", epsilon.shape());

        let delta = ndarray::Array::from_shape_vec(
            data.shape(), usages.iter().map(get_delta).collect::<Result<Vec<f64>>>()?)?;
//        println!("delta: {:?}", delta.shape());

        data.gencolumns_mut().into_iter()
            .zip(sensitivity.gencolumns().into_iter())
            .zip(epsilon.gencolumns().into_iter().zip(delta.gencolumns().into_iter()))
            .map(|((mut data_column, sensitivity), (epsilon, delta))| data_column.iter_mut()
                .zip(sensitivity.iter())
                .zip(epsilon.iter().zip(delta.iter()))
                .map(|((v, sens), (eps, del))| {
                    *v += utilities::mechanisms::gaussian_mechanism(&eps, &del, &sens)?;
                    Ok(())
                }).collect::<Result<()>>())
            .collect::<Result<()>>()?;

        Ok(ReleaseNode {
            value: data.into(),
            privacy_usages: Some(usages),
            public: true
        })
    }
}

impl Evaluable for proto::SimpleGeometricMechanism {
    fn evaluate(&self, arguments: &NodeArguments) -> Result<ReleaseNode> {
        let mut data = get_argument(&arguments, "data")?.array()?.i64()?.clone();
//        println!("data: {:?}", data.shape());

        let sensitivity = get_argument(&arguments, "sensitivity")?.array()?.f64()?;
//        println!("sensitivity: {:?}", sensitivity.shape());

        let usages = broadcast_privacy_usage(&self.privacy_usage, sensitivity.len())?;
        let epsilon = ndarray::Array::from_shape_vec(
            data.shape(), usages.iter().map(get_epsilon).collect::<Result<Vec<f64>>>()?)?;
//        println!("epsilon: {:?}", epsilon.shape());

        let lower = broadcast_ndarray(
            get_argument(&arguments, "lower")?.array()?.i64()?, data.shape())?;
//        println!("min: {:?}", min.shape());

        let upper = broadcast_ndarray(
            get_argument(&arguments, "upper")?.array()?.i64()?, data.shape())?;
//        println!("max: {:?}", max.shape());

        data.gencolumns_mut().into_iter()
            .zip(sensitivity.gencolumns().into_iter().zip(epsilon.gencolumns().into_iter()))
            .zip(lower.gencolumns().into_iter().zip(upper.gencolumns().into_iter()))
            .map(|((mut data_column, (sensitivity, epsilon)), (lower, upper))| data_column.iter_mut()
                .zip(sensitivity.iter().zip(epsilon.iter()))
                .zip(lower.iter().zip(upper.iter()))
                .map(|((v, (sens, eps)), (c_min, c_max))| {
                    *v += utilities::mechanisms::simple_geometric_mechanism(
                        &eps, &sens, &c_min, &c_max, &self.enforce_constant_time)?;
                    Ok(())
                })
                .collect::<Result<()>>())
            .collect::<Result<()>>()?;

        Ok(ReleaseNode {
            value: data.into(),
            privacy_usages: Some(usages),
            public: true
        })
    }
}
