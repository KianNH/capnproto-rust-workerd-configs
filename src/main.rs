use anyhow::Result;
use std::io;

mod workerd;

fn main() -> Result<()> {
    let mut message = capnp::message::Builder::new_default();
    let mut config = message.init_root::<workerd::config::Builder>();

    let services = config.reborrow().init_services(1);
    let mut service = services.get(0);
    service.set_name("main");

    let mut worker = service.init_worker();
    worker.set_compatibility_date("2022-09-16");

    let modules = worker.init_modules(1);
    let mut module = modules.get(0);
    module.set_name("worker");
    module.set_es_module(r#"export default { fetch() { return new Response("Hi") } }"#);

    let sockets = config.init_sockets(1);
    let mut socket = sockets.get(0);
    socket.set_name("http");
    socket.set_address("*:8080");

    let mut service_designator = socket.init_service();
    service_designator.set_name("main");

    capnp::serialize::write_message(&mut io::stdout(), &message)?;

    Ok(())
}
