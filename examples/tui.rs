extern crate bhyve_api;

use std::env;
use bhyve_api::system::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if "create" == &args[1] {
        cmd_create(&args[2]);
    } else if "destroy" == &args[1] {
        cmd_destroy(&args[2]);
    }
}

fn cmd_create(vm_name: &str) {
    let vmmctl = VMMSystem::new().expect("failed to create VMM system ioctl handle");
    match vmmctl.create_vm(vm_name) {
        Ok(_) => println!("Created a device at /dev/vmm/{}", vm_name),
        Err(e) => println!("Unable to create device at /dev/vmm/{}, with error: {}", vm_name, e),
    };
}

fn cmd_destroy(vm_name: &str) {
    let vmmctl = VMMSystem::new().expect("failed to create VMM system ioctl handle");
    match vmmctl.destroy_vm(vm_name) {
        Ok(_) => println!("Destroyed a device at /dev/vmm/{}", vm_name),
        Err(e) => println!("Unable to destroy device at /dev/vmm/{}, with error: {}", vm_name, e),
    };
}
