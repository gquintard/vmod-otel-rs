// import the generated boilerplate
varnish::boilerplate!();

use rand::prelude::*;
use varnish::vcl::ctx::{Ctx, LogTag};
use varnish::vcl::vpriv::VPriv;

varnish::vtc!(test01);
const VCL_LOG: LogTag = LogTag::Any(varnish_sys::VSL_tag_e_SLT_VCL_Log);

// makes sure a substring has the right length and is lowercase ascii
fn check_block(s: &str, l: usize) -> bool {
    s.chars()
        .filter(|c| c.is_ascii_digit() || (*c >= 'a' && *c <= 'f'))
        .count()
        == l
}

// split a
fn split_trace_parent(s: &str) -> Result<[&str; 4], &'static str> {
    let v: Vec<&str> = s.split('-').collect();
    if v.len() != 4
        || v[0] != "00"
        || !check_block(v[1], 32)
        || !check_block(v[2], 16)
        || !check_block(v[3], 2)
    {
        Err("something went wrong")
    } else {
        Ok([v[0], v[1], v[2], v[3]])
    }
}

// generate some random bytes
fn random_fill(buf: &mut [u8]) {
    for p in buf {
        *p = random();
    }
}

// give an existing traceparent (valid or not), generate a new one
fn new_span(ctx: &mut Ctx, top_traceparent: &str) -> String {
    let mut span_id = [0; 8];
    random_fill(&mut span_id);

    // try to split the top trace
    let trace = match split_trace_parent(&top_traceparent).ok() {
        // tell the logger we have a parent, and generate a child
        Some(stp) => {
            ctx.log(
                VCL_LOG,
                &format!("otel-parent-context: {}", &top_traceparent),
            );
            format!("{}-{}-{}-{}", stp[0], stp[1], hex::encode(span_id), stp[3])
        }
        // otherwise, generate a new trace and span
        None => {
            let mut trace_id = [0; 16];
            random_fill(&mut trace_id);
            format!("00-{}-{}-00", hex::encode(trace_id), hex::encode(span_id))
        }
    };
    ctx.log(VCL_LOG, &format!("otel-context: {}", &trace));
    trace
}

// just create a new span, no questions asked
pub fn new_bereq_span(ctx: &mut Ctx, s: &str) -> String {
    new_span(ctx, s)
}

pub fn new_req_span(ctx: &mut Ctx, vp: &mut VPriv<String>, s: &str) -> Result<String, &'static str> {
    // vp is a PRIV_TOP, we we only write to it when we are at the top
    let top_traceparent = vp.as_ref().map(|s| s.as_str()).unwrap_or(s);

    let trace = new_span(ctx, top_traceparent);
    if unsafe {(*(*ctx.raw).req).esi_level} == 0 {
        vp.store(trace.clone());
    }
    Ok(trace)
}

pub fn log(ctx: &mut Ctx, s: &str) {
    ctx.log(VCL_LOG, &format!("otel-log: {} {}", ctx.raw.now, s));
}
