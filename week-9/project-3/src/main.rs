use std::io::Write;
use std::fs;
fn main() {
    let mut file = fs::File::create("Convicted-Ministers.csv").expect("create failed");
    file.write_all("EFCC Convicted Ministers\n".as_bytes()).expect("write failed");
    let headers = "Commissioner,Ministry,Zone\n";
    file.write_all(headers.as_bytes()).expect("write failed");
    let names = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone = vec!["South West","North East","South South","South West","South East"];
    for i in 0..names.len() {
        let row = format!("{},{},{}\n",names[i],ministry[i],zone[i]);
        file.write_all(row.as_bytes()).expect("write failed");
    } 
    println!("Data written to file");
}
