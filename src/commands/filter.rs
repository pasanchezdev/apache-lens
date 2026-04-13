use anyhow::Result;

pub fn run(file: &str, ip: Option<&str>, code: Option<u16>, method: Option<&str>) -> Result<()> {
    println!("[mock] filter → {file} | ip={ip:?} code={code:?} method={method:?}");
    Ok(())
}
