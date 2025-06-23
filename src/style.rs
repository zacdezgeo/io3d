use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
#[derive(Clone, Debug)]
pub struct ColorRamp {
    pub start: [u8; 3],
    pub end: [u8; 3],
}

#[pymethods]
impl ColorRamp {
    #[new]
    fn new_py(start: [u8; 3], end: [u8; 3]) -> Self {
        ColorRamp { start, end }
    }
}

#[derive(Clone, Debug)]
pub enum BandStyle {
    Categorical(HashMap<u8, [u8; 3]>),
    Continuous(ColorRamp, f32, f32),
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct StyleSpec {
    pub base_layer: Option<BandStyle>,
    pub overlays: Vec<(String, BandStyle)>,
}

#[pymethods]
impl StyleSpec {
    #[new]
    fn new_py() -> Self {
        StyleSpec { base_layer: None, overlays: Vec::new() }
    }
}

impl ColorRamp {
    pub fn new(start: [u8; 3], end: [u8; 3]) -> Self {
        Self { start, end }
    }
}

impl BandStyle {
    pub fn color(&self, value: f32) -> [u8; 3] {
        match self {
            BandStyle::Categorical(map) => {
                if value.is_nan() {
                    return [255, 255, 255];
                }
                let key = value.round().clamp(0.0, 255.0) as u8;
                map.get(&key).copied().unwrap_or([255, 255, 255])
            }
            BandStyle::Continuous(ramp, min, max) => {
                if *max <= *min {
                    return ramp.end;
                }
                let t = ((value - *min) / (*max - *min)).clamp(0.0, 1.0);
                [
                    (ramp.start[0] as f32 + t * (ramp.end[0] as f32 - ramp.start[0] as f32))
                        as u8,
                    (ramp.start[1] as f32 + t * (ramp.end[1] as f32 - ramp.start[1] as f32))
                        as u8,
                    (ramp.start[2] as f32 + t * (ramp.end[2] as f32 - ramp.start[2] as f32))
                        as u8,
                ]
            }
        }
    }
}

fn extract_rgb(obj: &PyAny) -> PyResult<[u8; 3]> {
    let seq = obj.downcast::<pyo3::types::PySequence>()?;
    if seq.len()? != 3 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "RGB array must have length 3",
        ));
    }
    Ok([
        seq.get_item(0)?.extract()?,
        seq.get_item(1)?.extract()?,
        seq.get_item(2)?.extract()?,
    ])
}

pub fn parse_style_spec(dict: &pyo3::types::PyDict) -> PyResult<StyleSpec> {
    let base_layer = match dict.get_item("base_layer")? {
        Some(obj) => Some(parse_band_style(obj.downcast::<pyo3::types::PyDict>()?)?),
        None => None,
    };

    let overlays = match dict.get_item("overlays")? {
        Some(obj) => {
            let list = obj.downcast::<pyo3::types::PyList>()?;
            let mut vec = Vec::with_capacity(list.len());
            for item in list.iter() {
                let tuple = item.downcast::<pyo3::types::PyTuple>()?;
                if tuple.len() != 2 {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "overlay entries must be (name, style)",
                    ));
                }
                let name: String = tuple.get_item(0)?.extract()?;
                let style_dict = tuple.get_item(1)?.downcast::<pyo3::types::PyDict>()?;
                vec.push((name, parse_band_style(style_dict)?));
            }
            vec
        }
        None => Vec::new(),
    };

    Ok(StyleSpec { base_layer, overlays })
}

fn parse_band_style(dict: &pyo3::types::PyDict) -> PyResult<BandStyle> {
    let style_type: String = dict
        .get_item("type")?
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("style missing 'type'"))?
        .extract()?;
    match style_type.as_str() {
        "categorical" => {
            let map_dict = dict
                .get_item("mapping")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("categorical style missing mapping"))?
                .downcast::<pyo3::types::PyDict>()?;
            let mut map = HashMap::new();
            for (k, v) in map_dict.iter() {
                let key: u8 = k.extract()?;
                let value = extract_rgb(v)?;
                map.insert(key, value);
            }
            Ok(BandStyle::Categorical(map))
        }
        "continuous" => {
            let ramp_seq = dict
                .get_item("ramp")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("continuous style missing ramp"))?
                .downcast::<pyo3::types::PySequence>()?;
            if ramp_seq.len()? != 2 {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "ramp must have exactly two colors",
                ));
            }
            let start = extract_rgb(ramp_seq.get_item(0)?)?;
            let end = extract_rgb(ramp_seq.get_item(1)?)?;
            let min: f32 = dict
                .get_item("min")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("continuous style missing min"))?
                .extract()?;
            let max: f32 = dict
                .get_item("max")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("continuous style missing max"))?
                .extract()?;
            Ok(BandStyle::Continuous(ColorRamp::new(start, end), min, max))
        }
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("unknown style type")),
    }
}
