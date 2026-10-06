#[macro_use]
extern crate tokio;
use prusalink::settings::{PrusaIni, printers::PrinterModel};

#[tokio::main]
async fn main() {
    println!("hello world");
    let dir = "./PrusaSlicer-settings";
    let sections = PrusaIni::import_live(dir).await.unwrap();
    //println!("sections: {:?}", sections);
    for (vendor, data) in sections.into_iter() {
        println!("vendor: {}", vendor);
        for (version, sections) in data.into_iter() {
            println!("\t version: {}", version);
            for section in sections.sections.into_iter() {
                //println!("\t\t section kind: {}, id: {:?}", section.kind, section.id);
                if section.kind == "printer_model" {
                    let model = PrinterModel::from_ini_section(section).unwrap();
                    println!("model: {:?}", model);
                }
            }
        }
    }
}