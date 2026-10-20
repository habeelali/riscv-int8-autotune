//! Deterministic scalar Rust source generator; no target assembly is emitted.
pub mod depthwise_tile;
pub mod flash;
pub mod packed;
#[cfg(test)]
mod quant_bounded;
pub mod tiled;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workload {
    pub name: String,
    pub operator: String,
    pub height: usize,
    pub width: usize,
    pub input_channels: usize,
    pub output_channels: usize,
    pub kernel: usize,
    pub stride: usize,
    pub padding: usize,
    pub input_offset: i32,
    pub output_offset: i32,
    pub quant_shift: i32,
    pub activation_min: i32,
    pub activation_max: i32,
    pub seed: u32,
    #[serde(default)]
    pub multipliers: Option<Vec<i32>>,
    #[serde(default)]
    pub shifts: Option<Vec<i32>>,
    pub isa: String,
    pub flash_budget: usize,
    pub sram_budget: usize,
}
impl Workload {
    pub fn parse(text: &str) -> Result<Self, String> {
        let w: Self = toml::from_str(text).map_err(|e| e.to_string())?;
        w.validate()?;
        Ok(w)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self
            .multipliers
            .as_ref()
            .is_some_and(|v| v.len() != self.output_channels || v.iter().any(|&m| m < 0))
            || self.shifts.as_ref().is_some_and(|v| {
                v.len() != self.output_channels || v.iter().any(|&s| !(-30..=0).contains(&s))
            })
        {
            return Err("per-channel quantization arrays have invalid lengths or values".into());
        }
        let dims = [
            self.height,
            self.width,
            self.input_channels,
            self.output_channels,
        ];
        if self.isa != "rv32imc"
            || !dims.iter().all(|&v| v > 0 && v <= 1024)
            || !["conv", "depthwise", "fc"].contains(&self.operator.as_str())
            || ![1, 3].contains(&self.kernel)
            || !(1..=4).contains(&self.stride)
            || self.padding > 2
            || self.height + 2 * self.padding < self.kernel
            || self.width + 2 * self.padding < self.kernel
            || (self.kernel == 1 && self.padding != 0)
            || (self.operator == "depthwise" && self.input_channels != self.output_channels)
            || (self.operator == "fc" && (self.height != 1 || self.width != 1 || self.kernel != 1))
            || !(-128..=128).contains(&self.input_offset)
            || !(-128..=127).contains(&self.output_offset)
            || !(-25..=0).contains(&self.quant_shift)
            || self.activation_min < -128
            || self.activation_max > 127
            || self.activation_min > self.activation_max
            || !self
                .name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
            || self.name.is_empty()
        {
            return Err("unsupported or invalid scalar INT8 workload".into());
        }
        Ok(())
    }
    pub fn oh(&self) -> usize {
        (self.height + 2 * self.padding - self.kernel) / self.stride + 1
    }
    pub fn ow(&self) -> usize {
        (self.width + 2 * self.padding - self.kernel) / self.stride + 1
    }
    pub fn input_len(&self) -> usize {
        self.height * self.width * self.input_channels
    }
    pub fn weights_len(&self) -> usize {
        self.kernel
            * self.kernel
            * self.input_channels
            * if self.operator == "depthwise" {
                1
            } else {
                self.output_channels
            }
    }
    pub fn output_len(&self) -> usize {
        self.oh() * self.ow() * self.output_channels
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub block: usize,
    pub unroll: usize,
    pub spatial_unroll: bool,
    pub im2col: bool,
    pub interior: bool,
    pub pixels: usize,
    pub channel_outer: bool,
    pub fast_quant: bool,
}
impl Config {
    pub fn name(self) -> String {
        let mut base = format!(
            "b{}_u{}_s{}",
            self.block,
            self.unroll,
            usize::from(self.spatial_unroll)
        );
        if self.im2col {
            base.push_str("_im2col");
        }
        if self.interior {
            base.push_str("_interior");
        }
        if self.pixels > 1 {
            base.push_str(&format!("_p{}", self.pixels));
        }
        if self.channel_outer {
            base.push_str("_cxy");
        }
        if self.fast_quant {
            base.push_str("_qpos");
        }
        base
    }
}
/// Small complete grid. DW reduction-unroll is irrelevant, so omitted.
pub fn grid(w: &Workload) -> Vec<Config> {
    let mut r = Vec::new();
    for block in [1, 2, 4, 8] {
        for unroll in if w.operator == "depthwise" {
            vec![1]
        } else {
            vec![1, 4, 8, 16]
        } {
            for spatial_unroll in if w.kernel == 1 {
                vec![false]
            } else {
                vec![false, true]
            } {
                r.push(Config {
                    block,
                    unroll,
                    spatial_unroll,
                    im2col: false,
                    interior: false,
                    pixels: 1,
                    channel_outer: false,
                    fast_quant: false,
                });
            }
        }
    }
    if w.kernel == 3 {
        for block in [1, 2, 4, 8] {
            for unroll in if w.operator == "depthwise" {
                vec![1]
            } else {
                vec![1, 4]
            } {
                r.push(Config {
                    block,
                    unroll,
                    spatial_unroll: true,
                    im2col: false,
                    interior: true,
                    pixels: 1,
                    channel_outer: false,
                    fast_quant: false,
                });
            }
            if w.operator != "depthwise" {
                for unroll in [1, 4, 8, 16] {
                    r.push(Config {
                        block,
                        unroll,
                        spatial_unroll: false,
                        im2col: true,
                        interior: false,
                        pixels: 1,
                        channel_outer: false,
                        fast_quant: false,
                    });
                }
            }
        }
    }

    if w.operator != "depthwise" && w.ow() >= 2 {
        for pixels in [2, 4] {
            if pixels > w.ow() {
                continue;
            }
            for block in [1, 2, 4] {
                if pixels * block > 8 {
                    continue;
                }
                for unroll in [1, 4, 8] {
                    r.push(Config {
                        block,
                        unroll,
                        spatial_unroll: false,
                        im2col: w.kernel > 1,
                        interior: false,
                        pixels,
                        channel_outer: false,
                        fast_quant: false,
                    });
                }
            }
        }
    }
    if w.operator == "depthwise" {
        for block in [1, 2, 4, 8] {
            for (spatial_unroll, interior) in [(false, false), (true, false), (true, true)] {
                r.push(Config {
                    block,
                    unroll: 1,
                    spatial_unroll,
                    im2col: false,
                    interior,
                    pixels: 1,
                    channel_outer: true,
                    fast_quant: false,
                });
            }
        }
    }
    if w.operator == "fc" {
        for (block, unroll) in [(4, 2), (8, 2), (16, 1), (16, 2), (16, 4)] {
            r.push(Config {
                block,
                unroll,
                spatial_unroll: false,
                im2col: false,
                interior: false,
                pixels: 1,
                channel_outer: false,
                fast_quant: false,
            });
        }
    }
    let optimized_quant: Vec<_> = r
        .iter()
        .map(|c| Config {
            fast_quant: true,
            ..*c
        })
        .collect();
    r.extend(optimized_quant);

    r
}
fn line(s: &mut String, args: std::fmt::Arguments<'_>) {
    s.write_fmt(args).unwrap();
    s.push('\n');
}
macro_rules! emit { ($s:expr,$($a:tt)*) => {line($s,format_args!($($a)*))}; }

fn mac(s: &mut String, w: &Workload, block: usize, index: &str) {
    if w.operator == "depthwise" {
        for b in 0..block {
            emit!(
                s,
                "a{b} += (input[ip+c+{b}] as i32 + {}) * weights[wp+c+{b}] as i32;",
                w.input_offset
            );
        }
    } else {
        emit!(s, "let v = input[ip+{index}] as i32 + {};", w.input_offset);
        for b in 0..block {
            emit!(
                s,
                "a{b} += v * weights[((c+{b})*{}+wp)+{index}] as i32;",
                w.kernel * w.kernel * w.input_channels
            );
        }
    }
}
fn reduction(s: &mut String, w: &Workload, cfg: Config, block: usize) {
    if w.operator == "depthwise" {
        mac(s, w, block, "0");
        return;
    }
    let n = w.input_channels;
    let u = cfg.unroll;
    let end = n / u * u;
    emit!(s, "let mut i=0usize; while i < {end} {{");
    for t in 0..u {
        mac(s, w, block, &format!("i+{t}"));
    }
    emit!(s, "i+={u}; }}");
    for t in end..n {
        mac(s, w, block, &t.to_string());
    }
}
fn spatial(s: &mut String, w: &Workload, cfg: Config, block: usize) {
    if cfg.im2col {
        let n = w.kernel * w.kernel * w.input_channels;
        let u = cfg.unroll;
        let end = n / u * u;
        emit!(s, "let mut i=0usize; while i<{end} {{");
        for t in 0..u {
            emit!(s, "let v=column[i+{t}] as i32;");
            for b in 0..block {
                emit!(s, "a{b} += v * weights[(c+{b})*{n}+i+{t}] as i32;");
            }
        }
        emit!(s, "i+={u}; }}");
        for t in end..n {
            emit!(s, "let v=column[{t}] as i32;");
            for b in 0..block {
                emit!(s, "a{b} += v * weights[(c+{b})*{n}+{t}] as i32;");
            }
        }
        return;
    }
    if cfg.interior {
        emit!(
            s,
            "if y*{}>={} && y*{}+{}<{} && x*{}>={} && x*{}+{}<{} {{",
            w.stride,
            w.padding,
            w.stride,
            w.kernel - 1,
            w.height + w.padding,
            w.stride,
            w.padding,
            w.stride,
            w.kernel - 1,
            w.width + w.padding
        );
        for ky in 0..w.kernel {
            for kx in 0..w.kernel {
                emit!(
                    s,
                    "{{ let ip=((y*{}+{}-{})*{}+x*{}+{}-{})*{}; let wp={};",
                    w.stride,
                    ky,
                    w.padding,
                    w.width,
                    w.stride,
                    kx,
                    w.padding,
                    w.input_channels,
                    (ky * w.kernel + kx) * w.input_channels
                );
                reduction(s, w, cfg, block);
                emit!(s, "}}");
            }
        }
        emit!(s, "}} else {{");
        spatial(
            s,
            w,
            Config {
                interior: false,
                ..cfg
            },
            block,
        );
        emit!(s, "}}");
        return;
    }
    let k = w.kernel;
    let ic = w.input_channels;
    if cfg.spatial_unroll {
        for ky in 0..k {
            for kx in 0..k {
                emit!(
                    s,
                    "{{ let iy=(y*{}+{ky}) as isize-{}; let ix=(x*{}+{kx}) as isize-{};",
                    w.stride,
                    w.padding,
                    w.stride,
                    w.padding
                );
                emit!(
                    s,
                    "if iy>=0 && iy<{} && ix>=0 && ix<{} {{",
                    w.height,
                    w.width
                );
                emit!(
                    s,
                    "let ip=(iy as usize*{}+ix as usize)*{ic}; let wp={};",
                    w.width,
                    (ky * k + kx) * ic
                );
                reduction(s, w, cfg, block);
                emit!(s, "}} }}");
            }
        }
    } else {
        // Clip spatial bounds once per pixel/block, matching ESP-NN's border policy.
        emit!(
            s,
            "let by=(y*{}) as isize-{}; let bx=(x*{}) as isize-{};",
            w.stride,
            w.padding,
            w.stride,
            w.padding
        );
        emit!(
            s,
            "let ys=0isize.max(-by) as usize; let ye=({k}isize).min({}isize-by) as usize;",
            w.height
        );
        emit!(
            s,
            "let xs=0isize.max(-bx) as usize; let xe=({k}isize).min({}isize-bx) as usize;",
            w.width
        );
        emit!(s, "for ky in ys..ye {{ for kx in xs..xe {{");
        emit!(s,"let ip=(((by+ky as isize) as usize)*{}+(bx+kx as isize) as usize)*{ic}; let wp=(ky*{k}+kx)*{ic};",w.width);
        reduction(s, w, cfg, block);
        emit!(s, "}} }}");
    }
}
fn output_block(s: &mut String, w: &Workload, cfg: Config, block: usize) {
    for b in 0..block {
        emit!(s, "let mut a{b}=bias[c+{b}];");
    }
    spatial(s, w, cfg, block);
    for b in 0..block {
        emit!(s,"output[(y*{}+x)*{}+c+{b}] = (requantize(a{b},mult[c+{b}],shift[c+{b}])+{}).clamp({},{}) as i8;",w.ow(),w.output_channels,w.output_offset,w.activation_min,w.activation_max);
    }
}

fn tile_prepare(s: &mut String, w: &Workload, pixels: usize) {
    if w.kernel == 1 {
        return;
    }
    let n = w.kernel * w.kernel * w.input_channels;
    for p in 0..pixels {
        emit!(s,"for ky in 0..{} {{ for kx in 0..{} {{ let iy=(y*{}+ky) as isize-{}; let ix=((x+{p})*{}+kx) as isize-{}; for i in 0..{} {{",w.kernel,w.kernel,w.stride,w.padding,w.stride,w.padding,w.input_channels);
        emit!(s,"columns[{p}*{n}+(ky*{}+kx)*{}+i]=if iy>=0 && iy<{} && ix>=0 && ix<{} {{(input[(iy as usize*{}+ix as usize)*{}+i] as i32+{}) as i16}}else{{0}}; }} }} }}",w.kernel,w.input_channels,w.height,w.width,w.width,w.input_channels,w.input_offset);
    }
}
fn tile_mac(s: &mut String, w: &Workload, pixels: usize, block: usize, index: &str) {
    let n = w.kernel * w.kernel * w.input_channels;
    for p in 0..pixels {
        if w.kernel > 1 {
            emit!(s, "let v{p}=columns[{p}*{n}+{index}] as i32;");
        } else {
            emit!(
                s,
                "let v{p}=input[(y*{}*{}+(x+{p})*{})*{}+{index}] as i32+{};",
                w.stride,
                w.width,
                w.stride,
                w.input_channels,
                w.input_offset
            );
        }
    }
    for b in 0..block {
        emit!(s, "let wb=weights[(c+{b})*{n}+{index}] as i32;");
        for p in 0..pixels {
            emit!(s, "a{p}_{b} += v{p}*wb;");
        }
    }
}
fn tile_block(s: &mut String, w: &Workload, cfg: Config, pixels: usize, block: usize) {
    for b in 0..block {
        emit!(s, "let base{b}=bias[c+{b}];");
        for p in 0..pixels {
            emit!(s, "let mut a{p}_{b}=base{b};");
        }
    }
    let n = w.kernel * w.kernel * w.input_channels;
    let end = n / cfg.unroll * cfg.unroll;
    emit!(s, "let mut i=0usize; while i<{end} {{");
    for u in 0..cfg.unroll {
        emit!(s, "{{");
        tile_mac(s, w, pixels, block, &format!("i+{u}"));
        emit!(s, "}}");
    }
    emit!(s, "i+={}; }}", cfg.unroll);
    for i in end..n {
        tile_mac(s, w, pixels, block, &i.to_string());
    }
    for b in 0..block {
        for p in 0..pixels {
            emit!(s,"output[(y*{}+x+{p})*{}+c+{b}]=(requantize(a{p}_{b},mult[c+{b}],shift[c+{b}])+{}).clamp({},{}) as i8;",w.ow(),w.output_channels,w.output_offset,w.activation_min,w.activation_max);
        }
    }
}
fn tile_channels(s: &mut String, w: &Workload, cfg: Config, pixels: usize) {
    let end = w.output_channels / cfg.block * cfg.block;
    emit!(s, "let mut c=0usize; while c<{end} {{");
    tile_block(s, w, cfg, pixels, cfg.block);
    emit!(s, "c+={}; }}", cfg.block);
    if end < w.output_channels {
        emit!(s, "while c<{} {{", w.output_channels);
        tile_block(s, w, cfg, pixels, 1);
        emit!(s, "c+=1; }}");
    }
}
fn tiled_body(s: &mut String, w: &Workload, cfg: Config) {
    if w.kernel > 1 {
        emit!(
            s,
            "let mut columns=[0i16;{}];",
            w.kernel * w.kernel * w.input_channels * cfg.pixels
        );
    }
    let end = w.ow() / cfg.pixels * cfg.pixels;
    emit!(
        s,
        "for y in 0..{} {{ let mut x=0usize; while x<{end} {{",
        w.oh()
    );
    tile_prepare(s, w, cfg.pixels);
    tile_channels(s, w, cfg, cfg.pixels);
    emit!(s, "x+={}; }}", cfg.pixels);
    if end < w.ow() {
        emit!(s, "while x<{} {{", w.ow());
        tile_prepare(s, w, 1);
        tile_channels(s, w, cfg, 1);
        emit!(s, "x+=1; }}");
    }
    emit!(s, "}} }}"); // y loop, run function
}
fn channel_body(s: &mut String, w: &Workload, cfg: Config) {
    let end = w.output_channels / cfg.block * cfg.block;
    emit!(
        s,
        "let mut c=0usize; while c<{end} {{ for y in 0..{} {{ for x in 0..{} {{",
        w.oh(),
        w.ow()
    );
    output_block(s, w, cfg, cfg.block);
    emit!(s, "}} }} c+={}; }}", cfg.block);
    if end < w.output_channels {
        emit!(
            s,
            "while c<{} {{ for y in 0..{} {{ for x in 0..{} {{",
            w.output_channels,
            w.oh(),
            w.ow()
        );
        output_block(s, w, cfg, 1);
        emit!(s, "}} }} c+=1; }}");
    }
    emit!(s, "}}"); // run function
}
/// Independently reusable no_std source. Uses safe slices and explicit bounds checks.
/// Multipliers and shifts stay runtime values; weights are never compiled into code.
pub fn generate(w: &Workload, cfg: Config) -> Result<String, String> {
    w.validate()?;
    if ![1, 2, 4, 8, 16].contains(&cfg.block) || ![1, 2, 4, 8, 16].contains(&cfg.unroll) {
        return Err("invalid configuration".into());
    }
    if ![1, 2, 4].contains(&cfg.pixels)
        || (cfg.pixels > 1 && (w.operator == "depthwise" || w.ow() < cfg.pixels))
        || (cfg.channel_outer && w.operator != "depthwise")
    {
        return Err("invalid spatial schedule".into());
    }
    if (cfg.im2col && w.operator == "depthwise")
        || (cfg.pixels > 1 && cfg.im2col != (w.kernel > 1))
        || (cfg.channel_outer && cfg.im2col)
    {
        return Err("incompatible buffer/schedule configuration".into());
    }
    let core = include_str!("../../kernel-core/src/lib.rs");
    let quant = core
        .split("#[inline(always)]")
        .nth(1)
        .unwrap()
        .split("/// Generic correct reference")
        .next()
        .unwrap();
    let quant = if cfg.fast_quant {
        include_str!("quant_fast.rs").replace("requantize_nonpositive", "requantize")
    } else {
        format!("#[inline(always)]{quant}")
    };
    let mut s=format!("#![no_std]\n// Generated for {} with {}. Scratch: 0 bytes.\n// Input/weight/output lengths: {}, {}, {}.\n{}\n",w.name,cfg.name(),w.input_len(),w.weights_len(),w.output_len(),quant);
    if cfg.im2col {
        s = s.replace(
            "Scratch: 0 bytes.",
            &format!(
                "Scratch: {} bytes (local stack partial im2col, included in watermark).",
                2 * w.kernel * w.kernel * w.input_channels * cfg.pixels
            ),
        );
    }
    s.push_str("/// All buffers are caller owned. INT32 accumulation and shifted values must fit.\n#[inline(never)]\npub fn run(input:&[i8],weights:&[i8],bias:&[i32],mult:&[i32],shift:&[i32],output:&mut[i8]) {\n");
    emit!(
        &mut s,
        "assert!(input.len()>={} && weights.len()>={} && output.len()>={});",
        w.input_len(),
        w.weights_len(),
        w.output_len()
    );
    emit!(
        &mut s,
        "assert!(bias.len()>={} && mult.len()>={} && shift.len()>={});",
        w.output_channels,
        w.output_channels,
        w.output_channels
    );
    if cfg.fast_quant {
        emit!(&mut s,"assert!(mult[..{}].iter().all(|&m|m>=0) && shift[..{}].iter().all(|&q|(-30..=0).contains(&q)));",w.output_channels,w.output_channels);
    }
    if cfg.pixels > 1 {
        tiled_body(&mut s, w, cfg);
    } else if cfg.channel_outer {
        channel_body(&mut s, w, cfg);
    } else {
        if cfg.im2col {
            emit!(
                &mut s,
                "let mut column=[0i16;{}];",
                w.kernel * w.kernel * w.input_channels
            );
        }
        emit!(
            &mut s,
            "for y in 0..{} {{ for x in 0..{} {{",
            w.oh(),
            w.ow()
        );
        if cfg.im2col {
            emit!(&mut s,"for ky in 0..{} {{ for kx in 0..{} {{ let iy=(y*{}+ky) as isize-{}; let ix=(x*{}+kx) as isize-{}; for i in 0..{} {{",w.kernel,w.kernel,w.stride,w.padding,w.stride,w.padding,w.input_channels);
            emit!(&mut s,"column[(ky*{}+kx)*{}+i]=if iy>=0 && iy<{} && ix>=0 && ix<{} {{(input[(iy as usize*{}+ix as usize)*{}+i] as i32+{}) as i16}}else{{0}}; }} }} }}",w.kernel,w.input_channels,w.height,w.width,w.width,w.input_channels,w.input_offset);
        }
        let end = w.output_channels / cfg.block * cfg.block;
        emit!(&mut s, "let mut c=0usize; while c<{end} {{");
        output_block(&mut s, w, cfg, cfg.block);
        emit!(&mut s, "c+={}; }}", cfg.block);
        if end < w.output_channels {
            emit!(&mut s, "while c<{} {{", w.output_channels);
            output_block(&mut s, w, cfg, 1);
            emit!(&mut s, "c+=1; }}");
        }
        emit!(&mut s, "}} }} }}");
    }
    // Benchmark ABI also provides a convenient C interface, with exact shape contract.
    emit!(&mut s,"/// # Safety\n/// Pointers must be aligned, valid for lengths {}, {}, {}, {}, {}, {} respectively.\n/// Output must not overlap inputs. Quantization contract is documented above.\n#[no_mangle]\npub unsafe extern \"C\" fn kernel(i:*const i8,w:*const i8,b:*const i32,m:*const i32,s:*const i32,o:*mut i8) {{",w.input_len(),w.weights_len(),w.output_channels,w.output_channels,w.output_channels,w.output_len());
    emit!(&mut s,"run(core::slice::from_raw_parts(i,{}),core::slice::from_raw_parts(w,{}),core::slice::from_raw_parts(b,{}),core::slice::from_raw_parts(m,{}),core::slice::from_raw_parts(s,{}),core::slice::from_raw_parts_mut(o,{})); }}",w.input_len(),w.weights_len(),w.output_channels,w.output_channels,w.output_channels,w.output_len());
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_and_generates_different_schedules() {
        let w = Workload::parse(include_str!("../../../workloads/conv_small.toml")).unwrap();
        let a = generate(
            &w,
            Config {
                block: 1,
                unroll: 1,
                spatial_unroll: false,
                im2col: false,
                interior: false,
                pixels: 1,
                channel_outer: false,
                fast_quant: false,
            },
        )
        .unwrap();
        let b = generate(
            &w,
            Config {
                block: 4,
                unroll: 4,
                spatial_unroll: true,
                im2col: false,
                interior: false,
                pixels: 1,
                channel_outer: false,
                fast_quant: false,
            },
        )
        .unwrap();
        assert_ne!(a, b);
        assert!(b.contains("let mut a3="));
        let mut bad = w;
        bad.isa = "rv64gcv".into();
        assert!(bad.validate().is_err());
    }
}

#[cfg(test)]
mod quant_fast;
#[cfg(test)]
mod quant_tests {
    #[test]
    fn nonnegative_contract_matches_general_rounding() {
        let mut state = 20261018u32;
        for x in [
            i32::MIN,
            -1000001,
            -5,
            -2,
            -1,
            0,
            1,
            2,
            5,
            1000001,
            i32::MAX,
        ] {
            for m in [0, 1, 1 << 30, i32::MAX] {
                for q in -30..=0 {
                    assert_eq!(
                        super::quant_fast::requantize_nonpositive(x, m, q),
                        kernel_core::requantize(x, m, q)
                    );
                }
            }
        }
        for _ in 0..10000 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            let x = state as i32;
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            let m = (state & 0x7fffffff) as i32;
            let q = -((state % 31) as i32);
            assert_eq!(
                super::quant_fast::requantize_nonpositive(x, m, q),
                kernel_core::requantize(x, m, q)
            );
        }
    }
}
