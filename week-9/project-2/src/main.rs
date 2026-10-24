use std::io;
use std::io::Write;
use std::fs;
fn main() {
    
    let mut file = fs::File::create("PAU-SMIS.csv").expect("create failed");
    let headers = "Student Name, Matric No, Department, Level\n"; // creates the file and the headers
    file.write_all(headers.as_bytes()).expect("write failed");
    println!("How many students do you want to enter?");
   
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read");
    let qty = qty.trim().parse().expect("Invalid input");
    let mut vname:Vec<String> = Vec::new();
    let mut vmatric:Vec<String> = Vec::new();
    let mut vdept:Vec<String> = Vec::new();
    let mut vlevel:Vec<u16> = Vec::new();

    for i in 0..qty { //for loop lets you enter the student info as many times as you have selected
       
        let mut name = String::new();
        println!("Enter Student {} name", i + 1);
        io::stdin().read_line(&mut name).expect("Failed to read");
        let name = name.trim().to_string();
        vname.push(name); // pushes into the 
        
        let mut matricno = String::new();
        println!("Enter matric no {} ",i + 1);
        io::stdin().read_line(&mut matricno).expect("Failed to read");
        let matricno = matricno.trim().to_string();
        vmatric.push(matricno);
        
        let mut dept = String::new();
        println!("Enter the Department");
        io::stdin().read_line(&mut dept).expect("Failed to read");
        let dept = dept.trim().to_string();
        vdept.push(dept);
        
        let mut lvl = String::new();
        println!("Enter the level");
        io::stdin().read_line(&mut lvl).expect("Failed to read");
        let lvl:u16 = lvl.trim().parse().expect("Invalid input");
        vlevel.push(lvl); //pushes the info entered into the vector
       
        let row = format!("{},{},{},{}\n",vname[i],vmatric[i],vdept[i],vlevel[i]); /* format lets me get the data from
         the vectors and writes them as string. It allows me make the string my plugging things from my vectors into 
         placeholders*/
        file.write_all(row.as_bytes()).expect("write failed"); 
    }
    println!("Data written to file");
}
