use std::io::Write;

fn main() {

    let name_of_commisioner = ["NAME OF COMMISIONER", "\n1) Aigbogun Alamba Daudu", "\n2) Murtala Ateez Bendu", "\n3) Okorocha Calistus Ogbona", "\n4) Adewale Jimoh Akanbi", "\n5) Osazuwa Faith Etieye"];
    let mut name_of_commisioner : String = name_of_commisioner.join("");
    let mut file = std::fs::File::create("Name Of Commisioner.txt").expect("create failed");
    file.write_all(name_of_commisioner.as_bytes()).expect("write failed");
    println!("\nData written to file.");

    
    let ministry = ["MINISTRY","\n1) Internal Affairs", "\n2) Justice", "\n3) Defense", "\n4) Power & Steel", "\n5) Petroleum"];
    let mut ministry : String = ministry.join("");
    let mut file = std::fs::File::create("Ministry.txt").expect("create failed");
    file.write_all(ministry.as_bytes()).expect("write failed");
    println!("\nData written to file.");


    let geopolitical_zone = ["GEOPOLITICAL ZONE",  "\n1) South West", "\n2) North East", "\n3) South South", "\n4) South West ", "\n5) South East"];
    let mut geopolitical_zone : String = geopolitical_zone.join("");
    let mut file = std::fs::File::create("Geopolitical Zone.txt").expect("create failed");
    file.write_all(geopolitical_zone.as_bytes()).expect("Failed to write");
    println!("\nData written to file.");

    let name_of_commisioner = ["NAME OF COMMISIONER", "\n1) Aigbogun Alamba Daudu", "\n2) Murtala Ateez Bendu", "\n3) Okorocha Calistus Ogbona", "\n4) Adewale Jimoh Akanbi", "\n5) Osazuwa Faith Etieye" ,"\n"];
    let ministry = ["MINISTRY","\n1) Internal Affairs", "\n2) Justice", "\n3) Defense", "\n4) Power & Steel", "\n5) Petroleum", "\n"];
    let geopolitical_zone = ["GEOPOLITICAL ZONE",  "\n1) South West", "\n2) North East", "\n3) South South", "\n4) South West ", "\n5) South East"];
    let efcc1 = name_of_commisioner; 
    let efcc2 = ministry;
    let efcc3 = geopolitical_zone;
    let mut efcc1 : String = efcc1.join("");
    let mut efcc2 : String = efcc2.join("");
    let mut efcc3 : String = efcc3.join("");
    let mut file = std::fs::File::create("EFCC CONVICTED MINISTERS.txt").expect("create failed");
    file.write_all(efcc1.as_bytes()).expect("write failed");
    file.write_all(efcc2.as_bytes()).expect("write failed");
    file.write_all(efcc3.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}
