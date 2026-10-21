use::std::io::Write;
use std::fs;
fn main() {
   let categories = "Lager,Stout,Non-Alcohol\n";
   let row1 = "33Export,Legend,Maltina\n";
   let row2 = "Desperados,Turbo King,Amstel Malta\n";
   let row3 = "Goldberg,Williams,Malta Gold\n";
   let row4 = "Gulder,,Fayrouz\n";
   let row5 = "Heineken,,\n";
   let row6 = "Star,,\n";
   let mut file = fs::File::create("Ngb.csv").expect("create failed");
   file.write_all(categories.as_bytes()).expect("writing failed");
   println!("Data written to file");
   file.write_all(row1.as_bytes()).expect("writing failed");
   file.write_all(row2.as_bytes()).expect("writing failed");
   file.write_all(row3.as_bytes()).expect("writing failed");
   file.write_all(row4.as_bytes()).expect("writing failed");
   file.write_all(row5.as_bytes()).expect("writing failed");
   file.write_all(row6.as_bytes()).expect("writing failed");
}
