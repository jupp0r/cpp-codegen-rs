use handlebars::{Context, Helper, Handlebars, RenderContext, RenderError};
use serde_json::value::Value;

pub fn len(_: &Context,
           h: &Helper,
           _: &Handlebars,
           rc: &mut RenderContext)
           -> Result<(), RenderError> {
    let param = h.param(0).unwrap();
    let length = match param.value() {
        &Value::Array(ref a) => a.len(),
        _ => 0,
    };

    let rendered = format!("{}", length);
    try!(rc.writer.write(rendered.into_bytes().as_ref()));
    Ok(())
}

pub fn join(
    _: &Context,
    h: &Helper,
    _: &Handlebars,
    rc: &mut RenderContext)
    -> Result<(), RenderError> {
    let empty = vec![];
    let param = h.param(0).unwrap();
    let array = match param.value() {
        &Value::Array(ref a) => a,
        _ => &empty,
    };

    let separator = match h.param(1) {
        Some(x) =>
            match x.value() {
                &Value::String(ref s) => s,
                _ => ",",
            },
        None => ",",
    };

    let rendered = array
        .into_iter()
        .map(|x| format!("{:?}", x))
        .collect::<Vec<String>>()
        .join(separator);
    try!(rc.writer.write(rendered.into_bytes().as_ref()));
    Ok(())
}
