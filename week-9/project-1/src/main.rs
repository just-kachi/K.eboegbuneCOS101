use std::io::Write;

fn main() {

    let lager = ["\tLAGER", "\n1) 33 Export", "\n2) Desperados", "\n3) Goldberg", "\n4) Gulder", "\n5) Heineken", "\n6) Star\n"];

    let stout = ["\t\tSTOUT","\tLegend", "\tTurbo King", "\tWilliams", "\t", "\t", "\t"];
    
    let non_alcoholic = ["\t\tNon-Alcoholic",  "\tAmstel Maltina", "\tMalta Gold", "\tFayrouz", "\t", "\t"];

    let mut file = std::fs::File::create("Nigerian Brewery Limited High Quality Categories of Drinks.txt").expect("create failed");

    for x in 0..7{
    file.write_all(lager[x].as_bytes()).expect("Failed to write");

    file.write_all(stout[x].as_bytes()).expect("Failed to write");

    file.write_all(non_alcoholic[x].as_bytes()).expect("Failed to write");
    }

    println!("\nData written to file.");
}
