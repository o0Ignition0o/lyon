use std::io;
use lyon::path::default::Path;
use lyon::tessellation::{FillOptions, StrokeOptions};
use lyon::algorithms::hatching::HatchingOptions;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tessellator {
    Default,
    Tess2,
}

#[derive(Clone, Debug)]
pub struct TessellateCmd {
    pub path: Path,
    pub fill: Option<FillOptions>,
    pub stroke: Option<StrokeOptions>,
    pub hatch: Option<HatchingParams>,
    pub float_precision: Option<usize>,
    pub tessellator: Tessellator,
}

#[derive(Clone, Debug)]
pub struct HatchingParams {
    pub options: HatchingOptions,
    pub stroke: StrokeOptions,
    pub spacing: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AntiAliasing {
    Msaa(u16),
    None,
}

#[derive(Clone, Debug)]
pub struct RenderCmd {
    pub aa: AntiAliasing,
}

pub struct PathCmd {
    pub path: Path,
    pub output: Box<io::Write>,
    pub tolerance: f32,
    pub count: bool,
    pub flatten: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct FuzzCmd {
    pub fill: bool,
    pub stroke: bool,
    pub min_points: Option<u32>,
    pub max_points: Option<u32>,
    pub tessellator: Tessellator,
    pub ignore_errors: bool,
}
